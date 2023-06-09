//! # 🏠 Properties kontroller
//!
//! The `properties` __kontroller__ is used to manage real estate
//! properties. It supports the following functions:
//!
//! - Store properties in database
//! - Retrieve stored properties
//!
//! > To use the `properties` kontroller, enable it with the `properties` Cargo
//! > feature.
//!
//! The `properties` kontroller depends on the `login` kontroller,
//! enabling the `properties` kontroller automatically enables the
//! `login` kontroller.
pub mod create;
pub mod database;
pub mod get;
pub mod inputs;

use inputs::CreatePropertyInput;
use kong::{json, json_from_str, JsonError, JsonValue};
use serde::{Deserialize, Serialize};

/// ## 🏠 A real estate property.
#[derive(Serialize, Deserialize, Clone)]
pub struct Property {
    pub id: Option<u64>,
    pub name: String,
    pub price: Option<f64>,
    pub bedrooms: u16,
    pub bathrooms: u16,
    pub sqft: f64,
    pub address: String,
    pub agentid: u64,
    pub description: String,
    pub views: u64,
    pub likes: u64,
    pub bookmarks: u64,
    /// Contains a JSON string with paths to the actual images
    pub photos: String,
    pub added: String,
}
impl Property {
    /// new generic resource
    pub fn as_json(&self) -> JsonValue {
        json!({
        "id": self.id,
            "name": self.name,
            "price": self.price,
            "bedrooms": self.bedrooms,
            "bathrooms": self.bathrooms,
            "sqft": self.sqft,
            "address": self.address,
            "agentid": self.agentid,
            "description": self.description,
            "views": self.views,
            "likes": self.likes,
            "bookmarks": self.bookmarks,
            "photos": self.photos,
            "added": self.added
        })
    }

    /// from json
    pub fn from_json_str(json_str: String) -> Result<Property, JsonError> {
        let a: Property = json_from_str(&json_str)?;
        Ok(a)
    }
}
impl From<CreatePropertyInput> for Property {
    fn from(value: CreatePropertyInput) -> Self {
        Property {
            id: None,
            name: value.name,
            price: value.price,
            bedrooms: value.bedrooms,
            bathrooms: value.bathrooms,
            sqft: value.sqft,
            address: value.address,
            agentid: value.agentid,
            description: value.description,
            views: 0,
            likes: 0,
            bookmarks: 0,
            photos: value.photos,
            added: chrono::Utc::now().to_string(),
        }
    }
}
