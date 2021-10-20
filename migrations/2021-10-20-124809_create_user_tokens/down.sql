-- This file should undo anything in `up.sql`
DROP INDEX user_tokens_hashed_token_index;
DROP TABLE user_tokens;
