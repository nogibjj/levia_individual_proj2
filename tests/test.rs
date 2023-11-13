// tests/test.rs

use my_project::Database;
use std::error::Error;

#[test]
fn test_database_operations() -> Result<(), Box<dyn Error>> {
    let db = Database::new()?;

    // Test create data
    db.create_data("Test data 1")?;
    let mut data = db.get_all_data()?;
    assert_eq!(data.len(), 1);
    assert_eq!(data[0], "Test data 1");

    // Test update data
    db.update_data(1, "Updated Test data")?;
    data = db.get_all_data()?;
    assert_eq!(data.len(), 1);
    assert_eq!(data[0], "Updated Test data");

    // Test delete data
    db.delete_data(1)?;
    data = db.get_all_data()?;
    assert!(data.is_empty());

    Ok(())
}
