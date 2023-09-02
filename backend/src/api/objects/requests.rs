use poem_openapi::{types::multipart::Upload, Multipart, Object};

#[derive(Object)]
pub struct UpdateExcerpt {
    pub citation: Option<String>,
    pub quote: Option<String>,
}

#[derive(Debug, Multipart)]
pub struct AddImagePayload {
    pub file: Upload,
}
