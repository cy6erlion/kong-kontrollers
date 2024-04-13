//! # 🔓 Login kontroller
//!
//! The `login` __kontroller__ allows an account (from the `account`
//! kontroller) to login. The kontroller issues a `kpassport` token to
//! the account, if it provides the valid login credentials (username
//! accounts password).
//!
//! > To use the login kontroller, enable it with the `login` Cargo
//! > feature.
//!
//! The `login` kontroller depends on the `account` kontroller,
//! enabling the `login` kontroller automatically enables the
//! `accounts` kontroller.
pub mod inputs;

use crate::accounts::{database::AccountDatabase, Account};
use crate::error::KontrollerError;
use inputs::AccountLoginInput;
use kong::krypto::kpassport::Kpassport;
use kong::{inputs::UserInput, krypto, server, ErrorResponse, JsonValue, Kong, Kontrol, Method};
use serde::Serialize;
use std::sync::{Arc, Mutex};

pub fn is_admin<D: AccountDatabase>(
    k: &Kpassport,
    db: Arc<Mutex<D>>,
) -> Result<bool, KontrollerError> {
    let username = &k.content.username;

    // Find user account in database
    let account = db
        .lock()
        .unwrap()
        .private_get_account_by_username(&username);

    match account {
        // check account result
        Ok(account) => {
            // Check if an account was found
            if let Some(account) = account {
                if let Some(at) = account.account_type {
                    match at.as_str() {
                        "admin" => return Ok(true),
                        _ => return Ok(false),
                    }
                }
            }
            Ok(false)
        }
        Err(err) => Err(err),
    }
}

/// Login accounts API endpoint handler
pub struct LoginKontroller<D: AccountDatabase> {
    pub address: String,
    pub method: Method,
    pub database: Arc<Mutex<D>>,
}

impl<D: AccountDatabase> LoginKontroller<D> {
    /// Issue kpassport using an HTTP cookie
    fn cookie_auth(
        account: Account,
        host: &str,
        signing_key: &str,
        cookie_name: &str,
    ) -> server::Response {
        // Create cookie
        let cookie = krypto::authentication::Auth::issue_kpassport_cookie(
            &account.username,
            host,
            signing_key,
            cookie_name,
        );

        match cookie {
            Ok(cookie) => {
                let mut response = server::Response::json(&LoginResponse {
                    message: "Loggin successful".to_string(),
                    account_type: account.account_type,
                });
                response.headers.push(cookie);
                response.status_code = 200;
                response
            }
            Err(_) => ErrorResponse::internal(),
        }
    }

    // TODO: add error handling
    /// check if user is admin
    pub fn is_admin(&self, kong: &Kong) -> bool {
        if let Some(kpassport) = &kong.kpassport {
            // get admin from database
            let admin_email = if let Some(admin_email) = &kong.config.admin_email {
                admin_email.clone()
            } else {
                // admin email not set
                return false;
            };

            if let Ok(admin_account) = self
                .database
                .lock()
                .unwrap()
                .private_get_account_by_email(&admin_email)
            {
                if let Some(admin_account) = admin_account {
                    // check if admin account username matches the username is the kpassport
                    if admin_account.username == kpassport.content.username {
                        // user is admin
                        true
                    } else {
                        // user is not admin
                        false
                    }
                } else {
                    // Admin account not found, for some reason. Cannot check if user is admin
                    false
                }
            } else {
                // Could not get admin account from database
                false
            }
        } else {
            // No kpassport found (user not logged in), cannot check if user is admin
            false
        }
    }
}

impl<D: AccountDatabase> Kontrol for LoginKontroller<D> {
    /// Endpoint's address
    fn address(&self) -> String {
        self.address.clone()
    }

    /// Method supported by endpoint
    fn method(&self) -> Method {
        self.method.clone()
    }

    /// Get input from request
    fn get_input(&self, request: &server::Request) -> Option<JsonValue> {
        let input: Result<AccountLoginInput, server::input::json::JsonError> =
            server::input::json_input(request);

        if let Ok(input) = input {
            Some(input.as_json())
        } else {
            None
        }
    }

    /// Validate request input
    fn validate(&self, input: Option<JsonValue>) -> Result<Option<JsonValue>, ()> {
        if let Some(input) = input {
            let input = AccountLoginInput::from_json_str(input.to_string());

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
    /// Issue kpassport
    fn kontrol(&self, kong: &Kong) -> server::Response {
        // Check if user input exists
        if let Some(input) = &kong.input {
            let input = AccountLoginInput::from_json_str(input.to_string());

            // check if user input is Ok
            if let Ok(input) = input {
                // Find user account in database
                let account = self
                    .database
                    .lock()
                    .unwrap()
                    .private_get_account_by_username(&input.username);

                match account {
                    // check account result
                    Ok(account) => {
                        // Check if an account was found
                        if let Some(account) = account {
                            // Verify user password
                            match krypto::password::verify(&account.password, &input.password) {
                                Ok(password_verification) => {
                                    if password_verification {
                                        // Password correct, create cookie based sessions
                                        LoginKontroller::<D>::cookie_auth(
                                            account,
                                            &kong.config.hostname,
                                            &kong.config.secret_key,
                                            &kong.config.auth_cookie_name,
                                        )
                                    } else {
                                        // Wrong password provided
                                        ErrorResponse::bad_request()
                                    }
                                }
                                Err(_) => ErrorResponse::internal(),
                            }
                        } else {
                            // could not find account with that username
                            ErrorResponse::not_found()
                        }
                    }
                    // an error occured while getting account from DB
                    Err(_) => ErrorResponse::internal(),
                }
            } else {
                // user input is not Ok
                ErrorResponse::bad_request()
            }
        } else {
            // user input is not provided
            ErrorResponse::bad_request()
        }
    }
}

/// Login response message
#[derive(Serialize)]
pub struct LoginResponse {
    /// Message sent as JSON to user after successful login
    message: String,
    account_type: Option<String>,
}
