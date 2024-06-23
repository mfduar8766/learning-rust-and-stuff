use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
//use std::sync::Mutex;

// pub struct F(pub Mutex<config::Config>);
// unsafe impl Send for F {}

// #[derive(Debug)]
// pub struct MyType(pub config::Config);
// unsafe impl Send for MyType {}

// impl Deref for MyType {
//     type Target = config::Config;
//     fn deref(&self) -> &Self::Target {
//         return &self.0;
//     }
// }

// impl DerefMut for MyType {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         return &mut self.0;
//     }
// }

pub trait AsString {
    fn as_string(&self) -> &'static str;
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("something went wrong: {}", self.0),
        )
            .into_response();
    }
}

/*
This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
`Result<_, AppError>`. That way you don't need to do that manually.
*/
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        return Self(value.into());
    }
}

pub enum CustomHeaders {
    State,
}

impl AsString for CustomHeaders {
    fn as_string(&self) -> &'static str {
        match self {
            &CustomHeaders::State => "state",
        }
    }
}

pub fn create_html_response(
    code: axum::http::StatusCode,
    response_headers: [(&str, &str); 2],
    html: axum::response::Html<std::string::String>,
) -> impl IntoResponse {
    (code, response_headers, html);
}
