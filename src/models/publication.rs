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
pub struct Publication {
    /// Name of the publication.
    #[serde(
        rename = "name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    /// The publishing organisation body.
    #[serde(
        rename = "publisher",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher: Option<Option<String>>,
    #[serde(rename = "published_on", skip_serializing_if = "Option::is_none")]
    pub published_on: Option<Box<crate::models::Date>>,
    /// Description of the publication.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    /// URL of the publication.
    #[serde(
        rename = "url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<Option<String>>,
}

impl Publication {
    pub fn new() -> Publication {
        Publication {
            name: None,
            publisher: None,
            published_on: None,
            description: None,
            url: None,
        }
    }
}
