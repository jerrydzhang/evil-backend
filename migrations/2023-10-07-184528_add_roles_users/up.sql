-- Your SQL goes here
ALTER TABLE users ADD COLUMN roles text[] DEFAULT '{}';