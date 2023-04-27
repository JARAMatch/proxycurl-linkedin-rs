# EmployeeCount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_employee** | Option<**i32**> |  | [optional]
**linkedin_employee_count** | Option<**i32**> | The scraped value of employee count of this company from it's LinkedIn profile. This value does not respect `employement_status` parameter. It will always return the curent employee count of this company from LinkedIn. | [optional]
**linkdb_employee_count** | Option<**i32**> | The total number of employees found in LinkDB for this company. This value is limited by pre-crawled LinkedIn profiles stored in [LinkDB](https://nubela.co/proxycurl/linkdb) | [optional]
**regression_notice** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


