// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Integer,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        artist -> Varchar,
        #[max_length = 255]
        cover -> Varchar,
        year -> Integer,
        user_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::joinable!(albums -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(albums, users,);
