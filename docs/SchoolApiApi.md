# \SchoolApiApi

All URIs are relative to *https://nubela.co/proxycurl*

Method | HTTP request | Description
------------- | ------------- | -------------
[**school_profile_endpoint**](SchoolApiApi.md#school_profile_endpoint) | **GET** /api/linkedin/school | 
[**student_listing_endpoint**](SchoolApiApi.md#student_listing_endpoint) | **GET** /api/linkedin/school/students/ | 



## school_profile_endpoint

> crate::models::LinkedinSchool school_profile_endpoint(url, use_cache)


Cost: 1 credit / successful request. Get structured data of a LinkedIn School Profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** |      URL of the LinkedIn School Profile to crawl.      URL should be in the format of `https://www.linkedin.com/school/<public_identifier>`      | [required] |
**use_cache** | Option<**String**> | `if-present` The default behavior. Fetches profile from cache regardless of age of profile. If profile is not available in cache, API will attempt to source profile externally.  `if-recent` API will make a best effort to return a fresh profile no older than 29 days.Costs an extra `1` credit on top of the cost of the base endpoint. |  |

### Return type

[**crate::models::LinkedinSchool**](LinkedinSchool.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## student_listing_endpoint

> crate::models::StudentList student_listing_endpoint(linkedin_school_url, country, enrich_profiles, search_keyword, page_size, student_status, sort_by, resolve_numeric_id)


Cost: 3 credits / student returned. Get a list of students of a school or university.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linkedin_school_url** | **String** |      URL of the LinkedIn School Profile to target.      URL should be in the format of `https://www.linkedin.com/school/<public_identifier>`      | [required] |
**country** | Option<**String**> |      Limit the result set to the country locality of the profile. For example, set the parameter of `country=us` if you only want profiles from the US.      This parameter accepts a case-insensitive [Alpha-2 ISO3166 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).      Costs an extra `3` credit per result returned.      |  |
**enrich_profiles** | Option<**String**> |      Get the full profile of students instead of only their profile urls.      Each request respond with a streaming response of profiles.      The valid values are:          * `skip` (default): lists student's profile url     * `enrich`: lists full profile of students      *Calling this API endpoint with this parameter would add `1` credit per student returned.*      |  |
**search_keyword** | Option<**String**> |      Filter students by their major by matching the student's major against a *regular expression*.      The default value of this parameter is `null`.      The accepted value for this parameter is a **case-insensitive** regular expression.      (The base cost of calling this API endpoint with this parameter would be `10` credits.     Each student matched and returned would cost `6` credits per student returned.)      |  |
**page_size** | Option<**String**> |      Tune the maximum results returned per API call.      The default value of this parameter is `200000`.      Accepted values for this parameter is an integer ranging from `1` to `200000`.      When `enrich_profiles=enrich`, this parameter accepts value ranging from `1` to `100` and the default value is `100`.      |  |
**student_status** | Option<**String**> |      Parameter to tell the API to return past or current students.      Valid values are `current`, `past`, and `all`:      * `current` (default) : lists current students     * `past` : lists past students     * `all` : lists current & past students      |  |
**sort_by** | Option<**String**> |      Sort students by matriculation or graduation dates.      Valid values are:     * `recently-matriculated` - Sort students by their matriculation date. Students who had had most recently started school is on the top of the list.     * `recently-graduated` - Sort students by their graduation date. The most recently graduated student is on the top of this list.     * `none` - The default value. Do not sort.      If this parameter is supplied with a value other than `none`, will add `50` credits to the base cost of the API endpoint regardless number of results returned. It will also add an additional cost of `10` credits per student returned.      |  |
**resolve_numeric_id** | Option<**String**> |      Enable support for School Profile URLs with numerical IDs that you most frequently fetch from Sales Navigator.      We achieve this by resolving numerical IDs into vanity IDs with cached company profiles from [LinkDB](https://nubela.co/proxycurl/linkdb).      For example, we will turn `https://www.linkedin.com/school/1234567890` to `https://www.linkedin.com/school/acme-corp` -- for which the API endpoint only supports the latter.          This parameter accepts the following values:     - `false` (default value) - Will not resolve numerical IDs.     - `true` - Enable support for School Profile URLs with numerical IDs.      Costs an extra `2` credit on top of the base cost of the endpoint.      |  |

### Return type

[**crate::models::StudentList**](StudentList.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

