use crate::data::DbId;
use crate::{ClipError,ShortCode,Time};
use chrono::{NaiveDateTime,Utc};
use std::convert::TryFrom;
use std::str::FromStr;

// this structure is used to store the values directly coming out from the sql row
// this doesn't contain the validation that we emposed in our clip
// this is crate::data::Clip and the validates one is crate::domain::Clip
// the sqlx::FromRow function will convert a row from the data into this struct
// hence it becomes important for the datatypes at both places to match
// pub (in crate::data) means it is only public in data
#[derive(Debug,sqlx::FromRow)]
pub struct Clip{
    pub (in crate::data) clip_id : String,
    pub (in crate::data) shortcode : String,
    pub (in crate::data) content : String,
    pub (in crate::data) title : Option<String>,
    pub (in crate::data) posted : NaiveDateTime,
    pub (in crate::data) expires : Option<NaiveDateTime>,
    pub (in crate::data) password : Option<String>,
    pub (in crate::data) hits : i64,
}

// this will convert it to our validated clip
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

pub struct GetClip {
    pub (in crate::data) shortcode : String,
}

impl From<ShortCode> for GetClip {
    fn from(shortcode: ShortCode) -> Self {
        Self { shortcode: shortcode.into_inner() }
    }
}

impl From<String> for GetClip {
    fn from(shortcode: String) -> Self {
        Self { shortcode }
    }
}

impl From<crate::service::ask::GetClip> for GetClip {
    fn from(req: crate::service::ask::GetClip) -> Self {
        Self { shortcode: req.shortcode.into_inner() }
    }
}

// A structure to put new clip in databse
// hits is removed because we want it to be 0
// posted has been changed to i16
pub struct NewClip{
    pub (in crate::data) clip_id : String,
    pub (in crate::data) shortcode : String,
    pub (in crate::data) content : String,
    pub (in crate::data) title : Option<String>,
    pub (in crate::data) posted : i64,
    pub (in crate::data) expires : Option<i64>,
    pub (in crate::data) password : Option<String>,
}

impl From<crate::service::ask::NewClip> for NewClip{
    fn from(req: crate::service::ask::NewClip) -> Self {
        Self{
            clip_id : DbId::new().into(),
            shortcode : ShortCode::new().into_inner(),
            content : req.content.into_inner(),
            title : req.title.into_inner(),
            password : req.password.into_inner(),
            expires : req.expires.into_inner()
                                .map(|time| time.into_inner().timestamp()),
            posted : Utc::now().timestamp()
        }
    }
}

// A structure to update the clip
pub struct UpdateClip{
    pub (in crate::data) shortcode : String,
    pub (in crate::data) content : String,
    pub (in crate::data) title : Option<String>,
    pub (in crate::data) expires : Option<i64>,
    pub (in crate::data) password : Option<String>,
}

impl From<crate::service::ask::UpdateClip> for UpdateClip {
    fn from(req: crate::service::ask::UpdateClip) -> Self {
        Self{
            shortcode : req.shortcode.into_inner(),
            content : req.content.into_inner(),
            title : req.title.into_inner(),
            expires : req.expires.into_inner()
                                .map(|time| time.into_inner().timestamp()),
            password : req.password.into_inner()
        }
    }
}