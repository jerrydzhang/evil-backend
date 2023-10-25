mod database;
mod handlers;
mod middleware;
mod models;
mod routes;
mod server;
mod schema;
mod utils;
mod stripe;
mod extractors;

use crate::server::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}