use actix_web::web::{self, service};

use crate::{handlers::{
products::{
    get_all_products,
    get_product_by_id, 
    get_multiple_products_by_id,
    get_products_by_catagory, 
    // create_product, 
    // update_product, 
    // delete_product,
}, 
users::{
    create_user, 
    delete_user, 
    index, 
    login, 
    logout, 
    update_roles, 
    get_user,
}, 
carts::{
    get_cart_items,
    update_cart,
    add_to_cart,
    update_cart_item
}, }, stripe::webhook::webhook_handler};

pub(crate) fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(|| async { "Hello, world!" }))
        .service(
            web::scope("/api")
            .service(webhook_handler)
            .service(
                // products
                web::scope("/product")
                .service(get_all_products)
                .service(get_multiple_products_by_id)
                .service(get_product_by_id)
                .service(get_products_by_catagory)
                // .service(create_product)
                // .service(update_product)
                // .service(delete_product)
            )
            .service(
                // users
                web::scope("/user")
                .service(create_user)
                .service(update_roles)
                .service(delete_user)
                .service(get_user)
                .service(index)
                .service(login)
                .service(logout)
            )
            .service(
                // carts
                web::scope("/cart")
                .service(get_cart_items)
                .service(add_to_cart)
                .service(update_cart_item)
                .service(update_cart)
            )
        );
}