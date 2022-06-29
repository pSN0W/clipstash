use super::model;
use crate::data::{DataError,DatabasePool};


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

// Putting into database
pub async fn new_clip<M:Into<model::NewClip>> (
    model : M,
    pool : &DatabasePool
) -> Result<model::Clip> {
    let model = model.into();
    let _ = sqlx::query!(
        r#"INSERT INTO clips
        (
            clip_id,
            shortcode,
            content,
            title,
            posted,
            expires,
            password,
            hits
        ) 
        VALUES 
        (
            ?,?,?,?,?,?,?,?
        )
        "#,
        model.clip_id,
        model.shortcode,
        model.content,
        model.title,
        model.posted,
        model.expires,
        model.password,
        0
    )
    .execute(pool)
    .await?;
    // extracting from database
    get_clip(model.shortcode, pool).await
}

// Writing a query to update
pub async fn update_clip<M:Into<model::UpdateClip>> (
    model : M,
    pool : &DatabasePool
) -> Result<model::Clip> {
    let model = model.into();
    _ = sqlx::query!(
        r#"UPDATE clips SET
        content = ?,
        title = ?,
        expires = ?,
        password = ?
        WHERE shortcode = ?
        "#,
        model.content,
        model.title,
        model.expires,
        model.password,
        model.shortcode
    )
    .execute(pool)
    .await?;
    get_clip(model.shortcode, pool).await
}