-- Add down migration script here
DROP INDEX user_tokens_token_index;
DROP TABLE user_tokens;
