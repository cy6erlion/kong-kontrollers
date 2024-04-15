//! # 🗄️ Accounts database management
//!
use super::{Account, PublicAccount};
use crate::error::KontrollerError;
use rusqlite::{params, Connection};

/// Database management system
pub struct Database;

impl Database {
    /// Open SQLite connection, create tables
    pub fn init(conn: &mut Connection) -> Result<(), KontrollerError> {
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

    /// Create a new account
    pub fn create_account(conn: &mut Connection, account: &Account) -> Result<(), KontrollerError> {
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

    /// Create a new admin account
    pub fn create_admin_account(
        conn: &mut Connection,
        account: &Account,
    ) -> Result<(), KontrollerError> {
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

    /// Get an account's public data by its username
    pub fn public_get_account_by_username(
        conn: &mut Connection,
        username: &str,
    ) -> Result<Option<PublicAccount>, KontrollerError> {
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

    /// Get an account's public data by its email
    pub fn public_get_account_by_email(
        conn: &mut Connection,
        email: &str,
    ) -> Result<Option<PublicAccount>, KontrollerError> {
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

    /// Get an account's private data by its email
    pub fn private_get_account_by_email(
        conn: &mut Connection,
        email: &str,
    ) -> Result<Option<Account>, KontrollerError> {
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

    /// Get an account's private data by its username
    pub fn private_get_account_by_username(
        conn: &mut Connection,
        username: &str,
    ) -> Result<Option<Account>, KontrollerError> {
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
}
