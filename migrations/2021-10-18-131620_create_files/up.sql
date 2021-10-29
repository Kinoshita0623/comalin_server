-- Your SQL goes here
CREATE TABLE files(
    id UUID PRIMARY KEY,
    filename VARCHAR(255) NOT NULL UNIQUE,
    mime_type VARCHAR(127) NOT NULL,
    raw_name TEXT,
    hash VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX files_filename_index ON files(filename);