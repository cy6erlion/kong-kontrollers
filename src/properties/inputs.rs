//! # ⌨️ User Input
//!
//! User input, used to manage properties.
use kong::{
    inputs::UserInput, json, json_from_str, validate::ValidationError, JsonError, JsonValue,
};
use serde::{Deserialize, Serialize};

/// ## ⌨️ Property creation input
#[derive(Serialize, Deserialize, Clone)]
pub struct CreatePropertyInput {
    pub name: String,
    pub price: Option<f64>,
    pub bedrooms: u16,
    pub bathrooms: u16,
    pub sqft: f64,
    pub address: String,
    pub agentid: u64,
    pub description: String,
    pub photos: String,
}

impl CreatePropertyInput {
    /// new generic resource
    pub fn as_json(&self) -> JsonValue {
        json!({
            "name": self.name,
            "price": self.price,
            "bedrooms": self.bedrooms,
            "bathrooms": self.bathrooms,
            "sqft": self.sqft,
            "address": self.address,
            "agentid": self.agentid,
            "description": self.description,
            "photos": self.photos
        })
    }

    /// from json
    pub fn from_json_str(json_str: String) -> Result<CreatePropertyInput, JsonError> {
        let a: CreatePropertyInput = json_from_str(&json_str)?;
        Ok(a)
    }
}

impl UserInput for CreatePropertyInput {
    fn is_valid(&self) -> Result<(), ValidationError> {
        // TODO: validate input properly
        Ok(())
    }
}
