use serde::Serialize;

#[derive(Serialize)]
struct Ticket {
    id: String,
    title: String,
    status: String,
    description: String,
}

#[tauri::command]
fn get_dummy_tickets() -> Vec<Ticket> {
    vec![
        Ticket {
            id: "TICK-001".to_string(),
            title: "Fix login bug".to_string(),
            status: "In Progress".to_string(),
            description: "User cannot login with correct credentials".to_string(),
        },
        Ticket {
            id: "TICK-002".to_string(),
            title: "Update dependencies".to_string(),
            status: "Open".to_string(),
            description: "Update project dependencies to latest versions".to_string(),
        },
        Ticket {
            id: "TICK-003".to_string(),
            title: "Add new feature X".to_string(),
            status: "Backlog".to_string(),
            description: "Implement feature X according to specifications".to_string(),
        },
    ]
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_dummy_tickets])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
