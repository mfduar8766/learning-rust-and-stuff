use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

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
    TodoStatus,
    State,
}

impl AsString for CustomHeaders {
    fn as_string(&self) -> &'static str {
        match self {
            &CustomHeaders::TodoStatus => "todo_status",
            &CustomHeaders::State => "state",
        }
    }
}
