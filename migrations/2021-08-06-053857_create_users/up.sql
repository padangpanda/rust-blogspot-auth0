-- Your SQL goes here
CREATE TABLE users (
    id SERIAL NOT NULL PRIMARY KEY,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    latest_login TIMESTAMP,
    connection TEXT NOT NULL,
    provider TEXT NOT NULL,
    is_social boolean NOT NULL,
    picture TEXT NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    blocked boolean NOT NULL,
    blocked_for TEXT,
    guardian_authenticators TEXT,
    account_id INTEGER NOT NULL references accounts(id)
);