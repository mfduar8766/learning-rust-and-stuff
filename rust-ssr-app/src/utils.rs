use axum::{
    body::{self, Body, HttpBody},
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Response},
};
use serde_derive::{Deserialize, Serialize};
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

#[derive(Serialize, Debug)]
pub struct AppResponse<'a> {
    pub http_status: i32,
    pub body: String,
    pub headers: [(&'a str, &'a str); 2],
}

impl<'a> AppResponse<'a> {
    pub fn new(http_status: i32, body: &str, headers: [(&'a str, &'a str); 2]) -> Self {
        return Self {
            http_status,
            headers,
            body: body.to_string()
        };
    }
}

impl<'a> IntoResponse for AppResponse<'a> {
    fn into_response(self) -> Response {
        let status = match self.http_status {
            200 => StatusCode::OK,
            400 => StatusCode::BAD_REQUEST,
            500 => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::IM_A_TEAPOT,
        };
        (status, self.headers, self).into_response()
    }
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct AppResponse {
//     pub body: String,
// }

// impl IntoResponse for AppResponse {
//     fn into_response(self) -> Response<Body> {
//         Response::builder()
//             .status(StatusCode::OK)
//             .header("Content-Type", "application/json")
//             .body(self)
//             .unwrap()
//     }
// }

// impl<'a> IntoResponse for AppResponse<'a> {
//     fn into_response(self) -> Response {
//         return (self.status, self.headers, self.body).into_response();
//     }
// }

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
    (code, response_headers, html).into_response();
}
