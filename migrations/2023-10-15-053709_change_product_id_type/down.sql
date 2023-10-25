-- This file should undo anything in `up.sql`
ALTER TABLE carts DROP CONSTRAINT carts_product_id_fkey;
ALTER TABLE products ALTER COLUMN id TYPE integer USING (id::integer);
ALTER TABLE products ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY;
ALTER TABLE carts ALTER COLUMN product_id TYPE integer USING (product_id::integer);
ALTER TABLE carts ADD CONSTRAINT carts_product_id_fkey FOREIGN KEY (product_id) REFERENCES products(id);