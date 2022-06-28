use crate::data::DbId;
use crate::{ClipError,ShortCode,Time};
use chrono::{NaiveDateTime,Utc};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug,sqlx::FromRow)]
pub struct Clip{
    clip_id : String,
    shortcode : String,
    content : String,
    title : Option<String>,
    posted : NaiveDateTime,
    expires : Option<NaiveDateTime>,
    password : Option<String>,
    hits : i64,
}

impl TryFrom<Clip> for crate::domain::Clip {
    type Error = ClipError;

    fn try_from(clip: Clip) -> Result<Self, Self::Error> {
        use crate::domain::clip::field;
        Ok (
            Self {
                clip_id : field::ClipId::new(DbId::from_str(clip.clip_id.as_str())?),
                shortcode : field::ShortCode::from(clip.shortcode),
                content : field::Content::new(clip.content.as_str())?,
                title : field::Title::new(clip.title),
                posted : field::Posted::new(Time::fron_naive_utc(clip.posted)),
                expires : field::Expires::new(clip.expires.map(Time::fron_naive_utc)),
                password : field::Password::new(clip.password.unwrap_or_default())?,
                hits : field::Hits::new(u64::try_from(clip.hits)?),
            }
        )
    }
}