//! # 🗄️ Blog posts database management

use super::BlogPost;
use crate::error::KontrollerError;
use rusqlite::{params, Connection};

/// SQL statements and queries
pub mod sql {
    /// Create user accounts table
    pub const CREATE_BLOG_TABLE: &str = "
      CREATE TABLE IF NOT EXISTS blog (
        id INTEGER PRIMARY KEY,                      -- The Identifier of the blog post, the Rust Type is `i64`
        title TEXT NOT NULL,                         -- The title of the blog post
        subtitle TEXT,                               -- The subtitle of the blog post
        overview TEXT,                               -- The overview of the blog post
        author TEXT,                                 -- The username of the blog author
        cover TEXT,                                  -- The path to the cover image of the blog post        
        content TEXT NOT NULL,                       -- The actual content of the blog post
        date TEXT)                                   -- The date when the blog post was published`";

    /// Get blog by id
    pub const GET_BLOG_BY_ID: &str = "SELECT * FROM blog WHERE id = :id;";

    /// Insert a blog post in the blog table
    pub const CREATE_BLOG: &str = "
      INSERT INTO blog (
        title,
        subtitle,
        overview,
        author,
        cover,
        content,
        date
       )
      VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)";
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

                tx.execute(sql::CREATE_BLOG_TABLE, ())
                    .map_err(|_| KontrollerError::DbTableCreation)?;

                tx.commit().map_err(|_| KontrollerError::DbTableCreation)?;

                Ok(())
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Create a new blog
    pub fn create_blog(&self, blog: &BlogPost) -> Result<(), KontrollerError> {
        match &self.conn {
            Some(conn) => {
                conn.execute(
                    sql::CREATE_BLOG,
                    params![
                        &blog.title,
                        &blog.subtitle,
                        blog.overview,
                        blog.author,
                        blog.cover,
                        blog.content,
                        blog.date
                    ],
                )
                .map_err(|_| KontrollerError::DbField)?;
                Ok(())
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Get blog public data by its username
    pub fn get_blog_by_id(&self, id: i64) -> Result<Option<BlogPost>, KontrollerError> {
        match &self.conn {
            Some(conn) => {
                let mut stmt = conn
                    .prepare(sql::GET_BLOG_BY_ID)
                    .map_err(|_| KontrollerError::DbSQL)?;
                let mut rows = stmt
                    .query(&[(":id", &id)])
                    .map_err(|_| KontrollerError::DbSQL)?;
                match rows.next().map_err(|_| KontrollerError::DbSQL)? {
                    Some(s) => Ok(Some(BlogPost {
                        title: s.get(1).map_err(|_| KontrollerError::DbField)?,
                        subtitle: s.get(2).map_err(|_| KontrollerError::DbField)?,
                        overview: s.get(3).map_err(|_| KontrollerError::DbField)?,
                        author: s.get(4).map_err(|_| KontrollerError::DbField)?,
                        cover: s.get(5).map_err(|_| KontrollerError::DbField)?,
                        content: s.get(6).map_err(|_| KontrollerError::DbField)?,
                        date: s.get(7).map_err(|_| KontrollerError::DbField)?,
                    })),
                    None => Ok(None),
                }
            }
            None => Err(KontrollerError::DbConnection),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;

    const TEST_DB_PATH: &str = "BLOG_TEST_DATABASE.sqlite";

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
    fn test_store_get_account_blog() {
        remove_test_db();
        let mut db = Database::new(TEST_DB_PATH);

        let blog = BlogPost {
            title: "Test Title".to_string(),
            subtitle: Some("Test subtitle".to_string()),
            overview: Some("Test overview".to_string()),
            author: Some("Test author".to_string()),
            cover: None,
            content: "Test content".to_string(),
            date: None,
        };

        db.connect().unwrap();
        db.create_blog(&blog).unwrap();

        let res = db.get_blog_by_id(1).unwrap();

        if let Some(r) = res {
            assert_eq!(r, blog)
        } else {
            panic!("Account not found")
        }
    }

    fn remove_test_db() {
        let test_db_path = std::path::Path::new(TEST_DB_PATH);
        if std::path::Path::exists(test_db_path) {
            std::fs::remove_file(test_db_path).unwrap();
        }
    }
}
