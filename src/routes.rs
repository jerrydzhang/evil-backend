use actix_web::web;

use crate::handlers::{
products::{
    get_all_products,
    get_product_by_id, 
    get_multiple_products_by_id,
    get_products_by_catagory, 
    create_product, 
    update_product, 
    delete_product,
}, 
catagories::{
    get_all_catagories, 
    create_catagory, 
    delete_catagory,
},
users::{
    create_user, 
    delete_user,
}, carts::{get_cart_items, update_cart, add_to_cart, update_cart_item}, };

pub(crate) fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(|| async { "Hello, world!" }))
        .service(
            web::scope("/api")
            .service(
                // products
                web::scope("/product")
                .service(get_all_products)
                .service(get_multiple_products_by_id)
                .service(get_product_by_id)
                .service(get_products_by_catagory)
                .service(create_product)
                .service(update_product)
                .service(delete_product)
            )
            .service(
                // catagories
                web::scope("/catagory")
                .service(get_all_catagories)
                .service(create_catagory)
                .service(delete_catagory)
            )
            .service(
                // users
                web::scope("/user")
                .service(create_user)
                .service(delete_user)
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