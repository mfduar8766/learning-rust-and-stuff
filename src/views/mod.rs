pub mod types;

pub mod views {
    use crate::db;
    use askama::Template;

    #[derive(Template)]
    #[template(path = "index.html")]
    pub struct IndexTemplate<'a> {
        pub state: &'a str,
        pub api_url: &'a str
    }

    #[derive(Template)]
    #[template(path = "dashBoard.html")]
    pub struct DashBoardTemplate<'a> {
        pub user: db::User,
        pub itineary: Vec<db::Itinerary>,
        pub api_url: &'a str
    }

    #[derive(Template)]
    #[template(path = "pageNotFound.html")]
    pub struct PageNotFoundTemplate {}

    #[derive(Template)]
    #[template(path = "error.html")]
    pub struct ErrorTemplate {
        pub message: String,
    }
}
