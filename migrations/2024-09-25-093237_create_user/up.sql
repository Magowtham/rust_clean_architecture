-- Your SQL goes here
CREATE TABLE users (
    id  SERIAL PRIMARY KEY,
    name VARCHAR(20) NOT NULL,
    email VARCHAR(20) NOT NULL,
    phone VARCHAR(20) NOT NULL,
    address VARCHAR(40) NOT NULL
);

CREATE INDEX index_users_on_email ON users (email)