# \RevealApiApi

All URIs are relative to *https://nubela.co/proxycurl*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reveal_endpoint**](RevealApiApi.md#reveal_endpoint) | **GET** /api/reveal/company | 



## reveal_endpoint

> crate::models::CompanyReveal reveal_endpoint(ip, role_contact_number, role_personal_email, role)


Cost: 2 credits / successful request. Deanonymize an IPv4 address and associate the Company behind the IPv4 address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip** | **String** |      The target IPv4 address.      | [required] |
**role_contact_number** | Option<**String**> |      Append personal contact numbers to the response if the system finds a relevant person profile.      |  |
**role_personal_email** | Option<**String**> |      Append personal email addresses to the response if the system finds a relevant person profile.      |  |
**role** | Option<**String**> |      Lookup and append an employee of a certain role of the company.     Within the same API call, You can choose to lookup a person with a given role within this organisation that you might want to reach out to.      |  |

### Return type

[**crate::models::CompanyReveal**](CompanyReveal.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

