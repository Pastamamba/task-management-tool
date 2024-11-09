use crate::schema::projects;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Epic {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
}

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewEpic<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub status: &'a str,
}
