table! {
    use diesel::sql_types::*;
    use crate::diesel_util::sql_types::*;

    addresses (id) {
        id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::diesel_util::sql_types::*;

    posts (id) {
        id -> Int8,
        title -> Varchar,
        body -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::diesel_util::sql_types::*;

    questions (id) {
        id -> Uuid,
        title -> Varchar,
        text -> Nullable<Text>,
        longitude -> Numeric,
        latitude -> Numeric,
        location_point -> Geography,
        address_id -> Nullable<Uuid>,
        user_id -> Uuid,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::diesel_util::sql_types::*;

    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::diesel_util::sql_types::*;

    users (id) {
        id -> Uuid,
        username -> Varchar,
        avatar_icon -> Nullable<Varchar>,
        encrypted_password -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(questions -> addresses (address_id));
joinable!(questions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    addresses,
    posts,
    questions,
    spatial_ref_sys,
    users,
);
