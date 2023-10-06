use actix_web::{get, web, HttpResponse, Result, error, Responder, post, delete};

use crate::{models::{dbpool::PgPool, catagory::NewCatagory}, database::catagories::{db_get_all_catagories, db_create_catagory, db_delete_catagory}};

#[get("/catagories")]
async fn get_all_catagories(
    pool: web::Data<PgPool>,
) -> Result<impl Responder> {
    let catagories = web::block(move || {

        let mut conn = pool.get().unwrap();

        db_get_all_catagories(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(catagories))
}

#[post("/create_catagory")]
async fn create_catagory(
    pool: web::Data<PgPool>,
    catagory: web::Json<NewCatagory>,
) -> Result<impl Responder> {
    let catagory = web::block(move || {

        let mut conn = pool.get().unwrap();

        db_create_catagory(&mut conn, catagory.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(catagory))
}

#[delete("/delete_catagory")]
async fn delete_catagory(
    pool: web::Data<PgPool>,
    catagory_id: web::Json<i32>,
) -> Result<impl Responder> {
    let deleted_catagory = web::block(move || {

        let mut conn = pool.get().unwrap();

        db_delete_catagory(&mut conn, catagory_id.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(deleted_catagory))
}