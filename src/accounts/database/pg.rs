//! # 🗄️ Accounts database management
//!
use super::{Account, PublicAccount};
use crate::error::KontrollerError;
use postgres::{Client, NoTls};

/// Database management system
pub struct Database;

impl Database {
    /// Initialize accounts table
    pub fn init(client: &mut Client, db_owner: &str) -> Result<(), KontrollerError> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS public.accounts (
	id serial NOT NULL,
	username varchar(15) NOT NULL,
	password text NOT NULL,
	created date NOT NULL,
	fullname varchar(50),
	date_of_birth date,
	id_number varchar(100),
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

ALTER TABLE public.accounts OWNER TO {db_owner};
"
        );
        client
            .batch_execute(&sql)
            .map_err(|_| KontrollerError::DbTableCreation)?;

        Ok(())
    }

    /// Create a new account
    pub fn create_account(client: &mut Client, account: &Account) -> Result<(), KontrollerError> {
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

    /// Create a new admin account
    pub fn create_admin_account(
        client: &mut Client,
        account: &Account,
    ) -> Result<(), KontrollerError> {
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

    /// Get an account's public data by its username
    pub fn public_get_account_by_username(
        client: &mut Client,
        username: &str,
    ) -> Result<Option<PublicAccount>, KontrollerError> {
        let row = client
            .query("SELECT * FROM accounts WHERE username = $1;", &[&username])
            .map_err(|_| KontrollerError::DbSQL)?;

        if row.is_empty() {
            Ok(None)
        } else {
            Ok(Some(PublicAccount {
                username: row[0].get("username"),
            }))
        }
    }

    /// Get an account's public data by its email
    pub fn public_get_account_by_email(
        client: &mut Client,
        email: &str,
    ) -> Result<Option<PublicAccount>, KontrollerError> {
        let row = client
            .query("SELECT * FROM accounts WHERE email = $1;", &[&email])
            .map_err(|_| KontrollerError::DbSQL)?;

        if row.is_empty() {
            Ok(None)
        } else {
            Ok(Some(PublicAccount {
                username: row[0].get("username"),
            }))
        }
    }

    /// Get an account's private data by its email
    pub fn private_get_account_by_email(
        client: &mut Client,
        email: &str,
    ) -> Result<Option<Account>, KontrollerError> {
        let row = client
            .query("SELECT * FROM accounts WHERE email = $1;", &[&email])
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
                email: row[0].get("email"),
                mobile_number: row[0].get("mobile_number"),
                website: row[0].get("website"),
                description: row[0].get("description"),
                last_login: row[0].get("last_login"),
                account_type: row[0].get("account_type"),
            }))
        }
    }

    /// Get an account's private data by its username
    pub fn private_get_account_by_username(
        client: &mut Client,
        username: &str,
    ) -> Result<Option<Account>, KontrollerError> {
        let row = client
            .query("SELECT * FROM accounts WHERE username = $1;", &[&username])
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
                email: row[0].get("email"),
                mobile_number: row[0].get("mobile_number"),
                website: row[0].get("website"),
                description: row[0].get("description"),
                last_login: row[0].get("last_login"),
                account_type: row[0].get("account_type"),
            }))
        }
    }
}
