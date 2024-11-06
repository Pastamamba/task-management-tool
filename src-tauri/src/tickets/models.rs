use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;
use crate::schema::tickets;

#[derive(Serialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = tickets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ticket {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub description: Option<String>,
}
