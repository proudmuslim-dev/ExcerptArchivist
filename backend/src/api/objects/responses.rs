use poem_openapi::Object;

use crate::db::models::{Excerpt, Image};

#[derive(Object)]
pub struct ExcerptWithImages {
    pub id: i64,
    pub citation: String,
    pub quote: Option<String>,
    pub images: Vec<Image>,
}

impl ExcerptWithImages {
    pub fn new(Excerpt { id, citation, quote }: Excerpt, images: Vec<Image>) -> ExcerptWithImages {
        ExcerptWithImages {
            id,
            citation,
            quote,
            images,
        }
    }
}
