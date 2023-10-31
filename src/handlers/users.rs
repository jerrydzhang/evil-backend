use std::collections::HashSet;

use actix_web::{post, Result, web, Responder, HttpResponse, error, delete, get, put};

use crate::{models::{dbpool::PgPool, user::{User, SubmitRoles, UserId}}, database::users::{db_create_user, db_delete_user, db_update_user, db_get_user}, extractors::claims::Claims};


#[post("/add")]
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

#[put("/update")]
async fn update_user(
    pool: web::Data<PgPool>,
    submit_roles: web::Json<SubmitRoles>,
) -> Result<impl Responder> {
    let SubmitRoles { user_id, stripe_customer_id, roles } = submit_roles.into_inner();
    let user = web::block(move || {

        let mut conn = pool.get().unwrap();

        db_update_user(&mut conn, user_id, stripe_customer_id, roles)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/delete")]
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

#[get("")]
async fn get_user(
    pool: web::Data<PgPool>,
    user_id: web::Query<UserId>,
    claims: Claims,
) -> Result<impl Responder> {
    // verify the user is getting their own information
    // if the user is an admin bypass this check
    if !claims.validate_roles(&HashSet::from(["admin".to_string()])) {
        if claims.sub != user_id.id {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    }

    let user = web::block(move || {

        let mut conn = pool.get().unwrap();

        db_get_user(&mut conn, user_id.id.clone())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/index")]
async fn index(
    claims: Claims,
) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(claims.sub))
}