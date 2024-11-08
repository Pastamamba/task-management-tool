#[cfg(test)]
mod tests {
    use crate::db::establish_connection;
    use diesel::prelude::*;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use crate::epics::handlers::create_epic;

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    #[test]
    fn test_create_epic() {
        let connection = &mut establish_connection();

        connection
            .run_pending_migrations(MIGRATIONS)
            .expect("Migrations failed");

        connection.test_transaction::<_, (), _>(|_conn| {
            let title = String::from("Test Epic");
            let description = Some(String::from("This is a test epic"));
            let epic = create_epic(title.clone(), description.clone());

            assert_eq!(epic.title, title);
            assert_eq!(epic.description, description);
            assert_eq!(epic.status, "New");

            Ok(())
        });
    }

    #[test]
    fn test_get_epics() {
        let connection = &mut establish_connection();

        connection
            .run_pending_migrations(MIGRATIONS)
            .expect("Migrations failed");

        connection.test_transaction::<_, (), _>(|_conn| {
            let title = String::from("Test Epic");
            let description = Some(String::from("This is a test epic"));
            let epic = create_epic(title.clone(), description.clone());

            let epics = crate::epics::handlers::get_epics();

            let found_epic = epics.iter().find(|e| e.id == epic.id).unwrap();

            assert_eq!(found_epic.title, title);
            assert_eq!(found_epic.description, description);
            assert_eq!(found_epic.status, "New");

            Ok(())
        });
    }
}
