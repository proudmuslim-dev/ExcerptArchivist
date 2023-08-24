use crate::{
    api::objects::responses::ExcerptWithImages,
    db::models::{Excerpt, Image},
    DbPool,
};
use futures::TryStreamExt;
use poem::{
    error::{InternalServerError, NotFoundError},
    web::{Data, Path},
    Result,
};
use poem_openapi::{
    payload::{Json, PlainText},
    OpenApi,
};
use sqlx::Sqlite;

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
            .fetch_optional(pool.0)
            .await
            .map_err(InternalServerError)?
            .ok_or(NotFoundError)?;

        let images = sqlx::query_as::<Sqlite, Image>("SELECT * FROM image WHERE post_id = ?")
            .bind(id)
            .fetch_all(pool.0)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(ExcerptWithImages::new(excerpt, images.to_vec())))
    }

    // TOOD: Pagination
    #[oai(path = "/excerpts", method = "get")]
    pub async fn get_excerpts(&self, pool: Data<&DbPool>) -> Result<Json<Vec<ExcerptWithImages>>> {
        let mut excerpts = sqlx::query_as::<Sqlite, Excerpt>("SELECT * FROM excerpt").fetch(pool.0);

        let mut excerpt_vec = vec![];

        while let Some(excerpt) = excerpts.try_next().await.map_err(InternalServerError)? {
            let images = sqlx::query_as::<Sqlite, Image>("SELECT * FROM image WHERE post_id = ?")
                .bind(excerpt.id)
                .fetch_all(pool.0)
                .await
                .map_err(InternalServerError)?;

            excerpt_vec.push(ExcerptWithImages::new(excerpt, images))
        }

        Ok(Json(excerpt_vec))
    }

    // TODO: Create, modify, delete 
}
