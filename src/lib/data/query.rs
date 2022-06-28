use super::model;
use crate::data::{DataError,DatabasePool};
use crate::ShortCode;
use sqlx::Row;


// Typ alias for result
type Result<T> = std::result::Result<T,DataError>;

pub async fn get_clip<M:Into<model::GetClip>>(
    model:M,
    pool:&DatabasePool
) -> Result<model::Clip> {
    let model = model.into();
    let shortcode = model.shortcode.as_str();
    Ok(
        sqlx::query_as!(
            model::Clip,// The output format of the result
            "SELECT * FROM clips WHERE shortcode = ?", // The query
            shortcode // The question mark thing
        )
        .fetch_one(pool) // fetch only one (will show error for multiple but not an issue as shortcode is unique)
        .await?
    )
}