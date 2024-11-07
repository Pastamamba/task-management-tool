use diesel::{Insertable, Queryable, Selectablem, Associations};
use serde::{Serialize, Deserialize};
use crate::schema::epics;
use crate::tickets::models::Ticket;

#[derive(Serialize, Deserialize, Queryable, Insertable, Selectable, Associations)]
#[diesel(table_name = epics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Epic {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub description: Option<String>,
    pub tickets: Vec<Ticket>,
}

pub struct NewEpic {
    pub title: String,
    pub description: Option<String>,
}
