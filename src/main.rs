use std::io;
use std::io::prelude::*;
use std::process::exit;

use env_logger;
use log::{error, info, warn};

use user::User;

use crate::file_util::{get_users_from_json};

mod user;
mod file_util;
mod print_util;

const MIN_AGE: u32 = 18;

fn main() {
    initialise_logger();
    print_util::print_header();
    let user = get_user_from_input();
    if !is_user_age_valid(&user) {
        warn!("{:?} is not old enough to register", user);
        println!("You are not old enough to register!");
        return;
    }
    if !register_user(&user) {
        println!("Thank you for using this program!");
        exit(0);
    }
    println!("Have a nice day.");
}

fn initialise_logger() {
    for arg in std::env::args() {
        if arg == "log" {
            print_util::print_logging_header();
            std::env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();
}

fn register_user(user: &User) -> bool {
    if !file_util::file_exists() {
        let file_name = file_util::create_file();
        if file_name.is_empty() {
            error!("Failed to create file at {}", &file_name);
            println!("Please check the integrity of the user data file and try again.");
            return false;
        }
        info!("Created file: {}", &file_name)
    }
    if user_exists(&user) {
        info!("User {} {}, {} already exists", &user.first_name, &user.last_name, &user.age);
        println!("You are already registered!");
        return false;
    }
    if !file_util::write_to_file(&user) {
        error!("Failed to write to file");
        println!("Please check the integrity of the user data file and try again.");
        return false;
    }
    println!("Thank you, {}, you are now registered!", &user.first_name);
    return true;
}

fn user_exists(user: &User) -> bool {
    let users = get_users_from_json();
    return users.contains(user);
}

fn is_user_age_valid(user: &User) -> bool {
    return user.age >= MIN_AGE;
}

fn get_user_from_input() -> User {
    let mut user = User {
        first_name: String::new(),
        last_name: String::new(),
        age: 0,
    };
    let first_name: String = get_input("Enter your first name:");
    if first_name.is_empty() {
        error!("First name {} cannot be blank", &first_name);
        println!("First name cannot be blank.");
        return get_user_from_input();
    }
    let last_name: String = get_input("Enter your last name:");
    if last_name.is_empty() {
        error!("Last name {} cannot be blank", &last_name);
        println!("Last name cannot be blank.");
        return get_user_from_input();
    }
    let age_string: String = get_input("Enter your age:");
    if age_string.is_empty() {
        error!("Age {} cannot be blank", &age_string);
        println!("Age cannot be blank.");
        return get_user_from_input();
    }
    let age: u32 = match age_string.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            error!("Age {} must be a number", &age_string);
            println!("Age must be a number.");
            return get_user_from_input();
        }
    };
    user.first_name = first_name;
    user.last_name = last_name;
    user.age = age;
    info!("{:?} created", user);
    return user;
}

fn get_input(prompt_text: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt_text);
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}
