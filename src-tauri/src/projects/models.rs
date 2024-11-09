use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewProject<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub status: &'a str,
}
