use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::CookieSessionStore, config::PersistentSession};
use actix_web::{App, HttpServer, middleware::Logger, web, cookie::{Key, time::Duration}};

use crate::{routes::routes, database::init_db::initialize_db_pool};

pub(crate) async fn server() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    
    dotenv::dotenv().ok();

    let pool = initialize_db_pool();
    let stripe_client = stripe::Client::new(std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY should be set"));
    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            // identity middleware
            .wrap(IdentityMiddleware::default())
            // cookie session middleware
            .wrap(SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .cookie_secure(false)
                .session_lifecycle(PersistentSession::default().session_ttl(Duration::seconds(36000)))
                .build(),
            )
            // CORS
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()      
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            // pass the database pool to application so we can access it inside handlers
            .app_data(web::Data::new(pool.clone()))
            // pass the stripe client to application so we can access it inside handlers
            .app_data(web::Data::new(stripe_client.clone()))
            .configure(routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}