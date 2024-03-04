use diesel::result::Error;
use diesel::{BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::models::cart::{CartItem, NewCartItem};
use crate::schema::carts::dsl::*;

pub(crate) fn db_get_cart_items_by_user_id (
    conn: &mut PgConnection,
    input_id: String,
) -> Result<Option<Vec<CartItem>>, Error> {
    let cart_items = carts
        .filter(user_id.eq(input_id))
        .load::<CartItem>(conn)?;

    Ok(Some(cart_items))
}

pub(crate) fn db_create_cart_item (
    conn: &mut PgConnection,
    new_cart_item: NewCartItem,
) -> Result<CartItem, Error> {
    let cart_item = diesel::insert_into(carts)
        .values(&new_cart_item)
        .get_result::<CartItem>(conn)?;

    Ok(cart_item)
}

pub(crate) fn db_update_cart_item (
    conn: &mut PgConnection,
    input_user_id: String,
    input_product_id: String,
    new_quantity: i32,
) -> Result<CartItem, Error> {
    let cart_item = diesel::update(carts.find((input_user_id, input_product_id)))
        .set(quantity.eq(new_quantity))
        .get_result::<CartItem>(conn)?;

    Ok(cart_item)
}

pub(crate) fn db_update_cart_item_from_cart (
    conn: &mut PgConnection,
    new_cart: NewCartItem,
) -> Result<CartItem, Error> {
    let cart_item = diesel::update(carts.filter(user_id.eq(new_cart.user_id).and(product_id.eq(new_cart.product_id))))
        .set(quantity.eq(new_cart.quantity))
        .get_result::<CartItem>(conn)?;

    Ok(cart_item)
}

pub(crate) fn db_delete_cart_item (
    conn: &mut PgConnection,
    input_user_id: String,
    input_product_id: String, 
) -> Result<usize, Error> {
    let deleted_cart_item = diesel::delete(carts.find((input_user_id, input_product_id)))
        .execute(conn)?;

    Ok(deleted_cart_item)
}

pub(crate) fn db_delete_cart_items_by_product (
    conn: &mut PgConnection,
    product: String,
) -> Result<usize, Error> {
    let deleted_cart_items = diesel::delete(carts.filter(product_id.eq(product)))
        .execute(conn)?;

    Ok(deleted_cart_items)
}

pub(crate) fn db_delete_cart_items_by_user (
    conn: &mut PgConnection,
    user: String,
) -> Result<usize, Error> {
    let deleted_cart_items = diesel::delete(carts.filter(user_id.eq(user)))
        .execute(conn)?;

    Ok(deleted_cart_items)
}