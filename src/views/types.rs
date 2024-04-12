use crate::db;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewsParams {
    pub user: Option<db::User>,
}

impl ViewsParams {
    pub fn new() -> Self {
        return Self {
            user: None,
        };
    }
}