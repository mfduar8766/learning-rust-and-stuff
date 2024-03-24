pub mod renderers {
    use crate::{state::ApplicationState, views};
    use askama::Template;
    use axum::response::Html;
    use axum::{extract::State, http::HeaderMap};
    use std::sync::{Arc, Mutex};

    pub async fn render_index(mut headers: HeaderMap) -> axum::response::Html<std::string::String> {
        headers.insert("Content-Type", "text/html".parse().unwrap());
        let template = views::views::IndexTemplate {};
        let render = template.render();
        return match render {
            Ok(result) => Html(result),
            Err(_) => render_page_not_found(),
        };
    }

    pub async fn render_add_todo_form(
        mut headers: HeaderMap,
    ) -> axum::response::Html<std::string::String> {
        headers.insert("Content-Type", "text/html".parse().unwrap());
        let template = views::views::AddToDosFormTemplate {};
        let render = template.render();
        return match render {
            Ok(result) => Html(result),
            Err(_) => render_page_not_found(),
        };
    }

    pub async fn render_to_do_list(
        State(state): State<Arc<Mutex<ApplicationState>>>,
        mut headers: HeaderMap,
    ) -> axum::response::Html<std::string::String> {
        headers.insert("Content-Type", "text/html".parse().unwrap());
        let state_lock = state.lock().unwrap();
        let todos = state_lock.todos.get_todos();
        let template = views::views::TodoListTemplate { todos };
        let render = template.render();
        return match render {
            Ok(result) => Html(result),
            Err(_) => render_page_not_found(),
        };
    }

    fn render_page_not_found() -> axum::response::Html<std::string::String> {
        let template = views::views::PageNotFoundTemplate {};
        return Html(template.render().unwrap());
    }
}
