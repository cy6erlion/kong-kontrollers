#[cfg(feature = "postgres")]
use postgres::Client;
#[cfg(feature = "sqlite")]
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
#[cfg(feature = "postgres")]
mod pg;
#[cfg(feature = "sqlite")]
mod sqlite;
use super::{Account, PublicAccount};
use crate::error::KontrollerError;

#[cfg(feature = "postgres")]
pub use pg::Database;
#[cfg(feature = "sqlite")]
pub use sqlite::Database;

#[cfg(feature = "sqlite")]
pub type DB_Type = Arc<Mutex<Connection>>;

#[cfg(feature = "postgres")]
pub type DB_Type = Arc<Mutex<Client>>;
