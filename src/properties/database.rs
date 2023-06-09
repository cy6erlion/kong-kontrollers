use super::Property;
use crate::error::KontrollerError;
use rusqlite::{params, Connection};
mod sql {
    pub const CREATE_PROPERTIES_TABLE: &str = "
      CREATE TABLE IF NOT EXISTS properties (
        id INTEGER PRIMARY KEY,                       -- The Identifier of the property, the Rust Type is `i64`
        name TEXT NOT NULL,                           -- Name of the property
        price FLOAT,                                  -- Price of the property
        bedrooms INTEGER DEFAULT(0) NOT NULL,         -- Number of bedrooms
        bathrooms INTEGER DEFAULT(0) NOT NULL,        -- Number of bathrooms
        sqft FLOAT,                                   -- Square foot area size of the property
        address TEXT,                                 -- Address of the property
        agentid INTEGER,                              -- The id of the agent in charge of the property
        description TEXT,                             -- A description of the property
        views INTEGER DEFAULT(0) NOT NULL,            -- Number of times the property has been viewed
        likes INTEGER DEFAULT(0) NOT NULL,            -- Number of times the property has been liked
        bookmarks INTEGER DEFAULT(0) NOT NULL,        -- Number of times the property has been bookmarked
        photos TEXT,                                  -- Contains a JSON string with paths to the actual images
        added TEXT DEFAULT(date('now')) NOT NULL)     -- The date when the property was added, the Rust Type is `chrono::DateTime`";

    /// Add a property
    pub const ADD_PROPERTY: &str = "
      INSERT INTO properties (name,
        price,
        bedrooms,
        bathrooms,
        sqft,
        address,
        agentid,
        description,
        views,
        likes,
        bookmarks,
        photos,
        added
       )
      VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)";

    /// Get property by id
    pub const GET_PROPERTY_BY_ID: &str = "SELECT * FROM properties WHERE id = ?;";

    /// Get all properties
    pub const GET_ALL_PROPERTIES: &str = "SELECT * FROM properties;";
}

/// database controller
pub struct Database {
    /// Database file path
    path: String,
    /// An SQLite connection handle
    conn: Option<Connection>,
}

impl Database {
    pub fn new(path: &str) -> Self {
        Database {
            path: path.to_string(),
            conn: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), KontrollerError> {
        // Open database connection
        let conn =
            Connection::open(self.path.clone()).map_err(|_| KontrollerError::DbConnection)?;
        self.conn = Some(conn);

        // Create database tables if they do not already exist
        match &mut self.conn {
            Some(conn) => {
                let tx = conn
                    .transaction()
                    .map_err(|_| KontrollerError::DbTransaction)?;

                tx.execute(sql::CREATE_PROPERTIES_TABLE, ())
                    .map_err(|_| KontrollerError::DbTableCreation)?;

                tx.commit().map_err(|_| KontrollerError::DbTableCreation)?;

                Ok(())
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    pub fn add_property(&self, property: &Property) -> Result<(), KontrollerError> {
        match &self.conn {
            Some(conn) => {
                conn.execute(
                    sql::ADD_PROPERTY,
                    params![
                        &property.name,
                        &property.price,
                        &property.bedrooms,
                        &property.bathrooms,
                        &property.sqft,
                        &property.address,
                        &property.agentid,
                        &property.description,
                        &property.views,
                        &property.likes,
                        &property.bookmarks,
                        &property.photos,
                        &property.added
                    ],
                )
                .map_err(|_| KontrollerError::DbField)?;
                Ok(())
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    pub fn get_property_by_id(&self, id: i64) -> Result<Option<Property>, KontrollerError> {
        match &self.conn {
            Some(conn) => {
                let mut stmt = conn
                    .prepare(sql::GET_PROPERTY_BY_ID)
                    .map_err(|_| KontrollerError::DbSQL)?;
                let mut properties: Vec<Property> = vec![];

                let property_iter = stmt
                    .query_map(params![id], |s| {
                        Ok(Property {
                            id: s.get(0).map_err(|_| KontrollerError::DbField).unwrap(),
                            name: s.get(1).map_err(|_| KontrollerError::DbField).unwrap(),
                            price: s.get(2).map_err(|_| KontrollerError::DbField).unwrap(),
                            bedrooms: s.get(3).map_err(|_| KontrollerError::DbField).unwrap(),
                            bathrooms: s.get(4).map_err(|_| KontrollerError::DbField).unwrap(),
                            sqft: s.get(5).map_err(|_| KontrollerError::DbField).unwrap(),
                            address: s.get(6).map_err(|_| KontrollerError::DbField).unwrap(),
                            agentid: s.get(7).map_err(|_| KontrollerError::DbField).unwrap(),
                            description: s.get(8).map_err(|_| KontrollerError::DbField).unwrap(),
                            views: s.get(9).map_err(|_| KontrollerError::DbField).unwrap(),
                            likes: s.get(10).map_err(|_| KontrollerError::DbField).unwrap(),
                            bookmarks: s.get(11).map_err(|_| KontrollerError::DbField).unwrap(),
                            photos: s.get(12).map_err(|_| KontrollerError::DbField).unwrap(),
                            added: s.get(13).map_err(|_| KontrollerError::DbField).unwrap(),
                        })
                    })
                    .map_err(|_| KontrollerError::DbField)?;

                for c in property_iter {
                    properties.push(c.unwrap());
                }

                if properties.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(properties[0].clone()))
                }
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Get all properties
    pub fn get_all_properties(&self) -> Result<Vec<Property>, KontrollerError> {
        match &self.conn {
            Some(conn) => {
                let mut properties: Vec<Property> = vec![];
                let mut stmt = conn.prepare(sql::GET_ALL_PROPERTIES).unwrap();
                let property_iter = stmt
                    .query_map([], |row| {
                        Ok(Property {
                            id: row.get(0).unwrap(),
                            name: row.get(1).unwrap(),
                            price: row.get(2).unwrap(),
                            bedrooms: row.get(3).unwrap(),
                            bathrooms: row.get(4).unwrap(),
                            sqft: row.get(5).unwrap(),
                            address: row.get(6).unwrap(),
                            agentid: row.get(7).unwrap(),
                            description: row.get(8).unwrap(),
                            views: row.get(9).unwrap(),
                            likes: row.get(10).unwrap(),
                            bookmarks: row.get(11).unwrap(),
                            photos: row.get(12).unwrap(),
                            added: row.get(13).unwrap(),
                        })
                    })
                    .unwrap();

                for property in property_iter {
                    properties.push(property.unwrap());
                }

                Ok(properties)
            }
            None => Err(KontrollerError::DbConnection),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DB_PATH: &str = "test-data/EUM6O_TEST_DATABASE.sqlite";

    #[test]
    fn connect_db() {
        let mut db = SqliteDB::new(TEST_DB_PATH);

        // Connect to database
        db.connect().unwrap();

        match db.conn {
            Some(_conn) => assert!(true),
            _ => assert!(false),
        }
    }
    #[test]
    fn test_store_get_property() {
        let mut db = SqliteDB::new(TEST_DB_PATH);

        let property = Property {
            name: "Luxury Hill".to_string(),
            price: None,
            bedrooms: 1,
            bathrooms: 1,
            sqft: 334.44,
            address: "Windhoek".to_string(),
            agentid: 12,
            description: "Cool place".to_string(),
            views: 0,
            likes: 0,
            bookmarks: 0,
            photos: "".to_string(),
            added: "".to_string(),
        };

        db.connect().unwrap();
        db.add_property(&property).unwrap();

        let pr = db.get_property_by_id(1).unwrap();

        if let Some(p) = pr {
            assert_eq!(&p.name, &property.name);
        } else {
            panic!("Could not get property from database");
        }
    }

    fn remove_test_db() {
        let test_db_path = std::path::Path::new(TEST_DB_PATH);
        if std::path::Path::exists(test_db_path) {
            std::fs::remove_file(test_db_path).unwrap();
        }
    }
}
