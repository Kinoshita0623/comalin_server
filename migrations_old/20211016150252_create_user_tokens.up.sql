-- Add up migration script here
CREATE TABLE user_tokens (
    user_id UUID,
    token VARCHAR(255) UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE INDEX user_tokens_token_index ON user_tokens(token);