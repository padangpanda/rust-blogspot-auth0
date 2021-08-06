-- Your SQL goes here
CREATE TABLE settings (
    id SERIAL NOT NULL PRIMARY KEY,
    friendly_name TEXT NOT NULL,
    logo_url TEXT NOT NULL,
    support_email TEXT NOT NULL,
    support_url TEXT NOT NULL,
    environment_tag TEXT NOT NULL,
    default_audience TEXT NOT NULL,
    default_directory TEXT NOT NULL,
    default_error_page TEXT NOT NULL,
    default_error_page_url TEXT NOT NULL,
    default_language TEXT NOT NULL,
    account_id INTEGER NOT NULL references accounts(id)
);