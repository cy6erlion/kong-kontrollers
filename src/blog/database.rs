//! # 🗄️ Blog posts database management

use super::{BlogPost, DatabaseBlogPostInput};
use crate::error::KontrollerError;
use rusqlite::{params, Connection};

/// SQL statements and queries
pub mod sql {
    /// Create user accounts table
    pub const CREATE_TABLE: &str = "
      CREATE TABLE IF NOT EXISTS blog (
        id INTEGER PRIMARY KEY,                      -- The Identifier of the blog post, the Rust Type is `i64`
        title TEXT NOT NULL,                         -- The title of the blog post
        subtitle TEXT,                               -- The subtitle of the blog post
        overview TEXT,                               -- The overview of the blog post
        author TEXT,                                 -- The username of the blog author
        cover TEXT,                                  -- The path to the cover image of the blog post        
        content TEXT NOT NULL,                       -- The actual content of the blog post
        date TEXT)                                   -- The date when the blog post was published`";

    /// Insert a blog post in the blog table
    pub const CREATE: &str = "
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

    /// Get blog by id
    pub const READ: &str = "SELECT * FROM blog WHERE id = :id;";

    /// Get all blog posts
    pub const READ_ALL: &str = "SELECT * FROM blog ORDER BY rowid DESC";

    /// Update article
    pub const UPDATE: &str = "
      UPDATE blog
      SET title = ?1,
        subtitle = ?2,
        overview = ?3,
        author = ?4,
        cover = ?5,
        content = ?6
      WHERE id = ?7;";

    /// Delete a blog post
    pub const DELETE: &str = "DELETE FROM blog WHERE id = :id;";
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

                tx.execute(sql::CREATE_TABLE, ())
                    .map_err(|_| KontrollerError::DbTableCreation)?;

                tx.commit().map_err(|_| KontrollerError::DbTableCreation)?;

                Ok(())
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Create a new blog
    pub fn create_blog(&self, blog: &DatabaseBlogPostInput) -> Result<(), KontrollerError> {
        match &self.conn {
            Some(conn) => {
                conn.execute(
                    sql::CREATE,
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
                    .prepare(sql::READ)
                    .map_err(|_| KontrollerError::DbSQL)?;
                let mut rows = stmt
                    .query(&[(":id", &id)])
                    .map_err(|_| KontrollerError::DbSQL)?;
                match rows.next().map_err(|_| KontrollerError::DbSQL)? {
                    Some(s) => Ok(Some(BlogPost {
                        id: s.get(0).map_err(|_| KontrollerError::DbField)?,
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

    /// Get all posts
    pub fn get_all(&self) -> Result<Vec<BlogPost>, KontrollerError> {
        match &self.conn {
            Some(conn) => {
                let mut posts: Vec<BlogPost> = vec![];
                let mut stmt = conn
                    .prepare(sql::READ_ALL)
                    .map_err(|_| KontrollerError::DbSQL)?;
                let posts_iter = stmt
                    .query_map([], |row| {
                        Ok(BlogPost {
                            id: row.get(0).unwrap(),
                            title: row.get(1).unwrap(), //.map_err(|_| KontrollerError::DbField)?,
                            subtitle: row.get(2).unwrap(), //.map_err(|_| KontrollerError::DbField)?,
                            overview: row.get(3).unwrap(), //.map_err(|_| KontrollerError::DbField)?,
                            author: row.get(4).unwrap(), //.map_err(|_| KontrollerError::DbField)?,
                            cover: row.get(5).unwrap(),  //.map_err(|_| KontrollerError::DbField)?,
                            content: row.get(6).unwrap(), //.map_err(|_| KontrollerError::DbField)?,
                            date: row.get(7).unwrap(),   //.map_err(|_| KontrollerError::DbField)?,
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

    /// Delete article
    pub fn delete(&self, id: i64) -> Result<(), KontrollerError> {
        match &self.conn {
            Some(conn) => {
                conn.execute(sql::DELETE, &[(":id", &format!("{id}"))])
                    .map_err(|_| KontrollerError::DbSQL)?;
                Ok(())
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Update article
    pub fn update(&mut self, id: i64, blog: &DatabaseBlogPostInput) -> Result<(), KontrollerError> {
        match &mut self.conn {
            Some(conn) => {
                let tx = conn
                    .transaction()
                    .map_err(|_| KontrollerError::DbTransaction)?;

                // Update title
                tx.execute(
                    "UPDATE blog SET title = ?1 WHERE id = ?2",
                    &[&blog.title, &format!("{id}")],
                )
                .map_err(|_| KontrollerError::DbTableCreation)?;

                // Update content
                tx.execute(
                    "UPDATE blog SET content = ?1 WHERE id = ?2",
                    &[&blog.content, &format!("{id}")],
                )
                .map_err(|_| KontrollerError::DbTableCreation)?;

                // Update subtitle
                if let Some(subtitle) = &blog.subtitle {
                    tx.execute(
                        "UPDATE blog SET subtitle = ?1 WHERE id = ?2",
                        &[subtitle, &format!("{id}")],
                    )
                    .map_err(|_| KontrollerError::DbTableCreation)?;
                }

                // Update overview
                if let Some(overview) = &blog.overview {
                    tx.execute(
                        "UPDATE blog SET overview = ?1 WHERE id = ?2",
                        &[overview, &format!("{id}")],
                    )
                    .map_err(|_| KontrollerError::DbTableCreation)?;
                }

                // Update author
                if let Some(author) = &blog.author {
                    tx.execute(
                        "UPDATE blog SET author = ?1 WHERE id = ?2",
                        &[author, &format!("{id}")],
                    )
                    .map_err(|_| KontrollerError::DbTableCreation)?;
                }

                // Update cover
                if let Some(cover) = &blog.cover {
                    tx.execute(
                        "UPDATE blog SET cover = ?1 WHERE id = ?2",
                        &[cover, &format!("{id}")],
                    )
                    .map_err(|_| KontrollerError::DbTableCreation)?;
                }

                tx.commit().map_err(|_| KontrollerError::DbTableCreation)?;

                Ok(())
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

        let blog = DatabaseBlogPostInput {
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
