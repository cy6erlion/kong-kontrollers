//! # ⌨️ Account input
//!
//! User input data, that is used to create a new account.

use kong::{
    inputs::UserInput,
    json, json_from_str,
    validate::{Validate, ValidationError},
    JsonError, JsonValue,
};
use serde::Deserialize;

/// ## ⌨️ Account creation input
///
/// Data used as input to create a new account.
#[derive(Deserialize, Clone)]
pub struct AccountCreationInput {
    /// Account's username
    pub username: String,
    /// Account email address
    pub email: Option<String>,
    /// Account master key
    pub password: String,
}

impl AccountCreationInput {
    /// new generic resource
    pub fn as_json(&self) -> JsonValue {
        json!({
            "username": self.username,
            "email": self.email,
            "password": self.password
        })
    }

    /// from json
    pub fn from_json_str(json_str: String) -> Result<AccountCreationInput, JsonError> {
        let a: AccountCreationInput = json_from_str(&json_str)?;
        Ok(a)
    }
}
impl UserInput for AccountCreationInput {
    fn is_valid(&self) -> Result<(), ValidationError> {
        if !Validate::username(&self.username) {
            return Err(ValidationError::Username);
        }

        if !Validate::password(&self.password) {
            return Err(ValidationError::Password);
        }

        if let Some(email) = &self.email {
            if !Validate::email(email) {
                return Err(ValidationError::Email);
            }
        }

        Ok(())
    }
}
