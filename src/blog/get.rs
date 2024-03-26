//! # Get single blog post kontroller

use super::database::Database;
use crate::error::KontrollerError;
use kong::{server, ErrorResponse, Kong, Kontrol, Method};
use std::sync::{Arc, Mutex};

/// Get a single blog post kontroller
pub struct GetBlogPostByIdKontroller {
    /// Address to kontroller
    pub address: String,
    /// HTTP method supported by the kontroller
    pub method: Method,
    /// SQLite database handle
    pub database: Arc<Mutex<Database>>,
}

impl Kontrol for GetBlogPostByIdKontroller {
    /// Endpoint's address
    fn address(&self) -> String {
        self.address.clone()
    }

    /// Method supported by endpoint
    fn method(&self) -> Method {
        self.method
    }

    /// Get blogs
    fn kontrol(&self, kong: &Kong) -> server::Response {
        if let Some(url_params) = &kong.url_parameters {
            if let Some(id) = url_params.find("id") {
                let id: i64 = id.parse().unwrap();
                let res = self.database.lock().unwrap().get_blog_by_id(id);
                match res {
                    Ok(post) => {
                        if let Some(post) = post {
                            server::Response::json(&post).with_status_code(200)
                        } else {
                            ErrorResponse::not_found()
                        }
                    }
                    Err(err) => match err {
                        KontrollerError::DbField => ErrorResponse::bad_request(),
                        _ => ErrorResponse::internal(),
                    },
                }
            } else {
                ErrorResponse::bad_request()
            }
        } else {
            ErrorResponse::bad_request()
        }
    }
}
