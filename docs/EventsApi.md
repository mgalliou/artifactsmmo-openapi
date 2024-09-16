# \EventsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_events_events_get**](EventsApi.md#get_all_events_events_get) | **GET** /events | Get All Events



## get_all_events_events_get

> models::DataPageActiveEventSchema get_all_events_events_get(page, size)
Get All Events

Fetch events details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number |  |[default to 1]
**size** | Option<**i32**> | Page size |  |[default to 50]

### Return type

[**models::DataPageActiveEventSchema**](DataPage_ActiveEventSchema_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

