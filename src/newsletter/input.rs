//! # ⌨️ User Input

use chrono::prelude::*;
use kong::{
    inputs::UserInput, json, json_from_str, validate::ValidationError, JsonError, JsonValue,
};
use serde::{Deserialize, Serialize};

/// ## ⌨️ Blog creation input
#[derive(Serialize, Deserialize, Clone)]
pub struct SubscribeNewsletterInput {
    pub email: String,
}

impl SubscribeNewsletterInput {
    /// new generic resource
    pub fn as_json(&self) -> JsonValue {
        json!({
            "email": self.email
        })
    }

    /// from json
    pub fn from_json_str(json_str: String) -> Result<SubscribeNewsletterInput, JsonError> {
        let a: SubscribeNewsletterInput = json_from_str(&json_str)?;
        Ok(a)
    }
}

impl UserInput for SubscribeNewsletterInput {
    fn is_valid(&self) -> Result<(), ValidationError> {
        // TODO: validate input properly
        Ok(())
    }
}
