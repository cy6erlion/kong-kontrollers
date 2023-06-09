//! # 🚨 Kontroller Errors
//!
//! Contains errors that can occur in a __kontroller__

use std::fmt;

/// # 🚨 Kontroller Errors
#[derive(Debug)]
pub enum KontrollerError {
    /// Database connection error
    DbConnection,
    /// Database table creation error
    DbTableCreation,
    /// Database transaction error
    DbTransaction,
    /// Database sql statement error
    DbSQL,
    /// Database field refferencing error
    DbField,
    /// Password hashing error
    PasswordHashing,
    /// Password hash verification
    PasswordVerifyHash,
    /// Configuration error
    ConfigError,
}

impl std::error::Error for KontrollerError {}

impl fmt::Display for KontrollerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DbConnection => write!(f, "Database connection error"),
            Self::DbTableCreation => write!(
                f,
                "Ann error occured while trying to create a database table"
            ),
            Self::DbTransaction => write!(f, "Database transaction error"),
            Self::DbSQL => write!(f, "Something went wrong while processing the SQL statement"),
            Self::DbField => write!(f, "Could not refference the database table field"),
            Self::PasswordHashing => write!(f, "Could not hash password"),
            Self::PasswordVerifyHash => write!(f, "Could not verify password hash"),
            Self::ConfigError => write!(f, "Could not read config file"),
        }
    }
}
