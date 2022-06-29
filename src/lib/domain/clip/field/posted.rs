use crate::domain::Time;
use serde::{Serialize,Deserialize};
use derive_more::Constructor;

#[derive(Debug,Clone,Serialize,Deserialize,Constructor)]
pub struct Posted(Time);

impl Posted {
    pub fn into_inner(self) -> Time {
        self.0
    }
}