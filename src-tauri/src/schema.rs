// @generated automatically by Diesel CLI.

diesel::table! {
    tickets (id) {
        id -> Int4,
        title -> Varchar,
        status -> Varchar,
        description -> Nullable<Text>,
    }
}
