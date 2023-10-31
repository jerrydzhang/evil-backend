use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use diesel::{result::Error, PgConnection};

use crate::models::order::{Order, NewOrder, ExpandedOrder, OrderProduct};
use crate::schema::orders::dsl::*;

use super::products::db_expand_products;

pub(crate) fn db_get_all_orders(
    conn: &mut PgConnection,
) -> Result<Option<Vec<Order>>, Error> {
    let all_orders = orders
        .load::<Order>(conn)?;

    Ok(Some(all_orders))
}

pub(crate) fn db_get_expanded_orders(
    conn: &mut PgConnection,
) -> Result<Option<Vec<ExpandedOrder>>, Error> {
    let all_orders = orders
        .load::<Order>(conn)?;

    log::info!("all_orders: {:?}", all_orders);

    match all_orders.len() {
        0 => return Ok(None),
        _ => (),
    }

    let expanded_orders = all_orders.iter()
        .map(|order| {
            let product_ids = order.products.as_object().unwrap();
            let expanded_products = product_ids.iter()
                .map(|(product_id, quantity)| {
                    let product = db_expand_products(conn, vec![product_id.to_string()]).unwrap();
                    let product = product.first().unwrap();
                    let product = product.clone();
                    let quantity = quantity.as_i64().unwrap() as i32;
                    OrderProduct::new(product, quantity)
                })
                .collect::<Vec<OrderProduct>>();
    
            ExpandedOrder::new(order.clone(), expanded_products)
        })
        .map(Result::Ok)
        .collect::<Vec<Result<ExpandedOrder, Error>>>();
    
    let expanded_orders = expanded_orders.into_iter()
        .collect::<Result<Vec<ExpandedOrder>, Error>>()?;

    Ok(Some(expanded_orders))
}

pub(crate) fn db_get_order_by_id(
    conn: &mut PgConnection,
    order_id: String,
) -> Result<Order, Error> {
    let order = orders
        .find(order_id)
        .first::<Order>(conn)?;

    Ok(order)
}

pub(crate) fn db_get_orders_by_user_id(
    conn: &mut PgConnection,
    user: String,
) -> Result<Option<Vec<Order>>, Error> {
    let orders_by_user_id = orders
        .filter(user_id.eq(user))
        .load::<Order>(conn)?;

    Ok(Some(orders_by_user_id))
}

pub(crate) fn db_create_order(
    conn: &mut PgConnection,
    new_order: NewOrder,
) -> Result<Order, Error> {
    let order = diesel::insert_into(orders)
        .values(&new_order)
        .get_result::<Order>(conn)?;

    Ok(order)
}

pub(crate) fn db_update_order(
    conn: &mut PgConnection,
    order_id: String,
    new_order: NewOrder,
) -> Result<Order, Error> {
    let order = diesel::update(orders.find(order_id))
        .set(&new_order)
        .get_result::<Order>(conn)?;

    Ok(order)
}

pub(crate) fn db_delete_order(
    conn: &mut PgConnection,
    order_id: String,
) -> Result<usize, Error> {
    let deleted_order = diesel::delete(orders.find(order_id))
        .execute(conn)?;

    Ok(deleted_order)
}

