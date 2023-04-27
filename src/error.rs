use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum ProxyCurlError {
    /// Underlying error from reqwest library after an API call was made
    #[error("http error: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// ProxyCurl returns error object with details of API call failure
    #[error("{}: {}", .0.r#type, .0.message)]
    ApiError(ApiError),
    /// Error when a response cannot be deserialized into a Rust type
    #[error("failed to deserialize api response: {0}")]
    JSONDeserialize(serde_json::Error),
    /// Error from client side validation
    /// or when builder fails to build request before making API call
    #[error("invalid args: {0}")]
    InvalidArgument(String),
}

/// OpenAI API returns error object on failure
#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub r#type: String,
    pub param: Option<serde_json::Value>,
    pub code: Option<serde_json::Value>,
}

/// Wrapper to deserialize the error object nested in "error" JSON key
#[derive(Debug, Deserialize)]
pub(crate) struct WrappedError {
    pub(crate) error: ApiError,
}

pub(crate) fn map_deserialization_error(e: serde_json::Error, bytes: &[u8]) -> ProxyCurlError {
    tracing::error!(
        "failed deserialization of: {}",
        String::from_utf8_lossy(bytes.as_ref())
    );
    ProxyCurlError::JSONDeserialize(e)
}