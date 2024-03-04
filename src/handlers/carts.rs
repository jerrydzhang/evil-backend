use actix_web::{get, web, HttpResponse, Responder, Result, error, post, put};

use crate::extractors::claims::Claims;
use crate::models::cart::{CartSubmit, NewCartItem};
use crate::models::dbpool::PgPool;
use crate::database::carts::{db_get_cart_items_by_user_id, db_update_cart_item, db_create_cart_item, db_delete_cart_item, db_update_cart_item_from_cart};


#[get("")]
pub(crate) async fn get_cart_items(
    pool: web::Data<PgPool>,
    claims: Claims,
) -> Result<impl Responder> {
    let cart_items = web::block(move || {
        let mut conn = pool.get().unwrap();

        db_get_cart_items_by_user_id(&mut conn, claims.sub)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(cart_items))
}

#[post("/add")]
pub(crate) async fn add_to_cart(
    pool: web::Data<PgPool>,
    new_cart: web::Json<NewCartItem>,
    _claims: Claims,
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

#[post("/update")]
pub(crate) async fn update_cart_item(
    pool: web::Data<PgPool>,
    new_cart: web::Json<NewCartItem>,
    _claims: Claims,
) ->  Result<impl Responder> {
    let cart_items = web::block(move || {
        let mut conn = pool.get().unwrap();
        
        db_update_cart_item_from_cart(&mut conn, new_cart.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(cart_items))
}

// Determine which items to delete and which to update based on the current cart and the new cart
#[put("/update_cart")]
pub(crate) async fn update_cart(
    pool: web::Data<PgPool>,
    cart_submit: web::Json<CartSubmit>,
    _claims: Claims,
) -> Result<impl Responder> {
    let cart = cart_submit.cart.clone();
    let user_id = cart_submit.user_id.clone();

    let cart_items = web::block(move || {
        let mut conn = pool.get().unwrap();

        let current_cart = db_get_cart_items_by_user_id(&mut conn, user_id.clone())?;

        // Check if there is a current cart
        match current_cart {
            // If there is a current cart, update the cart with the submitted cart
            Some(current_cart) => {
                // Iterate through the current cart
                // If the current cart item is in the submitted cart, update the quantity
                // If the current cart item is not in the submitted cart, delete the item
                for cart_item in current_cart.clone() {
                    if cart.contains_key(&cart_item.product_id) {
                        if cart_item.quantity == cart.get(&cart_item.product_id).unwrap().clone() {
                            continue;
                        }

                        let new_quantity = cart.get(&cart_item.product_id).unwrap();

                        db_update_cart_item(&mut conn, user_id.clone(), cart_item.product_id.clone(), *new_quantity)?;
                    } else {
                        db_delete_cart_item(&mut conn, user_id.clone(), cart_item.product_id.clone())?;
                    }
                }

                // Iterate through the submitted cart
                // If the submitted cart item is not in the current cart, create a new cart item
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
            // If there is no current cart, create a new cart with the submitted cart
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