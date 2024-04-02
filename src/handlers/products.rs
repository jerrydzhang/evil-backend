use std::collections::HashSet;

use actix_web::{delete, error, get, post, put, web, HttpResponse, Responder, Result};
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use stripe::Object;

use crate::database::products::{
    db_create_product, db_delete_product, db_get_active_products,
    db_get_active_products_by_category, db_get_all_products, db_get_categories,
    db_get_multiple_products_by_id, db_get_product_by_id, db_get_product_by_name,
    db_get_products_by_category, db_update_product,
};
use crate::extractors::claims::Claims;
use crate::models::dbpool::PgPool;
use crate::models::product::{self, NewProductPayload, ProductIds, UpdatePayload};

// returns all products in the database
#[get("")]
async fn get_all_products(pool: web::Data<PgPool>) -> Result<impl Responder> {
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

#[get("/active")]
async fn get_active_products(pool: web::Data<PgPool>) -> Result<impl Responder> {
    let products = web::block(move || {
        let mut conn = pool.get().unwrap();
        db_get_active_products(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(products))
}

#[get("/name/{name}")]
async fn get_product_by_name(
    pool: web::Data<PgPool>,
    name: web::Path<String>,
) -> Result<impl Responder> {
    // unwrap the name from the web::Path<String> type
    let name = name.into_inner().replace("-", " ");

    let product = web::block(move || {
        let mut conn = pool.get().unwrap();
        db_get_product_by_name(&mut conn, name)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(product))
}

// returns multiple products by id though a query string
#[get("/by-id")]
async fn get_multiple_products_by_id(
    pool: web::Data<PgPool>,
    info: web::Query<ProductIds>,
) -> Result<impl Responder> {
    // parse the query string into a vector of Strings
    let ids = info
        .ids
        .split(",")
        .map(|id| id.parse::<String>().unwrap())
        .collect::<Vec<String>>();

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

#[get("/category")]
async fn get_all_categories(pool: web::Data<PgPool>) -> Result<impl Responder> {
    let categories = web::block(move || {
        let mut conn = pool.get().unwrap();
        db_get_categories(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(categories))
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

#[get("/active/category/{category}")]
async fn get_active_products_by_category(
    pool: web::Data<PgPool>,
    category: web::Path<String>,
) -> Result<impl Responder> {
    let category = category.into_inner();

    let products = web::block(move || {
        let mut conn = pool.get().unwrap();
        db_get_active_products_by_category(&mut conn, category.parse::<String>().unwrap())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(products))
}

#[post("/create")]
async fn create_product(
    client: web::Data<stripe::Client>,
    new_product_payload: web::Json<NewProductPayload>,
    // claims: Claims,
) -> Result<impl Responder> {
    // if !claims.validate_roles(&HashSet::from(["admin".to_string()])) {
    //     return Ok(HttpResponse::Unauthorized().finish());
    // };

    let product_name = new_product_payload.name.clone();
    let mut new_product = stripe::CreateProduct::new(&product_name);

    new_product.active = Some(false);
    let description = match new_product_payload.description.clone() {
        Some(description) => Some(description),
        None => None,
    };
    new_product.description = description.as_deref();
    new_product.images = Some(vec![new_product_payload.image.to_string()]);
    new_product.metadata = Some(std::collections::HashMap::from([
        (
            String::from("inventory"),
            String::from(new_product_payload.inventory.to_string().as_str()),
        ),
        (
            String::from("variant_id"),
            String::from(new_product_payload.variant_id.to_string().as_str()),
        ),
        (
            String::from("category"),
            String::from(new_product_payload.category.clone()),
        ),
    ]));

    log::info!("new_product: {:?}", new_product);

    let stripe_product = stripe::Product::create(&client, new_product).await.unwrap();
    let stripe_product_id = stripe_product.id().to_string();

    let mut stripe_price = stripe::CreatePrice::new(stripe::Currency::USD);
    stripe_price.unit_amount = (new_product_payload.price.to_f64().unwrap() * 100.0).to_i64();
    stripe_price.product = Some(stripe::IdOrCreate::Id(&stripe_product_id));

    stripe::Price::create(&client, stripe_price).await.unwrap();

    // stripe::Product::update(client, stripe_product.id(), stripe::UpdateProduct {
    //     default_price:
    //     ..Default::default()
    // }).await.unwrap();
    return Ok(HttpResponse::Ok().finish());
}

#[put("/update/{id}")]
async fn update_product(
    pool: web::Data<PgPool>,
    client: web::Data<stripe::Client>,
    product_id: web::Path<String>,
    update_payload: web::Json<UpdatePayload>,
    claims: Claims,
) -> Result<impl Responder> {
    if !claims.validate_roles(&HashSet::from(["admin".to_string()])) {
        return Ok(HttpResponse::Unauthorized().finish());
    };

    let products = stripe::Product::list(
        &client,
        &stripe::ListProducts {
            ids: Some(vec![product_id.clone()]),
            ..Default::default()
        },
    )
    .await
    .unwrap();

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

    let product = products
        .data
        .into_iter()
        .find(|product| product.id.as_str() == stripe_product_id)
        .unwrap();

    stripe::Product::update(
        &client,
        &product.id,
        stripe::UpdateProduct {
            name: Some(&update_payload.name.clone()),
            description: Some(update_payload.description.clone()),
            active: Some(is_active),
            images: Some(update_payload.images.clone()),
            metadata: Some(std::collections::HashMap::from([(
                String::from("inventory"),
                String::from(
                    (stripe_inventory + db_product.inventory.unwrap_or(0))
                        .to_string()
                        .as_str(),
                ),
            )])),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    Ok(HttpResponse::Ok().json(product))
}

#[delete("/delete/{id}")]
async fn delete_product(
    client: web::Data<stripe::Client>,
    product_id: web::Path<String>,
    claims: Claims,
) -> Result<impl Responder> {
    if !claims.validate_roles(&HashSet::from(["admin".to_string()])) {
        return Ok(HttpResponse::Unauthorized().finish());
    };

    let products = stripe::Product::list(
        &client,
        &stripe::ListProducts {
            ids: Some(vec![product_id.clone()]),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let product_id = product_id.into_inner();
    let product = products
        .data
        .into_iter()
        .find(|product| product.id.as_str() == product_id)
        .unwrap();

    stripe::Product::delete(&client, &product.id).await.unwrap();

    Ok(HttpResponse::Ok().finish())
}

pub(crate) async fn wh_create_product(
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

pub(crate) async fn wh_update_product(
    pool: web::Data<PgPool>,
    stripe_product: stripe::Product,
) -> Result<(), Box<dyn std::error::Error>> {
    let product = product::NewProduct::new(stripe_product);
    web::block(move || {
        let mut conn = pool.get().unwrap();
        db_update_product(&mut conn, product)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(())
}

pub(crate) async fn wh_delete_product(
    pool: web::Data<PgPool>,
    stripe_product: stripe::Product,
) -> Result<(), Box<dyn std::error::Error>> {
    let product = product::Product::new(stripe_product);
    web::block(move || {
        let mut conn = pool.get().unwrap();
        db_delete_product(&mut conn, product.id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(())
}

pub(crate) async fn wh_change_price(
    pool: web::Data<PgPool>,
    stripe_price: stripe::Price,
) -> Result<(), Box<dyn std::error::Error>> {
    let price: BigDecimal = BigDecimal::from_i64(stripe_price.unit_amount.unwrap()).unwrap() / 100;
    let price_id = stripe_price.id.as_str().to_string();
    let product_id = stripe_price.product.unwrap().id().to_string();

    let new_product = product::NewProduct {
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
