use serde::{Serialize,Deserialize};
use derive_more::{Display,From};
use uuid::Uuid;
use std::str::FromStr;

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