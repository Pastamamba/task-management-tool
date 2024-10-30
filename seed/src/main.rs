#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::env;
use serde::Serialize;
use diesel::insert_into;
use diesel::Queryable;
use diesel::Insertable;

table! {
    tickets (id) {
        id -> Int4,
        title -> Varchar,
        status -> Varchar,
        description -> Nullable<Text>,
    }
}

#[derive(Serialize, Queryable, Insertable)]
#[diesel(table_name = tickets)]
struct Ticket {
    id: i32,
    title: String,
    status: String,
    description: Option<String>,
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Error connecting to database")
}

fn insert_dummy_data() {
    let connection = &mut establish_connection();

    let dummy_tickets = vec![
        Ticket {
            id: 1,
            title: "Fix login bug".to_string(),
            status: "In Progress".to_string(),
            description: Some("User cannot login with correct credentials".to_string()),
        },
        Ticket {
            id: 2,
            title: "Update dependencies".to_string(),
            status: "Open".to_string(),
            description: Some("Update project dependencies to latest versions".to_string()),
        },
        Ticket {
            id: 3,
            title: "Add new feature X".to_string(),
            status: "Backlog".to_string(),
            description: Some("Implement feature X according to specifications".to_string()),
        },
    ];

    for ticket in dummy_tickets {
        insert_into(tickets::table)
            .values(&ticket)
            .execute(connection)
            .expect("Error inserting dummy data");
    }

    println!("Dummy data inserted successfully.");
}

fn main() {
    insert_dummy_data();
}
