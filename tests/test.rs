

use my_project::Database; 
use rusqlite::Result;

// Test creating data functionality
#[test]
fn test_create_data() -> Result<()> {
    let db = Database::new()?;

    // Add some test data
    db.create_data("Test data")?;

    // Verify if the data was added
    let data = db.get_all_data()?;
    assert!(data.contains(&"Test data".to_string()));

    Ok(())
}

// Test reading data functionality
#[test]
fn test_read_data() -> Result<()> {
    let db = Database::new()?;

    // Retrieve data
    let data = db.get_all_data()?;

    // Verify if the data exists
    assert!(!data.is_empty());

    Ok(())
}

// Test updating data functionality
#[test]
fn test_update_data() -> Result<()> {
    let db = Database::new()?;
    
    // First, add some test data
    db.create_data("Old data")?;
    
    // Assuming we update the record with ID 1
    let update_id = 1;
    db.update_data(update_id, "Updated data")?;

    // Verify if the data was updated
    let data = db.get_all_data()?;
    assert!(data.contains(&"Updated data".to_string()));

    Ok(())
}

// Test deleting data functionality
#[test]
fn test_delete_data() -> Result<()> {
    let db = Database::new()?;

    // Add some data to delete
    db.create_data("Data to delete")?;
    
    // Assuming we delete the record with ID 1
    let delete_id = 1;
    db.delete_data(delete_id)?;

    // Verify if the data was deleted
    let data = db.get_all_data()?;
    assert!(!data.contains(&"Data to delete".to_string()));

    Ok(())
}
