use actix_web::{post, Result, web, Responder, HttpResponse, error, delete};

use crate::{models::{dbpool::PgPool, user::User}, database::users::{db_create_user, db_delete_user}};


#[post("/add_user")]
async fn create_user(
    pool: web::Data<PgPool>,
    new_user: web::Json<User>,
) -> Result<impl Responder> {
    let user = web::block(move || {

        let mut conn = pool.get().unwrap();

        db_create_user(&mut conn, new_user.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/delete_user")]
async fn delete_user(
    pool: web::Data<PgPool>,
    user_id: web::Json<String>,
) -> Result<impl Responder> {
    let deleted_user = web::block(move || {

        let mut conn = pool.get().unwrap();

        db_delete_user(&mut conn, user_id.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(deleted_user))
}