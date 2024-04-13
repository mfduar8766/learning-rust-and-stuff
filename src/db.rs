use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: String,
    password: String,
}

impl User {
    fn new() -> Self {
        return Self {
            email: String::from("test@tester12.com"),
            password: String::from("123"),
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            dob: String::from("1992/06/07"),
        };
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Db {
    pub user: User,
}

impl Db {
    pub fn new() -> Self {
        return Self { user: User::new() };
    }

    pub fn authenticate(&self, email: &str, password: &str) -> bool {
        if email.len() == 0 || password.len() == 0 {
            return false;
        } else if email.contains("test@tester12.com") && password.contains("123") {
            return true;
        } else {
            return false;
        }
    }
}
