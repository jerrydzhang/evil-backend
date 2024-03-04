use actix_web::web::{self};

use crate::{handlers::{
products::{
    create_product, get_active_products, get_active_products_by_category, get_all_products, get_multiple_products_by_id, get_product_by_id, get_product_by_name, get_products_by_category, update_product_inventory 
}, 
users::{
    create_user, 
    delete_user, 
    index,
    update_user, 
    get_user,
}, 
carts::{
    get_cart_items,
    update_cart,
    add_to_cart,
    update_cart_item
}, checkout::{checkout, cancel_checkout}, orders::{create_order_handler, delete_order, get_expanded_orders, get_expanded_orders_by_user_id, get_order_by_id, get_orders, update_order, update_order_status}, }, stripe::webhook::webhook_handler};

pub(crate) fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(|| async { "Hello, world!" }))
        .service(
            web::scope("/api")
            .service(webhook_handler)
            .service(
                // checkout
                web::scope("/checkout")
                .service(checkout)
                .service(cancel_checkout)
            )
            .service(
                // products
                web::scope("/product")
                .service(get_all_products)
                .service(get_active_products)
                .service(get_product_by_name)
                .service(get_multiple_products_by_id)
                .service(get_product_by_id)
                .service(get_products_by_category)
                .service(get_active_products_by_category)
                .service(update_product_inventory)
                .service(create_product)
                // .service(update_product)
                // .service(delete_product)
            )
            .service(
                // users
                web::scope("/user")
                .service(create_user)
                .service(update_user)
                .service(delete_user)
                .service(get_user)
                .service(index)
            )
            .service(
                // carts
                web::scope("/cart")
                .service(get_cart_items)
                .service(add_to_cart)
                .service(update_cart_item)
                .service(update_cart)
            )
            .service(
                web::scope("/order")
                .service(get_orders)
                .service(get_order_by_id)
                .service(get_expanded_orders)
                .service(get_expanded_orders_by_user_id)
                .service(create_order_handler)
                .service(update_order)
                .service(update_order_status)
                .service(delete_order)
            )
        );
}