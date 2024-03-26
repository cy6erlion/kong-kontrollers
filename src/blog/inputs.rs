//! # ⌨️ User Input

use kong::{
    inputs::UserInput, json, json_from_str, validate::ValidationError, JsonError, JsonValue,
};
use serde::{Deserialize, Serialize};

/// ## ⌨️ Blog creation input
#[derive(Serialize, Deserialize, Clone)]
pub struct CreateBlogInput {
    pub title: String,
    pub subtitle: Option<String>,
    pub overview: Option<String>,
    pub author: Option<String>,
    pub cover: Option<String>,
    pub content: String,
}

impl CreateBlogInput {
    /// new generic resource
    pub fn as_json(&self) -> JsonValue {
        json!({
            "title": self.title,
            "subtitle": self.subtitle,
            "overview": self.overview,
            "author": self.author,
            "cover": self.cover,
            "content": self.content
        })
    }

    /// from json
    pub fn from_json_str(json_str: String) -> Result<CreateBlogInput, JsonError> {
        let a: CreateBlogInput = json_from_str(&json_str)?;
        Ok(a)
    }
}

impl UserInput for CreateBlogInput {
    fn is_valid(&self) -> Result<(), ValidationError> {
        // TODO: validate input properly
        Ok(())
    }
}
