use prisma_client_rust::QueryError;

use crate::models::{excerpts, prisma::excerpt};

excerpt::include!(excerpt_with_images { images });

pub async fn get_excerpt(excerpt_id: i32) -> Result<Option<excerpt_with_images::Data>, QueryError> {
    excerpts()
        .await
        .find_unique(excerpt::id::equals(excerpt_id))
        .include(excerpt_with_images::include())
        .exec()
        .await
}
