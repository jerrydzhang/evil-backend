-- Your SQL goes here
ALTER TABLE products ADD COLUMN created_at TIMESTAMP DEFAULT NOW();
