use diesel::r2d2;
use diesel::PgConnection;

// wrap database connection in r2d2 connection manager to allow for connection pooling
pub(crate) type PgPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;