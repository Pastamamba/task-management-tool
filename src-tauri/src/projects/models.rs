use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use crate::schema::projects;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = projects)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub status: &'a str,
}
