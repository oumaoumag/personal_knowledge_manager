use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Note {
    id: Uuid,
    content: String,
    tags: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Note {
    pub fn new(content: String) -> Note {}

    pub fn add_tag(&mut self, tag: String) {}

    pub fn update_content(&mut self, content: String) {}

    pub fn id(&self) {}

    pub fn tags(&self) {}

    pub fn content(&self) -> &str {}

    pub fn created_at(&self) -> DateTime<Utc> {}

    pub fn updated_at(&self) -> DateTime<Utc> {}
}