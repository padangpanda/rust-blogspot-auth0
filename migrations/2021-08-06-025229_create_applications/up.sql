-- Your SQL goes here
CREATE TABLE applications (
    id SERIAL NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    client_id TEXT NOT NULL UNIQUE,
    app_type TEXT NOT NULL,
    domain TEXT NOT NULL,
    client_secret TEXT NOT NULL,
    description TEXT NOT NULL,
    logo_url TEXT NOT NULL,
    token_auth_method TEXT NOT NULL,
    app_login_url TEXT,
    callback_url TEXT,
    logout_url TEXT,
    web_origin TEXT,
    cors TEXT,
    id_token_exp INTEGER NOT NULL,
    reuse_interval INTEGER NOT NULL,
    abs_lifetime INTEGER NOT NULL,
    inactivity_lifetime INTEGER NOT NULL,
    account_id INTEGER NOT NULL references accounts(id)
);