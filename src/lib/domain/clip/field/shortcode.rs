use serde::{ Deserialize, Serialize};
use super::super::ClipError;
use std::str::FromStr;
use derive_more::From;

#[derive(Debug,Clone,Serialize,Deserialize,From)]
pub struct ShortCode(String);

impl ShortCode {
    pub fn new() -> Self {
        use rand::prelude::*;
        // random characters to choose from
        let allowed_charecters = [
            'a','b','c','d','1','2','3','4'
        ];

        let mut rng = thread_rng();

        // tells the length of the string
        let mut shortcode = String::with_capacity(10);
        // choosing a random character each time and pushing it to shortcode
        // * here because .choose returs a borrowed char
        for _ in 0..10 {
            shortcode.push(
                *allowed_charecters
                    .choose(&mut rng)
                    .expect("Allowed characters can't be empty")
            )
        }
        Self(shortcode)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<ShortCode> for String {
    fn from(shortcode: ShortCode) -> Self {
        shortcode.0
    }
}

impl From<&str> for ShortCode {
    fn from(shortcode: &str) -> Self {
        Self(shortcode.to_owned())
    }
}

impl FromStr for ShortCode {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}