#[derive(Debug)]
pub struct Db {
    email: String,
    password: String,
}

impl Default for Db {
    fn default() -> Self {
        return Self {
            email: String::from("test@tester12.com"),
            password: String::from("123"),
        };
    }
}

impl Db {
    pub fn authenticate(email: &str, password: &str) -> bool {
        if email.len() == 0 || password.len() == 0 {
            return false;
        } else if email.contains("test@tester12.com") && password.contains("123") {
            return true;
        } else {
          return false;
        }
    }
}
