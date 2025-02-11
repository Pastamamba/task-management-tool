use crate::schema::epics;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use crate::common::models::LoggedHour;
use crate::projects::models::Project;

#[derive(Serialize,
    Deserialize,
    Queryable,
    Insertable,
    Associations,
    Identifiable,
    Selectable,)]
#[diesel(table_name = epics)]
#[diesel(belongs_to(Project, foreign_key = project_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Epic {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub project_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub logged_hours: Option<Vec<LoggedHour>>,
    pub total_hours: i32,
}

#[derive(Insertable)]
#[diesel(table_name = epics)]
pub struct NewEpic<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub status: &'a str,
}
