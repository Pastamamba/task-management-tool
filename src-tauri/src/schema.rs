// @generated automatically by Diesel CLI.

diesel::table! {
    epics (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        status -> Varchar,
    }
}

diesel::table! {
    tickets (id) {
        id -> Int4,
        title -> Varchar,
        status -> Varchar,
        description -> Nullable<Text>,
        epic_id -> Nullable<Int4>,
    }
}

diesel::joinable!(tickets -> epics (epic_id));

diesel::allow_tables_to_appear_in_same_query!(epics, tickets,);
