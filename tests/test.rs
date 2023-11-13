// In tests/test.rs

use my_project::Database; 
use rusqlite::{Result, NO_PARAMS};

// Function to create a new, isolated database for each test
fn setup_test_db() -> Result<Database> {
    let db = Database::new_in_memory()?; 
    db.connection.execute(
        "CREATE TABLE IF NOT EXISTS my_table (
            id INTEGER PRIMARY KEY,
            data_column TEXT NOT NULL
        )", NO_PARAMS)?;
    Ok(db)
}

#[test]
fn test_create_data() -> Result<()> {
    let db = setup_test_db()?;

    db.create_data("Test data")?;
    let data = db.get_all_data()?;
    assert!(data.contains(&"Test data".to_string()));

    Ok(())
}

#[test]
fn test_read_data() -> Result<()> {
    let db = setup_test_db()?;

    db.create_data("Test data")?;
    let data = db.get_all_data()?;
    assert!(!data.is_empty());

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
