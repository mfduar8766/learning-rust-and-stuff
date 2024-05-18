use crate::state::StateNames;
use crate::views::types::ViewsParams;
use crate::{db, types, CONFIG};
use crate::{state::ApplicationState, utils::AsString, views};
use askama::Template;
use axum::http::HeaderMap;
use axum::response::Html;
use tracing::info;

pub fn reder_index(
    state: &mut std::sync::MutexGuard<'_, ApplicationState>,
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
    match state {
        state_name => {
            if state_name == StateNames::Login.as_string() {
                let template = views::views::IndexTemplate {
                    state: state_name,
                    api_url: &CONFIG.lock().unwrap().url,
                };
                let render = template.render();
                return match render {
                    Ok(result) => Html(result),
                    Err(_) => render_page_not_found(),
                };
            } else if state_name == StateNames::DashBoard.as_string() {
                let view = match view_params {
                    Some(view) => view,
                    _ => ViewsParams::new(views::types::ViewParamsOptions {
                        user: None,
                        itineary: None,
                    }),
                };
                let user = match view.user {
                    Some(u) => u,
                    _ => db::Users::default(),
                };
                let itineary = match view.itineary {
                    Some(it) => it,
                    _ => vec![db::Itinieary::default()],
                };
                return render_dash_baord(user, itineary);
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

fn render_dash_baord(user: db::Users, itineary: Vec<db::Itinieary>) -> types::AxumResponse {
    let template = views::views::DashBoardTemplate {
        user,
        itineary,
        api_url: &CONFIG.lock().unwrap().url,
    };
    let render = template.render();
    match render {
        Ok(res) => Html(res),
        Err(_) => render_page_not_found(),
    }
}
