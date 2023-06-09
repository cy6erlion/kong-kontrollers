//! # 👥 Accounts kontroller
//!
//! The `accounts` __kontroller__ is used to manage user accounts.
//! It supports the following functions:
//!
//! - account creation
//! - ...
//!
//! > To use the accounts kontroller, enable it with the `accounts` Cargo
//! > feature.

pub mod create;
pub mod database;
pub mod inputs;

use chrono::prelude::*;
use inputs::AccountCreationInput;
use kong::{json, krypto, JsonValue};
use serde::{Deserialize, Serialize};

/// ## 😀 A generic user account
///
/// > Note that an `Account` contains private data, such as username
/// > and password, __it should not be published publicly__.
#[derive(Deserialize, Serialize)]
pub struct Account {
    //--- Required Data ---//
    /// Account's unique username
    pub username: String,
    /// Account's master key
    pub password: String,
    /// Date when account was created
    pub created: DateTime<Utc>,
    //--- Optional Personal Data ---//
    /// The fullname of the account holder.
    pub fullname: Option<String>,
    /// The date when the account holder was born.
    pub date_of_birth: Option<DateTime<Utc>>,
    /// ID number of the account owner
    pub id_number: Option<String>,
    /// The gender of the account holder
    pub gender: Option<String>,
    /// Short bio of Account
    pub description: Option<String>,
    //--- Optional Education Data ---//
    /// User's current school name
    pub current_school_name: Option<String>,
    /// User's student number
    pub student_number: Option<String>,
    //--- Optional Bussiness Data ---//
    /// Name of the account's bussiness
    pub bussiness_name: Option<String>,
    //--- Optional Contact Data ---//
    /// User's email address
    pub email: Option<String>,
    /// Account owner's mobile number
    pub mobile_number: Option<String>,
    /// Account owner's web-address
    pub website: Option<String>,
    //--- Optional Meta Data ---//
    /// Date account last logged in
    pub last_login: Option<DateTime<Utc>>,
    /// Type of account, eg `admin`
    pub account_type: Option<String>,
}

impl From<AccountCreationInput> for Account {
    fn from(input: AccountCreationInput) -> Account {
        let password = krypto::password::hash(&input.password).unwrap();

        Account {
            username: input.username,
            password,
            created: Utc::now(),
            fullname: None,
            date_of_birth: None,
            id_number: None,
            gender: None,
            current_school_name: None,
            student_number: None,
            bussiness_name: None,
            email: input.email,
            mobile_number: None,
            website: None,
            description: None,
            last_login: None,
            account_type: None,
        }
    }
}

///## 🥸 Account Public Data
///
/// This is public data of an account. __It can safely be published
/// publicly without invading the account holder's privacy__
#[derive(Deserialize, Serialize)]
pub struct PublicAccount {
    /// The username of the user, also used as an unique identifier
    pub username: String,
}

impl PublicAccount {
    /// convert to json value
    pub fn as_json(&self) -> JsonValue {
        json!({
            "username": self.username,
        })
    }
}
impl From<Account> for PublicAccount {
    fn from(account: Account) -> Self {
        PublicAccount {
            username: account.username,
        }
    }
}
