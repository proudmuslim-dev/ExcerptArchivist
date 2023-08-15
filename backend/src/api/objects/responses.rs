use poem_openapi::Object;

use crate::db::models::{Excerpt, Image};

#[derive(Object)]
pub struct ExcerptWithImages {
    pub id: i64,
    pub excerpt: String,
    pub images: Vec<Image>,
}

impl ExcerptWithImages {
    pub fn new(Excerpt { id, excerpt }: Excerpt, images: Vec<Image>) -> ExcerptWithImages {
        ExcerptWithImages { id, excerpt, images }
    }
}
