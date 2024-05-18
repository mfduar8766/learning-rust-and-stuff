use crate::state::State;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonPayloadParams {
    state: Option<State>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonPayload {
    pub status: String,
    pub payload: JsonPayloadParams,
    pub message: Option<String>,
}

impl JsonPayloadParams {
    pub fn new(state: Option<State>) -> Self {
        return Self { state };
    }
}

impl JsonPayload {
    pub fn new(status: String, message: Option<String>, payload: JsonPayloadParams) -> Self {
        return Self {
            status,
            payload,
            message,
        };
    }
    pub fn create_json_payload(&self) -> serde_json::Value {
        return serde_json::json!(self);
    }
}
