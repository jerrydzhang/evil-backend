use std::borrow::Borrow;

use actix_web::{post, HttpRequest, web, HttpResponse, Responder, Result};
use stripe::{Webhook, EventType, EventObject, Client};

use crate::{models::dbpool::PgPool, handlers::{products::{wh_create_product, wh_change_price, wh_update_product, wh_delete_product}, checkout::{checkout_success, checkout_expired}}};

#[post("stripe_webhooks")]
pub async fn webhook_handler(
    pool: web::Data<PgPool>,
    client: web::Data<Client>,
    req: HttpRequest, 
    payload: web::Bytes
) -> Result<impl Responder> {
    log::info!("Received webhook request: {:?}", req);
    handle_webhook(pool, client, req, payload).await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn handle_webhook(
    pool: web::Data<PgPool>,
    client: web::Data<Client>,
    req: HttpRequest,
    payload: web::Bytes,
) -> Result<(), Box<dyn std::error::Error>> {
    let payload_str = std::str::from_utf8(payload.borrow()).unwrap();

    let stripe_signature = get_header_value(&req, "Stripe-Signature").unwrap_or_default();

    if let Ok(event) = Webhook::construct_event(payload_str, stripe_signature, "whsec_dead008dac3b2554665d4d4a7fef5edd47f2cd699e480867c80ef987a0d2d9ef") {
        match event.type_ {
            EventType::ProductCreated => {
                if let EventObject::Product(product) = event.data.object {
                    wh_create_product(pool, product).await?;
                }
            }
            EventType::ProductUpdated => {
                if let EventObject::Product(product) = event.data.object {
                    wh_update_product(pool, product).await?;
                }
            }
            EventType::ProductDeleted => {
                if let EventObject::Product(product) = event.data.object {
                    wh_delete_product(pool, product).await?;
                }
            }
            EventType::PriceCreated | EventType::PriceUpdated => {
                if let EventObject::Price(price) = event.data.object {
                    wh_change_price(pool, price).await?;
                }
            }
            EventType::CheckoutSessionCompleted => {
                if let EventObject::CheckoutSession(session) = event.data.object {
                    checkout_success(pool, client, session).await?;
                }
            }
            EventType::CheckoutSessionExpired => {
                if let EventObject::CheckoutSession(session) = event.data.object {
                    checkout_expired(pool, session).await?;
                }
            }
            _ => {
                log::info!("Unknown event encountered in webhook: {:?}", event.type_);
            }
        }
    } else {
        log::warn!("Failed to construct webhook event, ensure your webhook secret is correct.");
    }

    Ok(())
}

fn get_header_value<'b>(req: &'b HttpRequest, key: &'b str) -> Option<&'b str> {
    req.headers().get(key)?.to_str().ok()
}