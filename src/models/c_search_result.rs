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
pub struct CSearchResult {
    ///          The LinkedIn Profile URL of the company         
    #[serde(
        rename = "linkedin_profile_url",
        skip_serializing_if = "Option::is_none"
    )]
    pub linkedin_profile_url: Option<String>,
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    pub profile: Option<Box<crate::models::LinkedinCompany>>,
}

impl CSearchResult {
    pub fn new() -> CSearchResult {
        CSearchResult {
            linkedin_profile_url: None,
            profile: None,
        }
    }
}