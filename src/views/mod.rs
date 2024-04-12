pub mod types;

pub mod views {
    use crate::{db, todos::Todos};
    use askama::Template;

    #[derive(Template)]
    #[template(path = "index2.html")]
    pub struct IndexTemplateII {
        pub state: String
    }

    #[derive(Template)]
    #[template(path = "dashBoard.html")]
    pub struct DashBoardTemplate {
        pub user: db::User,
    }

    #[derive(Template)]
    #[template(path = "index.html")]
    pub struct IndexTemplate<'a> {
        pub todos: &'a Vec<Todos>
    }

    #[derive(Template)]
    #[template(path = "login.html")]
    pub struct LogInTemplate {}

    #[derive(Template)]
    #[template(path = "pageNotFound.html")]
    pub struct PageNotFoundTemplate {}

    #[derive(Template)]
    #[template(path = "toDoListItem.html")]
    pub struct ToDoListItem<'a> {
        pub todo: &'a Todos,
    }

    #[derive(Template)]
    #[template(path = "error.html")]
    pub struct ErrorTemplate {
        pub message: String
    }
}
