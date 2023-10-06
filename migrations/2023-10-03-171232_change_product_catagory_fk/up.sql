-- Your SQL goes here
ALTER TABLE products DROP COLUMN catagory;
ALTER TABLE products ADD COLUMN catagory_id INTEGER;
ALTER TABLE products ADD CONSTRAINT fk_catagory FOREIGN KEY (catagory_id) REFERENCES catagories(id);
