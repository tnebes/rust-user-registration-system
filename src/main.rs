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
    if std::env::args().any(|arg| arg == "log") {
        print_util::print_logging_header();
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
}

fn register_user(user: &User) -> bool {
    if !file_util::file_exists() {
        let file_name = file_util::create_file();
        if file_name.is_empty() {
            error!("Failed to create file at {}", file_name);
            println!("Please check the integrity of the user data file and try again.");
            return false;
        }
        info!("Created file: {}", file_name);
    }

    if user_exists(user) {
        info!("{:?} already exists", user);
        println!("You are already registered!");
        return false;
    }

    if !file_util::write_to_file(user) {
        error!("Failed to write to file");
        println!("Please check the integrity of the user data file and try again.");
        return false;
    }

    println!("Thank you, {}, you are now registered!", user.first_name);
    true
}

fn user_exists(user: &User) -> bool {
    let users = get_users_from_json();
    return users.contains(user);
}

fn is_user_age_valid(user: &User) -> bool {
    return user.age >= MIN_AGE;
}

fn get_user_from_input() -> User {
    loop {
        let mut user = User {
            first_name: String::new(),
            last_name: String::new(),
            age: 0,
        };

        user.first_name = get_input("Enter your first name:");
        if user.first_name.is_empty() {
            error!("First name cannot be blank");
            println!("First name cannot be blank.");
            continue;
        }

        user.last_name = get_input("Enter your last name:");
        if user.last_name.is_empty() {
            error!("Last name cannot be blank");
            println!("Last name cannot be blank.");
            continue;
        }

        let age_string = get_input("Enter your age:");
        if age_string.is_empty() {
            error!("Age cannot be blank");
            println!("Age cannot be blank.");
            continue;
        }

        if let Ok(age) = age_string.trim().parse() {
            user.age = age;
            info!("{:?} created", user);
            return user;
        } else {
            error!("Age must be a number");
            println!("Age must be a number.");
            continue;
        }
    }
}

fn get_input(prompt_text: &str) -> String {
    println!("{}", prompt_text);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}
