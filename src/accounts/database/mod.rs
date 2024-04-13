//#[cfg(feature = "postgres")]
//pub mod postgres;
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
    fn create_account(&self, account: &Account) -> Result<(), KontrollerError>;

    /// Create a new admin account
    fn create_admin_account(&self, account: &Account) -> Result<(), KontrollerError>;

    /// Get an account's public data by its username
    fn public_get_account_by_username(
        &self,
        username: &str,
    ) -> Result<Option<PublicAccount>, KontrollerError>;

    /// Get an account's public data by its email
    fn public_get_account_by_email(
        &self,
        email: &str,
    ) -> Result<Option<PublicAccount>, KontrollerError>;

    /// Get an account's private data by its email
    fn private_get_account_by_email(&self, email: &str)
        -> Result<Option<Account>, KontrollerError>;

    /// Get an account's private data by its username
    fn private_get_account_by_username(
        &self,
        username: &str,
    ) -> Result<Option<Account>, KontrollerError>;
}
