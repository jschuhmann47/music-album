// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        artist -> Varchar,
        #[max_length = 255]
        cover -> Varchar,
        year -> Integer,
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

diesel::allow_tables_to_appear_in_same_query!(albums, users,);
