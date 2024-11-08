use crate::db::establish_connection;
use crate::epics::models::Epic;
use crate::epics::models::NewEpic;
use diesel::prelude::*;

#[tauri::command]
pub fn create_epic(new_title: String, new_description: Option<String>) -> Epic {
    use crate::schema::epics::dsl::*;

    let mut connection = establish_connection();

    let new_epic = NewEpic {
        title: &new_title,
        description: new_description.as_deref(),
        status: "New",
    };

    diesel::insert_into(epics)
        .values(&new_epic)
        .get_result::<Epic>(&mut connection)
        .expect("Error saving new epic")
}

#[tauri::command]
pub fn get_epics() -> Vec<Epic> {
    use crate::schema::epics::dsl::*;

    let mut connection = establish_connection();

    epics
        .load::<Epic>(&mut connection)
        .expect("Error loading epics")
}

#[tauri::command]
pub fn get_epic_by_id(epic_id: i32) -> Epic {
    use crate::schema::epics::dsl::*;

    let mut connection = establish_connection();

    epics
        .filter(id.eq(epic_id))
        .first::<Epic>(&mut connection)
        .expect("Error loading epic")
}
