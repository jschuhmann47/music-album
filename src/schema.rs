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
