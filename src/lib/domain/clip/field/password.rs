use serde::{ Deserialize, Serialize};
use super::super::ClipError;

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,PartialOrd)]
pub struct Password(Option<String>);

impl Password{
    fn new<T:Into<Option<String>>> (password:T) -> Result<Self,ClipError> {
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
}