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
pub struct PersonEndpointResponse {
    ///                  The vanity identifier of the public LinkedIn profile.                 The vanity identifier comes after the `/in/` part of the LinkedIn Profile URL                 in the following format: `https://www.linkedin.com/in/<public_identifier>`             
    #[serde(
        rename = "public_identifier",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub public_identifier: Option<Option<String>>,
    ///                  A temporary link to the user's profile picture that is valid for 30 minutes.                  The temporal nature of the link is by design to prevent having Proxycurl be the mirror for the images.                 The developer is expected to handle these images by downloading the image and re-hosting the image.                 See [this post](https://nubela.co/blog/why-is-the-api-returning-s3-links-for-profile-pictures-scraped-from-linkedin-profiles/) for context.                 Some profile pictures might be of the standard LinkedIn's profile picture placeholder. It is so because. See [this post](https://nubela.co/blog/why-do-most-linkedin-profiles-fetched-via-the-person-profile-endpoint-return-a-placeholder-profile-picture/) for context.             
    #[serde(
        rename = "profile_pic_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub profile_pic_url: Option<Option<String>>,
    ///                  A temporary link to the user's background cover picture                 that is valid for 30 minutes.                 The temporal nature of the link is by design to prevent                 having Proxycurl be the mirror for the images.                 The developer is expected to handle these images                  by downloading the image and re-hosting the image.                  See [this post](https://nubela.co/blog/why-is-the-api-returning-s3-links-for-profile-pictures-scraped-from-linkedin-profiles/) for context.             
    #[serde(
        rename = "background_cover_image_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub background_cover_image_url: Option<Option<String>>,
    /// First name of the user.
    #[serde(
        rename = "first_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub first_name: Option<Option<String>>,
    /// Last name of the user.
    #[serde(
        rename = "last_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_name: Option<Option<String>>,
    ///                  Full name of the user (`first_name` + `last_name`)             
    #[serde(
        rename = "full_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub full_name: Option<Option<String>>,
    /// Follower count for this profile
    #[serde(rename = "follower_count", skip_serializing_if = "Option::is_none")]
    pub follower_count: Option<i32>,
    ///                  The title and company name of the user's current employment.             
    #[serde(
        rename = "occupation",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub occupation: Option<Option<String>>,
    ///                  The tagline written by the user for his profile.             
    #[serde(
        rename = "headline",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub headline: Option<Option<String>>,
    ///                  A blurb (longer than the tagline) written by the user for his profile.             
    #[serde(
        rename = "summary",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub summary: Option<Option<String>>,
    ///                  The user's country of residence depicted by                 a 2-letter country code (ISO 3166-1 alpha-2).             
    #[serde(
        rename = "country",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub country: Option<Option<String>>,
    /// The user's country of residence, in English words.
    #[serde(
        rename = "country_full_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub country_full_name: Option<Option<String>>,
    /// The city that the user is living at.
    #[serde(
        rename = "city",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub city: Option<Option<String>>,
    /// The state that the user is living at.
    #[serde(
        rename = "state",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub state: Option<Option<String>>,
    /// The user's list of historic work experiences.
    #[serde(rename = "experiences", skip_serializing_if = "Option::is_none")]
    pub experiences: Option<Vec<crate::models::Experience>>,
    /// The user's list of education background.
    #[serde(rename = "education", skip_serializing_if = "Option::is_none")]
    pub education: Option<Vec<crate::models::Education>>,
    ///                  A list of languages that the user claims to be familiar with,                 and has added to his/her profile.                 Do note that we do not have the proficiency level as                 that data point is not available on a public LinkedIn profile.             
    #[serde(rename = "languages", skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    ///                  List of noteworthy organizations that this user is part of.             
    #[serde(
        rename = "accomplishment_organisations",
        skip_serializing_if = "Option::is_none"
    )]
    pub accomplishment_organisations: Option<Vec<crate::models::AccomplishmentOrg>>,
    ///                  List of noteworthy publications that this user has partook in.             
    #[serde(
        rename = "accomplishment_publications",
        skip_serializing_if = "Option::is_none"
    )]
    pub accomplishment_publications: Option<Vec<crate::models::Publication>>,
    ///                  List of noteworthy honours and awards that this user has won.             
    #[serde(
        rename = "accomplishment_honors_awards",
        skip_serializing_if = "Option::is_none"
    )]
    pub accomplishment_honors_awards: Option<Vec<crate::models::HonourAward>>,
    /// List of noteworthy patents won by this user.
    #[serde(
        rename = "accomplishment_patents",
        skip_serializing_if = "Option::is_none"
    )]
    pub accomplishment_patents: Option<Vec<crate::models::Patent>>,
    /// List of noteworthy courses partook by this user.
    #[serde(
        rename = "accomplishment_courses",
        skip_serializing_if = "Option::is_none"
    )]
    pub accomplishment_courses: Option<Vec<crate::models::Course>>,
    ///                  List of noteworthy projects undertaken by this user.             
    #[serde(
        rename = "accomplishment_projects",
        skip_serializing_if = "Option::is_none"
    )]
    pub accomplishment_projects: Option<Vec<crate::models::Project>>,
    ///                  List of noteworthy test scores accomplished by this user.             
    #[serde(
        rename = "accomplishment_test_scores",
        skip_serializing_if = "Option::is_none"
    )]
    pub accomplishment_test_scores: Option<Vec<crate::models::TestScore>>,
    /// List of historic volunteer work experiences.
    #[serde(rename = "volunteer_work", skip_serializing_if = "Option::is_none")]
    pub volunteer_work: Option<Vec<crate::models::VolunteeringExperience>>,
    ///                  List of noteworthy certifications accomplished by this user.             
    #[serde(rename = "certifications", skip_serializing_if = "Option::is_none")]
    pub certifications: Option<Vec<crate::models::Certification>>,
    /// Total *count* of LinkedIn connections.
    #[serde(
        rename = "connections",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub connections: Option<Option<i32>>,
    ///                  A list of other LinkedIn profiles closely related to this user.             
    #[serde(rename = "people_also_viewed", skip_serializing_if = "Option::is_none")]
    pub people_also_viewed: Option<Vec<crate::models::PeopleAlsoViewed>>,
    ///                  List of recommendations made by other users about this profile.             
    #[serde(rename = "recommendations", skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<String>>,
    /// A list of LinkedIn status activities.
    #[serde(rename = "activities", skip_serializing_if = "Option::is_none")]
    pub activities: Option<Vec<crate::models::Activity>>,
    ///                  A list of other LinkedIn profiles with similar names.             
    #[serde(
        rename = "similarly_named_profiles",
        skip_serializing_if = "Option::is_none"
    )]
    pub similarly_named_profiles: Option<Vec<crate::models::SimilarProfile>>,
    ///                  A list of content-based articles posted by this user.             
    #[serde(rename = "articles", skip_serializing_if = "Option::is_none")]
    pub articles: Option<Vec<crate::models::Article>>,
    ///                  A list of LinkedIn groups that this user is a part of.\",             
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<crate::models::PersonGroup>>,
    #[serde(rename = "phone_numbers", skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<String>>,
    #[serde(
        rename = "social_networking_services",
        skip_serializing_if = "Option::is_none"
    )]
    pub social_networking_services: Option<Vec<crate::models::PersonSocialNetworkingService>>,
    /// A list of keyword-based skills that this user boasts of on his LinkedIn profile.
    #[serde(rename = "skills", skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<String>>,
    #[serde(rename = "inferred_salary", skip_serializing_if = "Option::is_none")]
    pub inferred_salary: Option<Box<crate::models::InferredSalary>>,
    /// Gender of the user.
    #[serde(
        rename = "gender",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub gender: Option<Option<String>>,
    #[serde(rename = "birth_date", skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<Box<crate::models::Date>>,
    /// Industry that the user works in.
    #[serde(
        rename = "industry",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub industry: Option<Option<String>>,
    #[serde(rename = "extra", skip_serializing_if = "Option::is_none")]
    pub extra: Option<Box<crate::models::PersonExtra>>,
    /// A list of interests that the user has.
    #[serde(rename = "interests", skip_serializing_if = "Option::is_none")]
    pub interests: Option<Vec<String>>,
    /// A list of personal emails associated with this user.
    #[serde(rename = "personal_emails", skip_serializing_if = "Option::is_none")]
    pub personal_emails: Option<Vec<String>>,
    /// A list of personal mobile phone numbers associated with this user.
    #[serde(rename = "personal_numbers", skip_serializing_if = "Option::is_none")]
    pub personal_numbers: Option<Vec<String>>,
}

impl PersonEndpointResponse {
    pub fn new() -> PersonEndpointResponse {
        PersonEndpointResponse {
            public_identifier: None,
            profile_pic_url: None,
            background_cover_image_url: None,
            first_name: None,
            last_name: None,
            full_name: None,
            follower_count: None,
            occupation: None,
            headline: None,
            summary: None,
            country: None,
            country_full_name: None,
            city: None,
            state: None,
            experiences: None,
            education: None,
            languages: None,
            accomplishment_organisations: None,
            accomplishment_publications: None,
            accomplishment_honors_awards: None,
            accomplishment_patents: None,
            accomplishment_courses: None,
            accomplishment_projects: None,
            accomplishment_test_scores: None,
            volunteer_work: None,
            certifications: None,
            connections: None,
            people_also_viewed: None,
            recommendations: None,
            activities: None,
            similarly_named_profiles: None,
            articles: None,
            groups: None,
            phone_numbers: None,
            social_networking_services: None,
            skills: None,
            inferred_salary: None,
            gender: None,
            birth_date: None,
            industry: None,
            extra: None,
            interests: None,
            personal_emails: None,
            personal_numbers: None,
        }
    }
}
