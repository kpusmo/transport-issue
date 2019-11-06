use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Error {
    message: String,
    code: i32,
}

impl Error {
    #[allow(non_snake_case)]
    pub fn InternalServerError(message: impl Into<Option<String>>) -> Error {
        Error {
            message: message.into().unwrap_or("Server error".to_string()),
            code: 500
        }
    }
}