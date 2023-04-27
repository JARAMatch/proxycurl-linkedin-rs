use crate::{types::{PersonalProfileResponse, PersonProfileRequest}, error::ProxyCurlError};

pub mod person_profile;

pub struct People<'c> {
    client: &'c crate::Client,
}

impl<'c> People<'c> {
    pub fn new(client: &'c crate::Client) -> Self {
        Self { client }
    }

    /// Get structured data of a Personal Profile
    pub async fn person_profile(&self, request: PersonProfileRequest) -> Result<PersonalProfileResponse, ProxyCurlError> {
        let mut query = vec![
            ("url", request.url),
            ("fallback_to_cache", request.fallback_to_cache.to_string()),
        ];

        if let Some(use_cache) = request.use_cache {
            query.push(("use_cache", use_cache.to_string()));
        }

        if let Some(skills) = request.skills {
            query.push(("skills", skills.to_string()));
        }

        if let Some(inferred_salary) = request.inferred_salary {
            query.push(("inferred_salary", inferred_salary.to_string()));
        }

        if let Some(personal_email) = request.personal_email {
            query.push(("personal_email", personal_email.to_string()));
        }

        if let Some(personal_contact_number) = request.personal_contact_number {
            query.push(("personal_contact_number", personal_contact_number.to_string()));
        }

        if let Some(twitter_profile_id) = request.twitter_profile_id {
            query.push(("twitter_profile_id", twitter_profile_id.to_string()));
        }

        if let Some(facebook_profile_id) = request.facebook_profile_id {
            query.push(("facebook_profile_id", facebook_profile_id.to_string()));
        }

        if let Some(github_profile_id) = request.github_profile_id {
            query.push(("github_profile_id", github_profile_id.to_string()));
        }

        if let Some(extra) = request.extra {
            query.push(("extra", extra.to_string()));
        }
        
        self.client.get(
            "/proxycurl/api/v2/linkedin",
            Some(query.as_slice()),
        ).await
    }
}