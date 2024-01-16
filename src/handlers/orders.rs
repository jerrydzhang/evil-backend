use actix_web::{get, web, Responder, Result, HttpResponse, error, post};
use stripe::{Product, Client};

use crate::{models::{dbpool::PgPool, order::{Order, NewOrder}, product}, database::{orders::{db_get_all_orders, db_create_order, db_get_order_by_id, db_update_order, db_delete_order, db_get_expanded_orders, db_get_expanded_order_by_id}, carts::db_get_cart_items_by_user_id, users::db_user_stripe_to_user_id, products::db_get_product_by_id}};

#[get("")]
async fn get_orders(
    pool: web::Data<PgPool>,
) -> Result<impl Responder>{

    let orders = web::block(move || {
        let mut conn = pool.get().unwrap();
        db_get_all_orders(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    match orders {
        Some(orders) => Ok(HttpResponse::Ok().json(orders)),
        None => Ok(HttpResponse::NotFound().body("No orders found")),
    }
}

#[get("/id/{id}")]
async fn get_order_by_id(
    pool: web::Data<PgPool>,
    id: web::Path<String>,
) -> Result<impl Responder>{

    let order = web::block(move || {
        let mut conn = pool.get().unwrap();
        db_get_expanded_order_by_id(&mut conn, id.to_string())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(order))
}

#[get("/expand")]
async fn get_expanded_orders(
    pool: web::Data<PgPool>,
) -> Result<impl Responder>{
    let orders = web::block(move || {
        let mut conn = pool.get().unwrap();
        db_get_expanded_orders(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    match orders {
        Some(orders) => Ok(HttpResponse::Ok().json(orders)),
        None => Ok(HttpResponse::NotFound().body("No orders found")),
    }
}

#[post("/create")]
async fn create_order_handler(
    pool: web::Data<PgPool>,
    order: web::Json<NewOrder>,
) -> Result<impl Responder>{

    let order = web::block(move || {
        let mut conn = pool.get().unwrap();
        db_create_order(&mut conn, order.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(order))
}

#[post("/update/{id}")]
async fn update_order(
    pool: web::Data<PgPool>,
    id: web::Path<String>,
    order: web::Json<NewOrder>,
) -> Result<impl Responder>{

    let order = web::block(move || {
        let mut conn = pool.get().unwrap();
        db_update_order(&mut conn, id.to_string(), order.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(order))
}

#[derive(serde::Deserialize)]
struct OrderStatus {
    status: String,
}

#[post("/update/{id}/status")]
async fn update_order_status(
    pool: web::Data<PgPool>,
    id: web::Path<String>,
    status: web::Json<OrderStatus>,
) -> Result<impl Responder>{

    let order = web::block(move || {
        let mut conn = pool.get().unwrap();
        db_update_order(&mut conn, id.to_string(), NewOrder{
            status: Some(status.status.clone()),
            updated_at: Some(chrono::Local::now().naive_local()),
            ..Default::default()
        })
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(order))
}

#[post("/delete/{id}")]
async fn delete_order(
    pool: web::Data<PgPool>,
    id: web::Path<String>,
) -> Result<impl Responder>{

    let order = web::block(move || {
        let mut conn = pool.get().unwrap();
        db_delete_order(&mut conn, id.to_string())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(order))
}

pub(crate) async fn create_order(
    pool: web::Data<PgPool>,
    client: web::Data<Client>,
    user: String,
) -> Result<(), Box<dyn std::error::Error>> {

    let pool_clone = pool.clone();
    let user_clone = user.clone();

    web::block(move || {
        let mut conn = pool.get().unwrap();
        let user = db_user_stripe_to_user_id(&mut conn, user)?.unwrap();
        let cart_items = db_get_cart_items_by_user_id(&mut conn, user.clone().id)?.unwrap();

        let cart_items = cart_items.iter().map(|item| {
            let product_id = item.product_id.clone();
            let quantity = item.quantity.clone();
            (product_id, serde_json::Value::Number(serde_json::Number::from(quantity)))
        }).collect::<serde_json::Map<String, serde_json::Value>>();

        let order = NewOrder{
            user_id: Some(user.id),
            products: Some(serde_json::Value::Object(cart_items)),
            ..Default::default()
        };

        log::info!("new_order: {:?}", order);

        db_create_order(&mut conn, order)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    // let new_products = web::block(move || {
    //     let mut conn = pool_clone.get().unwrap();
    //     let user = db_user_stripe_to_user_id(&mut conn, user_clone).unwrap().unwrap();
    //     let cart_items = db_get_cart_items_by_user_id(&mut conn, user.clone().id).unwrap().unwrap();

    //     cart_items.iter().map(|item| {
    //         let id = item.product_id.clone();
    //         let new_quantity = item.quantity.clone();
    //         let product = db_get_product_by_id(&mut conn, id).unwrap();
    //         let new_product = product::NewProduct{
    //             id: Some(product.id.clone()),
    //             inventory: Some(product.inventory - new_quantity),
    //             ..Default::default()
    //         };
    //         new_product
    //     }).collect::<Vec<product::NewProduct>>()
    // })
    // .await?;

    // for product in new_products {
    //     let stripe_product_id = product.id.clone().unwrap();
    //     let stripe_inventory = product.inventory.clone().unwrap();

    //     let product = web::block(move || {
    //         let mut conn = pool_clone.get().unwrap();
    //         db_get_product_by_id(&mut conn, stripe_product_id.clone())
    //     })
    //     .await?
    //     .map_err(error::ErrorInternalServerError)?;

    //     stripe::Product::update(&client, Product::, stripe::UpdateProduct {
    //         metadata: Some(std::collections::HashMap::from([(
    //             String::from("inventory"),
    //             String::from((stripe_inventory).to_string().as_str()),
    //         )])),
    //         ..Default::default()
    //     }).await.unwrap();
    // }

    Ok(())
}