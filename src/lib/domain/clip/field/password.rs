use serde::{ Deserialize, Serialize};
use super::super::ClipError;
use std::str::FromStr;
use rocket::form::{FromFormField,ValueField,self};

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,PartialOrd)]
pub struct Password(Option<String>);

impl Password{
    pub fn new<T:Into<Option<String>>> (password:T) -> Result<Self,ClipError> {
        // Into<Option<String>> let's us have both string and optional string as argumen
        let password : Option<String> = password.into();

        match password {
            Some(pass) => {
                if pass.trim().is_empty() {
                    Ok(Self(None))
                }
                else {
                    Ok(Self(Some(pass)))
                }
            }
            None => Ok(Self(None))
        }
        // We are always returning Ok here because we want to have the option to have 
        // empty password, Here is the place where we can have checks on our password 
        // and return a ClipError ( checks like length and all )
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }

    pub fn has_password(&self) -> bool {
        self.0.is_some()
    }

}

impl Default for Password {
    fn default() -> Self {
        Self(None)
    }
}

impl FromStr for Password {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

#[rocket::async_trait]
impl<'v> FromFormField<'v> for Password {
    fn from_value(field:ValueField< 'v>) -> form::Result< 'v,Self> {
        Ok(
            Self::new(field.value.to_owned())
                .map_err(|e| form::Error::validation(format!("{}",e)))?
        )
    }
}