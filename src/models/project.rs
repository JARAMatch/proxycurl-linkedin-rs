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
pub struct Project {
    #[serde(rename = "starts_at", skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<Box<crate::models::Date>>,
    #[serde(rename = "ends_at", skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<Box<crate::models::Date>>,
    ///                  Name of the project that has been or is currently being worked on.             
    #[serde(
        rename = "title",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<Option<String>>,
    /// Description of the project.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    /// A web location related to the project.
    #[serde(
        rename = "url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<Option<String>>,
}

impl Project {
    pub fn new() -> Project {
        Project {
            starts_at: None,
            ends_at: None,
            title: None,
            description: None,
            url: None,
        }
    }
}