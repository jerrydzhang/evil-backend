use actix_identity::Identity;
use actix_web::{post, web, HttpResponse, Responder, Result, error};
use stripe::{Client, CheckoutSession, Customer, Expandable, CheckoutSessionMode};

use crate::{models::dbpool::PgPool, database::{carts::{db_get_cart_items_by_user_id, db_delete_cart_items_by_user}, products::db_get_product_by_id, users::{db_get_user, db_user_stripe_to_user_id}}};

#[post("/checkout")]
async fn checkout(
    client: web::Data<Client>,
    pool: web::Data<PgPool>,
    identity: Identity
) -> Result<impl Responder> {
    // get cart items
    let user_id = identity.id().unwrap();    
    let cloned_pool = pool.clone();
    let cart_items = web::block(move || {
        let mut conn = cloned_pool.get().unwrap();

        db_get_cart_items_by_user_id(&mut conn, user_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    // get user
    let user_id = identity.id().unwrap();
    let cloned_pool = pool.clone();
    let user = web::block(move || {
        let mut conn = cloned_pool.get().unwrap();

        db_get_user(&mut conn, user_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    // check if cart is empty
    let cart_items = match cart_items {
        Some(cart_items) => {
            if cart_items.len() == 0 {
                return Err(error::ErrorBadRequest("Cart is empty"));
            } else {
                cart_items
            }
        },
        None => return Err(error::ErrorBadRequest("Unable to find cart")),
    };

    // check if user exists
    let user = match user {
        Some(user) => user,
        None => return Err(error::ErrorBadRequest("User does not exist")),
    };

    // get the stripe customer object from stripe api
    let customer:Customer = client.get(&(format!("/customers/{}", user.stripe_id.unwrap())))
        .await
        .map_err(error::ErrorInternalServerError)?;

    // create a checkout session
    let checkout_session = {
        let frontend_url = std::env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");
        let success_url = format!("{}/checkout/success", frontend_url);
        let cancel_url = format!("{}/checkout/cancel", frontend_url);

        let mut params = stripe::CreateCheckoutSession::new(&success_url);
        params.cancel_url = Some(&cancel_url);
        params.customer = Some(customer.id);
        params.mode = Some(CheckoutSessionMode::Payment);
        params.line_items = Some(cart_items.into_iter().map(|item| {
            let product = db_get_product_by_id(&mut pool.get().unwrap(), item.product_id).unwrap();
            stripe::CreateCheckoutSessionLineItems {
                price: Some(product.price_id.unwrap()),
                quantity: Some(item.quantity as u64),
                ..Default::default()
            }
        }).collect());
        
        params.expand = &["line_items", "line_items.data.price.product"];

        CheckoutSession::create(&client, params).await.unwrap()
    };

    log::info!(
        "created a {} checkout session for {} {:?} for {} {} at {}",
        checkout_session.payment_status,
        checkout_session.line_items.data[0].quantity.unwrap(),
        match checkout_session.line_items.data[0].price.as_ref().unwrap().product.as_ref().unwrap()
        {
            Expandable::Object(p) => p.name.as_ref().unwrap(),
            _ => panic!("product not found"),
        },
        checkout_session.amount_subtotal.unwrap() / 100,
        checkout_session.line_items.data[0].price.as_ref().unwrap().currency.unwrap(),
        checkout_session.url.clone().unwrap()
    );

    Ok(HttpResponse::Ok().json(checkout_session.url.unwrap()))
}

pub(crate) async fn checkout_success(
    pool: web::Data<PgPool>,
    checkout_session: CheckoutSession,
) -> Result<(), Box<dyn std::error::Error>> {
    let stripe_user_id = checkout_session.customer.clone().unwrap().id().to_string();

    // convert stripe id to auth0 id and then delete cart associated with auth0 id
    let cart = web::block(move || {
        let mut conn = pool.get().unwrap();

        let user = db_user_stripe_to_user_id(&mut conn, stripe_user_id.clone())?;
        db_delete_cart_items_by_user(&mut conn, user.unwrap().id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    log::info!("deleted {} cart items for user {}", cart, checkout_session.customer.unwrap().id().to_string());
    Ok(())
}