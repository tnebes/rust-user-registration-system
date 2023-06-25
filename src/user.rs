use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub age: u32
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.first_name.trim().eq_ignore_ascii_case(other.first_name.trim()) &&
            self.last_name.trim().eq_ignore_ascii_case(other.last_name.trim()) &&
            self.age == other.age
    }
}

pub fn user_to_json(user: &User) -> String {
    return serde_json::to_string(&user).unwrap();
}
