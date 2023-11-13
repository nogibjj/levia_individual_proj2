use my_project::Database;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let db = Database::new()?;

    // Create data
    db.create_data("New data")?;
    println!("Data creation successful.");

    // Read data
    let data = db.get_all_data()?;
    println!("Data retrieved:");
    for item in &data {
        println!("{}", item);
    }


    let update_id = 1;
    db.update_data(update_id, "Updated data")?;
    println!("Data updated successfully for ID {}.", update_id);

    // Read data again to show the update effect
    let updated_data = db.get_all_data()?;
    println!("Data after update:");
    for item in &updated_data {
        println!("{}", item);
    }


    let delete_id = 1;
    db.delete_data(delete_id)?;
    println!("Data deleted successfully for ID {}.", delete_id);

    // Finally, read data again to confirm the delete effect
    let final_data = db.get_all_data()?;
    println!("Current data:");
    for item in &final_data {
        println!("{}", item);
    }

    Ok(())
}
