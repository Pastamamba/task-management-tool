mod commands;
pub mod db;
mod epics;
pub mod schema;
mod tests;
pub mod tickets;
mod projects;
mod common;

use tickets::{get_tickets_from_db, update_ticket_status};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Tauri application runner
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_tickets_from_db,
            update_ticket_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
