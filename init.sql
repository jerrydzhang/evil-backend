CREATE USER evil_admin WITH PASSWORD 'evil' CREATEDB;
CREATE DATABASE evil_db
    WITH 
    OWNER = evil_admin
    ENCODING = 'UTF8'
    CONNECTION LIMIT = -1;
CREATE DATABASE test_db
    WITH 
    OWNER = evil_admin
    ENCODING = 'UTF8'
    CONNECTION LIMIT = -1;