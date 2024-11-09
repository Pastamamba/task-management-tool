#[cfg(test)]
mod tests {
    use crate::db::establish_connection;
    use diesel::prelude::*;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use crate::projects::handlers::create_project;
    use crate::projects::handlers::get_projects;

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    #[test]
    fn test_create_project() {
        let connection = &mut establish_connection();

        connection
            .run_pending_migrations(MIGRATIONS)
            .expect("Migrations failed");

        connection.test_transaction::<_, (), _>(|_conn| {
            let title = String::from("Test Project");
            let description = Some(String::from("This is a test project"));
            let project = create_project(title.clone(), description.clone());

            assert_eq!(project.title, title);
            assert_eq!(project.description, description);
            assert_eq!(project.status, "New");

            Ok(())
        });
    }

    #[test]
    fn test_get_projects() {
        let connection = &mut establish_connection();

        connection
            .run_pending_migrations(MIGRATIONS)
            .expect("Migrations failed");

        connection.test_transaction::<_, (), _>(|_conn| {
            let title = String::from("Test Project");
            let description = Some(String::from("This is a test project"));
            let project = create_project(title.clone(), description.clone());

            let projects = get_projects();

            let found_project = projects.iter().find(|e| e.id == project.id).unwrap();

            assert_eq!(found_project.title, title);
            assert_eq!(found_project.description, description);
            assert_eq!(found_project.status, "New");

            Ok(())
        });
    }
}
