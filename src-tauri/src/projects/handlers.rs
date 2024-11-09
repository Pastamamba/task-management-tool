use crate::db::establish_connection;
use crate::projects::models::{NewProject, Project};
use diesel::prelude::*;

#[tauri::command]
pub fn create_project(new_title: String, new_description: Option<String>) -> Project {
    use crate::schema::projects::dsl::*;

    let mut connection = establish_connection();

    let new_project = NewProject {
        title: &new_title,
        description: new_description.as_deref(),
        status: "New",
    };

    diesel::insert_into(projects)
        .values(&new_project)
        .returning(Project::as_returning())
        .get_result::<Project>(&mut connection)
        .expect("Error saving new project")
}
