-- Add up migration script here
-- 端末から送信されたユーザの位置情報を時系列順で記録する
CREATE TABLE user_locations (
    id UUID PRIMARY KEY,
    longitude NUMERIC(10,7) NOT NULL,
    latitude NUMERIC(9,7) NOT NULL,
    location_point geography(POINT, 4326) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX user_locations_location_point_index ON questions USING GIST (location_point);
CREATE INDEX user_locations_created_at_index ON user_locations(created_at);
