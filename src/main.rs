use std::io;
use std::io::prelude::*;

use user::User;

mod user;

fn main() {
    let user = get_user();
    println!("Hello, {} {}! You are {} years old.", user.first_name, user.last_name, user.age);
}

fn get_user() -> User {
    let mut user = User {
        first_name: String::new(),
        last_name: String::new(),
        age: 0,
    };
    let first_name: String = get_input("Enter your first name:");
    if first_name.is_empty() {
        println!("First name cannot be blank");
        return get_user();
    }
    let last_name: String = get_input("Enter your last name:");
    if last_name.is_empty() {
        println!("Last name cannot be blank");
        return get_user();
    }
    let age_string: String = get_input("Enter your age:");
    if age_string.is_empty() {
        println!("Age cannot be blank");
        return get_user();
    }
    let age: u32 = match age_string.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Age must be a number");
            return get_user();
        }
    };
    user.first_name = first_name;
    user.last_name = last_name;
    user.age = age;
    return user;
}

fn get_input(prompt_text: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt_text);
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}
