mod utils;
mod entities;
mod services;
mod repositories;

use std::process::exit;

use crate::{
    entities::user::User,
    services::users::user_services::UserServices,
    repositories::user_repository::InMemoryUserRepository 
};

fn main() {
    let user_repository = Box::new(InMemoryUserRepository { users: Vec::new() });
    let mut user_service = UserServices::new(user_repository);

    let _ = user_service.register_user(String::from("John"), String::from("john@email"), String::from("123"));
    let _ = user_service.register_user(String::from("Michael"), String::from("michael@email"), String::from("123"));
    
    let users_result = user_service.list_users();

    let users = match users_result {
        Ok(users) => users,
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        }
    };

    for user in users {
        println!("id: {:?}", user.id);
        println!("name: {:?}", user.username);
        println!("email: {:?}", user.email);
        println!("password: {:?}", user.password);

        println!("\n")
    }

}
