//! # 🗄️ Accounts database management
//!
use super::{Account, AccountDatabase, PublicAccount};
use crate::error::KontrollerError;
use postgres::{Client, NoTls};

/// Database management system
pub struct Database {
    /// Database file path
    path: String,
    /// An Postgres connection handle
    client: Option<Client>,
}

impl AccountDatabase for Database {
    /// Create a new database controller
    fn new(path: &str) -> Self {
        Database {
            path: path.to_string(),
            client: None,
        }
    }

    /// Open Postgres connection, create tables
    fn connect(&mut self) -> Result<(), KontrollerError> {
        // Open database connection
        let mut client =
            Client::connect(&self.path, NoTls).map_err(|_| KontrollerError::DbConnection)?;

        client
            .batch_execute(
                "
DO $$ BEGIN
    CREATE TYPE public.gender AS
        ENUM ('Male','Female','Other');
        ALTER TYPE public.gender OWNER TO postgres;
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS public.accounts (
	id serial NOT NULL,
	username varchar(15) NOT NULL,
	password text NOT NULL,
	created date NOT NULL,
	fullname varchar(50),
	date_of_birth date,
	id_number varchar(100),
	gender public.gender,
	description varchar(200),
	email varchar(320),
	mobile_number varchar(15),
	website varchar(2048),
	last_login date,
	account_type varchar(50),
	CONSTRAINT \"Account id is a PRIMARY KEY\" PRIMARY KEY (id),
	CONSTRAINT \"Account ID is UNIQUE\" UNIQUE (id),
	CONSTRAINT \"Account username is UNIQUE\" UNIQUE (username)
);

ALTER TABLE public.accounts OWNER TO postgres;
",
            )
            .map_err(|_| KontrollerError::DbTableCreation)?;

        self.client = Some(client);

        Ok(())
    }

    /// Create a new account
    fn create_account(&mut self, account: &Account) -> Result<(), KontrollerError> {
        match &mut self.client {
            Some(client) => {
                client
                    .execute(
                        "INSERT INTO accounts (
                                username,
                                email,
                                password,
                                created
                              )
                              VALUES ($1, $2, $3, $4)",
                        &[
                            &account.username,
                            &account.email,
                            &account.password,
                            &account.created,
                        ],
                    )
                    .map_err(|_| KontrollerError::DbField)?;
                Ok(())
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Create a new admin account
    fn create_admin_account(&mut self, account: &Account) -> Result<(), KontrollerError> {
        match &mut self.client {
            Some(client) => {
                client
                    .execute(
                        "INSERT INTO accounts (
                                username,
                                email,
                                password,
                                created,
                                account_type
                              )
                              VALUES ($1, $2, $3, $4, $5)",
                        &[
                            &account.username,
                            &account.email,
                            &account.password,
                            &account.created,
                            &account.account_type,
                        ],
                    )
                    .map_err(|_| KontrollerError::DbField)?;

                Ok(())
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Get an account's public data by its username
    fn public_get_account_by_username(
        &mut self,
        username: &str,
    ) -> Result<Option<PublicAccount>, KontrollerError> {
        match &mut self.client {
            Some(client) => {
                let row = client
                    .query(
                        "SELECT * FROM accounts WHERE username = :username;",
                        &[&username],
                    )
                    .map_err(|_| KontrollerError::DbSQL)?;

                if row.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(PublicAccount {
                        username: row[0].get("username"),
                    }))
                }
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Get an account's public data by its email
    fn public_get_account_by_email(
        &mut self,
        email: &str,
    ) -> Result<Option<PublicAccount>, KontrollerError> {
        match &mut self.client {
            Some(client) => {
                let row = client
                    .query("SELECT * FROM accounts WHERE email = :email;", &[&email])
                    .map_err(|_| KontrollerError::DbSQL)?;

                if row.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(PublicAccount {
                        username: row[0].get("username"),
                    }))
                }
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Get an account's private data by its email
    fn private_get_account_by_email(
        &mut self,
        email: &str,
    ) -> Result<Option<Account>, KontrollerError> {
        match &mut self.client {
            Some(client) => {
                let row = client
                    .query("SELECT * FROM accounts WHERE email = :email;", &[&email])
                    .map_err(|_| KontrollerError::DbSQL)?;

                if row.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(Account {
                        username: row[0].get("username"),
                        password: row[0].get("password"),
                        created: row[0].get("created"),
                        fullname: row[0].get("fullname"),
                        date_of_birth: row[0].get("date_of_birth"),
                        id_number: row[0].get("id_number"),
                        gender: row[0].get("gender"),
                        email: row[0].get("email"),
                        mobile_number: row[0].get("mobile_number"),
                        website: row[0].get("website"),
                        description: row[0].get("description"),
                        last_login: row[0].get("last_login"),
                        account_type: row[0].get("account_type"),
                    }))
                }
            }
            None => Err(KontrollerError::DbConnection),
        }
    }

    /// Get an account's private data by its username
    fn private_get_account_by_username(
        &mut self,
        username: &str,
    ) -> Result<Option<Account>, KontrollerError> {
        match &mut self.client {
            Some(client) => {
                let row = client
                    .query(
                        "SELECT * FROM accounts WHERE username = :username;",
                        &[&username],
                    )
                    .map_err(|_| KontrollerError::DbSQL)?;

                if row.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(Account {
                        username: row[0].get("username"),
                        password: row[0].get("password"),
                        created: row[0].get("created"),
                        fullname: row[0].get("fullname"),
                        date_of_birth: row[0].get("date_of_birth"),
                        id_number: row[0].get("id_number"),
                        gender: row[0].get("gender"),
                        email: row[0].get("email"),
                        mobile_number: row[0].get("mobile_number"),
                        website: row[0].get("website"),
                        description: row[0].get("description"),
                        last_login: row[0].get("last_login"),
                        account_type: row[0].get("account_type"),
                    }))
                }
            }
            None => Err(KontrollerError::DbConnection),
        }
    }
}
