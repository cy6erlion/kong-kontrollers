//! Newsletter Kontroller

use chrono::prelude::*;
pub use input::SubscribeNewsletterInput;
use serde::{Deserialize, Serialize};

pub mod database;
mod input;
pub mod subscribe;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Subscriber {
    /// Subscribers email
    email: String,
    /// Date of subscription
    pub date: DateTime<Utc>,
}

impl From<SubscribeNewsletterInput> for Subscriber {
    fn from(value: SubscribeNewsletterInput) -> Self {
        Subscriber {
            email: value.email,
            // TODO: don't hardcode this, allow to use date from
            // user input
            date: Utc::now(),
        }
    }
}
