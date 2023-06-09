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
pub struct StudentList {
    ///          A list of student profiles (if enriched) and their associated profile URL.         
    #[serde(rename = "students", skip_serializing_if = "Option::is_none")]
    pub students: Option<Vec<crate::models::Student>>,
    ///          The API URI that will lead to the next page of results.         
    #[serde(
        rename = "next_page",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_page: Option<Option<String>>,
}

impl StudentList {
    pub fn new() -> StudentList {
        StudentList {
            students: None,
            next_page: None,
        }
    }
}
