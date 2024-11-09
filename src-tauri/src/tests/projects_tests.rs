#[cfg(test)]
mod tests {
    use crate::db::establish_connection;
    use diesel::prelude::*;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use crate::projects::handlers::create_project;

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
}
