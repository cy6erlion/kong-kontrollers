//! # ✨ Property creation kontroller
//!
//! This __kontroller__ is used to create a new user real estate
//! property. It gets the input from the HTTP request validates it
//! and creates the property from the validated input. The property
//! is stored in an SQLite database.

use super::database::Database;
use super::{CreatePropertyInput, Property};
use crate::error::KontrollerError;
use kong::{inputs::UserInput, server, ErrorResponse, JsonValue, Kong, Kontrol, Method};
use std::sync::{Arc, Mutex};

/// ✨ Create property kontroller
pub struct CreatePropertyKontroller {
    /// Address to kontroller
    pub address: String,
    /// HTTP method supported by the kontroller
    pub method: Method,
    /// SQLite database handle
    pub database: Arc<Mutex<Database>>,
}

impl CreatePropertyKontroller {
    /// Store uploaded property photos    
    fn store_photos(
        dir_name: &str,
        photos: Vec<server::input::post::BufferedFile>,
    ) -> std::io::Result<String> {
        let directory = format!("./www/uploads/property_photos/{dir_name}");
        // file paths to store in database
        let mut file_paths = "".to_string();

        // create upload directory if it does not exist
        std::fs::create_dir_all(&directory)?;

        for photo in photos {
            let photo_name = if let Some(photo_name) = &photo.filename {
                photo_name.clone()
            } else {
                "".to_string()
            };
            let timestamp = chrono::Utc::now().timestamp();
            let file = format!("{directory}/{timestamp}-{photo_name}");

            // Store photos in directory
            std::fs::write(&file, &photo.data)?;

            // remeber file path
            let directory = format!("uploads/property_photos/{dir_name}");
            let file = format!("{directory}/{timestamp}-{photo_name}");
            if file_paths == "" {
                file_paths = file;
            } else {
                file_paths = format!("{file_paths},{file}");
            }
        }

        Ok(file_paths)
    }
}

impl Kontrol for CreatePropertyKontroller {
    /// Endpoint's address
    fn address(&self) -> String {
        self.address.clone()
    }

    /// Method supported by endpoint
    fn method(&self) -> Method {
        self.method
    }

    fn get_input(&self, request: &server::Request) -> Option<JsonValue> {
        let input = server::post_input!(request, {
            name: String,
            price: Option<f64>,
            bedrooms: u16,
            bathrooms: u16,
            sqft: f64,
            address: String,
            agentid: u64,
            description: String,
            photo_0: Option<server::input::post::BufferedFile>,
            photo_1: Option<server::input::post::BufferedFile>,
            photo_2: Option<server::input::post::BufferedFile>,
            photo_3: Option<server::input::post::BufferedFile>,
        });

        if let Ok(input) = input {
            let mut photos = vec![];
            // extract Photo 0
            if let Some(photo_0) = input.photo_0 {
                photos.push(photo_0);
            }
            // extract Photo 1
            if let Some(photo_1) = input.photo_1 {
                photos.push(photo_1);
            }
            // extract Photo 2
            if let Some(photo_2) = input.photo_2 {
                photos.push(photo_2);
            }
            // extract Photo 3
            if let Some(photo_3) = input.photo_3 {
                photos.push(photo_3);
            }

            // Store photos
            if let Ok(photos) = CreatePropertyKontroller::store_photos(&input.name, photos) {
                let input = CreatePropertyInput {
                    name: input.name,
                    price: input.price,
                    bedrooms: input.bedrooms,
                    bathrooms: input.bathrooms,
                    sqft: input.sqft,
                    address: input.address,
                    agentid: input.agentid,
                    description: input.description,
                    photos,
                };

                Some(input.as_json())
            } else {
                // Could not store photos
                None
            }
        } else {
            // Invalid input
            None
        }
    }

    /// Validate user input
    fn validate(&self, input: Option<JsonValue>) -> Result<Option<JsonValue>, ()> {
        if let Some(input) = input {
            if let Ok(input) = CreatePropertyInput::from_json_str(input.to_string()) {
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
    /// Add property
    fn kontrol(&self, kong: &Kong) -> server::Response {
        if let Some(input) = &kong.input {
            let input = CreatePropertyInput::from_json_str(input.to_string());

            // Derive property from string
            let property: Property = if let Ok(input) = input {
                input.into()
            } else {
                return ErrorResponse::bad_request();
            };

            // Store property into the database
            let res = self.database.lock().unwrap().add_property(&property);

            match res {
                Ok(()) => server::Response::json(&property).with_status_code(201),
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
