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
pub struct LinkedinCompany {
    ///          LinkedIn's Internal and immutable ID of this Company profile.         
    #[serde(
        rename = "linkedin_internal_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub linkedin_internal_id: Option<Option<String>>,
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    #[serde(
        rename = "website",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub website: Option<Option<String>>,
    /// The `industry` attribute, found in a LinkedIn Company            profile, describes the industry in which the company operates.            The value of this attribute is an enumerator. [This CSV file            provides an exhaustive list of possible values for this attribute]            (https://drive.google.com/file/d/12yvYLuru7CRv3wKOIkHs5Ldocz31gJSS/            view?usp=share_link).
    #[serde(
        rename = "industry",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub industry: Option<Option<String>>,
    /// Sequenceed range of company head count
    #[serde(rename = "company_size", skip_serializing_if = "Option::is_none")]
    pub company_size: Option<Vec<crate::models::LinkedinCompanyCompanySizeInner>>,
    #[serde(
        rename = "company_size_on_linkedin",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_size_on_linkedin: Option<Option<i32>>,
    #[serde(rename = "hq", skip_serializing_if = "Option::is_none")]
    pub hq: Option<Box<crate::models::CompanyLocation>>,
    #[serde(rename = "company_type", skip_serializing_if = "Option::is_none")]
    pub company_type: Option<crate::models::CompanyType>,
    #[serde(
        rename = "founded_year",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub founded_year: Option<Option<i32>>,
    ///                  A list of specialities.             
    #[serde(rename = "specialities", skip_serializing_if = "Option::is_none")]
    pub specialities: Option<Vec<String>>,
    #[serde(rename = "locations", skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<crate::models::CompanyLocation>>,
    #[serde(
        rename = "name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    #[serde(
        rename = "tagline",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tagline: Option<Option<String>>,
    #[serde(
        rename = "universal_name_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub universal_name_id: Option<Option<String>>,
    #[serde(
        rename = "profile_pic_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub profile_pic_url: Option<Option<String>>,
    #[serde(
        rename = "background_cover_image_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub background_cover_image_url: Option<Option<String>>,
    ///          Useable with [Job listing endpoint](#jobs-api-jobs-listing-endpoint)         
    #[serde(
        rename = "search_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub search_id: Option<Option<String>>,
    #[serde(rename = "similar_companies", skip_serializing_if = "Option::is_none")]
    pub similar_companies: Option<Vec<crate::models::SimilarCompany>>,
    #[serde(
        rename = "affiliated_companies",
        skip_serializing_if = "Option::is_none"
    )]
    pub affiliated_companies: Option<Vec<crate::models::AffiliatedCompany>>,
    #[serde(rename = "updates", skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<crate::models::CompanyUpdate>>,
    #[serde(
        rename = "follower_count",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub follower_count: Option<Option<i32>>,
    #[serde(
        rename = "social_networking_services",
        skip_serializing_if = "Option::is_none"
    )]
    pub social_networking_services: Option<Vec<crate::models::CompanySocialNetworkingService>>,
    #[serde(rename = "acquisitions", skip_serializing_if = "Option::is_none")]
    pub acquisitions: Option<Box<crate::models::Acquisition>>,
    #[serde(
        rename = "exit_data",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub exit_data: Option<Option<Vec<crate::models::Exit>>>,
    #[serde(rename = "extra", skip_serializing_if = "Option::is_none")]
    pub extra: Option<Box<crate::models::CompanyDetails>>,
    /// Company Funding data when `funding_data=include`
    #[serde(
        rename = "funding_data",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub funding_data: Option<Option<Vec<crate::models::Funding>>>,
    /// The `categories` attribute is fetched from the                 company's Crunchbase profile. Values for this attribute are                 free-form text, and there is no exhaustive list of categories.                Consider the categories attribute as \"hints\" regarding the                 products or services offered by the company.
    #[serde(
        rename = "categories",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub categories: Option<Option<Vec<String>>>,
}

impl LinkedinCompany {
    pub fn new() -> LinkedinCompany {
        LinkedinCompany {
            linkedin_internal_id: None,
            description: None,
            website: None,
            industry: None,
            company_size: None,
            company_size_on_linkedin: None,
            hq: None,
            company_type: None,
            founded_year: None,
            specialities: None,
            locations: None,
            name: None,
            tagline: None,
            universal_name_id: None,
            profile_pic_url: None,
            background_cover_image_url: None,
            search_id: None,
            similar_companies: None,
            affiliated_companies: None,
            updates: None,
            follower_count: None,
            social_networking_services: None,
            acquisitions: None,
            exit_data: None,
            extra: None,
            funding_data: None,
            categories: None,
        }
    }
}