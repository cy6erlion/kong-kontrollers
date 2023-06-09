//! # ✨ Account creation kontroller
//!
//! This __kontroller__ is used to create a new user account. It gets
//! the input from the HTTP request validates it and creates an
//! account from the validated input. The account is stored in an SQLite
//! database.
//!
//! ## 👮‍♂️ Admin accounts
//! If the input from the request uses the admin email that is in
//! the `konfig` file, the account that is created is an admin
//! account. Admin accounts simply means an account stored in the
//! database with it's `account_type` field as `admin`, from this
//! you can build up your own higher level abstractions.

use super::database::Database;
use super::{Account, AccountCreationInput, PublicAccount};
use crate::error::KontrollerError;
use kong::{inputs::UserInput, server, ErrorResponse, JsonValue, Kong, Kontrol, Method};
use std::sync::{Arc, Mutex};

/// ## ✨ Accounts creation kontroller
/// Can be used to create both admin and non-admin accounts
pub struct CreateAccountKontroller {
    /// Address to kontroller (url path)
    pub address: String,
    /// HTTP method supported by the kontroller
    pub method: Method,
    /// SQLite database handle
    pub database: Arc<Mutex<Database>>,
}

impl Kontrol for CreateAccountKontroller {
    /// kontroller address
    fn address(&self) -> String {
        self.address.clone()
    }

    /// Method supported by kontroller
    fn method(&self) -> Method {
        self.method
    }

    /// Get user input from HTTP request
    fn get_input(&self, request: &server::Request) -> Option<JsonValue> {
        let input: Option<AccountCreationInput> =
            if let Ok(input) = server::input::json_input(request) {
                Some(input)
            } else {
                None
            };

        if let Some(input) = input {
            Some(input.as_json())
        } else {
            None
        }
    }

    /// Validate user input
    fn validate(&self, input: Option<JsonValue>) -> Result<Option<JsonValue>, ()> {
        if let Some(input) = input {
            let input = AccountCreationInput::from_json_str(input.to_string());

            match input {
                Ok(input) => {
                    if input.is_valid().is_ok() {
                        Ok(Some(input.as_json()))
                    } else {
                        Err(())
                    }
                }
                Err(_) => Err(()),
            }
        } else {
            Err(())
        }
    }
    /// Create a new user
    fn kontrol(&self, kong: &Kong) -> server::Response {
        if let Some(input) = &kong.input {
            let input = AccountCreationInput::from_json_str(input.to_string());

            match input {
                Ok(input) => {
                    let mut account: Account = input.clone().into();

                    // create admin account
                    if input.email == kong.config.admin_email {
                        account.account_type = Some("admin".to_string());
                        match self.database.lock().unwrap().create_admin_account(&account) {
                            Ok(_) => {
                                let public_account: PublicAccount = account.into();
                                server::Response::json(&public_account).with_status_code(201)
                            }
                            Err(err) => match err {
                                KontrollerError::DbField => ErrorResponse::bad_request(),
                                _ => ErrorResponse::internal(),
                            },
                        }
                    } else {
                        // Create normal account
                        match self.database.lock().unwrap().create_account(&account) {
                            Ok(_) => {
                                let public_account: PublicAccount = account.into();
                                server::Response::json(&public_account).with_status_code(201)
                            }

                            Err(err) => match err {
                                KontrollerError::DbField => ErrorResponse::bad_request(),
                                _ => ErrorResponse::internal(),
                            },
                        }
                    }
                }

                Err(_) => ErrorResponse::internal(),
            }
        } else {
            ErrorResponse::bad_request()
        }
    }
}
