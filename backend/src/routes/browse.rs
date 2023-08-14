use rocket::{http::Status, serde::json::Json};

use crate::models::prisma_ext::{self, excerpt::excerpt_with_images};

#[get("/excerpt/<id>")]
pub async fn get_excerpt(id: i32) -> Result<Json<excerpt_with_images::Data>, Status> {
    let excerpt = prisma_ext::excerpt::find_excerpt(id)
        .await
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    Ok(Json(excerpt))
}

#[get("/images/<image>")]
pub async fn get_image(image: String) -> Result<(), Status> {
    Err(Status::NotImplemented)
}
