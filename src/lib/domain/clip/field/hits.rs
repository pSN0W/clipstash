use serde::{ Serialize , Deserialize};
use derive_more::Constructor;

#[derive(Debug,Clone,Constructor,Serialize,Deserialize)]
pub struct Hits(u64);

/*
Constructor macro creates the new function automatically for us
pub fn new(data:u64) -> Self {
    Self(data)
}
*/

impl Hits {
    pub fn into_inner(self) -> u64{
        self.0
    }
}