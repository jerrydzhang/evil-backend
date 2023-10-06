-- Your SQL goes here
ALTER TABLE products RENAME TO old_products;
CREATE TABLE products (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR,
    catagory_id INTEGER NOT NULL REFERENCES catagories(id),
    price numeric(10, 2) NOT NULL,
    inventory INTEGER NOT NULL,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
INSERT INTO products (name, description, catagory_id, price, inventory, last_updated) SELECT name, description, catagory_id, price, inventory, last_updated FROM old_products;
DROP TABLE old_products;