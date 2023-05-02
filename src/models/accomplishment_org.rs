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
pub struct AccomplishmentOrg {
    #[serde(rename = "starts_at", skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<Box<crate::models::Date>>,
    #[serde(rename = "ends_at", skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<Box<crate::models::Date>>,
    #[serde(
        rename = "org_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub org_name: Option<Option<String>>,
    #[serde(
        rename = "title",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<Option<String>>,
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
}

impl AccomplishmentOrg {
    pub fn new() -> AccomplishmentOrg {
        AccomplishmentOrg {
            starts_at: None,
            ends_at: None,
            org_name: None,
            title: None,
            description: None,
        }
    }
}