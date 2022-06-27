pub mod field;

use serde::{Serialize,Deserialize};
use thiserror::Error;
use chrono;
use uuid;

#[derive(Debug,Error)]
pub enum ClipError{
    #[error("Password is invalid : {0}")]
    InvalidPassword(String),

    #[error("Title is invalid : {0}")]
    InvalidTitle(String),

    #[error("Content can not be empty")]
    EmptyContent,

    #[error("Invalid Date : {0}")]
    InvalidDate(String),

    #[error("Date parse error : {0}")]
    DateParse(#[from] chrono::ParseError),

    #[error("Id parse error : {0}")]
    Id(#[from] uuid::Error),

    #[error("Hits parse error : {0}")]
    Hits(#[from] std::num::TryFromIntError)
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Clip{
    pub clip_id : field::ClipId,
    pub shortcode : field::ShortCode,
    pub content : field::Content,
    pub title : field::Title,
    pub posted : field::Posted,
    pub expires : field::Expires,
    pub password : field::Password,
    pub hits : field::Hits,
}