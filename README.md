# Rust API client for openapi

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

put your API key in the PROXYCURL_API_KEY environment variable

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

add the following to `Cargo.toml` under `[dependencies]`:

```
proxycurl-linkedin-rs = "0"
```

## Documentation for API Endpoints

All URIs are relative to *https://nubela.co/proxycurl*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*CompanyApiApi* | [**company_lookup_endpoint**](docs/CompanyApiApi.md#company_lookup_endpoint) | **GET** /api/linkedin/company/resolve | 
*CompanyApiApi* | [**company_profile_endpoint**](docs/CompanyApiApi.md#company_profile_endpoint) | **GET** /api/linkedin/company | 
*CompanyApiApi* | [**company_profile_picture_endpoint**](docs/CompanyApiApi.md#company_profile_picture_endpoint) | **GET** /api/linkedin/company/profile-picture | 
*CompanyApiApi* | [**employee_count_endpoint**](docs/CompanyApiApi.md#employee_count_endpoint) | **GET** /api/linkedin/company/employees/count | 
*CompanyApiApi* | [**employee_listing_endpoint**](docs/CompanyApiApi.md#employee_listing_endpoint) | **GET** /api/linkedin/company/employees/ | 
*CompanyApiApi* | [**employee_search_endpoint**](docs/CompanyApiApi.md#employee_search_endpoint) | **GET** /api/linkedin/company/employee/search/ | 
*ContactApiApi* | [**disposable_email_address_check_endpoint**](docs/ContactApiApi.md#disposable_email_address_check_endpoint) | **GET** /api/disposable-email | 
*ContactApiApi* | [**personal_contact_number_lookup_endpoint**](docs/ContactApiApi.md#personal_contact_number_lookup_endpoint) | **GET** /api/contact-api/personal-contact | 
*ContactApiApi* | [**personal_email_lookup_endpoint**](docs/ContactApiApi.md#personal_email_lookup_endpoint) | **GET** /api/contact-api/personal-email | 
*ContactApiApi* | [**reverse_work_email_lookup_endpoint**](docs/ContactApiApi.md#reverse_work_email_lookup_endpoint) | **GET** /api/linkedin/profile/resolve/email | 
*ContactApiApi* | [**work_email_lookup_endpoint**](docs/ContactApiApi.md#work_email_lookup_endpoint) | **GET** /api/linkedin/profile/email | 
*JobsApiApi* | [**job_profile_endpoint**](docs/JobsApiApi.md#job_profile_endpoint) | **GET** /api/linkedin/job | 
*JobsApiApi* | [**jobs_listing_count_endpoint**](docs/JobsApiApi.md#jobs_listing_count_endpoint) | **GET** /api/v2/linkedin/company/job/count | 
*JobsApiApi* | [**jobs_listing_endpoint**](docs/JobsApiApi.md#jobs_listing_endpoint) | **GET** /api/v2/linkedin/company/job | 
*MetaApiApi* | [**view_credit_balance_endpoint**](docs/MetaApiApi.md#view_credit_balance_endpoint) | **GET** /api/credit-balance | 
*PeopleApiApi* | [**person_lookup_endpoint**](docs/PeopleApiApi.md#person_lookup_endpoint) | **GET** /api/linkedin/profile/resolve | 
*PeopleApiApi* | [**person_profile_endpoint**](docs/PeopleApiApi.md#person_profile_endpoint) | **GET** /api/v2/linkedin | 
*PeopleApiApi* | [**person_profile_picture_endpoint**](docs/PeopleApiApi.md#person_profile_picture_endpoint) | **GET** /api/linkedin/person/profile-picture | 
*PeopleApiApi* | [**role_lookup_endpoint**](docs/PeopleApiApi.md#role_lookup_endpoint) | **GET** /api/find/company/role/ | 
*RevealApiApi* | [**reveal_endpoint**](docs/RevealApiApi.md#reveal_endpoint) | **GET** /api/reveal/company | 
*SchoolApiApi* | [**school_profile_endpoint**](docs/SchoolApiApi.md#school_profile_endpoint) | **GET** /api/linkedin/school | 
*SchoolApiApi* | [**student_listing_endpoint**](docs/SchoolApiApi.md#student_listing_endpoint) | **GET** /api/linkedin/school/students/ | 
*SearchApiApi* | [**company_search_endpoint**](docs/SearchApiApi.md#company_search_endpoint) | **GET** /api/search/company | 
*SearchApiApi* | [**person_search_endpoint**](docs/SearchApiApi.md#person_search_endpoint) | **GET** /api/search/person/ | 


## Documentation For Models

 - [AccomplishmentOrg](docs/AccomplishmentOrg.md)
 - [AcquiredCompany](docs/AcquiredCompany.md)
 - [Acquisition](docs/Acquisition.md)
 - [Acquisitor](docs/Acquisitor.md)
 - [Activity](docs/Activity.md)
 - [AffiliatedCompany](docs/AffiliatedCompany.md)
 - [Article](docs/Article.md)
 - [CSearchResult](docs/CSearchResult.md)
 - [Certification](docs/Certification.md)
 - [CompanyDetails](docs/CompanyDetails.md)
 - [CompanyLocation](docs/CompanyLocation.md)
 - [CompanyReveal](docs/CompanyReveal.md)
 - [CompanySearchResult](docs/CompanySearchResult.md)
 - [CompanySocialNetworkingService](docs/CompanySocialNetworkingService.md)
 - [CompanyType](docs/CompanyType.md)
 - [CompanyUpdate](docs/CompanyUpdate.md)
 - [CompanyUrlEnrichResult](docs/CompanyUrlEnrichResult.md)
 - [Course](docs/Course.md)
 - [CreditBalance](docs/CreditBalance.md)
 - [Date](docs/Date.md)
 - [DisposableEmail](docs/DisposableEmail.md)
 - [Education](docs/Education.md)
 - [Employee](docs/Employee.md)
 - [EmployeeCount](docs/EmployeeCount.md)
 - [EmployeeList](docs/EmployeeList.md)
 - [Exit](docs/Exit.md)
 - [Experience](docs/Experience.md)
 - [ExtractionEmailResult](docs/ExtractionEmailResult.md)
 - [Funding](docs/Funding.md)
 - [HonourAward](docs/HonourAward.md)
 - [InferredSalary](docs/InferredSalary.md)
 - [Investor](docs/Investor.md)
 - [JobCompany](docs/JobCompany.md)
 - [JobListCount](docs/JobListCount.md)
 - [JobListEntry](docs/JobListEntry.md)
 - [JobListPage](docs/JobListPage.md)
 - [JobLocation](docs/JobLocation.md)
 - [JobProfile](docs/JobProfile.md)
 - [LinkedinCompany](docs/LinkedinCompany.md)
 - [LinkedinCompanyCompanySizeInner](docs/LinkedinCompanyCompanySizeInner.md)
 - [LinkedinSchool](docs/LinkedinSchool.md)
 - [OrganizationBase](docs/OrganizationBase.md)
 - [Patent](docs/Patent.md)
 - [PdlEmailResult](docs/PdlEmailResult.md)
 - [PdlPhoneNumberResult](docs/PdlPhoneNumberResult.md)
 - [PeopleAlsoViewed](docs/PeopleAlsoViewed.md)
 - [Person](docs/Person.md)
 - [PersonEndpointResponse](docs/PersonEndpointResponse.md)
 - [PersonExtra](docs/PersonExtra.md)
 - [PersonGroup](docs/PersonGroup.md)
 - [PersonLookupUrlEnrichResult](docs/PersonLookupUrlEnrichResult.md)
 - [PersonSearchResult](docs/PersonSearchResult.md)
 - [PersonSocialNetworkingService](docs/PersonSocialNetworkingService.md)
 - [ProfilePicture](docs/ProfilePicture.md)
 - [Project](docs/Project.md)
 - [Publication](docs/Publication.md)
 - [ReverseEmailUrlEnrichResult](docs/ReverseEmailUrlEnrichResult.md)
 - [RoleSearchErichedResult](docs/RoleSearchErichedResult.md)
 - [SearchResult](docs/SearchResult.md)
 - [SimilarCompany](docs/SimilarCompany.md)
 - [SimilarProfile](docs/SimilarProfile.md)
 - [Student](docs/Student.md)
 - [StudentList](docs/StudentList.md)
 - [TestScore](docs/TestScore.md)
 - [VolunteeringExperience](docs/VolunteeringExperience.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

Anthony Rubick
