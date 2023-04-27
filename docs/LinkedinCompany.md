# LinkedinCompany

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**linkedin_internal_id** | Option<**String**> |          LinkedIn's Internal and immutable ID of this Company profile.          | [optional]
**description** | Option<**String**> |  | [optional]
**website** | Option<**String**> |  | [optional]
**industry** | Option<**String**> | The `industry` attribute, found in a LinkedIn Company            profile, describes the industry in which the company operates.            The value of this attribute is an enumerator. [This CSV file            provides an exhaustive list of possible values for this attribute]            (https://drive.google.com/file/d/12yvYLuru7CRv3wKOIkHs5Ldocz31gJSS/            view?usp=share_link). | [optional]
**company_size** | Option<[**Vec<crate::models::LinkedinCompanyCompanySizeInner>**](LinkedinCompany_company_size_inner.md)> | Sequenceed range of company head count | [optional]
**company_size_on_linkedin** | Option<**i32**> |  | [optional]
**hq** | Option<[**crate::models::CompanyLocation**](CompanyLocation.md)> |  | [optional]
**company_type** | Option<[**crate::models::CompanyType**](CompanyType.md)> |  | [optional]
**founded_year** | Option<**i32**> |  | [optional]
**specialities** | Option<**Vec<String>**> |                  A list of specialities.              | [optional]
**locations** | Option<[**Vec<crate::models::CompanyLocation>**](CompanyLocation.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**tagline** | Option<**String**> |  | [optional]
**universal_name_id** | Option<**String**> |  | [optional]
**profile_pic_url** | Option<**String**> |  | [optional]
**background_cover_image_url** | Option<**String**> |  | [optional]
**search_id** | Option<**String**> |          Useable with [Job listing endpoint](#jobs-api-jobs-listing-endpoint)          | [optional]
**similar_companies** | Option<[**Vec<crate::models::SimilarCompany>**](SimilarCompany.md)> |  | [optional]
**affiliated_companies** | Option<[**Vec<crate::models::AffiliatedCompany>**](AffiliatedCompany.md)> |  | [optional]
**updates** | Option<[**Vec<crate::models::CompanyUpdate>**](CompanyUpdate.md)> |  | [optional]
**follower_count** | Option<**i32**> |  | [optional]
**social_networking_services** | Option<[**Vec<crate::models::CompanySocialNetworkingService>**](CompanySocialNetworkingService.md)> |  | [optional]
**acquisitions** | Option<[**crate::models::Acquisition**](Acquisition.md)> |  | [optional]
**exit_data** | Option<[**Vec<crate::models::Exit>**](Exit.md)> |  | [optional]
**extra** | Option<[**crate::models::CompanyDetails**](CompanyDetails.md)> |  | [optional]
**funding_data** | Option<[**Vec<crate::models::Funding>**](Funding.md)> | Company Funding data when `funding_data=include` | [optional]
**categories** | Option<**Vec<String>**> | The `categories` attribute is fetched from the                 company's Crunchbase profile. Values for this attribute are                 free-form text, and there is no exhaustive list of categories.                Consider the categories attribute as \"hints\" regarding the                 products or services offered by the company. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


