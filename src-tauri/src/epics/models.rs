use crate::schema::epics;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = epics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Epic {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub project_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = epics)]
pub struct NewEpic<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub status: &'a str,
}
