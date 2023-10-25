-- Your SQL goes here
ALTER TABLE carts DROP CONSTRAINT carts_product_id_fkey;
ALTER TABLE products ALTER COLUMN id DROP IDENTITY;
ALTER TABLE products ALTER COLUMN id TYPE VARCHAR;
ALTER TABLE carts ALTER COLUMN product_id TYPE VARCHAR;
ALTER TABLE carts ADD CONSTRAINT carts_product_id_fkey FOREIGN KEY (product_id) REFERENCES products(id);