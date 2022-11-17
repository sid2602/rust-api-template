-- Add migration script here

CREATE TABLE IF NOT EXISTS "users" (
    id VARCHAR PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    phone_number VARCHAR NOT NULL
)