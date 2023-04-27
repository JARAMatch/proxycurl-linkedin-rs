# \ContactApiApi

All URIs are relative to *https://nubela.co/proxycurl*

Method | HTTP request | Description
------------- | ------------- | -------------
[**disposable_email_address_check_endpoint**](ContactApiApi.md#disposable_email_address_check_endpoint) | **GET** /api/disposable-email | 
[**personal_contact_number_lookup_endpoint**](ContactApiApi.md#personal_contact_number_lookup_endpoint) | **GET** /api/contact-api/personal-contact | 
[**personal_email_lookup_endpoint**](ContactApiApi.md#personal_email_lookup_endpoint) | **GET** /api/contact-api/personal-email | 
[**reverse_work_email_lookup_endpoint**](ContactApiApi.md#reverse_work_email_lookup_endpoint) | **GET** /api/linkedin/profile/resolve/email | 
[**work_email_lookup_endpoint**](ContactApiApi.md#work_email_lookup_endpoint) | **GET** /api/linkedin/profile/email | 



## disposable_email_address_check_endpoint

> crate::models::DisposableEmail disposable_email_address_check_endpoint(email)


Cost: 0 credit / request. Given an email address, checks if the email address belongs to a disposable email service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email address to check | [required] |

### Return type

[**crate::models::DisposableEmail**](DisposableEmail.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## personal_contact_number_lookup_endpoint

> crate::models::PdlPhoneNumberResult personal_contact_number_lookup_endpoint(linkedin_profile_url)


Cost: 1 credit / contact number returned. Given an LinkedIn profile, returns a list of personal contact numbers belonging to this identity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linkedin_profile_url** | **String** |      LinkedIn Profile URL of the person you want to extract personal contact numbers from.      | [required] |

### Return type

[**crate::models::PdlPhoneNumberResult**](PDLPhoneNumberResult.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## personal_email_lookup_endpoint

> crate::models::PdlEmailResult personal_email_lookup_endpoint(linkedin_profile_url, email_validation)


Cost: 1 credit / email returned. Given an LinkedIn profile, returns a list of personal emails belonging to this identity. Emails are verified to be deliverable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linkedin_profile_url** | **String** |      LinkedIn Profile URL of the person you want to extract personal email addresses from.      | [required] |
**email_validation** | Option<**String**> |      Perform deliverability validation on each email. (Costs 1 extra credit per email found).          Takes the following values:      * include - Perform email validation.      * exclude (default) - Do not perform email validation.      |  |

### Return type

[**crate::models::PdlEmailResult**](PDLEmailResult.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reverse_work_email_lookup_endpoint

> crate::models::ReverseEmailUrlEnrichResult reverse_work_email_lookup_endpoint(work_email, enrich_profile)


Cost: 3 credits / successful request. Resolve LinkedIn Profile from a work email address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**work_email** | **String** | Work email address of the user | [required] |
**enrich_profile** | Option<**String**> |      Enrich the result with a cached profile of the lookup result.      The valid values are:      * `skip` (default): do not enrich the results with cached profile data     * `enrich`: enriches the result with cached profile data      Calling this API endpoint with this parameter would add 1 credit.      If you require [fresh profile data](https://nubela.co/blog/how-fresh-are-profiles-returned-by-proxycurl-api/),     please chain this API call with the [Person Profile Endpoint](https://nubela.co/proxycurl/docs#people-api-person-profile-endpoint) with the `use_cache=if-recent` parameter.      |  |

### Return type

[**crate::models::ReverseEmailUrlEnrichResult**](ReverseEmailUrlEnrichResult.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## work_email_lookup_endpoint

> crate::models::ExtractionEmailResult work_email_lookup_endpoint(linkedin_profile_url, callback_url)


Cost: 3 credits / request. Lookup work email address of a LinkedIn Person Profile.  Email addresses returned are verified to not be role-based or catch-all emails. Email addresses returned by our API endpoint come with a 95+% deliverability guarantee  **Endpoint behavior**  *This endpoint* **_may not_** *return results immediately.*  If you provided a webhook in your request parameter, our application will call your webhook with the result once. See `Webhook request` below.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linkedin_profile_url** | **String** |      Linkedin Profile URL of the person you want to     extract work email address from.      | [required] |
**callback_url** | Option<**String**> |      Webhook to notify your application when     the request has finished processing.      |  |

### Return type

[**crate::models::ExtractionEmailResult**](ExtractionEmailResult.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

