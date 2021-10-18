-- This file should undo anything in `up.sql`
DROP INDEX questions_location_point_index;
DROP INDEX questions_title_index;
DROP INDEX questions_created_at_index;
DROP INDEX questions_user_id_index;
DROP INDEX questions_address_id_index;
DROP TABLE questions;
