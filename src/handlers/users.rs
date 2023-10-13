use actix_identity::Identity;
use actix_web::{post, Result, web, Responder, HttpResponse, error, delete, HttpRequest, HttpMessage, get, put};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{models::{dbpool::PgPool, user::{User, SubmitRoles, UserId}}, database::users::{db_create_user, db_delete_user, db_update_roles, db_get_user}, utils::{jwt::verify_jwt, auth::verify_identity}, schema::carts::id};


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

#[put("/update_roles")]
async fn update_roles(
    pool: web::Data<PgPool>,
    submit_roles: web::Json<SubmitRoles>,
) -> Result<impl Responder> {
    let SubmitRoles { user_id, roles } = submit_roles.into_inner();
    let user = web::block(move || {

        let mut conn = pool.get().unwrap();

        db_update_roles(&mut conn, user_id, roles)
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

#[get("/get_user")]
async fn get_user(
    pool: web::Data<PgPool>,
    user_id: web::Query<UserId>,
    identity: Identity,
) -> Result<impl Responder> {
    // verify the user is getting their own information
    // if the user is an admin bypass this check
    if identity.id().unwrap() != user_id.id {
        // admin check
        if !verify_identity(pool.clone(), identity, Vec::from(["admin"])) {return Ok(HttpResponse::Unauthorized().finish());};
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
    user: Option<Identity>,
) -> Result<impl Responder> {
    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user.id().unwrap()))
    } else {
        Ok(HttpResponse::Ok().json("No user"))
    }
}

#[post("/login")]
async fn login(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    user: web::Json<User>,
    credientials: BearerAuth,
) -> Result<impl Responder> {
    // verify the token is valid
    let token: jsonwebtoken::TokenData<std::collections::HashMap<String, serde_json::Value>> = match verify_jwt(credientials.token()).await {
        Ok(token) => token,
        Err(_) => return Ok(HttpResponse::Unauthorized().json("Invalid token")),
    };

    // verify the token is for the user
    let sub = token.claims.get("sub").unwrap();

    if sub != &user.id {
        return Ok(HttpResponse::Unauthorized().json("Invalid token"));
    }

    // clone the email so we can use it later
    let email = user.email.clone();

    // get the user associated with the sub
    let user = web::block(move || {

        let mut conn = pool.get().unwrap();

        db_get_user(&mut conn, user.id.clone())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    // check if user is in database
    let user = match user {
        Some(user) => user,
        None => { return Ok(HttpResponse::Unauthorized().json("Invalid token"));}
    };

    // verify the token is for the user in the database
    if email != user.email {
        return Ok(HttpResponse::Unauthorized().json("Invalid token"));
    }

    // login the user
    Identity::login(&req.extensions() ,user.id.clone())?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/logout")]
async fn logout(
    user: Identity,
) -> Result<impl Responder> {
    user.logout();

    Ok(HttpResponse::Ok().json("Logged out"))
}