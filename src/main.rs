use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderName, HeaderValue, Method,
    },
    routing::{delete, get, post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
// use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tokio;
mod handlers;
mod json_payload;
mod renderers;
mod state;
mod todos;
mod views;

// lazy_static! {
//     static ref TODOS: Mutex<todos::TodoList> = Mutex::new(todos::init_todods());
// }

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000/*".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(false)
        .allow_headers(Any);
    let app = Router::new()
        .route("/", get(renderers::renderers::render_index))
        .route(
            "/views/add-todo-form",
            get(renderers::renderers::render_add_todo_form),
        )
        .route(
            "/views/to-do-list",
            get(renderers::renderers::render_to_do_list),
        )
        .route("/api/todos", get(handlers::handlers::get_todos))
        .route("/api/healthcheck", get(handlers::handlers::health_check))
        .route("/api/add/todos", post(handlers::handlers::add_todos))
        .route("/api/change-state", post(handlers::handlers::change_state))
        .route(
            "/api/delete/todos/:id",
            delete(handlers::handlers::delete_todo).patch(handlers::handlers::update_todo),
        )
        .with_state(Arc::new(Mutex::new(state::init_state())))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    print!("Listening on port {}...\n", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
