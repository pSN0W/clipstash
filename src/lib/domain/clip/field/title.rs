use super::super::ClipError;
use serde::{Serialize,Deserialize};
use std::str::FromStr;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Title(Option<String>);

impl Title {
    fn new<T:Into<Option<String>>>(title:T) -> Self {
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

    fn into_inner(self) -> Option<String> {
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