pub mod state {
    use std::mem::take;

    use crate::todos::{self, TodoList};
    use serde_derive::{Deserialize, Serialize};

    #[derive(Debug)]
    pub enum StateNames {
        Login,
        ToDos,
        PageNotFound
    }

    impl StateNames {
        pub fn as_string(&self) -> &'static str {
            match self {
                &StateNames::Login => "logIn",
                &StateNames::ToDos => "ToDos",
                &StateNames::PageNotFound => "PageNotFound"
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
            let initial_state = StateNames::ToDos;
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
    }

    #[derive(Debug)]
    pub struct ApplicationState {
        pub state: State,
        pub todos: TodoList,
    }

    impl Default for ApplicationState {
        fn default() -> Self {
            return Self {
                todos: todos::init_todods(),
                state: State::default(),
            };
        }
    }
}

pub use state::{ApplicationState, State, StateNames};

pub fn init_state() -> ApplicationState {
    return state::ApplicationState::default();
}
