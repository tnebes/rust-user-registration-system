use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::MAIN_SEPARATOR;

use log::{error, info};

use crate::user::User;

const FILE_NAME: &str = "data.json";
const USER_DIRECTORY: &str = "user_data";

pub fn file_exists() -> bool {
    return file_or_directory_exists(&get_file_path_name());
}

fn get_file_path_name() -> String {
    let mut file_path = std::path::PathBuf::new();
    file_path.push(env::current_dir().unwrap());
    file_path.push(USER_DIRECTORY);
    file_path.push(FILE_NAME);
    file_path.to_string_lossy().to_string()
}

pub fn create_file() -> String {
    let file_path_name = get_file_path_name();
    let file_path = std::path::Path::new(&file_path_name);

    if let Some(parent_directory) = file_path.parent() {
        if let Err(error) = std::fs::create_dir_all(parent_directory) {
            error!("Failed to create directory: {:?}", error);
            return String::new();
        }
    }

    if let Err(error) = std::fs::File::create(&file_path) {
        error!("Failed to create file: {:?}", error);
        return String::new();
    }

    file_path_name
}

fn file_or_directory_exists(file_path_name: &String) -> bool {
    std::path::Path::new(file_path_name).exists()
}

pub fn write_to_file(user: &User) -> bool {
    let mut users = get_users_from_json();
    users.push(user.clone());
    let serialised_users = serde_json::to_string(&users).unwrap();

    if let Err(error) = std::fs::write(&get_file_path_name(), &serialised_users) {
        error!("Could not write users to JSON: {:?}", error);
        return false;
    }
    true
}

pub fn get_users_from_json() -> Vec<User> {
    let json_content = read_user_file();
    serde_json::from_str(&json_content)
        .unwrap_or_else(|error| {
            error!("Failed to parse json content: {:?}", error);
            Vec::new()
        })
}

pub fn read_user_file() -> String {
    let file_path = get_file_path_name();

    match std::fs::read_to_string(&file_path) {
        Ok(content) => {
            info!("Successfully read file at {}, size is {}", file_path, content.len());
            content
        }
        Err(error) => {
            error!("Failed to read file at {} {}", file_path, error);
            String::new()
        }
    }
}
