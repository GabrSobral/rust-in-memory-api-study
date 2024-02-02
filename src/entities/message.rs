use chrono::{DateTime, Utc};

use crate::utils::utils::generate_random_id;

#[derive(Clone)]
pub struct Message {
    id: u64,
    content: String,
    written_at: DateTime<Utc>,
    read_at: Option<DateTime<Utc>>,
    author_id: u64,
    conversation_id: u64
}

impl Message {
    pub fn new(content: String, author_id: u64, conversation_id: u64) -> Self {
        Message {
            id: generate_random_id(),
            author_id,
            content,
            conversation_id,
            written_at: Utc::now(),
            read_at: Option::None
        }
    }

    pub fn read(&mut self) {
        self.read_at = Some(Utc::now());
    }
}