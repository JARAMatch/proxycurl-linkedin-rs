/*
 * Proxycurl API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DisposableEmail {
    /// Returns a boolean value of the disposable nature of the given email address
    #[serde(
        rename = "is_disposable_email",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_disposable_email: Option<i32>,
    /// Returns a boolean value of the free status of the given email address
    #[serde(rename = "is_free_email", skip_serializing_if = "Option::is_none")]
    pub is_free_email: Option<i32>,
}

impl DisposableEmail {
    pub fn new() -> DisposableEmail {
        DisposableEmail {
            is_disposable_email: None,
            is_free_email: None,
        }
    }
}
