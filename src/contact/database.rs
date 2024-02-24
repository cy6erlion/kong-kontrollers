//! # 🗄️ Contact database management

use super::ContactMessage;
use crate::error::KontrollerError;
use rusqlite::{params, Connection};

/// SQL statements and queries
pub mod sql {
    /// Create message table
    pub const CREATE_MESSAGE_TABLE: &str = "
      CREATE TABLE IF NOT EXISTS message (
        id INTEGER PRIMARY KEY,                      -- The Identifier of the message, the Rust Type is `i64`
        name TEXT NOT NULL,                          -- Name of the sender
        email TEXT,                                  -- The email of the message
        message TEXT NOT NULL,                       -- The message
        date TEXT NOT NULL)                          -- The date when the message post was published`";

    /// Get message by id
    pub const GET_ALL_MESSAGES: &str = "SELECT * FROM message;";

    /// Insert a message in the message table
    pub const CREATE_MESSAGE: &str = "
      INSERT INTO message (
        name,
        email,
        message,
        date
       )
      VALUES (?1, ?2, ?3, ?4)";
}

/// Database management system
pub struct Database {
    /// Database file path
    path: String,
    /// An SQLite connection handle
    conn: Option<Connection>,
}

impl Database {
    /// Create a new database controller
    pub fn new(path: &str) -> Self {
        Database {
            path: path.to_string(),
            conn: None,
        }
    }

    /// Open SQLite connection, create tables
    pub fn connect(&mut self) -> Result<(), KontrollerError> {
        // Open database connection
        let conn =
            Connection::open(self.path.clone()).map_err(|_| KontrollerError::DbConnection)?;
        self.conn = Some(conn);

        // Create database tables if they do not already exist
        match &mut self.conn {
            Some(conn) => {
                let tx = conn
                    .transaction()
                    .map_err(|_| KontrollerError::DbTransaction)?;

                tx.execute(sql::CREATE_MESSAGE_TABLE, ())
                    .map_err(|_| KontrollerError::DbTableCreation)?;

                tx.commit().map_err(|_| KontrollerError::DbTableCreation)?;

                Ok(())
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Create a new message
    pub fn create_message(&self, message: &ContactMessage) -> Result<(), KontrollerError> {
        match &self.conn {
            Some(conn) => {
                conn.execute(
                    sql::CREATE_MESSAGE,
                    params![
                        &message.name,
                        &message.email,
                        &message.message,
                        message.date
                    ],
                )
                .map_err(|_| KontrollerError::DbField)?;
                Ok(())
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Get all messages
    pub fn get_all(&self) -> Result<Vec<ContactMessage>, KontrollerError> {
        match &self.conn {
            Some(conn) => {
                let mut posts: Vec<ContactMessage> = vec![];
                let mut stmt = conn
                    .prepare(sql::GET_ALL_MESSAGES)
                    .map_err(|_| KontrollerError::DbField)?;
                let posts_iter = stmt
                    .query_map([], |row| {
                        Ok(ContactMessage {
                            name: row.get(1)?,
                            email: row.get(2)?,
                            message: row.get(3)?,
                            date: row.get(4)?,
                        })
                    })
                    .map_err(|_| KontrollerError::DbField)?;

                for post in posts_iter {
                    posts.push(post.unwrap());
                }

                Ok(posts)
            }
            None => Err(KontrollerError::DbConnection),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;

    const TEST_DB_PATH: &str = "CONTACT_MESSAGES_TEST_DATABASE.sqlite";

    #[test]
    fn connect_db() {
        let mut db = Database::new(TEST_DB_PATH);

        // Connect to database
        db.connect().unwrap();

        match db.conn {
            Some(_conn) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_store_get_contact_message_subscriber() {
        remove_test_db();
        let mut db = Database::new(TEST_DB_PATH);

        let message = ContactMessage {
            name: "John".to_string(),
            email: Some("Test Title".to_string()),
            message: "Hi there!".to_string(),
            date: Utc::now(),
        };

        db.connect().unwrap();
        db.create_message(&message).unwrap();

        let res = db.get_all().unwrap();
        assert_eq!(res[0], message)
    }

    fn remove_test_db() {
        let test_db_path = std::path::Path::new(TEST_DB_PATH);
        if std::path::Path::exists(test_db_path) {
            std::fs::remove_file(test_db_path).unwrap();
        }
    }
}
