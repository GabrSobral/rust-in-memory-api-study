use crate::utils::utils::generate_random_id;

#[derive(Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
        User {
            id: generate_random_id(),
            username,
            email,
            password,
        }
    }
}
