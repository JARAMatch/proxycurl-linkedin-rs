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
pub struct InferredSalary {
    #[serde(
        rename = "min",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub min: Option<Option<f32>>,
    #[serde(
        rename = "max",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max: Option<Option<f32>>,
}

impl InferredSalary {
    pub fn new() -> InferredSalary {
        InferredSalary {
            min: None,
            max: None,
        }
    }
}
