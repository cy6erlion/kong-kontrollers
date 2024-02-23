//! # 🗄️ Newsletter database management

use super::Subscriber;
use crate::error::KontrollerError;
use rusqlite::{params, Connection};

/// SQL statements and queries
pub mod sql {
    /// Create subscriber table
    pub const CREATE_SUBSCRIBER_TABLE: &str = "
      CREATE TABLE IF NOT EXISTS subscriber (
        id INTEGER PRIMARY KEY,                      -- The Identifier of the subscriber, the Rust Type is `i64`
        email TEXT UNIQUE NOT NULL,                  -- The email of the subscriber
        date TEXT)                                   -- The date when the subscriber post was published`";

    /// Get subscriber by id
    pub const GET_ALL_SUBSCRIBERS: &str = "SELECT * FROM subscriber;";

    /// Insert a subscriber in the subscriber table
    pub const CREATE_SUBSCRIBER: &str = "
      INSERT INTO subscriber (
        email,
        date
       )
      VALUES (?1, ?2)";
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

                tx.execute(sql::CREATE_SUBSCRIBER_TABLE, ())
                    .map_err(|_| KontrollerError::DbTableCreation)?;

                tx.commit().map_err(|_| KontrollerError::DbTableCreation)?;

                Ok(())
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Create a new subscriber
    pub fn create_subscriber(&self, subscriber: &Subscriber) -> Result<(), KontrollerError> {
        match &self.conn {
            Some(conn) => {
                conn.execute(
                    sql::CREATE_SUBSCRIBER,
                    params![&subscriber.email, subscriber.date],
                )
                .map_err(|_| KontrollerError::DbField)?;
                Ok(())
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Get all post
    pub fn get_all(&self) -> Result<Vec<Subscriber>, KontrollerError> {
        match &self.conn {
            Some(conn) => {
                let mut posts: Vec<Subscriber> = vec![];
                let mut stmt = conn
                    .prepare(sql::GET_ALL_SUBSCRIBERS)
                    .map_err(|_| KontrollerError::DbField)?;
                let posts_iter = stmt
                    .query_map([], |row| {
                        Ok(Subscriber {
                            email: row.get(1)?,
                            date: row.get(2)?,
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

    const TEST_DB_PATH: &str = "NEWSLETTER_TEST_DATABASE.sqlite";

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
    fn test_store_get_newsletter_subscriber() {
        remove_test_db();
        let mut db = Database::new(TEST_DB_PATH);

        let subscriber = Subscriber {
            email: "Test Title".to_string(),
            date: Utc::now(),
        };

        db.connect().unwrap();
        db.create_subscriber(&subscriber).unwrap();

        let res = db.get_all().unwrap();
        assert_eq!(res[0], subscriber)
    }

    fn remove_test_db() {
        let test_db_path = std::path::Path::new(TEST_DB_PATH);
        if std::path::Path::exists(test_db_path) {
            std::fs::remove_file(test_db_path).unwrap();
        }
    }
}
