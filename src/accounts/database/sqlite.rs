//! # 🗄️ Accounts database management
//!
use super::{Account, AccountDatabase, PublicAccount};
use crate::error::KontrollerError;
use rusqlite::{params, Connection};

/// Database management system
pub struct Database {
    /// Database file path
    path: String,
    /// An SQLite connection handle
    conn: Option<Connection>,
}

impl AccountDatabase for Database {
    /// Create a new database controller
    fn new(path: &str) -> Self {
        Database {
            path: path.to_string(),
            conn: None,
        }
    }

    /// Open SQLite connection, create tables
    fn connect(&mut self) -> Result<(), KontrollerError> {
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

                tx.execute("
      CREATE TABLE IF NOT EXISTS accounts (
        id INTEGER PRIMARY KEY,                      -- The Identifier of the account, the Rust Type is `i64`
        username TEXT UNIQUE NOT NULL,               -- The username of the account
        password TEXT NOT NULL,                      -- The user's login password
        created TEXT NOT NULL,                       -- The date when the account was created, the Rust Type is `chrono::DateTime`
        fullname TEXT,                               -- Fullname of the account
        date_of_birth TEXT,                          -- The date when the account holder was born
        id_number TEXT,                              -- ID number of the account owner
        gender TEXT,                                 -- The gender of the account holder
        email TEXT UNIQUE,                           -- The email address of the account
        mobile_number TEXT,                          -- Account owner's mobile number
        website TEXT,                                -- Account owner's web-address
        description TEXT,                            -- Short bio of Account
        last_login TEXT,                              -- Date account last logged in
        account_type TEXT)                           -- Type of account, eg `admin`", ())
                    .map_err(|_| KontrollerError::DbTableCreation)?;

                tx.commit().map_err(|_| KontrollerError::DbTableCreation)?;

                Ok(())
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Create a new account
    fn create_account(&mut self, account: &Account) -> Result<(), KontrollerError> {
        match &self.conn {
            Some(conn) => {
                conn.execute(
                    "INSERT INTO accounts (
                                username,
                                email,
                                password,
                                created
                              )
                              VALUES (?1, ?2, ?3, ?4)",
                    params![
                        &account.username,
                        &account.email,
                        account.password,
                        account.created
                    ],
                )
                .map_err(|_| KontrollerError::DbField)?;
                Ok(())
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Create a new admin account
    fn create_admin_account(&mut self, account: &Account) -> Result<(), KontrollerError> {
        match &self.conn {
            Some(conn) => {
                conn.execute(
                    "INSERT INTO accounts (
                                username,
                                email,
                                password,
                                created,
                                account_type
                              )
                              VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![
                        &account.username,
                        &account.email,
                        account.password,
                        account.created,
                        &account.account_type
                    ],
                )
                .map_err(|_| KontrollerError::DbField)?;

                Ok(())
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Get an account's public data by its username
    fn public_get_account_by_username(
        &mut self,
        username: &str,
    ) -> Result<Option<PublicAccount>, KontrollerError> {
        match &self.conn {
            Some(conn) => {
                let mut stmt = conn
                    .prepare("SELECT * FROM accounts WHERE username = :username;")
                    .map_err(|_| KontrollerError::DbSQL)?;
                let mut rows = stmt
                    .query(&[(":username", username)])
                    .map_err(|_| KontrollerError::DbSQL)?;
                match rows.next().map_err(|_| KontrollerError::DbSQL)? {
                    Some(s) => Ok(Some(PublicAccount {
                        username: s.get(2).map_err(|_| KontrollerError::DbField)?,
                    })),
                    None => Ok(None),
                }
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Get an account's public data by its email
    fn public_get_account_by_email(
        &mut self,
        email: &str,
    ) -> Result<Option<PublicAccount>, KontrollerError> {
        match &self.conn {
            Some(conn) => {
                let mut stmt = conn
                    .prepare("SELECT * FROM accounts WHERE email = :email;")
                    .map_err(|_| KontrollerError::DbSQL)?;
                let mut rows = stmt
                    .query(&[(":email", email)])
                    .map_err(|_| KontrollerError::DbSQL)?;
                match rows.next().map_err(|_| KontrollerError::DbSQL)? {
                    Some(s) => Ok(Some(PublicAccount {
                        username: s.get(2).map_err(|_| KontrollerError::DbField)?,
                    })),
                    None => Ok(None),
                }
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Get an account's private data by its email
    fn private_get_account_by_email(
        &mut self,
        email: &str,
    ) -> Result<Option<Account>, KontrollerError> {
        match &self.conn {
            Some(conn) => {
                let mut stmt = conn
                    .prepare("SELECT * FROM accounts WHERE email = :email;")
                    .map_err(|_| KontrollerError::DbSQL)?;
                let mut rows = stmt
                    .query(&[(":email", email)])
                    .map_err(|_| KontrollerError::DbSQL)?;
                match rows.next().map_err(|_| KontrollerError::DbSQL)? {
                    Some(s) => Ok(Some(Account {
                        username: s.get(1).map_err(|_| KontrollerError::DbField)?,
                        password: s.get(2).map_err(|_| KontrollerError::DbField)?,
                        created: s.get(3).map_err(|_| KontrollerError::DbField)?,
                        fullname: s.get(4).map_err(|_| KontrollerError::DbField)?,
                        date_of_birth: s.get(5).map_err(|_| KontrollerError::DbField)?,
                        id_number: s.get(6).map_err(|_| KontrollerError::DbField)?,
                        gender: s.get(7).map_err(|_| KontrollerError::DbField)?,
                        email: s.get(8).map_err(|_| KontrollerError::DbField)?,
                        mobile_number: s.get(9).map_err(|_| KontrollerError::DbField)?,
                        website: s.get(10).map_err(|_| KontrollerError::DbField)?,
                        description: s.get(11).map_err(|_| KontrollerError::DbField)?,
                        last_login: s.get(12).map_err(|_| KontrollerError::DbField)?,
                        account_type: s.get(13).map_err(|_| KontrollerError::DbField)?,
                    })),
                    None => Ok(None),
                }
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Get an account's private data by its username
    fn private_get_account_by_username(
        &mut self,
        username: &str,
    ) -> Result<Option<Account>, KontrollerError> {
        match &self.conn {
            Some(conn) => {
                let mut stmt = conn
                    .prepare("SELECT * FROM accounts WHERE username = :username;")
                    .map_err(|_| KontrollerError::DbSQL)?;
                let mut rows = stmt
                    .query(&[(":username", username)])
                    .map_err(|_| KontrollerError::DbSQL)?;
                match rows.next().map_err(|_| KontrollerError::DbSQL)? {
                    Some(s) => Ok(Some(Account {
                        username: s.get(1).map_err(|_| KontrollerError::DbField)?,
                        password: s.get(2).map_err(|_| KontrollerError::DbField)?,
                        created: s.get(3).map_err(|_| KontrollerError::DbField)?,
                        fullname: s.get(4).map_err(|_| KontrollerError::DbField)?,
                        date_of_birth: s.get(5).map_err(|_| KontrollerError::DbField)?,
                        id_number: s.get(6).map_err(|_| KontrollerError::DbField)?,
                        gender: s.get(7).map_err(|_| KontrollerError::DbField)?,
                        email: s.get(8).map_err(|_| KontrollerError::DbField)?,
                        mobile_number: s.get(9).map_err(|_| KontrollerError::DbField)?,
                        website: s.get(10).map_err(|_| KontrollerError::DbField)?,
                        description: s.get(11).map_err(|_| KontrollerError::DbField)?,
                        last_login: s.get(12).map_err(|_| KontrollerError::DbField)?,
                        account_type: s.get(13).map_err(|_| KontrollerError::DbField)?,
                    })),
                    None => Ok(None),
                }
            }
            None => Err(KontrollerError::DbConnection),
        }
    }
}
