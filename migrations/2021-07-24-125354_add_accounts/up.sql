-- Your SQL goes here
CREATE TABLE accounts (
  id SERIAL NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  email TEXT NOT NULL,
  password TEXT NOT NULL,
  account_type TEXT NOT NULL,
  tenant_domain TEXT NOT NULL,
  region TEXT NOT NULL,
  environment_tag TEXT NOT NULL,
  provider TEXT NOT NULL,
  -- setting_id INTEGER NOT NULL references settings(id),
  created_at TIMESTAMP NOT NULL
);