use actix_web::{get, web, HttpResponse, Responder, Result, error, post};

use crate::models::carts::{CartSubmit, NewCartItem};
use crate::models::dbpool::PgPool;
use crate::database::carts::{db_get_cart_items_by_user_id, db_update_cart_item, db_create_cart_item, db_delete_cart_item, db_update_cart_item_from_cart};

#[get("/cart/{id}")]
pub(crate) async fn get_cart_items(
    pool: web::Data<PgPool>,
    user_id: web::Path<String>,
) -> Result<impl Responder> {
    let cart_items = web::block(move || {
        let mut conn = pool.get().unwrap();

        db_get_cart_items_by_user_id(&mut conn, user_id.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(cart_items))
}

#[post("/add_to_cart")]
pub(crate) async fn add_to_cart(
    pool: web::Data<PgPool>,
    new_cart: web::Json<NewCartItem>,
) ->  Result<impl Responder> {
    let cart_item = new_cart.into_inner();

    let cart_items = web::block(move || {
        let mut conn = pool.get().unwrap();

        db_create_cart_item(&mut conn, cart_item)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(cart_items))
}

#[post("/update_cart_item")]
pub(crate) async fn update_cart_item(
    pool: web::Data<PgPool>,
    new_cart: web::Json<NewCartItem>,
) ->  Result<impl Responder> {

    let cart_items = web::block(move || {
        let mut conn = pool.get().unwrap();

        db_update_cart_item_from_cart(&mut conn, new_cart.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(cart_items))
}

#[post("/update_cart")]
pub(crate) async fn update_cart(
    pool: web::Data<PgPool>,
    cart_submit: web::Json<CartSubmit>,
) -> Result<impl Responder> {
    let cart = cart_submit.cart.clone();
    let user_id = cart_submit.user_id.clone();

    let cart_items = web::block(move || {
        let mut conn = pool.get().unwrap();

        let current_cart = db_get_cart_items_by_user_id(&mut conn, user_id.clone())?;

        match current_cart {
            Some(current_cart) => {
                for cart_item in current_cart.clone() {
                    if cart.contains_key(&cart_item.product_id) {
                        if cart_item.quantity == cart.get(&cart_item.product_id).unwrap().clone() {
                            continue;
                        }

                        let new_quantity = cart.get(&cart_item.product_id).unwrap();

                        db_update_cart_item(&mut conn, cart_item.id, *new_quantity)?;
                    } else {
                        db_delete_cart_item(&mut conn, cart_item.id)?;
                    }
                }

                for cart_item in cart {
                    if current_cart.iter().any(|item| item.product_id == cart_item.0) {
                        continue;
                    }

                    let new_cart_item = NewCartItem {
                        user_id: user_id.clone(),
                        product_id: cart_item.0,
                        quantity: cart_item.1,
                    };

                    db_create_cart_item(&mut conn, new_cart_item)?;
                }
            }
            None => {
                for cart_item in cart {
                    let new_cart_item = NewCartItem {
                        user_id: user_id.clone(),
                        product_id: cart_item.0,
                        quantity: cart_item.1,
                    };

                    db_create_cart_item(&mut conn, new_cart_item)?;
                }
            }
        }

        db_get_cart_items_by_user_id(&mut conn, cart_submit.user_id.clone())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(cart_items))
}