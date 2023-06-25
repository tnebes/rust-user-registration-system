use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::MAIN_SEPARATOR;

use log::{error, info};

use crate::user::User;

const FILE_NAME: &str = "data.json";
const USER_DIRECTORY: &str = "user_data";

pub fn file_exists() -> bool {
    let file_path_name = get_file_path_name();
    return file_or_directory_exists(&file_path_name);
}

fn get_file_path_name() -> String {
    let current_dir = env::current_dir().unwrap();
    let current_dir_string = current_dir.to_str().unwrap();
    let mut file_path_name = String::from(current_dir_string);
    file_path_name.push_str(MAIN_SEPARATOR.to_string().as_str());
    file_path_name.push_str(USER_DIRECTORY);
    file_path_name.push_str(MAIN_SEPARATOR.to_string().as_str());
    file_path_name.push_str(FILE_NAME);
    return file_path_name;
}

pub fn create_file() -> String {
    let file_path_name = get_file_path_name();
    let file_path = std::path::Path::new(file_path_name.as_str());
    let file_path_directory = file_path.parent().unwrap().to_str().unwrap().to_string();
    if !file_or_directory_exists(&file_path_directory) {
        info!("Creating directory {}", file_path_directory);
        std::fs::create_dir(&file_path_directory).unwrap();
    }
    let file = std::fs::File::create(file_path);
    if file.is_err() {
        return String::new();
    }
    return file_path_name;
}

fn file_or_directory_exists(file_path_name: &String) -> bool {
    let file_path = std::path::Path::new(file_path_name.as_str());
    return file_path.exists();
}

pub fn write_to_file(user: &User) -> bool {
    let json_content = read_user_file();
    let mut users: Vec<User> = serde_json::from_str(&json_content)
        .unwrap_or_else(|error| {
        error!("Failed to parse json content: {:?}", error);
        Vec::new()
    });
    users.push(user.clone());
    let serialised_users = serde_json::to_string(&users).unwrap();
    let mut file = match OpenOptions::new().write(true).truncate(true).open(&get_file_path_name()) {
        Ok(file) => file,
        Err(error) => {
            error!("Could not open file for writing {:?}", error);
            return false;
        }
    };
    return match file.write_all(&serialised_users.as_bytes()) {
        Ok(()) => true,
        Err(error) => {
            error!("Could not write users to json {:?}", error);
            false
        }
    }
}

pub fn read_user_file() -> String {
    let file_path = get_file_path_name();
    let mut file = match std::fs::File::open(&file_path) {
        Ok(file) => file,
        Err(error) => {
            error!("Could not read file at path {:?} {}", file_path, error);
            return String::new();
        }
    };
    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(_file) => info!("Successfully read file at {}, size is {}", file_path, _file),
        Err(error) => error!("Failed to read file at {} {}", file_path, error)
    }
    return buffer.to_string();
}