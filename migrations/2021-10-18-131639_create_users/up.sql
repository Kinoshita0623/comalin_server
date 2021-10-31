-- Your SQL goes here
-- Add up migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    avatar_icon VARCHAR(255),
    questions_count INT NOT NULL DEFAULT 0,
    answers_count INT NOT NULL DEFAULT 0,
    thanks_count INT NOT NULL DEFAULT 0,
    encrypted_password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
