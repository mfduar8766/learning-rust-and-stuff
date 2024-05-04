use crate::{db, utils::AsString};
use serde_derive::{Deserialize, Serialize};
use std::mem::take;

#[derive(Debug)]
pub enum StateNames {
    Login,
    DashBoard,
    PageNotFound,
}

impl AsString for StateNames {
    fn as_string(&self) -> &'static str {
        match self {
            &StateNames::Login => "LogIn",
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
    pub fn change_state(&mut self, state: &str) -> &mut Self {
        let new_state = state;
        self.previous_state = take(&mut self.state);
        self.state = new_state.to_string();
        return self;
    }
    pub fn get_state(&self) -> String {
        let state = &self.state;
        return state.to_string();
    }
    pub fn _get_previous_state(&self) -> String {
        let prev_state = &self.previous_state;
        return prev_state.to_string();
    }
}

#[derive(Debug)]
pub struct ApplicationState {
    pub state: State,
    pub db: db::Db,
}

impl ApplicationState {
    pub fn new(db: db::Db) -> Self {
        return Self {
            state: State::default(),
            db,
        };
    }
}
