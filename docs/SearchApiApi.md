# \SearchApiApi

All URIs are relative to *https://nubela.co/proxycurl*

Method | HTTP request | Description
------------- | ------------- | -------------
[**company_search_endpoint**](SearchApiApi.md#company_search_endpoint) | **GET** /api/search/company | 
[**person_search_endpoint**](SearchApiApi.md#person_search_endpoint) | **GET** /api/search/person/ | 



## company_search_endpoint

> crate::models::CompanySearchResult company_search_endpoint(enrich_profiles, page_size, founded_before_year, founded_after_year, description, employee_count_min, employee_count_max, industry, name, follower_count_max, follower_count_min, r#type, city, region, country)


Cost: 35 credits / successful request base charge. Search for companies that meet a set of criteria within     our exhaustive dataset of company profiles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enrich_profiles** | Option<**String**> |      Get the company's complete profile data rather than just the URLs to their LinkedIn profiles.      Each request respond with a streaming response of profiles.      The valid values are:      - skip (default): lists company's profile url     - enrich: include company's profile data in the list      Calling this API endpoint with this parameter would add 1 credit per result returned.      |  |
**page_size** | Option<**String**> |      Tune the maximum results returned per API call.      The default value of this parameter is 100.      Accepted values for this parameter is an integer ranging from 1 to 100.      |  |
**founded_before_year** | Option<**String**> |      Filter companies that were founded **before** this year.      |  |
**founded_after_year** | Option<**String**> |      Filter companies that were founded **after** this year.      |  |
**description** | Option<**String**> |      Filter companies with a description matching the provided regular expression.      The accepted value for this parameter is a **case-sensitive** regular expression.     The default value of this parameter is null.      |  |
**employee_count_min** | Option<**String**> |      Filter companies with **at least** this many employees.      |  |
**employee_count_max** | Option<**String**> |      Filter companies with **at most** this many employees.      |  |
**industry** | Option<**String**> |      Use this parameter to filter companies belonging to an `industry` that matches the provided regular expression. The `industry` attribute, found in a LinkedIn Company profile, describes the industry in which the company operates. The value of this attribute is an enumerator. [This CSV file provides an exhaustive list of possible values for this attribute](https://drive.google.com/file/d/12yvYLuru7CRv3wKOIkHs5Ldocz31gJSS/view?usp=share_link).      The accepted value for this parameter is a **case-sensitive** regular expression. The default value of this parameter is `null`.      |  |
**name** | Option<**String**> |      Filter companies with a name matching the provided regular expression.      The accepted value for this parameter is a **case-sensitive** regular expression.     The default value of this parameter is null.      |  |
**follower_count_max** | Option<**String**> |      Filter companies with a LinkedIn follower count **less than** this value.      |  |
**follower_count_min** | Option<**String**> |      Filter companies with a LinkedIn follower count **more than** this value.      |  |
**r#type** | Option<**String**> |      Filter companies of the provided LinkedIn type.      Possible values:      * `EDUCATIONAL`: Educational Institution     * `GOVERNMENT_AGENCY`: Government Agency     * `NON_PROFIT` : Nonprofit     * `PARTNERSHIP` : Partnership     * `PRIVATELY_HELD` : Privately Held     * `PUBLIC_COMPANY` : Public Company     * `SELF_EMPLOYED` : Self-Employed     * `SELF_OWNED` : Sole Proprietorship      |  |
**city** | Option<**String**> |      Filter companies based in cities matching the provided regular expression.      The accepted value for this parameter is a **case-sensitive** regular expression.     The default value of this parameter is null.      |  |
**region** | Option<**String**> |      Filter companies based in regions matching the provided regular expression.      The accepted value for this parameter is a **case-sensitive** regular expression.     The default value of this parameter is null.      |  |
**country** | Option<**String**> |      Filter companies with an office based in this country.      This parameter accepts a case-insensitive [Alpha-2 ISO3166 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).      |  |

### Return type

[**crate::models::CompanySearchResult**](CompanySearchResult.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## person_search_endpoint

> crate::models::PersonSearchResult person_search_endpoint(enrich_profiles, page_size, summary, headline, city, region, country, languages, linkedin_groups, past_company_name, current_company_name, past_job_description, current_job_description, past_company_linkedin_profile_url, current_company_linkedin_profile_url, current_role_before, current_role_after, past_role_title, current_role_title, education_school_linkedin_profile_url, education_school_name, education_degree_name, education_field_of_study, last_name, first_name)


Cost: 35 credits / successful request base charge. Search for people who meet a set of criteria within our exhaustive dataset of people profiles.  This API endpoint is powered by [LinkDB](https://nubela.co/proxycurl/linkdb), our exhaustive dataset of people and company profiles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enrich_profiles** | Option<**String**> |      Get the person's complete profile data rather than just the URLs to their LinkedIn profiles.      Each request respond with a streaming response of profiles.      The valid values are:      * `skip` (default): lists person's profile url only     * `enrich`: include person's profile data in the list      Calling this API endpoint with this parameter would add `1` credit per result returned.      |  |
**page_size** | Option<**String**> |      Tune the maximum results returned per API call.      The default value of this parameter is `100`.      Accepted values for this parameter is an integer ranging from `1` to `100`.      |  |
**summary** | Option<**String**> |      Filter people whose LinkedIn summary fields match the provided regular expression.      The default value of this parameter is `null`.      The accepted value for this parameter is a **case-sensitive** regular expression.      |  |
**headline** | Option<**String**> |      Filter people whose LinkedIn headline fields match the provided regular expression.      The default value of this parameter is `null`.      The accepted value for this parameter is a **case-sensitive** regular expression.      |  |
**city** | Option<**String**> |      Filter people located in a city matching the provided regular expression.      The default value of this parameter is `null`.      The accepted value for this parameter is a **case-sensitive** regular expression.      |  |
**region** | Option<**String**> |      Filter people located in a region matching the provided regular expression.      The default value of this parameter is `null`.      The accepted value for this parameter is a **case-sensitive** regular expression.      |  |
**country** | Option<**String**> |      Filter people located in this country.     This parameter accepts a case-insensitive [Alpha-2 ISO3166 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2). If you want to search for profiles worldwide then do not fill any values for this parameter.      The default value of this parameter is `null`.      |  |
**languages** | Option<**String**> |      Filter people who list a language matching the provided regular expression.      The accepted value for this parameter is a **case-sensitive** regular expression. The default value of this parameter is `null`.      |  |
**linkedin_groups** | Option<**String**> |      Filter people who are members of LinkedIn groups whose names match the provided regular expression.      The accepted value for this parameter is a **case-sensitive** regular expression. The default value of this parameter is `null`.      |  |
**past_company_name** | Option<**String**> |      Filter people who have **in the past** worked at a company whose name matches the provided regular expression.      The accepted value for this parameter is a **case-sensitive** regular expression. The default value of this parameter is `null`.      |  |
**current_company_name** | Option<**String**> |      Filter people who are **currently** working at a company whose name matches the provided regular expression.      The accepted value for this parameter is a **case-sensitive** regular expression. The default value of this parameter is `null`.      |  |
**past_job_description** | Option<**String**> |      Filter people with **past** job descriptions matching the provided regular expression.      The accepted value for this parameter is a **case-sensitive** regular expression. The default value of this parameter is `null`.      |  |
**current_job_description** | Option<**String**> |      Filter people with **current** job descriptions matching the provided regular expression.      The accepted value for this parameter is a **case-sensitive** regular expression. The default value of this parameter is `null`.      |  |
**past_company_linkedin_profile_url** | Option<**String**> |      Filter people who have **in the past** worked at the company represented by this LinkedIn Company Profile URL.      This parameter takes a LinkedIn Company Profile URL. Default value of this parameter is `null`.      |  |
**current_company_linkedin_profile_url** | Option<**String**> |      Filter people who are **currently** working at the company represented by this LinkedIn Company Profile URL.      Default value of this parameter is `null`.      |  |
**current_role_before** | Option<**String**> |      Filter people who started their current role **before** this date.      This parameter takes a ISO8601 date. Default value of this parameter is `null`.      |  |
**current_role_after** | Option<**String**> |      Filter people who started their current role **after** this date.      This parameter takes a ISO8601 date. Default value of this parameter is `null`.      |  |
**past_role_title** | Option<**String**> |      Filter people who have **in the past** worked as a role whose title matches the provided regular expression.      The accepted value for this parameter is a **case-sensitive** regular expression. The default value of this parameter is `null`.      |  |
**current_role_title** | Option<**String**> |      Filter people who are **currently** working as a role whose title matches the provided regular expression.      The accepted value for this parameter is a **case-sensitive** regular expression. The default value of this parameter is `null`.      |  |
**education_school_linkedin_profile_url** | Option<**String**> |      Filter people who have attended a school with a specific LinkedIn profile URL, based on education history.      The default value of this parameter is `null`.      |  |
**education_school_name** | Option<**String**> |      Filter people who have attended a school whose name matches the provided regular expression, based on education history.      The default value of this parameter is `null`.      The accepted value for this parameter is a **case-sensitive** regular expression.      |  |
**education_degree_name** | Option<**String**> |      Filter people who earned a degree matching the provided regular expression, based on education history.      The default value of this parameter is `null`.      The accepted value for this parameter is a **case-sensitive** regular expression.      |  |
**education_field_of_study** | Option<**String**> |      Filter people with a field of study matching the provided regular expression, based on education history.      The default value of this parameter is `null`.      The accepted value for this parameter is a **case-sensitive** regular expression.      |  |
**last_name** | Option<**String**> |      Filter people whose last names match the provided regular expression.      The default value of this parameter is `null`.      The accepted value for this parameter is a **case-sensitive** regular expression.      |  |
**first_name** | Option<**String**> |      Filter people whose first names match the provided regular expression.      The default value of this parameter is `null`.      The accepted value for this parameter is a **case-sensitive** regular expression.      |  |

### Return type

[**crate::models::PersonSearchResult**](PersonSearchResult.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

