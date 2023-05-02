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
pub struct AcquiredCompany {
    ///          LinkedIn Company Profile URL of company that was involved         
    #[serde(
        rename = "linkedin_profile_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub linkedin_profile_url: Option<Option<String>>,
    /// Crunchbase Profile URL of company that was involved
    #[serde(
        rename = "crunchbase_profile_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub crunchbase_profile_url: Option<Option<String>>,
    #[serde(rename = "announced_date", skip_serializing_if = "Option::is_none")]
    pub announced_date: Option<Box<crate::models::Date>>,
    /// Price of acquisition
    #[serde(
        rename = "price",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Option<i32>>,
}

impl AcquiredCompany {
    pub fn new() -> AcquiredCompany {
        AcquiredCompany {
            linkedin_profile_url: None,
            crunchbase_profile_url: None,
            announced_date: None,
            price: None,
        }
    }
}