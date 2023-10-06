use diesel::{
    r2d2, 
    PgConnection,
};
use crate::models::dbpool::PgPool;

// wrap database connection in r2d2 connection manager to allow for connection pooling
pub(crate) fn initialize_db_pool() -> PgPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to Postgres DB file")
}