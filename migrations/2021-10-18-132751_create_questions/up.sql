-- Your SQL goes here
CREATE TABLE questions (
    id UUID PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    text TEXT,
    longitude NUMERIC(10,7) NOT NULL,
    latitude NUMERIC(9,7) NOT NULL,
    location_point geography(POINT, 4326) NOT NULL,
    address_id UUID,
    user_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (address_id) REFERENCES addresses(id)
);

CREATE INDEX questions_location_point_index ON questions USING GIST (location_point);
CREATE INDEX questions_title_index ON questions(title);
CREATE INDEX questions_created_at_index ON questions(created_at);
CREATE INDEX questions_user_id_index ON questions(user_id);
CREATE INDEX questions_address_id_index ON questions(address_id);