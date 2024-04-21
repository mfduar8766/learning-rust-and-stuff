use crate::state::StateNames;
use crate::{db, types};
use crate::{state::ApplicationState, utils::AsString, views};
use askama::Template;
use axum::http::HeaderMap;
use axum::response::Html;
use tracing::info;

pub fn reder_index(
    state: std::sync::MutexGuard<'_, ApplicationState>,
    mut headers: HeaderMap,
    view_params: views::types::ViewsParams,
) -> types::AxumResponse {
    headers.insert("Content-Type", "text/html".parse().unwrap());
    let current_state = state.state.get_state();
    return handle_page_render(&current_state, headers, Some(view_params));
}

pub fn handle_page_render(
    state: &str,
    mut headers: HeaderMap,
    view_params: Option<views::types::ViewsParams>,
) -> types::AxumResponse {
    headers.insert("Content-Type", "text/html".parse().unwrap());
    info!("renderers::handlePageRender()::state: {}", state);
    match state.to_string() {
        state_name => {
            if state_name == StateNames::Login.as_string() {
                let template = views::views::IndexTemplate { state: state_name };
                let render = template.render();
                return match render {
                    Ok(result) => Html(result),
                    Err(_) => render_page_not_found(),
                };
            } else if state_name == StateNames::DashBoard.as_string() {
                return match view_params {
                    Some(view) => {
                        return match view.user {
                            Some(user) => render_dash_baord(user),
                            _ => render_page_not_found(),
                        };
                    }
                    None => render_page_not_found(),
                };
            } else {
                return render_page_not_found();
            }
        }
    }
}

pub fn render_error_message(message: &str) -> types::AxumResponse {
    let template = views::views::ErrorTemplate {
        message: message.to_string(),
    };
    let render = template.render();
    match render {
        Ok(res) => Html(res),
        Err(_) => render_page_not_found(),
    }
}

pub fn render_page_not_found() -> types::AxumResponse {
    let template = views::views::PageNotFoundTemplate {};
    return Html(template.render().unwrap());
}

pub async fn auth(mut headers: HeaderMap) -> types::AxumResponse {
    headers.insert("Content-Type", "text/html".parse().unwrap());
    let template = views::views::LogInTemplate {};
    let render = template.render();
    return match render {
        Ok(result) => Html(result),
        Err(_) => render_page_not_found(),
    };
}

fn render_dash_baord(user: db::User) -> types::AxumResponse {
    let template = views::views::DashBoardTemplate { user };
    let render = template.render();
    match render {
        Ok(res) => Html(res),
        Err(_) => render_page_not_found(),
    }
}
