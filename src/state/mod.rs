pub mod state {
    use crate::{
        db,
        todos::{self, TodoList},
        utils::AsString,
        views,
    };
    use serde_derive::{Deserialize, Serialize};
    use std::mem::take;

    #[derive(Debug)]
    pub enum StateNames {
        Login,
        DashBoard,
        ToDos,
        PageNotFound,
    }

    impl AsString for StateNames {
        fn as_string(&self) -> &'static str {
            match self {
                &StateNames::Login => "LogIn",
                &StateNames::ToDos => "ToDos",
                &StateNames::PageNotFound => "PageNotFound",
                &StateNames::DashBoard => "DashBoard",
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct State {
        state: String,
        previous_state: String,
    }

    impl Default for State {
        fn default() -> Self {
            let initial_state = StateNames::Login;
            return Self {
                state: StateNames::as_string(&initial_state).to_string(),
                previous_state: String::new(),
            };
        }
    }
    impl State {
        pub fn change_state(&mut self, state: String) -> &mut Self {
            let new_state = state;
            self.previous_state = take(&mut self.state);
            self.state = new_state;
            return self;
        }
        pub fn get_state(&self) -> String {
            let state = &self.state;
            return state.to_string();
        }
        pub fn get_previous_state(&self) -> String {
            let prev_state = &self.previous_state;
            return prev_state.to_string();
        }
    }

    #[derive(Debug)]
    pub struct ApplicationState {
        pub state: State,
        pub todos: TodoList,
        pub db: db::Db,
        pub views: views::types::ViewsParams,
    }

    impl Default for ApplicationState {
        fn default() -> Self {
            return Self {
                todos: todos::init_todods(),
                state: State::default(),
                db: db::Db::new(),
                views: views::types::ViewsParams::new(),
            };
        }
    }
}

pub use state::{ApplicationState, State, StateNames};

pub fn init_state() -> ApplicationState {
    return state::ApplicationState::default();
}
