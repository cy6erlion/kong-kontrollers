//! # ✨ Message receiving kontroller
//!
use super::database::Database;
use super::{ContactMessage, ContactMessageInput};
use crate::error::KontrollerError;
use kong::{inputs::UserInput, server, ErrorResponse, JsonValue, Kong, Kontrol, Method};
use std::sync::{Arc, Mutex};

/// ✨ Message receiving kontroller
pub struct ReceiveMessageKontroller {
    /// Address to kontroller
    pub address: String,
    /// HTTP method supported by the kontroller
    pub method: Method,
    /// SQLite database handle
    pub database: Arc<Mutex<Database>>,
}

impl Kontrol for ReceiveMessageKontroller {
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
        name: String,
            email: Option<String>,
        message: String
        }) {
            let input = ContactMessageInput {
                name: input.name,
                email: input.email,
                message: input.message,
            };
            Some(input.as_json())
        } else {
            None
        }
    }

    /// Validate user input
    fn validate(&self, input: Option<JsonValue>) -> Result<Option<JsonValue>, ()> {
        if let Some(input) = input {
            if let Ok(input) = ContactMessageInput::from_json_str(input.to_string()) {
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
            let input = ContactMessageInput::from_json_str(input.to_string());

            // Derive subscriber from string
            let message: ContactMessage = if let Ok(input) = input {
                input.into()
            } else {
                return ErrorResponse::bad_request();
            };

            // Store subscriber into the database
            let res = self.database.lock().unwrap().create_message(&message);

            match res {
                Ok(()) => server::Response::json(&message).with_status_code(201),
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
