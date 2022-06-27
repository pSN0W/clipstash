use chrono::{DateTime,Utc,NaiveDateTime};
use serde::{Serialize,Deserialize};
use derive_more::From;
use std::str::FromStr;

#[derive(Debug,Clone,Serialize,Deserialize,From)]
struct Time(DateTime<Utc>);

impl Time {
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }

    pub fn fron_naive_utc(local:NaiveDateTime) -> Self {
        Time(DateTime::from_utc(local, Utc))
    }
}

impl FromStr for Time{
    type Err = chrono::ParseError;
    // what we are trying to do here is whenever we get a time like 2022-05-22 we 
    // append time with it and then check if it can be parsed into DateTime<Utc>
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match format!("{}T00:00:00Z",s).parse::<DateTime<Utc>>() {
            Ok(time) => Ok(time.into()),
            Err(e) => Err(e)
        }
    }
}