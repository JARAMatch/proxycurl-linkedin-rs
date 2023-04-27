use serde::de::DeserializeOwned;

use crate::{apis::*, error::{ProxyCurlError, WrappedError, map_deserialization_error}};


pub struct Client {
    http_client: reqwest::Client,
    api_key: String,
    api_base: String,
    backoff: backoff::ExponentialBackoff,
}

pub const API_BASE: &str = "https://nubela.co";

impl Default for Client {
    fn default() -> Self {
        Self {
            http_client: reqwest::Client::default(),
            api_key: std::env::var("PROXYCURL_API_KEY").unwrap_or_else(|_| "".to_string()),
            api_base: API_BASE.to_string(),
            backoff: Default::default(),
        }
    }
}

impl Client {
    /// Create client with default [API_BASE] url and default API key from OPENAI_API_KEY env var
    pub fn new() -> Self {
        Self::default()
    }


    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = api_key.to_string();
        self
    }

    /// To use a API base url different from default [API_BASE]
    pub fn with_api_base(mut self, api_base: &str) -> Self {
        self.api_base = api_base.to_string();
        self
    }

    /// Exponential backoff for retrying rate_limited requests.
    /// Form submissions are not retried.
    pub fn with_backoff(mut self, backoff: backoff::ExponentialBackoff) -> Self {
        self.backoff = backoff;
        self
    }

    pub fn api_base(&self) -> &str {
        &self.api_base
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Provide your own [client] to make HTTP requests with.
    ///
    /// [client]: reqwest::Client
    pub fn with_http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = http_client;
        self
    }

    // API groups

    /// To call [People] group related APIs using this client.
    pub fn people(&self) -> people::People {
        people::People::new(self)
    }
    
    /// Make a GET request to {path} and deserialize the response body
    pub(crate) async fn get<O>(&self, path: &str, query: Option<&[(&str,String)]>) -> Result<O, ProxyCurlError>
    where
        O: DeserializeOwned,
    {
        let mut request = self
            .http_client
            .get(format!("{}{path}", self.api_base()))
            .bearer_auth(self.api_key());

        if let Some(query) = query {
            request = request.query(query);
        }


        let request = request.build()?;

        self.execute(request).await
    }

    /// Deserialize response body from either error object or actual response object
    async fn process_response<O>(&self, response: reqwest::Response) -> Result<O, ProxyCurlError>
    where
        O: DeserializeOwned,
    {
        let status = response.status();
        let bytes = response.bytes().await?;

        if !status.is_success() {
            let wrapped_error: WrappedError = serde_json::from_slice(bytes.as_ref())
                .map_err(|e| map_deserialization_error(e, bytes.as_ref()))?;

            return Err(ProxyCurlError::ApiError(wrapped_error.error));
        }

        let response: O = serde_json::from_slice(bytes.as_ref())
            .map_err(|e| map_deserialization_error(e, bytes.as_ref()))?;
        Ok(response)
    }

    /// Execute any HTTP requests and retry on rate limit, except streaming ones as they cannot be cloned for retrying.
    async fn execute<O>(&self, request: reqwest::Request) -> Result<O, ProxyCurlError>
    where
        O: DeserializeOwned,
    {
        let client = self.http_client.clone();

        match request.try_clone() {
            // Only clone-able requests can be retried
            Some(request) => {
                backoff::future::retry(self.backoff.clone(), || async {
                    let response = client
                        .execute(request.try_clone().unwrap())
                        .await
                        .map_err(ProxyCurlError::Reqwest)
                        .map_err(backoff::Error::Permanent)?;

                    let status = response.status();
                    let bytes = response
                        .bytes()
                        .await
                        .map_err(ProxyCurlError::Reqwest)
                        .map_err(backoff::Error::Permanent)?;

                    // Deserialize response body from either error object or actual response object
                    if !status.is_success() {
                        let wrapped_error: WrappedError = serde_json::from_slice(bytes.as_ref())
                            .map_err(|e| map_deserialization_error(e, bytes.as_ref()))
                            .map_err(backoff::Error::Permanent)?;

                        if status.as_u16() == 429
                            // API returns 429 also when:
                            // "You exceeded your current quota, please check your plan and billing details."
                            && wrapped_error.error.r#type != "insufficient_quota"
                        {
                            // Rate limited retry...
                            tracing::warn!("Rate limited: {}", wrapped_error.error.message);
                            return Err(backoff::Error::Transient {
                                err: ProxyCurlError::ApiError(wrapped_error.error),
                                retry_after: None,
                            });
                        } else {
                            return Err(backoff::Error::Permanent(ProxyCurlError::ApiError(
                                wrapped_error.error,
                            )));
                        }
                    }

                    let response: O = serde_json::from_slice(bytes.as_ref())
                        .map_err(|e| map_deserialization_error(e, bytes.as_ref()))
                        .map_err(backoff::Error::Permanent)?;
                    Ok(response)
                })
                .await
            }
            None => {
                let response = client.execute(request).await?;
                self.process_response(response).await
            }
        }
    }
}