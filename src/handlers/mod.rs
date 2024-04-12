pub mod handlers {
    use crate::{
        db, json_payload, renderers,
        state::{ApplicationState, StateNames},
        types,
        utils::{AsString, CustomHeaders},
        views,
    };
    use askama::Template;
    use axum::{
        extract::{Path, State},
        http::{HeaderMap, StatusCode},
        response::{Html, IntoResponse},
        Form, Json,
    };
    use std::{
        mem::take,
        sync::{Arc, Mutex},
    };
    use tracing::{info, warn};

    pub async fn health_check() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
    {
        let response = serde_json::json!({
            "status": format!("{}", StatusCode::OK),
            "message": "service up and healthy",
        });
        Ok(Json(response))
    }

    pub async fn change_state(
        State(state): State<Arc<Mutex<ApplicationState>>>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        let headers_state = get_state_from_headers(headers);
        let mut state_lock = state.lock().unwrap();
        let updated_state = state_lock.state.change_state(headers_state.to_string());
        let payload_params = json_payload::json_payload::JsonPayloadParams::new(
            std::option::Option::None,
            std::option::Option::None,
            std::option::Option::None,
            std::option::Option::Some(take(updated_state)),
        );
        let payload = json_payload::json_payload::JsonPayload::new(
            format!("{}", StatusCode::OK),
            std::option::Option::None,
            payload_params,
        );
        Ok(Json(payload.create_json_payload()))
    }

    pub async fn get_todos(
        State(state): State<Arc<Mutex<ApplicationState>>>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        let todo_list = take(state.lock().unwrap().todos.get_todos_as_mut());
        let payload_params = json_payload::json_payload::JsonPayloadParams::new(
            std::option::Option::Some(todo_list),
            std::option::Option::None,
            std::option::Option::None,
            std::option::Option::None,
        );
        let payload = json_payload::json_payload::JsonPayload::new(
            format!("{}", StatusCode::OK),
            std::option::Option::None,
            payload_params,
        );
        Ok(Json(payload.create_json_payload()))
    }

    pub async fn delete_todo(
        State(state): State<Arc<Mutex<ApplicationState>>>,
        Path(id): Path<uuid::Uuid>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        if id.is_nil() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"status": "error","message": "id is empty"})),
            ));
        }
        info!("Handlers::delete_todo()::id::{:?}", id);
        // TODOS.lock().unwrap().delete_todo(id);
        state.lock().unwrap().todos.delete_todo(id);
        let payload_params = json_payload::json_payload::JsonPayloadParams::new(
            std::option::Option::None,
            std::option::Option::None,
            std::option::Option::Some(id),
            std::option::Option::None,
        );
        let payload = json_payload::json_payload::JsonPayload::new(
            format!("{}", StatusCode::OK),
            std::option::Option::None,
            payload_params,
        );
        Ok(StatusCode::OK)
        //Ok(Json(payload.create_json_payload()))
    }

    pub async fn update_todo(
        State(state): State<Arc<Mutex<ApplicationState>>>,
        Path(id): Path<uuid::Uuid>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        let custom_status = CustomHeaders::TodoStatus;
        let todo_status = headers
            .get(CustomHeaders::as_string(&custom_status))
            .unwrap()
            .to_str()
            .unwrap();
        let status = match todo_status {
            "true" => true,
            "false" => false,
            "" => false,
            _ => false,
        };
        // let updated_todo = take(TODOS
        //     .lock()
        //     .unwrap()
        //     .update_todo_status(id, status)
        //     .unwrap());
        // let mut update_todo_lock = TODOS.lock().unwrap();
        let mut update_todo_lock = state.lock().unwrap();
        let update_totdo = update_todo_lock.todos.update_todo_status(id, status);
        return match update_totdo {
            Ok(todo) => {
                let taken_todo = take(todo);
                let payload_params = json_payload::json_payload::JsonPayloadParams::new(
                    std::option::Option::None,
                    std::option::Option::Some(taken_todo),
                    std::option::Option::None,
                    std::option::Option::None,
                );
                let payload = json_payload::json_payload::JsonPayload::new(
                    format!("{}", StatusCode::OK),
                    std::option::Option::None,
                    payload_params,
                );
                Ok(Json(payload.create_json_payload()))
            }
            Err(error) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "status": format!("{}", StatusCode::EXPECTATION_FAILED),
                    "message": format!("{}", error),
                })),
            )),
        };
    }

    pub async fn add_todos(
        State(state): State<Arc<Mutex<ApplicationState>>>,
        mut headers: HeaderMap,
        Form(payload): Form<types::AddTodos>,
    ) -> types::AxumResponse {
        info!("Handlers::addTodo()::payload: {:?}", payload);
        // let mut todo_lock = TODOS.lock().unwrap();
        let mut todo_lock = state.lock().unwrap();
        let new_todo = todo_lock.todos.add_todo(&payload.todo);
        // let payload_params = json_payload::json_payload::JsonPayloadParams::new(
        //     std::option::Option::None,
        //     std::option::Option::Some(new_todo),
        //     std::option::Option::None,
        //     std::option::Option::None,
        // );
        // let payload = json_payload::json_payload::JsonPayload::new(
        //     format!("{}", StatusCode::CREATED),
        //     std::option::Option::None,
        //     payload_params,
        // );
        // Ok(Json(payload.create_json_payload()))

        headers.insert("Content-Type", "text/html".parse().unwrap());
        let template = views::views::ToDoListItem { todo: &new_todo };
        let render = template.render();
        return match render {
            Ok(result) => Html(result),
            Err(_) => renderers::renderers::render_page_not_found(),
        };
    }

    pub async fn index(
        State(state): State<Arc<Mutex<ApplicationState>>>,
        headers: HeaderMap,
    ) -> types::AxumResponse {
        let view_params = views::types::ViewsParams { user: None };
        let state_instance = state.lock().unwrap();
        return renderers::renderers::reder_index_2(state_instance, headers, view_params);
    }

    pub async fn handle_login(
        State(state): State<Arc<Mutex<ApplicationState>>>,
        mut headers: HeaderMap,
        Form(payload): Form<types::LogInForm>,
    ) -> types::AxumResponse {
        info!("handlers::handleLogIn()::payload: {:?}\n", payload);
        headers.insert("Content-Type", "text/html".parse().unwrap());
        let state_lock = &mut state.lock().unwrap();
        if !state_lock
            .db
            .authenticate(&payload.email, &payload.password)
        {
            warn!("handlers::handleLogIn():: email or password is invalid");
            return renderers::renderers::render_error_message(
                "email or password is invalid. Please try again.",
            );
        }
        // let state_lock = &mut state.lock().unwrap().state;
        state_lock
            .state
            .change_state(StateNames::DashBoard.as_string().to_string());
        let view_params = Some(views::types::ViewsParams {
            user: Some(take(&mut state_lock.db.user)),
        });
        return renderers::renderers::handle_page_render(
            &state_lock.state.get_state(),
            headers,
            view_params,
        );
    }

    fn get_state_from_headers(headers: HeaderMap) -> &'static str {
        let state = CustomHeaders::State;
        let current_state = headers
            .get(CustomHeaders::as_string(&state))
            .unwrap()
            .to_str()
            .unwrap();
        match current_state {
            "LogIn" => {
                let log_in = StateNames::Login;
                return StateNames::as_string(&log_in);
            }
            "ToDo" => {
                let todo = StateNames::ToDos;
                return StateNames::as_string(&todo);
            }
            _ => {
                let page_not_found = StateNames::PageNotFound;
                return StateNames::as_string(&page_not_found);
            }
        };
    }

    // fn dispplay_view(
    //     State(state): State<Arc<ApplicationState>>,
    //     headers: HeaderMap,
    // ) -> axum::response::Html<std::string::String> {
    //     let mut headers_data = HeaderMap::new();
    //     headers_data.insert("Content-Type", "text/html".parse().unwrap());
    //     match &state.views {
    //         view => {
    //             if view.todo {
    //                 return handle_render_todo(&state.todos);
    //             } else {
    //                 return Html((&"Page Not Found").to_string());
    //             }
    //         }
    //     };
    // }

    // fn handle_render_todo(todos: &TodoList) -> axum::response::Html<std::string::String> {
    //     let todos = todos.get_todos();
    //     let template = views::views::TodoListTemplate { todos };
    //     let render = template.render();
    //     return match render {
    //         Ok(result) => Html(result),
    //         Err(error) => Html(format!("404 Page Not Found: {}", error)),
    //     };
    // }
}
