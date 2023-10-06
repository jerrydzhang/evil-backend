mod database;
mod handlers;
mod middleware;
mod models;
mod routes;
mod errors;
mod server;
mod schema;

use crate::server::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}