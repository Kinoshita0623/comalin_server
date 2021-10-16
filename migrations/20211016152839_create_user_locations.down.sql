-- Add down migration script here
DROP INDEX user_locations_created_at_index;
DROP INDEX user_locations_location_point_index;
DROP TABLE user_locations;