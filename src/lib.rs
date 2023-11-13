// lib.rs

use rusqlite::{Connection, Result};
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),
}

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let connection = Connection::open("my_database.db")?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS my_table (
                id INTEGER PRIMARY KEY,
                data_column TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Database { connection })
    }
    //create data
    pub fn create_data(&self, data: &str) -> Result<()> {
        self.connection.execute("INSERT INTO my_table (data_column) VALUES (?1)", &[data])?;
        Ok(())
    }

    //read data
    pub fn get_all_data(&self) -> Result<Vec<String>> {
        let mut stmt = self.connection.prepare("SELECT * FROM my_table")?;
        let data_iter = stmt.query_map([], |row| row.get(0))?;

        let mut result = Vec::new();
        for data in data_iter {
            result.push(data?);
        }

        Ok(result)
    }

    //update data
    pub fn update_data(&self, id: i32, new_data: &str) -> Result<()> {
        self.connection.execute("UPDATE my_table SET data_column = ?1 WHERE id = ?2", &[new_data, &id.to_string()])?;
        Ok(())
    }

    //delete data
    pub fn delete_data(&self, id: i32) -> Result<()> {
        self.connection.execute("DELETE FROM my_table WHERE id = ?1", &[&id.to_string()])?;
        Ok(())
    }
}
