use crate::{
    json_payload, renderers,
    state::{ApplicationState, StateNames},
    types,
    utils::{AsString, CustomHeaders},
    views::{self, types::ViewParamsOptions},
};
use axum::{
    extract::{Path, State},
    http::{
        header::{CONTENT_DISPOSITION, CONTENT_TYPE},
        HeaderMap, StatusCode,
    },
    response::{AppendHeaders, IntoResponse},
    Form, Json,
};
use axum_macros::debug_handler;
use futures::lock::Mutex;
use std::{mem::take, sync::Arc};
use tokio_util;
use tracing::{info, warn};

#[debug_handler]
pub async fn index(
    State(state): State<Arc<Mutex<ApplicationState>>>,
    headers: HeaderMap,
) -> types::AxumResponse {
    let mut view_params = views::types::ViewsParams::new(ViewParamsOptions {
        user: None,
        itineary: None,
    });
    let state_instance = &mut state.lock().await;
    if state_instance.get_db_types().is_authenticated() {
        view_params.user = Some(take(state_instance.get_db_types().get_user()));
        view_params.itineary = Some(take(state_instance.get_db_types().get_itineary()));
    }
    return renderers::reder_index(&state_instance.state.get_state(), headers, view_params);
}

#[debug_handler]
pub async fn handle_login(
    State(state): State<Arc<Mutex<ApplicationState>>>,
    mut headers: HeaderMap,
    Form(payload): Form<types::LogInForm>,
) -> types::AxumResponse {
    info!("handlers::handleLogIn()");
    headers.insert("Content-Type", "text/html".parse().unwrap());
    headers.insert("HX-Location", "/dash-board".parse().unwrap());
    let mut is_authenticated = false;
    let mut state_ref = state.lock().await;
    let db_types = state_ref.get_db_types();
    match db_types
        .authenticate(&payload.email, &payload.password)
        .await
    {
        Ok(()) => {
            is_authenticated = true;
        }
        Err(_) => warn!("handlers::handleLogIn()::unable to login user not authenticated"),
    };
    if !is_authenticated {
        warn!("handlers::handleLogIn():email or password is invalid");
        return renderers::render_error_message("email or password is invalid. Please try again.");
    }
    state
        .lock()
        .await
        .state
        .change_state(StateNames::DashBoard.as_string());
    let view_params = Some(views::types::ViewsParams {
        user: Some(take(db_types.get_user())),
        itineary: Some(take(db_types.get_itineary())),
    });
    // return utils::create_html_response(
    //     StatusCode::OK,
    //     [
    //         ("Content-type", "text/html"),
    //         ("HX-Location", "/dash-baord"),
    //     ],
    //     renderers::handle_page_render(&state.lock().await.state.get_state(), headers, view_params),
    // );
    return renderers::handle_page_render(&state_ref.state.get_state(), headers, view_params);
    // (
    //     StatusCode::OK,
    //     [
    //         ("Content-type", "text/html"),
    //         ("HX-Location", "/dash-board"),
    //     ],
    //     renderers::handle_page_render(&state.lock().unwrap().state.get_state(), headers, view_params),
    // )
}

pub async fn handle_logout(
    State(state): State<Arc<Mutex<ApplicationState>>>,
    mut headers: HeaderMap,
) -> types::AxumResponse {
    info!("handlers::handleLogOut()");
    headers.insert("Content-Type", "text/html".parse().unwrap());
    let state_lock = &mut state.lock().await;
    state_lock.get_db_types().set_is_authenticated(false);
    state_lock.state.change_state(StateNames::Login.as_string());
    let view_params: Option<views::types::ViewsParams> = Some(views::types::ViewsParams {
        user: None,
        itineary: None,
    });
    return renderers::handle_page_render(&state_lock.state.get_state(), headers, view_params);
}

pub async fn get_image(Path(resource_id): Path<String>) -> impl IntoResponse {
    let path = &format!("../../images/{}", resource_id);
    info!("handlers::getImage()::resource_id:{}", resource_id);
    let file = match tokio::fs::File::open(path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("image not found: {}", err))),
    };
    let content_type = match mime_guess::from_path(&path).first_raw() {
        Some(mime) => mime,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                "MIME Type couldn't be determined".to_string(),
            ))
        }
    };
    let stream = tokio_util::io::ReaderStream::new(file);
    let body = axum::body::Body::from_stream(stream);
    let headers = AppendHeaders([
        (CONTENT_TYPE, content_type),
        (CONTENT_DISPOSITION, "imag from user iteniary"),
    ]);
    Ok((headers, body))
}

pub async fn health_check() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
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
    let mut state_lock = state.lock().await;
    let updated_state = state_lock.state.change_state(&headers_state.to_string());
    let payload_params =
        json_payload::JsonPayloadParams::new(std::option::Option::Some(take(updated_state)));
    let payload = json_payload::JsonPayload::new(
        format!("{}", StatusCode::OK),
        std::option::Option::None,
        payload_params,
    );
    Ok(Json(payload.create_json_payload()))
}

// pub async fn get_todos(
//     State(state): State<Arc<Mutex<ApplicationState>>>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let todo_list = take(state.lock().unwrap().todos.get_todos_as_mut());
//     let payload_params = json_payload::JsonPayloadParams::new(std::option::Option::Some(todo_list));
//     let payload = json_payload::JsonPayload::new(
//         format!("{}", StatusCode::OK),
//         std::option::Option::None,
//         payload_params,
//     );
//     Ok(Json(payload.create_json_payload()))
// }

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
        _ => {
            let page_not_found = StateNames::PageNotFound;
            return StateNames::as_string(&page_not_found);
        }
    };
}
