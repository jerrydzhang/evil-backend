-- Your SQL goes here
CREATE TABLE products (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR,
    catagory VARCHAR NOT NULL,
    price numeric(10, 2) NOT NULL,
    inventory INTEGER NOT NULL,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)