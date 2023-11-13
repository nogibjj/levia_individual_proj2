// In tests/test.rs

use my_project::Database; 
use rusqlite::Result;
use std::fs;

// Function to create a new, isolated database for each test
fn setup_test_db() -> Result<(Database, String)> {
    // Using a random file name for each test database
    let db_path = format!("test_db_{}.db", uuid::Uuid::new_v4());
    let db = Database::new(&db_path)?;

    db.connection.execute(
        "CREATE TABLE IF NOT EXISTS my_table (
            id INTEGER PRIMARY KEY,
            data_column TEXT NOT NULL
        )", [])?;

    Ok((db, db_path))
}

// A helper function to clean up the database file after each test
fn cleanup_test_db(db_path: &str) {
    let _ = fs::remove_file(db_path);
}

#[test]
fn test_create_data() -> Result<()> {
    let (db, db_path) = setup_test_db()?;
    
    db.create_data("Test data")?;
    let data = db.get_all_data()?;
    assert!(data.contains(&"Test data".to_string()));

    cleanup_test_db(&db_path);
    Ok(())
}

#[test]
fn test_read_data() -> Result<()> {
    let (db, db_path) = setup_test_db()?;

    db.create_data("Test data")?;
    let data = db.get_all_data()?;
    assert!(!data.is_empty());

    cleanup_test_db(&db_path);
    Ok(())
}


#[test]
fn test_update_data() -> Result<()> {
    let db = setup_test_db()?;

    db.create_data("Old data")?;
    let update_id = 1;
    db.update_data(update_id, "Updated data")?;
    let data = db.get_all_data()?;
    assert!(data.contains(&"Updated data".to_string()));

    Ok(())
}

#[test]
fn test_delete_data() -> Result<()> {
    let db = setup_test_db()?;

    db.create_data("Data to delete")?;
    let delete_id = 1;
    db.delete_data(delete_id)?;
    let data = db.get_all_data()?;
    assert!(!data.contains(&"Data to delete".to_string()));

    Ok(())
}
