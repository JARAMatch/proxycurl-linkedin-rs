# \JobsApiApi

All URIs are relative to *https://nubela.co/proxycurl*

Method | HTTP request | Description
------------- | ------------- | -------------
[**job_profile_endpoint**](JobsApiApi.md#job_profile_endpoint) | **GET** /api/linkedin/job | 
[**jobs_listing_count_endpoint**](JobsApiApi.md#jobs_listing_count_endpoint) | **GET** /api/v2/linkedin/company/job/count | 
[**jobs_listing_endpoint**](JobsApiApi.md#jobs_listing_endpoint) | **GET** /api/v2/linkedin/company/job | 



## job_profile_endpoint

> crate::models::JobProfile job_profile_endpoint(url)


Cost: 2 credits / successful request. Get structured data of a LinkedIn Job Profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** |      URL of the LinkedIn Job Profile to target.      URL should be in the format of     `https://www.linkedin.com/jobs/view/<job_id>`.     [Jobs Listing Endpoint](#jobs-api-jobs-listing-endpoint)     can be used to retrieve a job URL.      | [required] |

### Return type

[**crate::models::JobProfile**](JobProfile.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_listing_count_endpoint

> crate::models::JobListCount jobs_listing_count_endpoint(job_type, experience_level, when, flexibility, geo_id, keyword, search_id)


Cost: 2 credits / successful request. Count number of jobs posted by a company on LinkedIn

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_type** | Option<**String**> |      The nature of the job.     It accepts the following 7 case-insensitive values only:     - `full-time`     - `part-time`     - `contract`     - `internship`     - `temporary`     - `volunteer`     - `anything` (default)      |  |
**experience_level** | Option<**String**> |      The experience level needed for the job.     It accepts the following 6 case-insensitive values only:     - `internship`     - `entry_level`     - `associate`     - `mid_senior_level`     - `director`     - `anything` (default)      |  |
**when** | Option<**String**> |      The time when the job is posted,     It accepts the following case-insensitive values only:     - `yesterday`     - `past-week`     - `past-month`     - `anytime` (default)      |  |
**flexibility** | Option<**String**> |      The flexibility of the job.     It accepts the following 3 case insensitive values only:     - `remote`     - `on-site`     - `hybrid`     - `anything` (default)      |  |
**geo_id** | Option<**String**> |      The `geo_id` of the location to search for.     For example, `92000000` is the `geo_id` of world wide.      See [this article](https://nubela.co/blog/how-to-fetch-geo_id-parameter-for-the-job-api/?utm_source=blog&utm_medium=web&utm_campaign=docs-redirect-to-geo_id-article) as to how you may be able to match regions to `geo_id` input values.      |  |
**keyword** | Option<**String**> |      The keyword to search for.      |  |
**search_id** | Option<**String**> |      The `search_id` of the company on LinkedIn.     You can get the `search_id` of a LinkedIn company via     [Company Profile API](#company-api-company-profile-endpoint).      |  |

### Return type

[**crate::models::JobListCount**](JobListCount.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_listing_endpoint

> crate::models::JobListPage jobs_listing_endpoint(job_type, experience_level, when, flexibility, geo_id, keyword, search_id)


Cost: 2 credits / successful request. List jobs posted by a company on LinkedIn

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_type** | Option<**String**> |      The nature of the job.     It accepts the following 7 case-insensitive values only:     - `full-time`     - `part-time`     - `contract`     - `internship`     - `temporary`     - `volunteer`     - `anything` (default)      |  |
**experience_level** | Option<**String**> |      The experience level needed for the job.     It accepts the following 6 case-insensitive values only:     - `internship`     - `entry_level`     - `associate`     - `mid_senior_level`     - `director`     - `anything` (default)      |  |
**when** | Option<**String**> |      The time when the job is posted,     It accepts the following case-insensitive values only:     - `yesterday`     - `past-week`     - `past-month`     - `anytime` (default)      |  |
**flexibility** | Option<**String**> |      The flexibility of the job.     It accepts the following 3 case insensitive values only:     - `remote`     - `on-site`     - `hybrid`     - `anything` (default)      |  |
**geo_id** | Option<**String**> |      The `geo_id` of the location to search for.     For example, `92000000` is the `geo_id` of world wide.      See [this article](https://nubela.co/blog/how-to-fetch-geo_id-parameter-for-the-job-api/?utm_source=blog&utm_medium=web&utm_campaign=docs-redirect-to-geo_id-article) as to how you may be able to match regions to `geo_id` input values.      |  |
**keyword** | Option<**String**> |      The keyword to search for.      |  |
**search_id** | Option<**String**> |      The `search_id` of the company on LinkedIn.     You can get the `search_id` of a LinkedIn company via     [Company Profile API](#company-api-company-profile-endpoint).      |  |

### Return type

[**crate::models::JobListPage**](JobListPage.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

