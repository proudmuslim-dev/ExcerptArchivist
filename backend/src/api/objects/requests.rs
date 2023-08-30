use poem_openapi::Object;

#[derive(Object)]
pub struct UpdateExcerpt {
    pub citation: Option<String>,
    pub quote: Option<String>,
}
