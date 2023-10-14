use actix_identity::Identity;
use actix_web::{get, Responder, Result, web, error, HttpResponse, post, delete, put};

use crate::database::products::{
    db_get_all_products, 
    db_get_product_by_id, 
    db_get_multiple_products_by_id,
    db_get_products_by_catagory, 
    db_create_product, 
    db_update_product, 
    db_delete_product,
};
use crate::models::dbpool::PgPool;
use crate::models::product::{ProductIds, NewProduct};
use crate::utils::auth::verify_identity;

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
    // parse the query string into a vector of i32
    let ids = info.ids.split(",").map(|id| id.parse::<i32>().unwrap()).collect::<Vec<i32>>();

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
    product_id: web::Path<i32>,
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

#[get("/by-catagory/{catagory}")]
async fn get_products_by_catagory(
    pool: web::Data<PgPool>,
    catagory: web::Path<String>,
) -> Result<impl Responder> {
    // unwrap the catagory from the web::Path<String> type
    let catagory = catagory.into_inner();

    let products = web::block(move || {

        let mut conn = pool.get().unwrap();

        db_get_products_by_catagory(&mut conn, catagory.parse::<i32>().unwrap())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(products))
}

#[post("/create")]
async fn create_product(
    pool: web::Data<PgPool>,
    product: web::Json<NewProduct>,
    user: Identity,
) -> Result<impl Responder> {
    if !verify_identity(pool.clone(), user, Vec::from(["admin"])) {return Ok(HttpResponse::Unauthorized().finish());};
    let product = web::block(move || {

        let mut conn = pool.get().unwrap();

        db_create_product(&mut conn, product.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(product))
}

#[put("/update/{id}")]
async fn update_product(
    pool: web::Data<PgPool>,
    product_id: web::Path<i32>,
    product: web::Json<NewProduct>,
    user: Identity,
) -> Result<impl Responder> {
    if !verify_identity(pool.clone(), user, Vec::from(["admin"])) {return Ok(HttpResponse::Unauthorized().finish());};
    
    let product = web::block(move || {

        let mut conn = pool.get().unwrap();

        db_update_product(&mut conn, product_id.into_inner(), product.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(product))
}

#[delete("/delete/{id}")]
async fn delete_product(
    pool: web::Data<PgPool>,
    product_id: web::Path<i32>,
    user: Identity,
) -> Result<impl Responder> {
    if !verify_identity(pool.clone(), user, Vec::from(["admin"])) {return Ok(HttpResponse::Unauthorized().finish());};
    let deleted_product = web::block(move || {

        let mut conn = pool.get().unwrap();

        db_delete_product(&mut conn, product_id.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(deleted_product))
}