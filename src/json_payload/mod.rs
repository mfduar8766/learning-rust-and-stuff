pub mod json_payload {
    use crate::{
        state::State,
        todos::Todos,
    };
    use serde_derive::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct JsonPayloadParams {
        pub todos: Option<Vec<Todos>>,
        pub todo: Option<Todos>,
        pub id: Option<Uuid>,
        state: Option<State>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct JsonPayload {
        pub status: String,
        pub payload: JsonPayloadParams,
        pub message: Option<String>,
    }

    impl JsonPayloadParams {
        pub fn new(
            todos: Option<Vec<Todos>>,
            todo: Option<Todos>,
            id: Option<Uuid>,
            state: Option<State>,
        ) -> Self {
            return Self {
                todos,
                todo,
                id,
                state,
            };
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
}
