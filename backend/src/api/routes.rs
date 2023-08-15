use crate::{
    db::models::{Excerpt, Image},
    DbPool,
};
use poem::{
    web::{Data, Path},
    Result,
};
use poem_openapi::{
    payload::{Json, PlainText},
    OpenApi,
};
use sqlx::Sqlite;

use super::objects::responses::ExcerptWithImages;

pub struct Api;

#[OpenApi]
impl Api {
    /// Say hello
    #[oai(path = "/", method = "get")]
    pub async fn index(&self) -> PlainText<String> {
        PlainText("Hello, world!".to_owned())
    }

    /// Get an excerpt
    #[oai(path = "/excerpts/:id", method = "get")]
    pub async fn get_excerpt(&self, pool: Data<&DbPool>, Path(id): Path<i64>) -> Result<Json<ExcerptWithImages>> {
        let excerpt = sqlx::query_as::<Sqlite, Excerpt>("SELECT * FROM excerpt WHERE id = ?")
            .bind(id)
            .fetch_one(pool.0)
            .await
            .unwrap();

        let images = sqlx::query_as::<Sqlite, Image>("SELECT * FROM image WHERE post_id = ?")
            .bind(id)
            .fetch_all(pool.0)
            .await
            .unwrap();

        Ok(Json(ExcerptWithImages::new(excerpt, images.to_vec())))
    }

    // TODO: Create, modify, delete + paginated get
}
