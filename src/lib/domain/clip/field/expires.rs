use crate::domain::Time;
use serde::{Deserialize,Serialize};
use super::super::ClipError;
use std::str::FromStr;
use rocket::form::{FromFormField,ValueField,self};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Expires(Option<Time>);

impl Expires {
    pub fn new<T:Into<Option<Time>>> (expires:T) -> Self {
        Self(expires.into())
    }

    pub fn into_inner(self) -> Option<Time> {
        self.0
    }
}

impl Default for Expires {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Expires {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Self(None))
        }else{
            match Time::from_str(s) {
                Ok(time) => Ok(Self::new(time)),
                Err(e) => Err(e.into())
                // Will return ClipError::DateParseError because from chrono::ParseError
                // is implemented for that only
            }
        }
    }
}

#[rocket::async_trait]
impl<'v> FromFormField<'v> for Expires{
    fn from_value(field:ValueField< 'v>) -> form::Result< 'v,Self> {
        if field.value.trim().is_empty() {
            Ok(
                Self(None)
            )
        }
        else{
            Ok(
                Self::from_str(field.value)
                    .map_err(|e| form::Error::validation(format!("{}",e)))?
            )
        }
    }
}