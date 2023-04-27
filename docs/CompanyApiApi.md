# \CompanyApiApi

All URIs are relative to *https://nubela.co/proxycurl*

Method | HTTP request | Description
------------- | ------------- | -------------
[**company_lookup_endpoint**](CompanyApiApi.md#company_lookup_endpoint) | **GET** /api/linkedin/company/resolve | 
[**company_profile_endpoint**](CompanyApiApi.md#company_profile_endpoint) | **GET** /api/linkedin/company | 
[**company_profile_picture_endpoint**](CompanyApiApi.md#company_profile_picture_endpoint) | **GET** /api/linkedin/company/profile-picture | 
[**employee_count_endpoint**](CompanyApiApi.md#employee_count_endpoint) | **GET** /api/linkedin/company/employees/count | 
[**employee_listing_endpoint**](CompanyApiApi.md#employee_listing_endpoint) | **GET** /api/linkedin/company/employees/ | 
[**employee_search_endpoint**](CompanyApiApi.md#employee_search_endpoint) | **GET** /api/linkedin/company/employee/search/ | 



## company_lookup_endpoint

> crate::models::CompanyUrlEnrichResult company_lookup_endpoint(company_location, company_domain, company_name, enrich_profile)


Cost: 2 credits / successful request. Resolve Company LinkedIn Profile from company name,     domain name and location.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_location** | Option<**String**> |      The location / region of company.     ISO 3166-1 alpha-2 codes      |  |
**company_domain** | Option<**String**> | Company website or Company domain Requires either `company_domain` or `company_name` |  |
**company_name** | Option<**String**> | Company Name Requires either `company_domain` or `company_name` |  |
**enrich_profile** | Option<**String**> |      Enrich the result with a cached profile of the lookup result.      The valid values are:      * `skip` (default): do not enrich the results with cached profile data     * `enrich`: enriches the result with cached profile data      Calling this API endpoint with this parameter would add 1 credit.      If you require [fresh profile data](https://nubela.co/blog/how-fresh-are-profiles-returned-by-proxycurl-api/),     please chain this API call with the [Company Profile Endpoint](https://nubela.co/proxycurl/docs#company-api-company-profile-endpoint) with the `use_cache=if-recent` parameter.      |  |

### Return type

[**crate::models::CompanyUrlEnrichResult**](CompanyUrlEnrichResult.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_profile_endpoint

> crate::models::LinkedinCompany company_profile_endpoint(url, resolve_numeric_id, categories, funding_data, extra, exit_data, acquisitions, use_cache)


Cost: 1 credit / successful request. Get structured data of a Company Profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** |      URL of the LinkedIn Company Profile to crawl.      URL should be in the format of `https://www.linkedin.com/company/<public_identifier>`      | [required] |
**resolve_numeric_id** | Option<**String**> |      Enable support for Company Profile URLs with numerical IDs that you most frequently fetch from Sales Navigator.     We achieve this by resolving numerical IDs into vanity IDs with cached company profiles from [LinkDB](https://nubela.co/proxycurl/linkdb).     For example, we will turn `https://www.linkedin.com/company/1234567890` to `https://www.linkedin.com/company/acme-corp` -- for which the API endpoint only supports the latter.      This parameter accepts the following values:     - `false` (default value) - Will not resolve numerical IDs.     - `true` - Enable support for Company Profile URLs with numerical IDs.     Costs an extra `2` credit on top of the base cost of the endpoint.      |  |
**categories** | Option<**String**> |      Appends categories data of this company.      Default value is `\"exclude\"`.     The other acceptable value is `\"include\"`, which will include these categories (if available) for `1` extra credit.      |  |
**funding_data** | Option<**String**> |      Returns a list of funding rounds that this company has received.      Default value is `\"exclude\"`.     The other acceptable value is `\"include\"`, which will include these categories (if available) for `1` extra credit.      |  |
**extra** | Option<**String**> |      Enriches the Company Profile with extra details from external sources.     Details include Crunchbase ranking, contact email, phone number, Facebook account, Twitter account, funding rounds and amount, IPO status, investor information, etc.      Default value is `\"exclude\"`.     The other acceptable value is `\"include\"`, which will include these extra details (if available) for `1` extra credit.      |  |
**exit_data** | Option<**String**> |      Returns a list of investment portfolio exits.      Default value is `\"exclude\"`.     The other acceptable value is `\"include\"`, which will include these categories (if available) for `1` extra credit.      |  |
**acquisitions** | Option<**String**> |      Provides further enriched data on acquisitions made by this company from external sources.      Default value is `\"exclude\"`.     The other acceptable value is `\"include\"`, which will include these acquisition data (if available) for `1` extra credit.      |  |
**use_cache** | Option<**String**> | `if-present` The default behavior. Fetches profile from cache regardless of age of profile. If profile is not available in cache, API will attempt to source profile externally.  `if-recent` API will make a best effort to return a fresh profile no older than 29 days.Costs an extra `1` credit on top of the cost of the base endpoint. |  |

### Return type

[**crate::models::LinkedinCompany**](LinkedinCompany.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_profile_picture_endpoint

> crate::models::ProfilePicture company_profile_picture_endpoint(linkedin_company_profile_url)


Cost: 0 credit / successful request. Get the profile picture of a company.  Profile pictures are served from cached company profiles found within [LinkDB](https://nubela.co/proxycurl/linkdb). If the profile does not exist within [LinkDB](https://nubela.co/proxycurl/linkdb), then the API will return a `404` status code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linkedin_company_profile_url** | **String** |      LinkedIn Profile URL of the company that you are trying to get the profile picture of.      | [required] |

### Return type

[**crate::models::ProfilePicture**](ProfilePicture.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## employee_count_endpoint

> crate::models::EmployeeCount employee_count_endpoint(url, use_cache, linkedin_employee_count, employment_status)


Cost: 1 credit / successful request. Get a number of total employees of a Company.  Get an employee count of this company from various sources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** |      URL of the LinkedIn Company Profile to target.      URL should be in the format of `https://www.linkedin.com/company/<public_identifier>`      | [required] |
**use_cache** | Option<**String**> |      `if-present`: The default behavior. Fetches data from LinkDB cache regardless of age of profile.      `if-recent`: API will make a best effort to return a fresh data no older than 29 days. Costs an extra 1 credit on top of the cost of the base endpoint.      |  |
**linkedin_employee_count** | Option<**String**> |      Option to include a scraped employee count value from the target company's LinkedIn profile.      Valid values are `include` and `exclude`:      * `exclude` (default) : To exclude the scraped employee count.     * `include` : To include the scraped employee count.      Costs an extra `1` credit on top of the base cost of the endpoint.      |  |
**employment_status** | Option<**String**> |      Parameter to tell the API to filter past or current employees.      Valid values are `current`, `past`, and `all`:      * `current` (default) : count current employees     * `past` : count past employees     * `all` : count current & past employees      |  |

### Return type

[**crate::models::EmployeeCount**](EmployeeCount.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## employee_listing_endpoint

> crate::models::EmployeeList employee_listing_endpoint(url, country, enrich_profiles, role_search, page_size, employment_status, sort_by, resolve_numeric_id)


Cost: 3 credits / employee returned. Get a list of employees of a Company.  This API endpoint is limited by LinkDB which is populated with profiles in the US, UK, Canada, Israel, Australia, Ireland, New Zealand and Singapore. As such, this endpoint is best used to list employees working in companies based in those locations only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** |      URL of the LinkedIn Company Profile to target.      URL should be in the format of `https://www.linkedin.com/company/<public_identifier>`      | [required] |
**country** | Option<**String**> |      Limit the result set to the country locality of the profile. For example, set the parameter of `country=us` if you only want profiles from the US.      This parameter accepts a case-insensitive [Alpha-2 ISO3166 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).      Costs an extra `3` credit per result returned.      |  |
**enrich_profiles** | Option<**String**> |      Get the full profile of employees instead of only their profile urls.      Each request respond with a streaming response of profiles.      The valid values are:          * `skip` (default): lists employee's profile url     * `enrich`: lists full profile of employees      Calling this API endpoint with this parameter would add `1` credit per employee returned.      |  |
**role_search** | Option<**String**> |      Filter employees by their title by matching the employee's title against a *regular expression*.      The default value of this parameter is `null`.      The accepted value for this parameter is a **case-insensitive** regular expression.      (The base cost of calling this API endpoint with this parameter would be `10` credits.     Each employee matched and returned would cost `6` credits per employee returned.)      |  |
**page_size** | Option<**String**> |      Tune the maximum results returned per API call.      The default value of this parameter is `200000`.      Accepted values for this parameter is an integer ranging from `1` to `200000`.      When `enrich_profiles=enrich`, this parameter accepts value ranging from `1` to `100`.      |  |
**employment_status** | Option<**String**> |      Parameter to tell the API to return past or current employees.      Valid values are `current`, `past`, and `all`:      * `current` (default) : lists current employees     * `past` : lists past employees     * `all` : lists current & past employees      |  |
**sort_by** | Option<**String**> |      Sort employees by recency.      Valid values are:     * `recently-joined` - Sort employees by their join date. The most recent employee is on the top of the list.     * `recently-left` - Sort employees by their departure date. The most recent employee who had just left is on the top of this list.     * `none` - The default value. Do not sort.      If this parameter is supplied with a value other than `none`, will add `50` credits to the base cost of the API endpoint regardless number of results returned. It will also add an additional cost of `10` credits per employee returned.      |  |
**resolve_numeric_id** | Option<**String**> |      Enable support for Company Profile URLs with numerical IDs that you most frequently fetch from Sales Navigator.      We achieve this by resolving numerical IDs into vanity IDs with cached company profiles from [LinkDB](https://nubela.co/proxycurl/linkdb).      For example, we will turn `https://www.linkedin.com/company/1234567890` to `https://www.linkedin.com/company/acme-corp` -- for which the API endpoint only supports the latter.          This parameter accepts the following values:     - `false` (default value) - Will not resolve numerical IDs.     - `true` - Enable support for Company Profile URLs with numerical IDs.      Costs an extra `2` credit on top of the base cost of the endpoint.      |  |

### Return type

[**crate::models::EmployeeList**](EmployeeList.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## employee_search_endpoint

> crate::models::EmployeeList employee_search_endpoint(linkedin_company_profile_url, keyword_regex, page_size, country, enrich_profiles, resolve_numeric_id)


Cost: 10 credits / successful request. Search employees of a target by their job title. This API endpoint is syntactic sugar for the role_search parameter under the [Employee Listing Endpoint](https://nubela.co/proxycurl/docs#company-api-employee-listing-endpoint).  Results are limited by data that we have within [LinkDB](https://nubela.co/proxycurl/linkdb). Use [Role Lookup API Endpoint](https://nubela.co/proxycurl/docs#people-api-role-lookup-endpoint) if you need to query for profiles without LinkDB constraints. The drawbacks of the Role Lookup API Endpoint is that it is less precise and can return at most one result per query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linkedin_company_profile_url** | **String** |      LinkedIn Profile URL of the target company.      | [required] |
**keyword_regex** | **String** |      Job title keyword to search for in regular expression format.      The accepted value for this parameter is a **case-insensitive** regular expression.      | [required] |
**page_size** | Option<**String**> |      Tune the maximum results returned per API call.     The default value of this parameter is `200000`.     Accepted values for this parameter is an integer ranging from `1` to `200000`.     When `enrich_profiles=enrich`, this parameter accepts value ranging from `1` to `100` and the default value is `100`.      |  |
**country** | Option<**String**> |      Limit the result set to the country locality of the profile. For example, set the parameter of `country=us` if you only want profiles from the US.      This parameter accepts a case-insensitive [Alpha-2 ISO3166 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).      Costs an extra `3` credit per result returned.      |  |
**enrich_profiles** | Option<**String**> |      Get the full profile of employees instead of only their profile urls.      Each request respond with a streaming response of profiles.      The valid values are:          * `skip` (default): lists employee's profile url     * `enrich`: lists full profile of employees      Calling this API endpoint with this parameter would add `1` credit per employee returned.      |  |
**resolve_numeric_id** | Option<**String**> |      Enable support for Company Profile URLs with numerical IDs that you most frequently fetch from Sales Navigator.      We achieve this by resolving numerical IDs into vanity IDs with cached company profiles from [LinkDB](https://nubela.co/proxycurl/linkdb).      For example, we will turn `https://www.linkedin.com/company/1234567890` to `https://www.linkedin.com/company/acme-corp` -- for which the API endpoint only supports the latter.          This parameter accepts the following values:     - `false` (default value) - Will not resolve numerical IDs.     - `true` - Enable support for Company Profile URLs with numerical IDs.      Costs an extra `2` credit on top of the base cost of the endpoint.      |  |

### Return type

[**crate::models::EmployeeList**](EmployeeList.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

