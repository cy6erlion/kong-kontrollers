//! Properties API endpoint controller

use super::database::Database;
use kong::{json, server, ErrorResponse, Kong, Kontrol, Method};
pub const ADDRESS: &str = "/admin-properties";
use std::sync::{Arc, Mutex};

/// Admin add property API endpoint handler
pub struct GetPropertyKontrolHandler {
    address: String,
    method: Method,
    database: Arc<Mutex<Database>>,
}

impl Kontrol for GetPropertyKontrolHandler {
    fn address(&self) -> String {
        self.address.clone()
    }

    fn method(&self) -> Method {
        self.method.clone()
    }

    /// Get property
    fn kontrol(&self, _kong: &Kong) -> server::Response {
        let properties = self.database.clone();
        let properties = properties.lock().unwrap().get_all_properties();

        match properties {
            Ok(properties) => {
                let properties = json!({ "properties": properties });
                server::Response::json(&properties).with_status_code(200)
            }
            Err(_) => ErrorResponse::internal(),
        }
    }
}
