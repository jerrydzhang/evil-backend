use std::collections::HashSet;

use actix_web::{get, Responder, Result, web, error, HttpResponse, put};
use bigdecimal::{BigDecimal, FromPrimitive};


use crate::database::products::{
    db_get_all_products, 
    db_get_product_by_id, 
    db_get_multiple_products_by_id,
    db_get_products_by_category, 
    db_create_product, 
    db_update_product, 
    db_delete_product,
};
use crate::extractors::claims::Claims;
use crate::models::dbpool::PgPool;
use crate::models::product::{ProductIds, self, UpdatePayload};

// returns all products in the database
#[get("")]
async fn get_all_products(
    pool: web::Data<PgPool>,
) -> Result<impl Responder> {
    // block the current thread until the async operation is complete
    let products = web::block(move || {

        // get a connection from the pool
        let mut conn = pool.get().unwrap();
        
        // pass the connection to the database function
        db_get_all_products(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(products))
}

// returns multiple products by id though a query string
#[get("/by-id")]
async fn get_multiple_products_by_id(
    pool: web::Data<PgPool>,
    info: web::Query<ProductIds>,
) -> Result<impl Responder> {
    // parse the query string into a vector of Strings
    let ids = info.ids.split(",").map(|id| id.parse::<String>().unwrap()).collect::<Vec<String>>();

    let products = web::block(move || {
        let mut conn = pool.get().unwrap();
        db_get_multiple_products_by_id(&mut conn, ids)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(products))
}

// returns a single product by id
#[get("/{id}")]
async fn get_product_by_id(
    pool: web::Data<PgPool>,
    product_id: web::Path<String>,
) -> Result<impl Responder> {
    // unwrap the product_id from the web::Path<i32> type
    let product_id = product_id.into_inner();

    let product = web::block(move || {
        let mut conn = pool.get().unwrap();
        db_get_product_by_id(&mut conn, product_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(product))
}

#[get("/category/{category}")]
async fn get_products_by_category(
    pool: web::Data<PgPool>,
    category: web::Path<String>,
) -> Result<impl Responder> {
    // unwrap the category from the web::Path<String> type
    let category = category.into_inner();

    let products = web::block(move || {
        let mut conn = pool.get().unwrap();
        db_get_products_by_category(&mut conn, category.parse::<String>().unwrap())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(products))
}

#[put("/update/{id}")]
async fn update_product_inventory(
    pool: web::Data<PgPool>,
    client: web::Data<stripe::Client>,
    product_id: web::Path<String>,
    update_payload: web::Json<UpdatePayload>,
    claims: Claims
) -> Result<impl Responder> {
    if !claims.validate_roles(&HashSet::from(["admin".to_string()])) {
        return Ok(HttpResponse::Unauthorized().finish());
    };

    let products = stripe::Product::list(&client, &stripe::ListProducts::default()).await.unwrap();

    let db_product_id = product_id.clone();
    let db_product = web::block(move || {
        let mut conn = pool.get().unwrap();
        db_get_product_by_id(&mut conn, db_product_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let stripe_product_id = product_id.clone();
    let stripe_inventory = update_payload.inventory.clone();
    let is_active = update_payload.is_active.clone();

    let product = products.data.into_iter().find(|product| product.id.as_str() == stripe_product_id).unwrap();
    
    stripe::Product::update(&client, &product.id, stripe::UpdateProduct {
        active: Some(is_active),
        metadata: Some(std::collections::HashMap::from([(
            String::from("inventory"),
            String::from((stripe_inventory + db_product.inventory.unwrap_or(0)).to_string().as_str()),
            )])),
        ..Default::default()
    }).await.unwrap();

    Ok(HttpResponse::Ok().json(product))
}

pub(crate) async fn create_product(
    pool: web::Data<PgPool>,
    stripe_product: stripe::Product,
) -> Result<(), Box<dyn std::error::Error>> {
    let product = product::Product::new(stripe_product);

    web::block(move || {
        let mut conn = pool.get().unwrap();
        db_create_product(&mut conn, product)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(())
}

pub(crate) async fn update_product(
    pool: web::Data<PgPool>,
    stripe_product: stripe::Product,
) -> Result<(), Box<dyn std::error::Error>> {
    let product = product::NewProduct::new(stripe_product);
    web::block(move|| {
        let mut conn = pool.get().unwrap();
        db_update_product(&mut conn, product)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(())
}

pub(crate) async fn delete_product(
    pool: web::Data<PgPool>,
    stripe_product: stripe::Product,
) -> Result<(), Box<dyn std::error::Error>> {
    let product = product::Product::new(stripe_product);
    web::block(move|| {
        let mut conn = pool.get().unwrap();
        db_delete_product(&mut conn, product.id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(())
}

pub(crate) async fn change_price(
    pool: web::Data<PgPool>,
    stripe_price: stripe::Price,
) -> Result<(), Box<dyn std::error::Error>> {
    let price:BigDecimal = BigDecimal::from_i64(stripe_price.unit_amount.unwrap()).unwrap()/100;
    let price_id = stripe_price.id.as_str().to_string();
    let product_id = stripe_price.product.unwrap().id().to_string();

    let new_product = product::NewProduct{
        id: Some(product_id.clone()),
        price: Some(price.clone()),
        price_id: Some(price_id.clone()),
        ..Default::default()
    };

    web::block(move || {
        let mut conn = pool.get().unwrap();
        db_update_product(&mut conn, new_product)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(())
}