use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Date {
    /// The day of the month.
    ///
    /// Example: 1
    pub day: u8,

    /// The month of the year.
    ///
    /// Example: 1
    pub month: u8,

    /// The year.
    ///
    /// Example: 2013
    pub year: u16,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Experience {
    /// The start date of this experience.
    ///
    /// See [Date] for more details.
    pub starts_at: Date,

    /// The end date of this experience.
    ///
    /// See [Date] for more details.
    pub ends_at: Option<Date>,

    /// The company's display name.
    ///
    /// Example: "Freedom Fund Real Estate"
    pub company: String,

    /// The company's profile URL.
    /// If present, could be used with
    /// Company Profile Endpoint for more info.
    ///
    /// Example: "https://www.linkedin.com/company/freedomfund"
    pub company_linkedin_profile_url: Option<String>,

    /// Example: "Co-Founder"
    pub title: String,

    /// The location of this experience.
    ///
    /// Example: "San Francisco Bay Area"
    pub location: Option<String>,

    /// The description of this experience.
    ///
    /// Example: "Our mission is to provide everyday people seeking financial freedom long before the age of 65 with the ability to invest in high yield, short-term real estate investments that were only accessible in the past for a select few wealthy individuals. Each of our single family rehab projects require a minimum investment contribution of only $10K, we have simple terms, no multi-year hold periods, and no fees. With our unique model investors can log into our easy to use website, select the projects that they want to invest in, and get realtime updates on the status of their investments.\n\nWebsite: https://www.freedomfundinvestments.com/home"
    pub description: Option<String>,

    /// URL of the organization
    ///
    /// Example: "https://media.licdn.com/dms/image/C560BAQEYxazZM_hXgQ/company-logo_100_100/0/1634934418976?e=2147483647\u0026v=beta\u0026t=wI0YdMmxIctkzvnKxRfuAbT8h5eok_DlUqEph68J37s"
    pub logo_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Education {
    /// The start date of this education.
    ///
    /// See [Date] for more details.
    ///
    /// Example: {"day": 1, "month": 1, "year": 2013}
    pub starts_at: Date,

    /// The end date of this education.
    ///
    /// See [Date] for more details.
    pub ends_at: Option<Date>,

    /// Example: "Economics"
    pub field_of_study: Option<String>,

    /// Example: "Bachelor of Arts (B.A.)"
    pub degree_name: Option<String>,

    /// The school's display name.
    ///
    /// Example: "University of California, Berkeley"
    pub school: String,

    /// The school's profile URL.
    /// If present, could be used with
    /// School Profile Endpoint for more info.
    ///
    /// Example: "https://www.linkedin.com/school/university-of-california-berkeley/"
    pub school_linkedin_profile_url: Option<String>,

    /// The description of this education.
    ///
    /// Example: "Graduated with High Honors"
    pub description: Option<String>,

    /// URL of the organization
    ///
    /// Example: "https://media.licdn.com/dms/image/C560BAQGVi9eAHgWxFw/company-logo_100_100/0/1673448029676?e=2147483647\u0026v=beta\u0026t=NG6ttckXvnS2DX3abTfVACRY2E9Q1EcryNaJLRbE9OE"
    pub logo_url: Option<String>,

    /// The grade of this education.
    ///
    /// Example: "3.7"
    pub grade: Option<String>,

    /// The activities and societies of this education.
    ///
    /// Example: "Phi Beta Kappa"
    pub activities_and_societies: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct AccomplishmentOrg {
    /// The start date of this Organization.
    ///
    /// See [Date] for more details.
    ///
    /// Example: {"day": 1, "month": 1, "year": 2013}
    pub starts_at: Date,

    /// The end date of this Organization.
    ///
    /// See [Date] for more details.
    pub ends_at: Option<Date>,

    /// The organization's display name.
    ///
    /// Example: "Microsoft"
    pub org_name: String,

    /// Title of the user in this organization.
    ///
    /// Example: "Software Engineer"
    pub title: Option<String>,

    /// The description of this Organization expeerience.
    ///
    /// Example: "Worked on the Windows team"
    /// Example: null
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Publication {
    /// The title of this publication.
    ///
    /// Example: "The Future of AI"
    pub title: String,

    /// The publisher of this publication.
    ///
    /// Example: "MIT Press"
    pub publisher: Option<String>,

    /// The date of this publication.
    ///
    /// See [Date] for more details.
    ///
    /// Example: {"day": 1, "month": 1, "year": 2013}
    pub published_on: Option<Date>,

    /// The description of this publication.
    ///
    /// Example: "A book about the future of AI"
    pub description: Option<String>,

    /// The URL of this publication.
    ///
    /// Example: "https://www.amazon.com/Future-AI-Introduction-Humanitys-Intelligence/dp/0262043062"
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct HonourAward {
    /// The title of this honour/award.
    ///
    /// Example: "Employee of the Month"
    pub title: String,

    /// The issuer of this honour/award.
    ///
    /// Example: "Microsoft"
    pub issuer: Option<String>,

    /// The date of this honour/award.
    ///
    /// See [Date] for more details.
    ///
    /// Example: {"day": 1, "month": 1, "year": 2013}
    pub issued_on: Option<Date>,

    /// The description of this honour/award.
    ///
    /// Example: "Awarded for outstanding performance"
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Patent {
    /// The title of this patent.
    ///
    /// Example: "System and method for managing resource access"
    pub title: String,

    /// The issuer of this patent.
    ///
    /// Example: "Microsoft"
    pub issuer: Option<String>,

    /// The date of this patent.
    ///
    /// See [Date] for more details.
    ///
    /// Example: {"day": 1, "month": 1, "year": 2013}
    pub issued_on: Option<Date>,

    /// The description of this patent.
    ///
    /// Example: "A system and method for managing access to resources"
    pub description: Option<String>,

    /// The URL of this patent.
    ///
    /// Example: "https://patents.google.com/patent/US20120036018A1/en"
    pub url: Option<String>,

    /// The number of this patent.
    ///
    /// Example: "US20120036018A1"
    pub patent_number: Option<String>,

    /// Application number of this patent.
    ///
    /// Example: "US20120036018A1"
    pub application_number: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Course {
    /// The name of this course.
    ///
    /// Example: "Machine Learning"
    pub name: String,

    /// The number of this course.
    ///
    /// Example: "CS229"
    pub number: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Project {
    /// The title of this project.
    ///
    /// Example: "AI for Good"
    pub title: String,

    /// The description of this project.
    ///
    /// Example: "A project to use AI to help the world"
    pub description: Option<String>,

    /// The URL of this project.
    ///
    /// Example: "https://www.ai4good.org/"
    pub url: Option<String>,

    /// The start date of this project.
    ///
    /// See [Date] for more details.
    ///
    /// Example: {"day": 1, "month": 1, "year": 2013}
    pub starts_at: Option<Date>,

    /// The end date of this project.
    ///
    /// See [Date] for more details.
    ///
    /// Example: {"day": 1, "month": 1, "year": 2013}
    pub ends_at: Option<Date>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct TestScore {
    /// The name of this test.
    ///
    /// Example: "SAT"
    pub name: String,

    /// The score of this test.
    ///
    /// Example: "1500"
    pub score: Option<String>,

    /// The description of this test.
    ///
    /// Example: "Scored in the 99th percentile"
    pub description: Option<String>,

    /// The date of this test.
    ///
    /// See [Date] for more details.
    ///
    /// Example: {"day": 1, "month": 1, "year": 2013}
    pub date_on: Option<Date>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct VolunteerExperience {
    /// The start date of this volunteer experience.
    ///
    /// See [Date] for more details.
    ///
    /// Example: {"day": 1, "month": 1, "year": 2013}
    pub starts_at: Option<Date>,

    /// The end date of this volunteer experience.
    ///
    /// See [Date] for more details.
    ///
    /// Example: {"day": 1, "month": 1, "year": 2013}
    pub ends_at: Option<Date>,

    /// The title of this volunteer experience.
    ///
    /// Example: "Tutor"
    pub title: String,

    /// The cause of this volunteer experience.
    ///
    /// Example: "Education"
    pub cause: Option<String>,

    /// The organization of this volunteer experience.
    ///
    /// Example: "Microsoft"
    pub company: Option<String>,

    /// The Company's profile URL.
    /// If present, could be used with
    /// Company Profile Endpoint for more info.
    ///
    /// Example: "https://www.linkedin.com/company/microsoft"
    pub company_profile_url: Option<String>,

    /// The description of this volunteer experience.
    ///
    /// Example: "Tutored students in math"
    pub description: Option<String>,

    /// The URL of the logo of the organization.
    ///
    /// Example: null
    pub logo_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Certification {
    /// The start date of this volunteer experience.
    ///
    /// See [Date] for more details.
    ///
    /// Example: {"day": 1, "month": 1, "year": 2013}
    pub starts_at: Option<Date>,

    /// The end date of this volunteer experience.
    ///
    /// See [Date] for more details.
    ///
    /// Example: {"day": 1, "month": 1, "year": 2013}
    pub ends_at: Option<Date>,

    /// Name of the course or program.
    ///
    /// Example: "Microsoft Certified Professional"
    pub name: String,

    pub license_number: Option<String>,

    pub display_source: Option<String>,

    /// The issuing organization of this certification.
    ///
    /// Example: "Microsoft"
    pub authority: Option<String>,

    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct PeopleAlsoViewed {
    /// URL of the profile.
    /// Useable with Person profile endpoint
    ///
    /// Example: "https://www.linkedin.com/in/jeffweiner08"
    pub link: String,
    /// The name of the person.
    ///
    /// Example: "Jeff Weiner"
    pub name: String,
    /// The headline of the person.
    ///
    /// Example: "CEO at LinkedIn"
    pub summary: Option<String>,
    /// The location of the person.
    ///
    /// Example: "San Francisco Bay Area"
    pub location: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Activity {
    pub title: String,
    pub link: String,
    pub activity_status: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct SimilarProfile {
    /// URL of the profile.
    /// Useable with Person profile endpoint
    ///
    /// Example: "https://www.linkedin.com/in/jeffweiner08"
    pub link: String,
    /// The name of the person.
    ///
    /// Example: "Jeff Weiner"
    pub name: String,
    /// The headline of the person.
    ///
    /// Example: "CEO at LinkedIn"
    pub summary: Option<String>,
    /// The location of the person.
    ///
    /// Example: "San Francisco Bay Area"
    pub location: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Article {
    /// The title of the article.
    ///
    /// Example: "The Most Important Thing I Learned at Stanford"
    pub title: String,
    /// The URL of the article.
    ///
    /// Example: "https://www.linkedin.com/pulse/most-important-thing-i-learned-stanford-jeff-weiner"
    pub link: Option<String>,
    /// The date of the article.
    ///
    /// See [Date] for more details.
    ///
    /// Example: {"day": 1, "month": 1, "year": 2013}
    pub published_date: Option<Date>,
    /// The Author of the article.
    ///
    /// Example: "Jeff Weiner"
    pub author: Option<String>,
    /// Example: "https://media-exp1.licdn.com/dms/image/C4E12AQFftuPi0UiqWA/article-cover_image-shrink_720_1280/0/1574801149114?e=1640822400\u0026v=beta\u0026t=ZAe3ERmQCM8QHGmRPS2LJ-C76GD5PR7FBHMVL4Z6iVg"
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct PersonGroup {
    /// The name of the group.
    ///
    /// Example: "Stanford University"
    pub name: String,
    /// The URL of the group.
    ///
    /// Example: "https://www.linkedin.com/school/stanford-university/"
    pub link: Option<String>,
    /// The URL to the profile picture of this LinkedIn Group
    ///
    /// Example: "https://media-exp1.licdn.com/dms/image/C4E0BAQG6Qk1aH4s4XQ/company-logo_100_100/0/1519851238681?e=1640822400\u0026v=beta\u0026t=9XUjZ6U7nRnJw6Q8Z6h5d5VzR0X0N5X5W5q3q1i0x5c"
    pub profile_pic_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct PersonExtra {
    /// This profiles GitHub account.
    ///
    /// Example: "github-username"
    pub github_profile_id: Option<String>,
    /// This profiles Twitter account.
    ///
    /// Example: "twitter-username"
    pub twitter_profile_id: Option<String>,
    /// This profiles Facebook account.
    ///
    /// Example: "facebook-username"
    pub facebook_profile_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct InferredSalary {
    pub min: Option<u32>,
    pub max: Option<u32>,
}
