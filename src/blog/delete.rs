//! # Delete single article post kontroller

use super::database::Database;
use crate::accounts::database::AccountDatabase;
use crate::login::is_admin;
use kong::{server, ErrorResponse, Kong, Kontrol, Method};
use std::sync::{Arc, Mutex};

/// Delete a single posts kontroller
pub struct DeleteArticleByIdKontroller<D: AccountDatabase> {
    /// Address to kontroller
    pub address: String,
    /// HTTP method supported by the kontroller
    pub method: Method,
    /// SQLite database handle
    pub database: Arc<Mutex<Database>>,
    /// Accounts database
    pub accounts_database: Arc<Mutex<D>>,
}

impl<D: AccountDatabase> Kontrol for DeleteArticleByIdKontroller<D> {
    /// Endpoint's address
    fn address(&self) -> String {
        self.address.clone()
    }

    /// Method supported by endpoint
    fn method(&self) -> Method {
        self.method
    }

    /// Delete article
    fn kontrol(&self, kong: &Kong) -> server::Response {
        if let Some(k) = &kong.kpassport {
            if let Ok(admin) = is_admin(k, self.accounts_database.clone()) {
                if admin {
                    if let Some(url_params) = &kong.url_parameters {
                        if let Some(id) = url_params.find("id") {
                            let id: i64 = id.parse().unwrap();
                            if let Ok(_) = self.database.lock().unwrap().delete(id) {
                                server::Response::text("Article has been deleted, succesfully.")
                                    .with_status_code(200)
                            } else {
                                ErrorResponse::internal()
                            }
                        } else {
                            ErrorResponse::bad_request()
                        }
                    } else {
                        ErrorResponse::bad_request()
                    }
                } else {
                    ErrorResponse::unauthorized()
                }
            } else {
                ErrorResponse::internal()
            }
        } else {
            ErrorResponse::unauthorized()
        }
    }
}
