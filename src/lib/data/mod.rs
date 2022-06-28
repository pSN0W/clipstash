pub mod model;

use serde::{Serialize,Deserialize};
use derive_more::{Display,From};
use uuid::Uuid;
use std::str::FromStr;
use sqlx::Sqlite;

#[derive(Debug,thiserror::Error)]
pub enum DataError {
    #[error("database error : {0}")]
    DataBase(#[from] sqlx::Error)
}


// We are making lots og type annotations here so that if we want to go
// go from sqlite to postgres or mangoDb we will only have to make 
// changes here
pub type AppDatabase = Database<Sqlite>;

// This is a pool of connection to our dataset. Sql library will make multiple
// connection with our dataset and constantly reuse them this will greatly speed up things
// as we don't need to make multiple connections
pub type DatabasePool = sqlx::sqlite::SqlitePool;

// Transaction allows us to roll back in case of any issues.
// So if there are multiple requests and some error occurs then transaction will
// allow us to roll back  
pub type Transaction<'t> = sqlx::Transaction<'t,Sqlite>;

// These are just rows and query returned by the database
pub type AppDatabaseRow = sqlx::sqlite::SqliteRow;
pub type AppQueryResult = sqlx::sqlite::SqliteQueryResult;


pub struct Database<D:sqlx::Database> (sqlx::Pool<D>);

impl Database<Sqlite> {
    pub async fn new(connection_string:&str) -> Self{
        // Try to make connection to a pool using string
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
                                        .connect(connection_string)
                                        .await;
        match pool {
            Ok(pool) => Self(pool),
            // If connection fails the print these in error consoles and close the program
            Err(e) => {
                eprintln!("{}\n",e);
                eprintln!("If the database has not been created run : \n $ sqlx database setup \n");
                panic!("database connection error");
            }
        }
    } 

    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}


#[derive(Debug,Clone,Display,From,Serialize,Deserialize)]
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> Self {
        // using into function converts because we have implemented From
        Uuid::new_v4().into()
    }

    // this is important when we want to send all 0 to some client
    // useful for hiding the actual uuid
    pub fn nil() -> Self {
        Self(Uuid::nil())
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}