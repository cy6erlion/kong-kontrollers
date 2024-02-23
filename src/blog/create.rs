//! # ✨ Blog post creation kontroller
//!
//! This __kontroller__ is used to post a new blog post
//! It gets the input from the HTTP request validates it
//! and creates the blog post from the validated input. The blog post
//! is stored in an SQLite database.

use super::database::Database;
use super::{BlogPost, CreateBlogInput};
use crate::accounts::database::Database as AccountsDatabase;
use crate::error::KontrollerError;
use crate::login::is_admin;
use kong::{inputs::UserInput, server, ErrorResponse, JsonValue, Kong, Kontrol, Method};
use std::sync::{Arc, Mutex};

/// ✨ Create blog kontroller
pub struct CreateBlogPostKontroller {
    /// Address to kontroller
    pub address: String,
    /// HTTP method supported by the kontroller
    pub method: Method,
    /// SQLite database handle
    pub database: Arc<Mutex<Database>>,
    /// Accounts database
    pub accounts_database: Arc<Mutex<AccountsDatabase>>,
}

impl CreateBlogPostKontroller {
    /// Store uploaded blog photos
    fn store_cover_photo(
        dir_name: &str,
        photo: server::input::post::BufferedFile,
    ) -> std::io::Result<String> {
        let directory = format!("./www/uploads/blog_photos/{dir_name}");

        // create upload directory if it does not exist
        std::fs::create_dir_all(&directory)?;

        let photo_name = if let Some(photo_name) = &photo.filename {
            photo_name.clone()
        } else {
            "".to_string()
        };
        let timestamp = chrono::Utc::now().timestamp();
        let file = format!("{directory}/{timestamp}-{photo_name}");

        // Store photo in directory
        std::fs::write(&file, &photo.data)?;

        // remeber file path
        let directory = format!("uploads/blog_photos/{dir_name}");
        let file = format!("{directory}/{timestamp}-{photo_name}");

        Ok(file)
    }
}

impl Kontrol for CreateBlogPostKontroller {
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
            title: String,
            subtitle: Option<String>,
            overview: Option<String>,
            author: Option<String>,
            cover: Option<server::input::post::BufferedFile>,
            content: String,
        }) {
            // store cover image
            if let Some(cover) = input.cover {
                if let Ok(cover) = CreateBlogPostKontroller::store_cover_photo(&input.title, cover)
                {
                    let input = CreateBlogInput {
                        title: input.title,
                        subtitle: input.subtitle,
                        overview: input.overview,
                        author: input.author,
                        cover: Some(cover),
                        content: input.content,
                    };

                    Some(input.as_json())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Validate user input
    fn validate(&self, input: Option<JsonValue>) -> Result<Option<JsonValue>, ()> {
        if let Some(input) = input {
            if let Ok(input) = CreateBlogInput::from_json_str(input.to_string()) {
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
    /// Add blog
    fn kontrol(&self, kong: &Kong) -> server::Response {
        if let Some(k) = &kong.kpassport {
            if let Ok(admin) = is_admin(k, self.accounts_database.clone()) {
                if admin {
                    if let Some(input) = &kong.input {
                        let input = CreateBlogInput::from_json_str(input.to_string());

                        // Derive blog from string
                        let blog: BlogPost = if let Ok(input) = input {
                            input.into()
                        } else {
                            return ErrorResponse::bad_request();
                        };

                        // Store blog into the database
                        let res = self.database.lock().unwrap().create_blog(&blog);

                        match res {
                            Ok(()) => server::Response::json(&blog).with_status_code(201),
                            Err(err) => match err {
                                KontrollerError::DbField => ErrorResponse::bad_request(),
                                _ => ErrorResponse::internal(),
                            },
                        }
                    } else {
                        ErrorResponse::unauthorized()
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
