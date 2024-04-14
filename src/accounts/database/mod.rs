#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "sqlite")]
pub mod sqlite;

use super::{Account, PublicAccount};
use crate::error::KontrollerError;

pub trait AccountDatabase {
    /// Create a new accounts database
    fn new(location: &'static str) -> Self;

    /// Connect database
    fn connect(&mut self) -> Result<(), KontrollerError>;

    /// Create a new account
    fn create_account(&mut self, account: &Account) -> Result<(), KontrollerError>;

    /// Create a new admin account
    fn create_admin_account(&mut self, account: &Account) -> Result<(), KontrollerError>;

    /// Get an account's public data by its username
    #[cfg(feature = "sqlite")]
    fn public_get_account_by_username(
        &mut self,
        username: &str,
    ) -> Result<Option<PublicAccount>, KontrollerError>;

    /// Get an account's public data by its email
    fn public_get_account_by_email(
        &mut self,
        email: &str,
    ) -> Result<Option<PublicAccount>, KontrollerError>;

    /// Get an account's private data by its email
    fn private_get_account_by_email(
        &mut self,
        email: &str,
    ) -> Result<Option<Account>, KontrollerError>;

    /// Get an account's private data by its username
    fn private_get_account_by_username(
        &mut self,
        username: &str,
    ) -> Result<Option<Account>, KontrollerError>;
}

/// SQL statements and queries
pub mod sql {

    /// Get account by username
    pub const GET_ACCOUNT_BY_USERNAME: &str = "SELECT * FROM accounts WHERE username = :username;";

    /// Get account by email
    pub const GET_ACCOUNT_BY_EMAIL: &str = "SELECT * FROM accounts WHERE email = :email;";

    /// Insert a account in the accounts table
    pub const CREATE_ACCOUNT: &str = "
      INSERT INTO accounts (
        username,
        email,
        password,
        created
       )
      VALUES (?1, ?2, ?3, ?4)";

    /// Insert a admin account in the accounts table
    pub const CREATE_ADMIN_ACCOUNT: &str = "
      INSERT INTO accounts (
        username,
        email,
        password,
        created,
        account_type
       )
      VALUES (?1, ?2, ?3, ?4, ?5)";
}
