-- Your SQL goes here
CREATE TABLE apis (
    id SERIAL NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    api_id TEXT NOT NULL UNIQUE,
    identifier TEXT NOT NULL,
    token_exp INTEGER NOT NULL,
    token_exp_browser INTEGER NOT NULL,
    sign_algorithm TEXT NOT NULL,
    rbac boolean NOT NULL,
    permission_acc_token boolean NOT NULL,
    allow_skip_user boolean NOT NULL,
    allow_off_acc boolean NOT NULL,
    account_id INTEGER NOT NULL references accounts(id)
);