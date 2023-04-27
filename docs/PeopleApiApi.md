# \PeopleApiApi

All URIs are relative to *https://nubela.co/proxycurl*

Method | HTTP request | Description
------------- | ------------- | -------------
[**person_lookup_endpoint**](PeopleApiApi.md#person_lookup_endpoint) | **GET** /api/linkedin/profile/resolve | 
[**person_profile_endpoint**](PeopleApiApi.md#person_profile_endpoint) | **GET** /api/v2/linkedin | 
[**person_profile_picture_endpoint**](PeopleApiApi.md#person_profile_picture_endpoint) | **GET** /api/linkedin/person/profile-picture | 
[**role_lookup_endpoint**](PeopleApiApi.md#role_lookup_endpoint) | **GET** /api/find/company/role/ | 



## person_lookup_endpoint

> crate::models::PersonLookupUrlEnrichResult person_lookup_endpoint(company_domain, first_name, enrich_profile, location, title, last_name)


Cost: 2 credits / successful request. Look up a person with a name and company information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_domain** | **String** | Company name or domain | [required] |
**first_name** | **String** | First name of the user | [required] |
**enrich_profile** | Option<**String**> |      Enrich the result with a cached profile of the lookup result.      The valid values are:      * `skip` (default): do not enrich the results with cached profile data     * `enrich`: enriches the result with cached profile data      Calling this API endpoint with this parameter would add 1 credit.          If you require [fresh profile data](https://nubela.co/blog/how-fresh-are-profiles-returned-by-proxycurl-api/),     please chain this API call with the [Person Profile Endpoint](https://nubela.co/proxycurl/docs#people-api-person-profile-endpoint) with the `use_cache=if-recent` parameter.      |  |
**location** | Option<**String**> |      The location of this user.      Name of country, city or state.      |  |
**title** | Option<**String**> | Title that user is holding at his/her current job |  |
**last_name** | Option<**String**> | Last name of the user |  |

### Return type

[**crate::models::PersonLookupUrlEnrichResult**](PersonLookupUrlEnrichResult.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## person_profile_endpoint

> crate::models::PersonEndpointResponse person_profile_endpoint(url, fallback_to_cache, use_cache, skills, inferred_salary, personal_email, personal_contact_number, twitter_profile_id, facebook_profile_id, github_profile_id, extra)


Cost: 1 credit / successful request. Get structured data of a Personal Profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** |      URL of the LinkedIn Profile to crawl.      URL should be in the format of `https://www.linkedin.com/in/<public-identifier>`      | [required] |
**fallback_to_cache** | **String** |      Tweaks the fallback behavior if an error arises from fetching a fresh profile.      This parameter accepts the following values:     * `on-error` (default value) - Fallback to reading the profile from cache if an error arises.     * `never` - Do not ever read profile from cache.      | [required] |
**use_cache** | Option<**String**> | `if-present` The default behavior. Fetches profile from cache regardless of age of profile. If profile is not available in cache, API will attempt to source profile externally.  `if-recent` API will make a best effort to return a fresh profile no older than 29 days.Costs an extra `1` credit on top of the cost of the base endpoint. |  |
**skills** | Option<**String**> |      Include skills data from external sources.      This parameter accepts the following values:     - `exclude` (default value) - Does not provide skills data field.     - `include` - Append skills data to the person profile object.     Costs an extra `1` credit on top of the cost of the base endpoint (if data is available).      |  |
**inferred_salary** | Option<**String**> |      Include inferred salary range from external sources.      This parameter accepts the following values:     - `exclude` (default value) - Does not provide inferred salary data field.     - `include` - Append inferred salary range data to the person profile object.     Costs an extra `1` credit on top of the cost of the base endpoint (if data is available).      |  |
**personal_email** | Option<**String**> |      Enriches the Person Profile with personal emails from external sources.      This parameter accepts the following values:     - `exclude` (default value) - Does not provide personal emails data field.     - `include` - Append personal emails data to the person profile object.     Costs an extra `1` credit per email returned on top of the cost of the base endpoint (if data is available).      |  |
**personal_contact_number** | Option<**String**> |      Enriches the Person Profile with personal numbers from external sources.      This parameter accepts the following values:     - `exclude` (default value) - Does not provide personal numbers data field.     - `include` - Append personal numbers data to the person profile object.     Costs an extra `1` credit per email returned on top of the cost of the base endpoint (if data is available).      |  |
**twitter_profile_id** | Option<**String**> |      Enriches the Person Profile with Twitter Id from external sources.      This parameter accepts the following values:     - `exclude` (default value) - Does not provide Twitter Id data field.     - `include` - Append Twitter Id data to the person profile object.     Costs an extra `1` credit on top of the cost of the base endpoint (if data is available).      |  |
**facebook_profile_id** | Option<**String**> |      Enriches the Person Profile with Facebook Id from external sources.      This parameter accepts the following values:     - `exclude` (default value) - Does not provide Facebook Id data field.     - `include` - Append Facebook Id data to the person profile object.     Costs an extra `1` credit on top of the cost of the base endpoint (if data is available).      |  |
**github_profile_id** | Option<**String**> |      Enriches the Person Profile with Github Id from external sources.      This parameter accepts the following values:     - `exclude` (default value) - Does not provide Github Id data field.     - `include` - Append Github Id data to the person profile object.     Costs an extra `1` credit on top of the cost of the base endpoint (if data is available).      |  |
**extra** | Option<**String**> |      Enriches the Person Profile with extra details from external sources.     Extra details include gender, birth date, industry and interests.      This parameter accepts the following values:     - `exclude` (default value) - Does not provide extra data field.     - `include` - Append extra data to the person profile object.     Costs an extra `1` credit on top of the cost of the base endpoint (if data is available).      |  |

### Return type

[**crate::models::PersonEndpointResponse**](PersonEndpointResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## person_profile_picture_endpoint

> crate::models::ProfilePicture person_profile_picture_endpoint(linkedin_person_profile_url)


Cost: 0 credit / successful request. Get the profile picture of a person.  Profile pictures are served from cached people profiles found within [LinkDB](https://nubela.co/proxycurl/linkdb). If the profile does not exist within [LinkDB](https://nubela.co/proxycurl/linkdb), then the API will return a `404` status code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linkedin_person_profile_url** | **String** |      LinkedIn Profile URL of the person that you are trying to get the profile picture of.      | [required] |

### Return type

[**crate::models::ProfilePicture**](ProfilePicture.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role_lookup_endpoint

> crate::models::RoleSearchErichedResult role_lookup_endpoint(role, company_name, enrich_profile)


Cost: 3 credits / successful request. Finds the closest (person) profile with a given role in a Company. For example, you can use this endpoint to find the \"CTO\" of \"Apple\". This API endpoint returns only one result that is the closest match.  There is also the [Employee Search Endpoint] (https://nubela.co/proxycurl/docs#company-api-employee-search-api-endpoint)  which is powered by [LinkDB](https://nubela.co/proxycurl/linkdb) if you  require:  * precision on the target company * a list of employees that matches a role (instead of one result).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** | Role of the profile that you are lookin up | [required] |
**company_name** | **String** | Name of the company that you are searching for | [required] |
**enrich_profile** | Option<**String**> |      Enrich the result with a cached profile of the lookup result.      The valid values are:      * `skip` (default): do not enrich the results with cached profile data     * `enrich`: enriches the result with cached profile data      Calling this API endpoint with this parameter would add 1 credit.      If you require [fresh profile data](https://nubela.co/blog/how-fresh-are-profiles-returned-by-proxycurl-api/),     please chain this API call with the [Person Profile Endpoint](https://nubela.co/proxycurl/docs#people-api-person-profile-endpoint) with the `use_cache=if-recent` parameter.      |  |

### Return type

[**crate::models::RoleSearchErichedResult**](RoleSearchErichedResult.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

