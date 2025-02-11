use crate::common::models::LoggedHour;
use crate::schema::projects;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable, QueryableByName};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, QueryableByName, Selectable)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub logged_hours: Option<Vec<LoggedHour>>,
    pub total_hours: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub status: &'a str,
}
