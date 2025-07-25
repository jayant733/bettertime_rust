// @generated automatically by Diesel CLI.

diesel::table! {
    regions (id) {
        id -> Varchar,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        username -> Text,
        password -> Text,
    }
}

diesel::table! {
    website_ticks (id) {
        id -> Int8,
        response_time_ms -> Nullable<Int4>,
        status -> Text,
        website_id -> Nullable<Text>,
        region_id -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    websites (id) {
        id -> Text,
        url -> Text,
        date_time -> Nullable<Timestamp>,
        user_id -> Text,
    }
}

diesel::joinable!(website_ticks -> regions (region_id));
diesel::joinable!(website_ticks -> websites (website_id));
diesel::joinable!(websites -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    regions,
    users,
    website_ticks,
    websites,
);
