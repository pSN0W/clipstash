use super::super::ClipError;
use serde::{Serialize,Deserialize};
use std::str::FromStr;
use rocket::form::{FromFormField,ValueField,self};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Title(Option<String>);

impl Title {
    pub fn new<T:Into<Option<String>>>(title:T) -> Self {
        let title:Option<String> = title.into();
        match title {
            Some(title) => {
                if title.trim().is_empty() {
                    Self(None)
                } else {
                    Self(Some(title))
                }
            },
            None => Self(None)
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}

impl Default for Title {
    fn default() -> Self {
        Self(None)
    }
}

impl FromStr for Title {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Title{
    fn from_value(field:ValueField<'r>) -> form::Result<'r,Self> {
        Ok(Self::new(field.value.to_owned()))
    }
}