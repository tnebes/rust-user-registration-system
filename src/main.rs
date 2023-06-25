use std::io;
use std::io::prelude::*;
use log::{error, info, warn};
use env_logger;

use user::User;
use user::user_to_json;

mod user;
mod file_util;

const MIN_AGE: u32 = 18;

fn main() {
    initialise_logger();
    print_header();
    let user = get_user();
    if !is_user_age_valid(&user) {
        warn!("User {} {}, {} is not old enough to register", user.first_name, user.last_name, user.age);
        println!("You are not old enough to register");
        return;
    }
    register_user(&user);
}

fn initialise_logger() {
    for arg in std::env::args() {
        if arg == "log" {
            println!("Logging enabled");
            info!("Logging enabled");
            std::env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();
}

fn register_user(user: &User) {
    if !file_util::file_exists() {
        let file_name = file_util::create_file();
        if file_name.is_empty() {
            error!("Failed to create file at {}", file_name);
            return;
        }
        info!("Created file: {}", file_name)
    }
    let user_json = user_to_json(user);
    if user_exists(&user_json) {
        println!("You are already registered!");
        info!("User {} {}, {} already exists", user.first_name, user.last_name, user.age);
        return;
    }
    if !file_util::write_to_file(user_json) {
        error!("Failed to write to file");
        return;
    }
    println!("You are now registered!");

}

fn user_exists(user: &String) -> bool {
    return false;
}

fn is_user_age_valid(user: &User) -> bool {
    return user.age >= MIN_AGE;
}

fn get_user() -> User {
    let mut user = User {
        first_name: String::new(),
        last_name: String::new(),
        age: 0,
    };
    let first_name: String = get_input("Enter your first name:");
    if first_name.is_empty() {
        error!("First name {} cannot be blank", first_name);
        println!("First name cannot be blank");
        return get_user();
    }
    let last_name: String = get_input("Enter your last name:");
    if last_name.is_empty() {
        error!("Last name {} cannot be blank", last_name);
        println!("Last name cannot be blank");
        return get_user();
    }
    let age_string: String = get_input("Enter your age:");
    if age_string.is_empty() {
        error!("Age {} cannot be blank", age_string);
        println!("Age cannot be blank");
        return get_user();
    }
    let age: u32 = match age_string.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            error!("Age {} must be a number", age_string);
            println!("Age must be a number");
            return get_user();
        }
    };
    user.first_name = first_name;
    user.last_name = last_name;
    user.age = age;
    info!("User {} {}, {} created", user.first_name, user.last_name, user.age);
    return user;
}

fn print_header() {
    println!();
    let header:String = String::from("Welcome to the User Registration App");
    println!("{}", header);
    println!("{}", "=".repeat(header.len()));
    println!();
}

fn get_input(prompt_text: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt_text);
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}
