use crate::{
    json_payload, renderers,
    state::{ApplicationState, StateNames},
    types,
    utils::{AsString, CustomHeaders},
    views::{self, types::ViewParamsOptions},
    CONFIG,
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
use std::{env, mem::take, sync::Arc};
use tokio_util::{self};
use tracing::{error, info, warn};

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
) -> impl IntoResponse {
    info!("handlers::handleLogIn()");
    headers.insert("Content-Type", "text/html".parse().unwrap());
    headers.insert("HX-Location", "/dash-board".parse().unwrap());
    let mut is_authenticated = false;
    let mut state_ref = state.lock().await;
    // let db_types = state_ref.get_db_types();
    match state_ref
        .get_db_types()
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
        let mut h = HeaderMap::new();
        h.insert(CONTENT_TYPE, "text/html".parse().unwrap());
        h.insert("HX-Retarget", "#error-message".parse().unwrap());
        h.insert("HX-Reswap", "innerHTML".parse().unwrap());
        return (
            StatusCode::BAD_REQUEST,
            h,
            renderers::render_error_message("email or password is invalid. Please try again."),
        );
        // return Response::builder()
        //     .status(StatusCode::BAD_REQUEST)
        //     .header(CONTENT_TYPE, "text/html")
        //     .header("HX-Retarget", "#error-message")
        //     .header("HX-Reswap", "innerHTML")
        //     .body(Body::new(renderers::render_error_message(
        //         "email or password is invalid. Please try again.",
        //     ).0))
        //     .unwrap();
        // return renderers::render_error_message("email or password is invalid. Please try again.");
    }
    state_ref
        .state
        .change_state(StateNames::DashBoard.as_string());
    let view_params = Some(views::types::ViewsParams {
        user: Some(take(state_ref.get_db_types().get_user())),
        itineary: Some(take(state_ref.get_db_types().get_itineary())),
    });
    let mut h = HeaderMap::new();
    h.insert(CONTENT_TYPE, "text/html".parse().unwrap());
    return (
        StatusCode::OK,
        h,
        renderers::handle_page_render(&state_ref.state.get_state(), headers, view_params),
    );
    // utils::create_html_response(
    //     StatusCode::OK,
    //     [
    //         ("Content-type", "text/html"),
    //         ("HX-Location", "/dash-baord"),
    //     ],
    //     renderers::handle_page_render(&state.lock().await.state.get_state(), headers, view_params),
    // )
    // return renderers::handle_page_render(&state_ref.state.get_state(), headers, view_params);
    // (
    //     StatusCode::OK,
    //     [
    //         ("Content-type", "text/html"),
    //         ("HX-Location", "/dash-board"),
    //     ],
    //     renderers::handle_page_render(&state.lock().await.state.get_state(), headers, view_params),
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
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            error!("handlers::getImage()::dir does not exist: {}", e);
            return Err((StatusCode::BAD_REQUEST, "cannout open".to_string()));
        }
    };
    let dir_name = &std::path::Path::join(
        &current_dir,
        &CONFIG.get().unwrap().get_envs().assets_location,
    );
    if !std::path::Path::exists(&dir_name) {
        error!("handlers::getImage()::dir does not exist");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "dir does not exist cannot get image".to_string(),
        ));
    }
    let full_path = std::path::Path::join(std::path::Path::new(dir_name), resource_id.clone());
    let path = full_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    info!(
        "handlers::getImage()::path:{}, resource_id:{}",
        path, resource_id
    );
    let file = match tokio::fs::File::open(path.clone()).await {
        Ok(file) => file,
        Err(err) => {
            error!("MIME type cannot be determined");
            return Err((StatusCode::BAD_REQUEST, "cannout open".to_string()));
        }
    };
    let content_type = match mime_guess::from_path(&path).first_raw() {
        Some(mime) => mime,
        None => {
            error!("MIME type cannot be determined");
            return Err((
                StatusCode::BAD_REQUEST,
                "MIME Type couldn't be determined".to_string(),
            ));
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
