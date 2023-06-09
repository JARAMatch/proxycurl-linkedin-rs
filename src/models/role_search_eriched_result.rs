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
pub struct RoleSearchErichedResult {
    /// LinkedIn Profile URL of the person that most closely matches the role
    #[serde(
        rename = "linkedin_profile_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub linkedin_profile_url: Option<Option<String>>,
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    pub profile: Option<Box<crate::models::PersonEndpointResponse>>,
}

impl RoleSearchErichedResult {
    pub fn new() -> RoleSearchErichedResult {
        RoleSearchErichedResult {
            linkedin_profile_url: None,
            profile: None,
        }
    }
}
