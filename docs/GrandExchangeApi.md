# \GrandExchangeApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_ge_items_ge_get**](GrandExchangeApi.md#get_all_ge_items_ge_get) | **GET** /ge/ | Get All Ge Items
[**get_ge_item_ge_code_get**](GrandExchangeApi.md#get_ge_item_ge_code_get) | **GET** /ge/{code} | Get Ge Item



## get_all_ge_items_ge_get

> models::DataPageGeItemSchema get_all_ge_items_ge_get(page, size)
Get All Ge Items

Fetch Grand Exchange items details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number |  |[default to 1]
**size** | Option<**i32**> | Page size |  |[default to 50]

### Return type

[**models::DataPageGeItemSchema**](DataPage_GEItemSchema_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ge_item_ge_code_get

> models::GeItemResponseSchema get_ge_item_ge_code_get(code)
Get Ge Item

Retrieve the details of a Grand Exchange item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The code of the item. | [required] |

### Return type

[**models::GeItemResponseSchema**](GEItemResponseSchema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

