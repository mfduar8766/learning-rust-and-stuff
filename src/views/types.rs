use crate::db;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewsParams {
    pub user: Option<db::User>,
}

pub struct ViewParamsOptions {
    pub user: Option<db::User>,
}

impl ViewsParams {
    pub fn new(options: ViewParamsOptions) -> Self {
        return Self { user: options.user };
    }
}
