use poem_openapi::Object;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Excerpt {
    pub id: i64,
    pub excerpt: String,
}

#[derive(Debug, Object, FromRow, Clone)]
pub struct Image {
    pub path: String,
    pub post_id: i64,
}
