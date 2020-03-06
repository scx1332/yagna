/*
 * Yagna Activity API
 *
 * It conforms with capability level 1 of the [Activity API specification](https://docs.google.com/document/d/1BXaN32ediXdBHljEApmznSfbuudTU8TmvOmHKl0gmQM).
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Error, Default)]
#[error("Yagna API error: {message:?}")]
pub struct ErrorMessage {
    pub message: Option<String>,
}

impl ErrorMessage {
    pub fn new(message: String) -> ErrorMessage {
        ErrorMessage {
            message: Some(message),
        }
    }
}

impl<T: Into<String>> From<T> for ErrorMessage {
    fn from(s: T) -> Self {
        Self::new(s.into())
    }
}
