mod schema;

use diesel::prelude::*;
use diesel::{Queryable, Insertable, Selectable};
use serde::Serialize;
use crate::schema::tickets;
use diesel::pg::PgConnection;
use std::env;

#[derive(Serialize, Queryable, Insertable, Selectable)]
#[table_name = "tickets"]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Ticket {
    id: i32,
    title: String,
    status: String,
    description: Option<String>
}

fn establish_connection() -> PgConnection {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Error connecting to database")
}

// Komento, joka hakee tiedot tietokannasta
#[tauri::command]
fn get_tickets_from_db() -> Vec<Ticket> {
    use crate::schema::tickets::dsl::*;

    let mut connection = establish_connection();
    tickets
        .load::<Ticket>(&mut connection)
        .expect("Error loading tickets")
}

// Mock-dataa (jos haluat säilyttää get_dummy_tickets)
#[tauri::command]
fn get_dummy_tickets() -> Vec<Ticket> {
    vec![
        Ticket {
            id: 1,
            title: "Fix login bug".to_string(),
            status: "In Progress".to_string(),
            description: Option::from("User cannot login with correct credentials".to_string()),
        },
        Ticket {
            id: 2,
            title: "Update dependencies".to_string(),
            status: "Open".to_string(),
            description: Option::from("Update project dependencies to latest versions".to_string()),
        },
        Ticket {
            id: 3,
            title: "Add new feature X".to_string(),
            status: "Backlog".to_string(),
            description: Option::from("Implement feature X according to specifications".to_string()),
        },
    ]
}

// Tervetuloviesti-komento (jos haluat säilyttää greet-funktion)
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Tauri-sovelluksen käynnistys
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_dummy_tickets, get_tickets_from_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
