use poem_openapi::Object;
use sqlx::FromRow;

#[derive(Debug, Object, FromRow)]
pub struct Excerpt {
    #[oai(read_only)]
    pub id: i64,
    pub citation: String,
    pub quote: Option<String>,
}

#[derive(Debug, Object, FromRow, Clone)]
pub struct Image {
    pub path: String,
    pub post_id: i64,
}
