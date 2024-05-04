use crate::{
    json_payload, renderers,
    state::{ApplicationState, StateNames},
    types,
    utils::{AsString, CustomHeaders},
    views::{self, types::ViewParamsOptions},
};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Form, Json,
};
use std::{
    mem::take,
    sync::{Arc, Mutex},
};
use tracing::{info, warn};

pub async fn index(
    State(state): State<Arc<Mutex<ApplicationState>>>,
    headers: HeaderMap,
) -> types::AxumResponse {
    let mut view_params = views::types::ViewsParams::new(ViewParamsOptions { user: None });
    let state_instance = &mut state.lock().unwrap();
    if state_instance.db.is_authenticated() {
        view_params.user = Some(take(&mut state_instance.db.get_user()));
    }
    return renderers::reder_index(state_instance, headers, view_params);
}

pub async fn handle_login(
    State(state): State<Arc<Mutex<ApplicationState>>>,
    mut headers: HeaderMap,
    Form(payload): Form<types::LogInForm>,
) -> types::AxumResponse {
    info!("handlers::handleLogIn()");
    headers.insert("Content-Type", "text/html".parse().unwrap());
    headers.insert("HX-Location", "/dash-board".parse().unwrap());
    let state_lock = &mut state.lock().unwrap();
    if !state_lock
        .db
        .authenticate(&payload.email, &payload.password)
    {
        warn!("handlers::handleLogIn():email or password is invalid");
        return renderers::render_error_message("email or password is invalid. Please try again.");
    }
    state_lock
        .state
        .change_state(StateNames::DashBoard.as_string());
    let view_params = Some(views::types::ViewsParams {
        user: Some(take(&mut state_lock.db.get_user())),
    });
    return renderers::handle_page_render(&state_lock.state.get_state(), headers, view_params);
    // (
    //     StatusCode::OK,
    //     [
    //         ("Content-type", "text/html"),
    //         ("HX-Redirect", "/dash-board"),
    //     ],
    //     renderers::handle_page_render(&state_lock.state.get_state(), headers, view_params),
    // )
}

pub async fn handle_logout(
    State(state): State<Arc<Mutex<ApplicationState>>>,
    mut headers: HeaderMap,
) -> types::AxumResponse {
    info!("handlers::handleLogOut()");
    headers.insert("Content-Type", "text/html".parse().unwrap());
    let state_lock = &mut state.lock().unwrap();
    state_lock.db.set_is_authenticated();
    state_lock.state.change_state(StateNames::Login.as_string());
    let view_params = Some(views::types::ViewsParams { user: None });
    return renderers::handle_page_render(&state_lock.state.get_state(), headers, view_params);
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
    let mut state_lock = state.lock().unwrap();
    let updated_state = state_lock.state.change_state(headers_state);
    let payload_params =
        json_payload::JsonPayloadParams::new(std::option::Option::Some(take(updated_state)));
    let payload = json_payload::JsonPayload::new(
        format!("{}", StatusCode::OK),
        std::option::Option::None,
        payload_params,
    );
    Ok(Json(payload.create_json_payload()))
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
        _ => {
            let page_not_found = StateNames::PageNotFound;
            return StateNames::as_string(&page_not_found);
        }
    };
}
