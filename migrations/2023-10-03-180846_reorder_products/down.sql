-- This file should undo anything in `up.sql`
ALTER TABLE products RENAME TO products_new;
CREATE TABLE products (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR,
    price numeric(10, 2) NOT NULL,
    inventory INTEGER NOT NULL,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    catagory_id INTEGER NOT NULL REFERENCES catagories(id)
);
INSERT INTO products (name, description, price, inventory, last_updated, catagory_id) SELECT name, description, price, inventory, last_updated, catagory_id FROM products_new;
DROP TABLE products_new;