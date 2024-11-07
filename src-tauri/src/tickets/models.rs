use crate::epics::models::Epic;
use crate::schema::tickets;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Insertable,
    Associations,
    Identifiable,
    Selectable,
)]
#[diesel(table_name = tickets)]
#[diesel(belongs_to(Epic, foreign_key = epic_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ticket {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub description: Option<String>,
    pub epic_id: Option<i32>,
}
