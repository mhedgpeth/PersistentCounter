#[cfg(test)]
mod tests {
    use rusqlite::Connection;
    use shared::database::{fetch_counter, get_db_path, initialize_db, update_counter};
    use std::fs;
    use tempfile::tempdir;

    fn setup_test_db() -> (Connection, tempfile::TempDir) {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let db_path = temp_dir.path().join("test.db");

        // Initialize the database
        let conn = initialize_db(db_path.to_str().unwrap()).expect("Failed to initialize database");

        (conn, temp_dir)
    }

    #[test]
    fn test_db_initialization() {
        let (conn, _temp_dir) = setup_test_db();

        // Verify the counters table exists
        let table_count: i32 = conn
            .query_row(
                "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='counters'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(table_count, 1, "Counters table should exist");

        // Verify the 'app' counter was created
        let app_counter_count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM counters WHERE name = ?1",
                ["app"],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(app_counter_count, 1, "App counter should exist");
    }

    #[test]
    fn test_counter_operations() {
        let (conn, _temp_dir) = setup_test_db();

        // Get the ID of the 'app' counter
        let app_id: isize = conn
            .query_row("SELECT id FROM counters WHERE name = ?1", ["app"], |row| {
                row.get(0)
            })
            .unwrap();

        // Test initial counter value
        let initial_count = fetch_counter(&conn, app_id).unwrap();
        assert_eq!(initial_count, 0, "Initial counter value should be 0");

        // Test updating counter
        update_counter(&conn, app_id, 42).unwrap();
        let updated_count = fetch_counter(&conn, app_id).unwrap();
        assert_eq!(updated_count, 42, "Counter should be updated to 42");
    }

    #[test]
    fn test_get_db_path() {
        let path = get_db_path().unwrap();
        assert!(path.ends_with("persistent-counter/counter.db"));

        // Clean up any created directories
        if let Some(parent) = path.parent() {
            let _ = fs::remove_dir_all(parent);
        }
    }
}
