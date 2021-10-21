-- Your SQL goes here
CREATE TABLE posts (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL
)