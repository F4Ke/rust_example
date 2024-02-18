// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        summary -> Text,
        key -> Varchar,
    }
}
