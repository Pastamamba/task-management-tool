use crate::db::establish_connection;
use crate::schema::tickets::dsl::*;
use crate::tickets::models::Ticket;
use diesel::prelude::*;

#[tauri::command]
pub fn get_tickets_from_db() -> Vec<Ticket> {
    let mut connection = establish_connection();
    tickets
        .load::<Ticket>(&mut connection)
        .expect("Error loading tickets")
}

#[tauri::command]
pub fn update_ticket_status(ticket_id: i32, new_status: String) -> Ticket {
    let mut connection = establish_connection();
    diesel::update(tickets.filter(id.eq(ticket_id)))
        .set(status.eq(new_status.clone()))
        .execute(&mut connection)
        .expect("Error updating ticket status");

    tickets
        .filter(id.eq(ticket_id))
        .first::<Ticket>(&mut connection)
        .expect("Error fetching updated ticket")
}
