use actix_web::{get, web, Responder, Result, HttpResponse, error, post};
use stripe::{Client, Product};

use crate::{models::{dbpool::PgPool, order::{NewOrder}}, database::{orders::{db_create_order, db_delete_order, db_get_all_orders, db_get_expanded_order_by_id, db_get_expanded_orders, db_get_expanded_orders_by_user_id, db_update_order}, carts::db_get_cart_items_by_user_id, users::db_user_stripe_to_user_id}};

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

#[get("/expand/user/{id}")]
async fn get_expanded_orders_by_user_id(
    pool: web::Data<PgPool>,
    id: web::Path<String>,
) -> Result<impl Responder>{
    let orders = web::block(move || {
        let mut conn = pool.get().unwrap();
        db_get_expanded_orders_by_user_id(&mut conn, id.to_string())
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
    name: String,
    address: String,
) -> Result<(), Box<dyn std::error::Error>> {

    let order = web::block(move || {
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
            name: Some(name),
            address: Some(address),
            ..Default::default()
        };

        log::info!("new_order: {:?}", order);

        db_create_order(&mut conn, order)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let stripe_products = stripe::Product::list(&client, &stripe::ListProducts{
        active: Some(true),
        ids: Some(order.products.as_object().unwrap().keys().map(|id| id.clone()).collect()),
        limit: Some(100),
        ..Default::default()
    })
    .await
    .map_err(error::ErrorInternalServerError)?;

    let needs_updateing = order.products.as_object().unwrap().iter().map(|(product_id, quantity)|  {
        let product_id = product_id.clone();
        let quantity = quantity.as_u64().unwrap() as i32;

        let stripe_product = stripe_products.clone().data.into_iter().find(|product| product.id.as_str() == product_id).unwrap();

        // log::info!("stripe_product: {:?}", stripe_product);
        // log::info!("inventory before: {:?}", stripe_product.clone().metadata.unwrap().get("inventory").unwrap().parse::<i32>().unwrap());
        // log::info!("quantity: {:?}", quantity);
        // log::info!("inventory after: {:?}", stripe_product.clone().metadata.unwrap().get("inventory").unwrap().parse::<i32>().unwrap() - quantity);

        (stripe_product, quantity)
    }).collect::<Vec<(Product, i32)>>();

    for (product, quantity) in needs_updateing {
        stripe::Product::update(&client, &product.id, stripe::UpdateProduct {
            metadata: Some(std::collections::HashMap::from([(
                String::from("inventory"),
                String::from((product.metadata.unwrap().get("inventory").unwrap().parse::<i32>().unwrap() - quantity).to_string().as_str()),
                )])),
            ..Default::default()
        })
        .await
        .map_err(error::ErrorInternalServerError)?;
    }

    Ok(())
}