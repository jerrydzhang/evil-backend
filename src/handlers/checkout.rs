use actix_web::{post, web, HttpResponse, Responder, Result, error};
use stripe::{Client, CheckoutSession, Customer, Expandable, CheckoutSessionMode, CreateCheckoutSessionShippingAddressCollectionAllowedCountries, CheckoutSessionStatus};

use crate::{models::{dbpool::PgPool, product}, database::{carts::{db_get_cart_items_by_user_id, db_delete_cart_items_by_user}, products::{db_get_product_by_id, db_update_product}, users::{db_get_user, db_user_stripe_to_user_id, db_user_id_to_stripe_id}}, extractors::claims::Claims, handlers::orders::create_order};

#[post("/")]
async fn checkout(
    pool: web::Data<PgPool>,
    client: web::Data<Client>,
    claims: Claims,
) -> Result<impl Responder> {
    remove_checkoutsessions(pool.clone(), &client, claims.sub.clone()).await?;
    // get cart items
    let user_id = claims.sub.clone();    
    let cloned_pool = pool.clone();
    let cart_items = web::block(move || {
        let mut conn = cloned_pool.get().unwrap();
        db_get_cart_items_by_user_id(&mut conn, user_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    // get user
    let user_id = claims.sub.clone();
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

    let update_cart_items = cart_items.clone();
    let cloned_pool = pool.clone();
    let enough_stock = web::block(move || {
        let mut conn = cloned_pool.get().unwrap();
        let enough_stock = update_cart_items.iter().try_for_each(|item| {
            let id = item.product_id.clone();
            let new_quantity = item.quantity.clone();
            let product = db_get_product_by_id(&mut conn, id).unwrap();
            if product.inventory.unwrap_or(0) < new_quantity {
                return Err(());
            }
            Ok(())
        });

        if enough_stock.is_err() {
            return Err(());
        }

        update_cart_items.iter().for_each(|item| {
            let id = item.product_id.clone();
            let new_quantity = item.quantity.clone();
            let product = db_get_product_by_id(&mut conn, id).unwrap();
            let new_product = product::NewProduct{
                id: Some(product.id.clone()),
                inventory: Some(product.inventory.unwrap_or(0) - new_quantity),
                ..Default::default()
            };
            db_update_product(&mut conn, new_product).unwrap();
        });
        enough_stock
    })
    .await?;

    if enough_stock.is_err() {
        return Err(error::ErrorBadRequest("Not enough stock"));
    }

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
        let success_url = format!("{}/checkout-approved", frontend_url);
        let cancel_url = format!("{}/checkout-canceled", frontend_url);

        let mut params = stripe::CreateCheckoutSession::new(&success_url);
        params.cancel_url = Some(&cancel_url);
        params.customer = Some(customer.id);
        params.mode = Some(CheckoutSessionMode::Payment);
        params.shipping_address_collection = Some(stripe::CreateCheckoutSessionShippingAddressCollection{
            allowed_countries: vec![CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Us],
            ..Default::default()});
        params.expires_at = Some((chrono::Utc::now() + chrono::Duration::hours(1)).timestamp());
        params.line_items = Some(cart_items.clone().into_iter().map(|item| {
            let product = db_get_product_by_id(&mut pool.get().unwrap(), item.product_id).unwrap();
            stripe::CreateCheckoutSessionLineItems {
                price: Some(product.price_id.unwrap()),
                quantity: Some(item.quantity as u64),
                ..Default::default()
            }
        }).collect());
        
        params.expand = &["line_items", "line_items.data.price.product"];

        CheckoutSession::create(&client, params).await.map_err(error::ErrorInternalServerError)?
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

#[post("/cancel")]
async fn cancel_checkout(
    pool: web::Data<PgPool>,
    client: web::Data<Client>,
    claims: Claims,
) -> Result<impl Responder> {
    remove_checkoutsessions(pool, &client, claims.sub).await?;
    Ok(HttpResponse::Ok())
}

async fn remove_checkoutsessions(
    pool: web::Data<PgPool>,
    client: &web::Data<Client>,
    user_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let stripe_id = web::block(move || {
        let mut conn = pool.get().unwrap();
        db_user_id_to_stripe_id(&mut conn, user_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    
    let stripe_id = match stripe_id {
        Some(stripe_id) => stripe_id,
        None => return Err(Box::new(error::ErrorBadRequest("User does not exist"))),
    };

    let checkout_session = CheckoutSession::list(&client, &Default::default()).await.map_err(error::ErrorInternalServerError)?;
    let checkout_session = checkout_session.data.into_iter().find(|session| (session.customer.as_ref().unwrap().id().to_string() == stripe_id) && (session.status == Some(CheckoutSessionStatus::Open)));
    log::info!("checkout session: {:?}", checkout_session);
    if let Some(checkout_session) = checkout_session {
        CheckoutSession::expire(&client, &checkout_session.id).await.map_err(error::ErrorInternalServerError)?;
    }
    Ok(())
}

pub(crate) async fn checkout_success(
    pool: web::Data<PgPool>,
    client: web::Data<Client>,
    checkout_session: CheckoutSession,
) -> Result<(), Box<dyn std::error::Error>> {
    let stripe_user_id = checkout_session.customer.clone().unwrap().id().to_string();

    let address = {
        checkout_session.shipping_details.clone().unwrap().address.clone().unwrap().line1.clone().unwrap_or("".to_string()) + ", " +
        &checkout_session.shipping_details.clone().unwrap().address.clone().unwrap().line2.clone().unwrap_or("".to_string()) + ", " +
        &checkout_session.shipping_details.clone().unwrap().address.clone().unwrap().city.clone().unwrap_or("".to_string()) + " " +
        &checkout_session.shipping_details.clone().unwrap().address.clone().unwrap().state.clone().unwrap_or("".to_string()) + " " +
        &checkout_session.shipping_details.clone().unwrap().address.clone().unwrap().postal_code.clone().unwrap_or("".to_string()) + " " +
        &checkout_session.shipping_details.clone().unwrap().address.clone().unwrap().country.clone().unwrap_or("".to_string())
    };

    create_order(
        pool.clone(), 
        client, 
        checkout_session.customer.clone().unwrap().id().to_string(),
        checkout_session.shipping_details.clone().unwrap().name.clone().unwrap_or("".to_string()),
        address,
    ).await?;

    // convert stripe id to auth0 id and then delete cart associated with auth0 id
    let cart = web::block(move || {
        let mut conn = pool.get().unwrap();

        let user = db_user_stripe_to_user_id(&mut conn, stripe_user_id.clone())?;

        // delete the cart
        db_delete_cart_items_by_user(&mut conn, user.unwrap().id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    log::info!("deleted {:?} cart items for user {}", cart, checkout_session.customer.unwrap().id().to_string());
    Ok(())
}

pub(crate) async fn checkout_expired(
    pool: web::Data<PgPool>,
    checkout_session: CheckoutSession,
) -> Result<(), Box<dyn std::error::Error>> {
    let stripe_user_id = checkout_session.customer.clone().unwrap().id().to_string();

    // convert stripe id to auth0 id and then delete cart associated with auth0 id
    web::block(move || {
        let mut conn = pool.get().unwrap();

        let user = db_user_stripe_to_user_id(&mut conn, stripe_user_id.clone()).unwrap();
        let cart = db_get_cart_items_by_user_id(&mut conn, user.clone().unwrap().id).unwrap().unwrap();
        // // for each item deleted from the cart, update the product inventory
        cart.iter().for_each(|item| {
            let id = item.product_id.clone();
            let new_quantity = item.quantity.clone();
            let product = db_get_product_by_id(&mut conn, id).unwrap();
            let new_product = product::NewProduct{
                id: Some(product.id.clone()),
                inventory: Some(new_quantity + product.inventory.unwrap_or(0)),
                ..Default::default()
            };
            db_update_product(&mut conn, new_product).unwrap();
        });
    })
    .await?;

    Ok(())
}