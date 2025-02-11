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

#[tauri::command]
pub fn get_projects() -> Vec<Project> {
    use crate::schema::projects::dsl::*;

    let mut connection = establish_connection();

    projects
        .select(Project::as_select())
        .load::<Project>(&mut connection)
        .expect("Error loading projects")
}

#[tauri::command]
pub fn get_project_by_id(project_id: i32) -> Project {
    use crate::schema::projects::dsl::*;

    let mut connection = establish_connection();

    projects
        .filter(id.eq(project_id))
        .select(Project::as_select())
        .first::<Project>(&mut connection)
        .expect("Error loading project")
}

/*#[tauri::command]
pub fn add_logged_hour_to_project(project_id: i32, logged_hour: LoggedHour) -> Project {
    use crate::schema::projects::dsl::*;

    let mut connection = establish_connection();

    // Hae Project
    let project = projects
        .filter(id.eq(project_id))
        .select(Project::as_select()) // Käytä .select(Project::as_select())
        .first::<Project>(&mut connection)
        .expect("Project not found");

    // Lisää uusi kirjaus
    let mut updated_logged_hours = project.logged_hours.unwrap_or_default();
    updated_logged_hours.push(logged_hour);

    // Laske total_hours
    let total_minutes: i32 = updated_logged_hours.iter().map(|lh| lh.minutes).sum();
    let total_hours_calculated = Some(total_minutes / 60); // 120 min = 2 h

    // Päivitä Project
    diesel::update(projects.filter(id.eq(project_id)))
        .set((
            logged_hours.eq(Some(updated_logged_hours)), // Tämä toimii nyt, koska ToSql on implementoitu
            total_hours.eq(total_hours_calculated),        // Tämä toimii, koska total_hours on Option<i32>
        ))
        .select(Project::as_select()) // Varmista, että Diesel tietää mitä palauttaa
        .get_result::<Project>(&mut connection)
        .expect("Error updating project")
}*/
