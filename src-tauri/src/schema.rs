// @generated automatically by Diesel CLI.

diesel::table! {
    epics (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        status -> Varchar,
        project_id -> Nullable<Int4>,
        created_at -> Timestamp,
        logged_hours -> Nullable<Jsonb>,
        total_hours -> Nullable<Int4>,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        status -> Varchar,
        created_at -> Timestamp,
        logged_hours -> Nullable<Jsonb>,
        total_hours -> Nullable<Int4>,
    }
}

diesel::table! {
    tickets (id) {
        id -> Int4,
        title -> Varchar,
        status -> Varchar,
        description -> Nullable<Text>,
        epic_id -> Nullable<Int4>,
        created_at -> Timestamp,
        logged_hours -> Nullable<Jsonb>,
        total_hours -> Nullable<Int4>,
    }
}

diesel::joinable!(epics -> projects (project_id));
diesel::joinable!(tickets -> epics (epic_id));

diesel::allow_tables_to_appear_in_same_query!(
    epics,
    projects,
    tickets,
);
