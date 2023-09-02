use crate::{
    api::objects::{
        requests::{AddImagePayload, UpdateExcerpt},
        responses::ExcerptWithImages,
    },
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
    param::Query,
    payload::{Json, PlainText},
    OpenApi,
};
use rand::distributions::{Alphanumeric, DistString};
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

    // TOOD: Test pagination
    #[oai(path = "/excerpts", method = "get")]
    pub async fn get_excerpts(
        &self,
        pool: Data<&DbPool>,
        Query(page): Query<Option<u32>>,
        Query(count): Query<Option<u32>>,
    ) -> Result<Json<Vec<ExcerptWithImages>>> {
        let page = page.unwrap_or(1);
        let count = count.unwrap_or(10);

        let mut excerpts = sqlx::query_as::<Sqlite, Excerpt>("SELECT * FROM excerpt LIMIT ? OFFSET ?")
            .bind(count)
            .bind(page - 1)
            .fetch(pool.0);

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

    #[oai(path = "/excerpts", method = "post")]
    pub async fn create_excerpt(
        &self,
        pool: Data<&DbPool>,
        Json(excerpt): Json<Excerpt>,
    ) -> Result<Json<ExcerptWithImages>> {
        let excerpt_id = {
            let query = match excerpt.quote {
                Some(quote) => sqlx::query("INSERT INTO excerpt ( citation, quote ) VALUES ( ?, ? )")
                    .bind(excerpt.citation)
                    .bind(quote),
                None => sqlx::query("INSERT INTO excerpt ( citation ) VALUES ( ? )").bind(excerpt.citation),
            };

            query
                .execute(pool.0)
                .await
                .map_err(InternalServerError)?
                .last_insert_rowid()
        };

        let excerpt = sqlx::query_as::<Sqlite, Excerpt>("SELECT * FROM excerpt WHERE id = ?")
            .bind(excerpt_id)
            .fetch_optional(pool.0)
            .await
            .map_err(InternalServerError)?
            .ok_or(NotFoundError)?;

        Ok(Json(ExcerptWithImages::new(excerpt, vec![])))
    }

    #[oai(path = "/excerpts/:id", method = "delete")]
    pub async fn delete_excerpt(&self, pool: Data<&DbPool>, Path(id): Path<i64>) -> Result<()> {
        sqlx::query("DELETE FROM excerpt WHERE id = ?")
            .bind(id)
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;

        // TODO: Check if this is needed
        sqlx::query("DELETE * FROM image WHERE post_id = ?")
            .bind(id)
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;

        Ok(())
    }

    #[oai(path = "/excerpts/:id", method = "patch")]
    pub async fn update_excerpt(
        &self,
        pool: Data<&DbPool>,
        Path(id): Path<i64>,
        Json(updates): Json<UpdateExcerpt>,
    ) -> Result<Json<ExcerptWithImages>> {
        if let Some(citation) = updates.citation {
            sqlx::query("UPDATE excerpt SET citation = ? WHERE id = ?")
                .bind(citation)
                .bind(id)
                .execute(pool.0)
                .await
                .map_err(InternalServerError)?;
        }

        match updates.quote {
            Some(quote) => sqlx::query("UPDATE EXCERPT SET quote = ? WHERE id = ?").bind(quote),
            None => sqlx::query("UPDATE EXCERPT SET quote = null WHERE id = ?"),
        }
        .bind(id)
        .execute(pool.0)
        .await
        .map_err(InternalServerError)?;

        let excerpt_and_images = {
            let excerpt = sqlx::query_as::<Sqlite, Excerpt>("SELECT * FROM excerpt WHERE id = ?")
                .bind(id)
                .fetch_one(pool.0)
                .await
                .map_err(InternalServerError)?;

            let images = sqlx::query_as::<Sqlite, Image>("SELECT * FROM image WHERE post_id = ?")
                .bind(id)
                .bind(id)
                .fetch_all(pool.0)
                .await
                .map_err(InternalServerError)?;

            ExcerptWithImages::new(excerpt, images)
        };

        Ok(Json(excerpt_and_images))
    }

    #[oai(path = "/excerpts/:id/images", method = "post")]
    pub async fn add_image(
        &self,
        pool: Data<&DbPool>,
        Path(id): Path<i64>,
        payload: AddImagePayload,
    ) -> Result<Json<Image>> {
        // TODO: Check if image exists

        let path = format!(
            // TODO: Deduce file ext
            "./images/{id}-{rand}.png",
            rand = Alphanumeric.sample_string(&mut rand::thread_rng(), 4)
        );

        tokio::fs::write(
            path.as_str(),
            payload.file.into_vec().await.map_err(InternalServerError)?,
        )
        .await
        .map_err(InternalServerError)?;

        sqlx::query("INSERT INTO image ( path, post_id ) VALUES ( ?, ? )")
            .bind(path.as_str())
            .bind(id)
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(Image { post_id: id, path }))
    }

    // TODO: Work out path for img removal
    pub async fn remove_image(&self) -> Result<()> {
        todo!()
    }
}
