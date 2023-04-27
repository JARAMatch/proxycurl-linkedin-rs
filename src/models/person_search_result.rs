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
pub struct PersonSearchResult {
    ///          A list of SearchResult objects         
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<crate::models::SearchResult>>,
    ///          The URL to the next page of search results.         
    #[serde(
        rename = "next_page",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_page: Option<Option<String>>,
}

impl PersonSearchResult {
    pub fn new() -> PersonSearchResult {
        PersonSearchResult {
            results: None,
            next_page: None,
        }
    }
}
