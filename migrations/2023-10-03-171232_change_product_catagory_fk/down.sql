-- This file should undo anything in `up.sql`
ALTER TABLE products DROP COLUMN catagory_id;
ALTER TABLE products ADD COLUMN catagory VARCHAR;