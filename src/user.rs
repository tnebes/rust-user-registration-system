use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub age: u32
}

pub fn user_to_json(user: &User) -> String {
    return serde_json::to_string(&user).unwrap();
}
