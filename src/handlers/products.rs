use actix_web::{get, Responder, Result, web, error, HttpResponse};
use bigdecimal::{BigDecimal, FromPrimitive};


use crate::database::products::{
    db_get_all_products, 
    db_get_product_by_id, 
    db_get_multiple_products_by_id,
    db_get_products_by_catagory, 
    db_create_product, 
    db_update_product, 
    db_update_price, db_delete_product,
};
use crate::models::dbpool::PgPool;
use crate::models::product::{ProductIds, self};

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

#[get("/by-catagory/{catagory}")]
async fn get_products_by_catagory(
    pool: web::Data<PgPool>,
    catagory: web::Path<String>,
) -> Result<impl Responder> {
    // unwrap the catagory from the web::Path<String> type
    let catagory = catagory.into_inner();

    let products = web::block(move || {

        let mut conn = pool.get().unwrap();

        db_get_products_by_catagory(&mut conn, catagory.parse::<String>().unwrap())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(products))
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
    let product = product::Product::new(stripe_product);
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
    let price = BigDecimal::from_i64(stripe_price.unit_amount.unwrap()).unwrap()/100;
    let price_id = stripe_price.id.as_str().to_string();
    let product_id = stripe_price.product.unwrap().id().to_string();

    web::block(move || {
        let mut conn = pool.get().unwrap();
        db_update_price(&mut conn, product_id, price, price_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(())
}


// #[post("/create")]
// async fn create_product(
//     pool: web::Data<PgPool>,
//     stripe_client: web::Data<Client>,
//     product: web::Json<NewProduct>,
//     user: Identity,
// ) -> Result<impl Responder> {
//     if !verify_identity(pool.clone(), user, Vec::from(["admin"])) {return Ok(HttpResponse::Unauthorized().finish());};

//     let stripe_constructor = product.clone();

//     let stripe_product = stripe_create_product(&stripe_client, stripe_constructor)
//         .await
//         .map_err(error::ErrorInternalServerError)?;

//     let product = DbProduct::new(
//         stripe_product.id.as_str().to_string(),
//         product.into_inner(),
//     );
    
//     let product = web::block(move || {

//         let mut conn = pool.get().unwrap();

//         db_create_product(&mut conn, product)
//     })
//     .await?
//     .map_err(error::ErrorInternalServerError)?;

//     Ok(HttpResponse::Ok().json(product))
// }

// #[put("/update/{id}")]
// async fn update_product(
//     pool: web::Data<PgPool>,
//     product_id: web::Path<String>,
//     product: web::Json<NewProduct>,
//     user: Identity,
// ) -> Result<impl Responder> {
//     if !verify_identity(pool.clone(), user, Vec::from(["admin"])) {return Ok(HttpResponse::Unauthorized().finish());};
    
//     let product = web::block(move || {

//         let mut conn = pool.get().unwrap();

//         db_update_product(&mut conn, product_id.into_inner(), product.into_inner())
//     })
//     .await?
//     .map_err(error::ErrorInternalServerError)?;

//     Ok(HttpResponse::Ok().json(product))
// }

// #[delete("/delete/{id}")]
// async fn delete_product(
//     pool: web::Data<PgPool>,
//     product_id: web::Path<String>,
//     user: Identity,
// ) -> Result<impl Responder> {
//     if !verify_identity(pool.clone(), user, Vec::from(["admin"])) {return Ok(HttpResponse::Unauthorized().finish());};
//     let deleted_product = web::block(move || {

//         let mut conn = pool.get().unwrap();

//         db_delete_product(&mut conn, product_id.into_inner())
//     })
//     .await?
//     .map_err(error::ErrorInternalServerError)?;

//     Ok(HttpResponse::Ok().json(deleted_product))
// }