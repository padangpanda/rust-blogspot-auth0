-- Your SQL goes here
CREATE TABLE user_role (
    id SERIAL NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL references users(id),
    role_id INTEGER NOT NULL references roles(id)
);