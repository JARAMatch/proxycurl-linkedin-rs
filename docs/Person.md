# Person

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**public_identifier** | Option<**String**> |                  The vanity identifier of the public LinkedIn profile.                 The vanity identifier comes after the `/in/` part of the LinkedIn Profile URL                 in the following format: `https://www.linkedin.com/in/<public_identifier>`              | [optional]
**profile_pic_url** | Option<**String**> |                  A temporary link to the user's profile picture that is valid for 30 minutes.                  The temporal nature of the link is by design to prevent having Proxycurl be the mirror for the images.                 The developer is expected to handle these images by downloading the image and re-hosting the image.                 See [this post](https://nubela.co/blog/why-is-the-api-returning-s3-links-for-profile-pictures-scraped-from-linkedin-profiles/) for context.                 Some profile pictures might be of the standard LinkedIn's profile picture placeholder. It is so because. See [this post](https://nubela.co/blog/why-do-most-linkedin-profiles-fetched-via-the-person-profile-endpoint-return-a-placeholder-profile-picture/) for context.              | [optional]
**background_cover_image_url** | Option<**String**> |                  A temporary link to the user's background cover picture                 that is valid for 30 minutes.                 The temporal nature of the link is by design to prevent                 having Proxycurl be the mirror for the images.                 The developer is expected to handle these images                  by downloading the image and re-hosting the image.                  See [this post](https://nubela.co/blog/why-is-the-api-returning-s3-links-for-profile-pictures-scraped-from-linkedin-profiles/) for context.              | [optional]
**first_name** | Option<**String**> | First name of the user. | [optional]
**last_name** | Option<**String**> | Last name of the user. | [optional]
**full_name** | Option<**String**> |                  Full name of the user (`first_name` + `last_name`)              | [optional]
**follower_count** | Option<**i32**> | Follower count for this profile | [optional]
**occupation** | Option<**String**> |                  The title and company name of the user's current employment.              | [optional]
**headline** | Option<**String**> |                  The tagline written by the user for his profile.              | [optional]
**summary** | Option<**String**> |                  A blurb (longer than the tagline) written by the user for his profile.              | [optional]
**country** | Option<**String**> |                  The user's country of residence depicted by                 a 2-letter country code (ISO 3166-1 alpha-2).              | [optional]
**country_full_name** | Option<**String**> | The user's country of residence, in English words. | [optional]
**city** | Option<**String**> | The city that the user is living at. | [optional]
**state** | Option<**String**> | The state that the user is living at. | [optional]
**experiences** | Option<[**Vec<crate::models::Experience>**](Experience.md)> | The user's list of historic work experiences. | [optional]
**education** | Option<[**Vec<crate::models::Education>**](Education.md)> | The user's list of education background. | [optional]
**languages** | Option<**Vec<String>**> |                  A list of languages that the user claims to be familiar with,                 and has added to his/her profile.                 Do note that we do not have the proficiency level as                 that data point is not available on a public LinkedIn profile.              | [optional]
**accomplishment_organisations** | Option<[**Vec<crate::models::AccomplishmentOrg>**](AccomplishmentOrg.md)> |                  List of noteworthy organizations that this user is part of.              | [optional]
**accomplishment_publications** | Option<[**Vec<crate::models::Publication>**](Publication.md)> |                  List of noteworthy publications that this user has partook in.              | [optional]
**accomplishment_honors_awards** | Option<[**Vec<crate::models::HonourAward>**](HonourAward.md)> |                  List of noteworthy honours and awards that this user has won.              | [optional]
**accomplishment_patents** | Option<[**Vec<crate::models::Patent>**](Patent.md)> | List of noteworthy patents won by this user. | [optional]
**accomplishment_courses** | Option<[**Vec<crate::models::Course>**](Course.md)> | List of noteworthy courses partook by this user. | [optional]
**accomplishment_projects** | Option<[**Vec<crate::models::Project>**](Project.md)> |                  List of noteworthy projects undertaken by this user.              | [optional]
**accomplishment_test_scores** | Option<[**Vec<crate::models::TestScore>**](TestScore.md)> |                  List of noteworthy test scores accomplished by this user.              | [optional]
**volunteer_work** | Option<[**Vec<crate::models::VolunteeringExperience>**](VolunteeringExperience.md)> | List of historic volunteer work experiences. | [optional]
**certifications** | Option<[**Vec<crate::models::Certification>**](Certification.md)> |                  List of noteworthy certifications accomplished by this user.              | [optional]
**connections** | Option<**i32**> | Total *count* of LinkedIn connections. | [optional]
**people_also_viewed** | Option<[**Vec<crate::models::PeopleAlsoViewed>**](PeopleAlsoViewed.md)> |                  A list of other LinkedIn profiles closely related to this user.              | [optional]
**recommendations** | Option<**Vec<String>**> |                  List of recommendations made by other users about this profile.              | [optional]
**activities** | Option<[**Vec<crate::models::Activity>**](Activity.md)> | A list of LinkedIn status activities. | [optional]
**similarly_named_profiles** | Option<[**Vec<crate::models::SimilarProfile>**](SimilarProfile.md)> |                  A list of other LinkedIn profiles with similar names.              | [optional]
**articles** | Option<[**Vec<crate::models::Article>**](Article.md)> |                  A list of content-based articles posted by this user.              | [optional]
**groups** | Option<[**Vec<crate::models::PersonGroup>**](PersonGroup.md)> |                  A list of LinkedIn groups that this user is a part of.\",              | [optional]
**phone_numbers** | Option<**Vec<String>**> |  | [optional]
**social_networking_services** | Option<[**Vec<crate::models::PersonSocialNetworkingService>**](PersonSocialNetworkingService.md)> |  | [optional]
**skills** | Option<**Vec<String>**> | A list of keyword-based skills that this user boasts of on his LinkedIn profile. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


