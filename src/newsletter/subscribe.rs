//! # ✨ Newsletter subscription kontroller
//!
use super::database::Database;
use super::{SubscribeNewsletterInput, Subscriber};
use crate::error::KontrollerError;
use kong::{inputs::UserInput, server, ErrorResponse, JsonValue, Kong, Kontrol, Method};
use std::sync::{Arc, Mutex};

/// ✨ Newsletter subscription kontroller
pub struct SubscribeNewsletterKontroller {
    /// Address to kontroller
    pub address: String,
    /// HTTP method supported by the kontroller
    pub method: Method,
    /// SQLite database handle
    pub database: Arc<Mutex<Database>>,
}

impl Kontrol for SubscribeNewsletterKontroller {
    /// Endpoint's address
    fn address(&self) -> String {
        self.address.clone()
    }

    /// Method supported by endpoint
    fn method(&self) -> Method {
        self.method
    }

    fn get_input(&self, request: &server::Request) -> Option<JsonValue> {
        if let Ok(input) = server::post_input!(request, {
            email: String,
        }) {
            let input = SubscribeNewsletterInput { email: input.email };
            Some(input.as_json())
        } else {
            None
        }
    }

    /// Validate user input
    fn validate(&self, input: Option<JsonValue>) -> Result<Option<JsonValue>, ()> {
        if let Some(input) = input {
            if let Ok(input) = SubscribeNewsletterInput::from_json_str(input.to_string()) {
                if input.is_valid().is_ok() {
                    Ok(Some(input.as_json()))
                } else {
                    // TODO: proper error handling
                    Err(())
                }
            } else {
                // TODO: proper error handling
                Err(())
            }
        } else {
            // TODO: proper error handling
            Err(())
        }
    }
    /// Add subscriber
    fn kontrol(&self, kong: &Kong) -> server::Response {
        if let Some(input) = &kong.input {
            let input = SubscribeNewsletterInput::from_json_str(input.to_string());

            // Derive subscriber from string
            let subscriber: Subscriber = if let Ok(input) = input {
                input.into()
            } else {
                return ErrorResponse::bad_request();
            };

            // Store subscriber into the database
            let res = self.database.lock().unwrap().create_subscriber(&subscriber);

            match res {
                Ok(()) => server::Response::json(&subscriber).with_status_code(201),
                Err(err) => match err {
                    KontrollerError::DbField => ErrorResponse::bad_request(),
                    _ => ErrorResponse::internal(),
                },
            }
        } else {
            ErrorResponse::unauthorized()
        }
    }
}
