use std::borrow::Borrow;

use actix_web::{post, HttpRequest, web, HttpResponse, Responder, Result};
use stripe::{Webhook, EventType, EventObject};

use crate::{models::dbpool::PgPool, handlers::{products::{create_product, change_price, update_product, delete_product}, checkout::checkout_success}};

#[post("stripe_webhooks")]
pub async fn webhook_handler(
    pool: web::Data<PgPool>,
    req: HttpRequest, 
    payload: web::Bytes
) -> Result<impl Responder> {
    handle_webhook(pool, req, payload).await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn handle_webhook(
    pool: web::Data<PgPool>,
    req: HttpRequest, 
    payload: web::Bytes,
) -> Result<(), Box<dyn std::error::Error>> {
    let payload_str = std::str::from_utf8(payload.borrow()).unwrap();

    let stripe_signature = get_header_value(&req, "Stripe-Signature").unwrap_or_default();

    if let Ok(event) = Webhook::construct_event(payload_str, stripe_signature, "whsec_dead008dac3b2554665d4d4a7fef5edd47f2cd699e480867c80ef987a0d2d9ef") {
        match event.type_ {
            EventType::ProductCreated => {
                if let EventObject::Product(product) = event.data.object {
                    create_product(pool, product).await?;
                }
            }
            EventType::ProductUpdated => {
                if let EventObject::Product(product) = event.data.object {
                    update_product(pool, product).await?;
                }
            }
            EventType::ProductDeleted => {
                if let EventObject::Product(product) = event.data.object {
                    delete_product(pool, product).await?;
                }
            }
            EventType::PriceCreated | EventType::PriceUpdated => {
                if let EventObject::Price(price) = event.data.object {
                    change_price(pool, price).await?;
                }
            }
            EventType::CheckoutSessionCompleted => {
                if let EventObject::CheckoutSession(session) = event.data.object {
                    checkout_success(pool, session).await?;
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