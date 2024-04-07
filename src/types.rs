use serde_derive::{Deserialize, Serialize};

pub type AxumResponse = axum::response::Html<std::string::String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LogInForm {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddTodos {
    pub todo: String,
}
