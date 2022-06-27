use crate::data::DbId;
use serde::{Deserialize,Serialize};
use derive_more::Constructor;

#[derive(Debug,Clone,Serialize,Deserialize,Constructor)]
pub struct  ClipId(DbId);

impl ClipId {
    pub fn into_inner(self) -> DbId {
        self.0
    }
}

impl From<DbId> for ClipId {
    fn from(id: DbId) -> Self {
        Self(id)
    }
}

impl Default for ClipId {
    fn default() -> Self {
        Self(DbId::nil())
    }
}