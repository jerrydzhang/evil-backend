-- Your SQL goes here
DROP TABLE IF EXISTS orders;

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE orders (
    id VARCHAR NOT NULL DEFAULT concat('order-', uuid_generate_v4()) PRIMARY KEY,
    user_id VARCHAR NOT NULL REFERENCES users(id),
    products JSONB NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'processing' CHECK (status IN ('processing', 'shipped', 'delievered', 'canceled', 'returned')),
    name VARCHAR NOT NULL,
    address VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)