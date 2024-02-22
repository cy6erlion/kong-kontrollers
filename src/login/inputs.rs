//! 🔑 User inputs for the `login` __kontroller__
//!
//! For an user account to login, they should provide the accounts
//! __username__ and __password__

use kong::{
    inputs::UserInput,
    json, json_from_str,
    validate::{Validate, ValidationError},
    JsonError, JsonValue,
};
use serde::{Deserialize, Serialize};

/// Account authentication input
#[derive(Serialize, Deserialize, Clone)]
pub struct AccountLoginInput {
    /// Account's username
    pub username: String,
    /// Account master key
    pub password: String,
}

impl AccountLoginInput {
    /// new generic resource
    pub fn as_json(&self) -> JsonValue {
        json!({
            "username": self.username,
            "password": self.password
        })
    }

    /// from json
    pub fn from_json_str(json_str: String) -> Result<AccountLoginInput, JsonError> {
        let a: AccountLoginInput = json_from_str(&json_str)?;
        Ok(a)
    }
}

impl UserInput for AccountLoginInput {
    fn is_valid(&self) -> Result<(), ValidationError> {
        if !Validate::username(&self.username) {
            return Err(ValidationError::Username);
        }

        if !Validate::password(&self.password) {
            return Err(ValidationError::Password);
        }

        Ok(())
    }
}
