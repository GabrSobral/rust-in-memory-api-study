use crate::{
    entities::user::User, 
    repositories::user_repository::UserRepository
};

pub struct UserServices {
    user_repository: Box<dyn UserRepository>
}

impl UserServices {
    pub fn new(repository: Box<dyn UserRepository>) -> Self {
        UserServices { user_repository: repository }
    }
    
    pub fn authenticate_user(&self, email: String, password: String) -> Result<User, &'static str>  {
        let user_stored = self.user_repository.get_user_by_email(&email);

        if let Some(user) = user_stored {
            if user.password != password {
                return Err("Email/password invalid.");
            }
    
            return Ok(user.clone())
        }

        Err("Email/password invalid.")
    }

    pub fn register_user(&mut self, name: String, email: String, password: String) -> Result<User, &'static str> {
        if self.user_repository.get_user_by_email(&email).is_some() {
            return Err("Email already registered, try another.")
        }

        let new_user = User::new(name.clone(), email.clone(), password.clone());
        self.user_repository.add_user(new_user.clone());

        Ok(new_user)
    }

    pub fn list_users(&self) -> Result<Vec<&User>, &'static str> {
        let users = self.user_repository.get_all_users();
        Ok(users)
    }
}