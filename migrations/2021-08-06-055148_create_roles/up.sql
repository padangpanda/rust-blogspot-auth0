-- Your SQL goes here
CREATE TABLE roles (
    id SERIAL NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    account_id INTEGER NOT NULL references accounts(id)
);