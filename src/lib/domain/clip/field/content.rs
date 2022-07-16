use serde::{ Deserialize,Serialize };
use super::super::ClipError;
use rocket::form::{FromFormField,ValueField,self};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Content(String);

impl Content{
    pub fn new(content:&str) -> Result<Self,ClipError> {
        if content.trim().is_empty() {
            Err(ClipError::EmptyContent)
        }
        else {
            Ok(Self(content.to_owned()))
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

// The fields must implement these trait to be used with 
// macro rocket::FromFrom in web/form.rs
#[rocket::async_trait]
impl <'r> FromFormField<'r> for Content{
    fn from_value(field:ValueField< 'r>) -> form::Result<'r,Self> {
        // If error then convert to validation error in rocket so that rocket knows
        Ok(
            Self::new(field.value)
                .map_err(
                    |e| form::Error::validation(format!("{}",e))
                )?
        )
    }
}