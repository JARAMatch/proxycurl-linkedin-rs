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
pub struct Patent {
    /// Title of the patent.
    #[serde(
        rename = "title",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<Option<String>>,
    /// The organisation body that issued the patent.
    #[serde(
        rename = "issuer",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub issuer: Option<Option<String>>,
    #[serde(rename = "issued_on", skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<Box<crate::models::Date>>,
    /// Description of the patent.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    /// Numerical representation that identifies the patent.
    #[serde(
        rename = "application_number",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub application_number: Option<Option<String>>,
    /// Application number of the patent.
    #[serde(
        rename = "patent_number",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub patent_number: Option<Option<String>>,
    #[serde(
        rename = "url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<Option<String>>,
}

impl Patent {
    pub fn new() -> Patent {
        Patent {
            title: None,
            issuer: None,
            issued_on: None,
            description: None,
            application_number: None,
            patent_number: None,
            url: None,
        }
    }
}
