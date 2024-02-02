use crate::User;

pub trait UserRepository {
    fn add_user(&mut self, user: User);

    fn get_user_by_email(&self, email: &str) -> Option<&User>;

    fn get_user_by_id(&self, id: &u64) -> Option<&User>;

    fn get_all_users(&self) -> Vec<&User>;
}

pub struct InMemoryUserRepository {
    pub users: Vec<User>
}

impl UserRepository for InMemoryUserRepository {
    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    fn get_user_by_email(&self, email: &str) -> Option<&User> {
        self.users.iter().find(|&user| user.email == email)
    }

    fn get_user_by_id(&self, id: &u64) -> Option<&User> {
        self.users.iter().find(|&user| user.id == *id)
    }

    fn get_all_users(&self) -> Vec<&User> {
        self.users.iter().collect()
    }
}