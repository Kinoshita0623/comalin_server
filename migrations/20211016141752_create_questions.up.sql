-- Add up migration script here
CREATE TABLE questions (
    id UUID PRIMARY KEY,
    title VARCHAR(255) NOT NULL INDEX,
    text TEXT,
    longitude NUMERIC(10,7) NOT NULL,
    latitude NUMERIC(9,7) NOT NULL,
    location_point geography(POINT, 4326) NOT NULL,
    address_id UUID,
    user_id UUID NOT NULL INDEX,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP INDEX,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY user_id REFERENCES users(id),
    FOREIGN KEY address_id REFERENCES addresses(id)
);

CREATE INDEX questions_location_point_index on questions USING GIST (location_point);