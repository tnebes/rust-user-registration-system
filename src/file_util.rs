use std::env;
use std::path::MAIN_SEPARATOR;
use log::{error, info};

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

pub fn write_to_file(user_json: String) -> bool {
    let file_name_path = get_file_path_name();


    return false;
}

pub fn read_file() -> String {
    return String::new();
}