use std::fmt::Display;

use crate::error::ProxyCurlError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::types::{
    AccomplishmentOrg, Activity, Article, Certification, Course, Date, Education, Experience,
    HonourAward, InferredSalary, Patent, PeopleAlsoViewed, PersonExtra, PersonGroup, Project,
    Publication, SimilarProfile, TestScore, VolunteerExperience,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FallbackToCache {
    #[serde(rename = "on-error")]
    /// (default value) - Fallback to reading the profile from cache if an error arises.
    OnError,
    #[serde(rename = "never")]
    /// Do not ever read profile from cache.
    Never,
}
impl Default for FallbackToCache {
    fn default() -> Self {
        FallbackToCache::OnError
    }
}
impl Display for FallbackToCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FallbackToCache::OnError => write!(f, "on-error"),
            FallbackToCache::Never => write!(f, "never"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// tweaks the fallback behavior if an error arises from fetching a fresh profile.
pub enum UseCache {
    #[serde(rename = "if-present")]
    /// The default behavior. Fetches profile from cache regardless of age of profile. If profile is not available in cache, API will attempt to source profile externally.
    IfPresent,
    #[serde(rename = "if-recent")]
    /// API will make a best effort to return a fresh profile no older than 29 days.Costs an extra 1 credit on top of the cost of the base endpoint.
    IfRecent,
}
impl Display for UseCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UseCache::IfPresent => write!(f, "if-present"),
            UseCache::IfRecent => write!(f, "if-recent"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// Include the data from external sources.
pub enum IncludeExclude {
    #[serde(rename = "exclude")]
    /// (default value) - Does not provide the data field
    Exclude,
    #[serde(rename = "include")]
    /// Append the data to the person profile object.
    Include,
}
impl Display for IncludeExclude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IncludeExclude::Exclude => write!(f, "exclude"),
            IncludeExclude::Include => write!(f, "include"),
        }
    }
}

#[derive(Clone, Serialize, Default, Debug, Builder, PartialEq)]
#[builder(name = "PersonProfileRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ProxyCurlError"))]
pub struct PersonProfileRequest {
    /// URL of the LinkedIn Profile to crawl.
    ///
    /// URL should be in the format of https://www.linkedin.com/in/<public-identifier>
    ///
    /// Example:`https://www.linkedin.com/in/johnrmarty/`
    pub url: String,

    /// tweaks the fallback behavior if an error arises from fetching a fresh profile.
    pub fallback_to_cache: FallbackToCache,

    pub use_cache: Option<UseCache>,

    /// Include skills data from external sources.
    ///
    /// Costs an extra 1 credit on top of the cost of the base endpoint (if data is available).
    pub skills: Option<IncludeExclude>,

    /// Include inferred salary data from external sources.
    ///
    /// Costs an extra 1 credit on top of the cost of the base endpoint (if data is available).
    pub inferred_salary: Option<IncludeExclude>,

    /// Enriches the Person Profile with personal emails from external sources.
    ///
    /// Costs an extra 1 credit on top of the cost of the base endpoint (if data is available).
    pub personal_email: Option<IncludeExclude>,

    /// Enriches the Person Profile with personal phone numbers from external sources.
    ///
    /// Costs an extra 1 credit on top of the cost of the base endpoint (if data is available).
    pub personal_contact_number: Option<IncludeExclude>,

    /// Enriches the Person Profile with Twitter Id from external sources.
    ///
    /// Costs an extra 1 credit on top of the cost of the base endpoint (if data is available).
    pub twitter_profile_id: Option<IncludeExclude>,

    /// Enriches the Person Profile with Facebook Id from external sources.
    ///
    /// Costs an extra 1 credit on top of the cost of the base endpoint (if data is available).
    pub facebook_profile_id: Option<IncludeExclude>,

    /// Enriches the Person Profile with Github Id from external sources.
    ///
    /// Costs an extra 1 credit on top of the cost of the base endpoint (if data is available).
    pub github_profile_id: Option<IncludeExclude>,

    /// Enriches the Person Profile with extra details from external sources.
    /// Extra details include gender, birth date, industry and interests.
    ///
    /// Costs an extra 1 credit on top of the cost of the base endpoint (if data is available).
    pub extra: Option<IncludeExclude>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct PersonalProfileResponse {
    /// The vanity identifier of the public LinkedIn profile.
    /// The vanity identifier comes after the /in/ part of the LinkedIn Profile URL
    /// in the following format: https://www.linkedin.com/in/<public_identifier>
    ///
    /// Example: `https://www.linkedin.com/in/johnrmarty/` has a public identifier of `johnrmarty`
    pub public_identifier: String,

    /// A temporary link to the user's profile picture that is valid for 30 minutes.
    /// The temporal nature of the link is by design to prevent having Proxycurl be the mirror for the images.
    /// The developer is expected to handle these images by downloading the image and re-hosting the image.
    /// See [this post](https://nubela.co/blog/why-is-the-api-returning-s3-links-for-profile-pictures-scraped-from-linkedin-profiles/) for context.
    /// Some profile pictures might be of the standard LinkedIn's profile picture placeholder. It is so because. See [this post](https://nubela.co/blog/why-do-most-linkedin-profiles-fetched-via-the-person-profile-endpoint-return-a-placeholder-profile-picture/) for context.
    ///
    /// Example: "https://media.licdn.com/dms/image/C5603AQHaJSx0CBAUIA/profile-displayphoto-shrink_800_800/0/1558325759208?e=2147483647\u0026v=beta\u0026t=BluXpPg88xFnU2wMGLjuCUykSk_wKNdh8x3PI9wm6MI"    
    pub profile_pic_url: Option<String>,

    /// A temporary link to the user's background cover picture
    /// that is valid for 30 minutes.
    /// The temporal nature of the link is by design to prevent
    /// having Proxycurl be the mirror for the images.
    /// The developer is expected to handle these images
    /// by downloading the image and re-hosting the image.
    /// See this post for context.
    ///
    /// Example: "https://media.licdn.com/dms/image/C5616AQH9tkBTUhHfng/profile-displaybackgroundimage-shrink_200_800/0/1614530499015?e=2147483647\u0026v=beta\u0026t=VEoCyedtZulnAVYWT9BXfKHi5OFp8avElNjiz8kjSTU"
    pub background_cover_image_url: Option<String>,

    /// The first name of the person.
    ///
    /// Example: "John"
    pub first_name: Option<String>,

    /// The last name of the person.
    ///
    /// Example: "Marty"
    pub last_name: Option<String>,

    /// The full name of the person. (first_name + last_name)
    ///
    /// Example: "John Marty"
    pub full_name: Option<String>,

    /// the follower count of the person.
    ///
    /// Example: 100
    /// Example: null
    pub follower_count: Option<u32>,

    /// The title and company name of the user's current employment.
    ///
    /// Example: "Co-Founder at Freedom Fund Real Estate"
    pub occupation: Option<String>,

    /// the tagline written by the user for their profile.
    ///
    /// Example: "Financial Freedom through Real Estate - LinkedIn Top Voice"
    pub tagline: Option<String>,

    /// A blurb (longer than the tagline) written by the user for their profile.
    ///
    /// Example: "Most people go through life lost, disengaged, and unhappy at work and in their lives - I\u0027m on a mission to solve that.\n\nI spent 10 years as the founder of Axxis Audio, an electronics company that grew to multi-million dollar sales, which I sold in 2012. At that time, I funneled my earnings into the creation of an Internet of Things company, but numerous factors lead to its demise after 2 hard fought years. \n\nAt 31, I was penny-less, had a baby on the way, and had zero job prospects (despite applying to 150 companies). My desperate situation led me to take a job at Best Buy for $12 an hour while reinventing myself through the completion of an MBA at the University of Colorado, and a 6-month software development boot camp. \n\nAfter graduation, I landed at American Express as a Senior Product Manager and then got poached by Amazon in 2017 (because of my LinkedIn profile). My journey has led to a deep sense of perspective, humility, and purpose that I draw on to help others find clarity, meaning, and happiness in their careers and lives. \n\nCheck out my website for details on my Mindset Reset Podcast, Public Speaking, Consulting, or my free 40 page LinkedIn guide\n\nhttp://www.johnraphaelmarty.com/\n\nFAQ\u0027s\n\nQ: Can you speak at my Company, University, event or podcast?\nA: I\u0027d love to! I\u0027ve shared my message on the future of employment, breaking into big tech, and my personal story of reinventing myself and discovering my sense of purpose (and how you can too!).\n\n\u2611\ufe0f YouTube Channel #1 (John Marty) : http://www.youtube.com/c/JohnMarty-uncommon\n\u2611\ufe0f YouTube Channel #2 (Tech Careers for non-engineers: https://www.youtube.com/channel/UC900gMMPLwRGGXSTW1gdZHA\n\nFUN FACTS:\n\u2611\ufe0f I am an Avid cyclist and runner, and I just started learning to skateboard a half-pipe.\n\u2611\ufe0f Into the Enneagram? - I\u0027m a #3 (The Achiever)\n\nLETS CONNECT:\n\u2611\ufe0f Email: JohnRmarty@gmail.com (don\u0027t forget that \"R\"....The other guy gets my emails all the time)""
    pub summary: Option<String>,

    /// The user's country of residence depicted by
    /// a 2-letter country code (ISO 3166-1 alpha-2).
    ///
    /// Example: "US"
    pub country: Option<String>,

    /// The user's country of residence, in English words.
    ///
    /// Example: "United States"
    pub country_full_name: Option<String>,

    /// The city that the user is living at.
    ///
    /// Example: "Denver"
    pub city: Option<String>,

    /// The state that the user is living at.
    ///
    /// Example: "Colorado"
    pub state: Option<String>,

    /// The user's list of historic work experiences.
    ///
    /// See [Experience] for more details.
    pub experiences: Option<Vec<Experience>>,

    /// The user's list of education background
    ///
    /// See [Education] for more details.
    pub education: Option<Vec<Education>>,

    /// A list of languages that the user claims to be familiar with,
    /// and has added to his/her profile.
    /// Do note that we do not have the proficiency level as
    /// that data point is not available on a public LinkedIn profile.
    ///
    /// Example: ["English", "Spanish"]
    pub languages: Option<Vec<String>>,

    /// List of noteworthy organizations that this user is part of.
    ///
    /// See [AccomplishmentOrg] for more details.
    pub accomplishment_organisations: Option<Vec<AccomplishmentOrg>>,

    /// List of noteworthy publications that this user has partook in.
    ///
    /// See [Publication] for more details.
    pub accomplishment_publications: Option<Vec<Publication>>,

    /// List of noteworthy honours and awards that this user has received.
    ///
    /// See [HonourAward] for more details.
    pub accomplishment_honors_awards: Option<Vec<HonourAward>>,

    /// List of noteworthy patents that this user has won.
    ///
    /// See [Patent] for more details.
    pub accomplishment_patents: Option<Vec<Patent>>,

    /// List of noteworthy courses that this user has taken.
    ///
    /// See [Course] for more details.
    pub accomplishment_courses: Option<Vec<Course>>,

    /// List of noteworthy projects that this user has worked on.
    ///
    /// See [Project] for more details.
    pub accomplishment_projects: Option<Vec<Project>>,

    /// List of noteworthy test scores that this user has achieved.
    ///
    /// See [TestScore] for more details.
    pub accomplishment_test_scores: Option<Vec<TestScore>>,

    /// List of historic volunteer work experiences
    ///
    /// See [VolunteerExperience] for more details.
    pub volunteer_work: Option<Vec<VolunteerExperience>>,

    /// List of noteworthy certifications accomplished by the user.
    ///
    /// See [Certification] for more details.
    pub certifications: Option<Vec<Certification>>,

    /// Total count of LinkedIn connections.
    ///
    /// Example: 500
    pub connections: Option<u32>,

    /// A list of other LinkedIn profiles closely related to this user
    ///
    /// See [PeopleAlsoViewed] for more details.
    pub people_also_viewed: Option<Vec<PeopleAlsoViewed>>,

    /// List of recommendations that this user has received.
    ///
    /// Example: ["Professional and dedicated approach towards clients and collegues."]
    pub recommendations: Option<Vec<String>>,

    /// A list of LinkedIn status activities.
    ///
    /// See [Activity] for more details.
    pub activities: Option<Vec<Activity>>,

    /// A list of other LinkedIn profiles with similar names.
    ///
    /// See [SimilarProfile] for more details.
    pub similarly_named_profiles: Option<Vec<SimilarProfile>>,

    /// A list of content-based articles posted by this user.
    ///
    /// See [Article] for more details.
    pub articles: Option<Vec<Article>>,

    /// A list of groups that this user is part of.
    ///
    /// See [PersonGroup] for more details.
    pub groups: Option<Vec<PersonGroup>>,

    /// A salary range inferred from the user's current job title and company.
    ///
    /// See [InferredSalary] for more details.
    pub inferred_salary: Option<InferredSalary>,

    /// Gender of the user.
    ///
    /// Example: "male"
    pub gender: Option<String>,

    /// Birth date of the user.
    ///
    /// See [Date] for more details.
    pub birth_date: Option<Date>,

    /// A bundle of extra data on this user.
    ///
    /// See [PersonExtra] for more details.
    pub extra: Option<PersonExtra>,

    /// A list of interests that the user has.
    ///
    /// Example: ["education", "health", "human rights"]
    pub interests: Option<Vec<String>>,

    /// A list of personal emails associated with this user.
    ///
    /// Example: ["abc@gmail.com", "bcd@gmail.com", "cde@@outlook.com"]
    pub emails: Option<Vec<String>>,

    /// A list of personal mobile phone numbers associated with this user.
    ///
    /// Example: ["+6512345678", "+6285123450953", "+6502300340"]
    pub personal_numbers: Option<Vec<String>>,
}
