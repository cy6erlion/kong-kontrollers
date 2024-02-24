//! contact kontroller

use chrono::prelude::*;
pub use input::ContactMessageInput;
use serde::{Deserialize, Serialize};

pub mod database;
mod input;
pub mod receive;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ContactMessage {
    /// Name of the sender
    name: String,
    /// Sender email
    email: Option<String>,
    /// The message
    message: String,
    /// Date of message was sent
    pub date: DateTime<Utc>,
}

impl From<ContactMessageInput> for ContactMessage {
    fn from(value: ContactMessageInput) -> Self {
        ContactMessage {
            name: value.name,
            email: value.email,
            message: value.message,
            // TODO: don't hardcode this, allow to use date from
            // user input
            date: Utc::now(),
        }
    }
}
