pub mod types;

pub mod views {
    use crate::db;
    use askama::Template;

    #[derive(Template)]
    #[template(path = "index.html")]
    pub struct IndexTemplate {
        pub state: String,
    }

    #[derive(Template)]
    #[template(path = "dashBoard.html")]
    pub struct DashBoardTemplate {
        pub user: db::User,
    }

    #[derive(Template)]
    #[template(path = "login.html")]
    pub struct LogInTemplate {}

    #[derive(Template)]
    #[template(path = "pageNotFound.html")]
    pub struct PageNotFoundTemplate {}

    #[derive(Template)]
    #[template(path = "error.html")]
    pub struct ErrorTemplate {
        pub message: String,
    }
}
