-- Your SQL goes here
CREATE TABLE user_tokens (
    id UUID PRIMARY KEY,
    hashed_token VARCHAR(255) NOT NULL UNIQUE,
    user_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX user_tokens_hashed_token_index ON user_tokens(hashed_token);