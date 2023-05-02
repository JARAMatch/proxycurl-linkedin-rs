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
pub struct HonourAward {
    /// Title of the honour/award.
    #[serde(
        rename = "title",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<Option<String>>,
    /// The organisation body issuing this honour/award.
    #[serde(
        rename = "issuer",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub issuer: Option<Option<String>>,
    #[serde(rename = "issued_on", skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<Box<crate::models::Date>>,
    /// Description of the honour/award.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
}

impl HonourAward {
    pub fn new() -> HonourAward {
        HonourAward {
            title: None,
            issuer: None,
            issued_on: None,
            description: None,
        }
    }
}