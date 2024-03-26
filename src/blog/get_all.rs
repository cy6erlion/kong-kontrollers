//! # Get blog posts kontroller
//!
//! This __kontroller__ is used to post a new blog post
//! It gets the input from the HTTP request validates it
//! and creates the blog post from the validated input. The blog post
//! is stored in an SQLite database.

use super::database::Database;
use crate::error::KontrollerError;
use kong::{server, ErrorResponse, Kong, Kontrol, Method};
use std::sync::{Arc, Mutex};

/// Get all blog posts kontroller
pub struct GetAllBlogPostsKontroller {
    /// Address to kontroller
    pub address: String,
    /// HTTP method supported by the kontroller
    pub method: Method,
    /// SQLite database handle
    pub database: Arc<Mutex<Database>>,
}

impl Kontrol for GetAllBlogPostsKontroller {
    /// Endpoint's address
    fn address(&self) -> String {
        self.address.clone()
    }

    /// Method supported by endpoint
    fn method(&self) -> Method {
        self.method
    }

    /// Get blogs
    fn kontrol(&self, _kong: &Kong) -> server::Response {
        // Store blog into the database
        let res = self.database.lock().unwrap().get_all();

        match res {
            Ok(posts) => server::Response::json(&posts).with_status_code(200),
            Err(err) => match err {
                KontrollerError::DbField => ErrorResponse::bad_request(),
                _ => ErrorResponse::internal(),
            },
        }
    }
}
