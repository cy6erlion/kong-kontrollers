//! # ⌨️ User Input

use chrono::prelude::*;
use kong::{
    inputs::UserInput, json, json_from_str, validate::ValidationError, JsonError, JsonValue,
};
use serde::{Deserialize, Serialize};

/// ## ⌨️ Blog creation input
#[derive(Serialize, Deserialize, Clone)]
pub struct ContactMessageInput {
    pub name: String,
    pub email: Option<String>,
    pub message: String,
}

impl ContactMessageInput {
    /// new generic resource
    pub fn as_json(&self) -> JsonValue {
        json!({
            "name": self.name,
            "email": self.email,
            "message": self.message
        })
    }

    /// from json
    pub fn from_json_str(json_str: String) -> Result<ContactMessageInput, JsonError> {
        let a: ContactMessageInput = json_from_str(&json_str)?;
        Ok(a)
    }
}

impl UserInput for ContactMessageInput {
    fn is_valid(&self) -> Result<(), ValidationError> {
        // TODO: validate input properly
        Ok(())
    }
}
