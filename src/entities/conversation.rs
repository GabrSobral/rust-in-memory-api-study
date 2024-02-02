pub struct Conversation {
    id: u64,
    created_at: DateTime<Utc>
}

impl Conversation {
    pub fn new() {
        Conversation { 
            id: generate_random_id(), 
            created_at: Utc::now() 
        }
    }
}