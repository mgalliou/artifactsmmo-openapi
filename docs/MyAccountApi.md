# \MyAccountApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**change_password_my_change_password_post**](MyAccountApi.md#change_password_my_change_password_post) | **POST** /my/change_password | Change Password
[**get_bank_golds_my_bank_gold_get**](MyAccountApi.md#get_bank_golds_my_bank_gold_get) | **GET** /my/bank/gold | Get Bank Golds
[**get_bank_items_my_bank_items_get**](MyAccountApi.md#get_bank_items_my_bank_items_get) | **GET** /my/bank/items | Get Bank Items



## change_password_my_change_password_post

> models::ResponseSchema change_password_my_change_password_post(change_password)
Change Password

Change your account password. Changing the password reset the account token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_password** | [**ChangePassword**](ChangePassword.md) |  | [required] |

### Return type

[**models::ResponseSchema**](ResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bank_golds_my_bank_gold_get

> models::GoldBankResponseSchema get_bank_golds_my_bank_gold_get()
Get Bank Golds

Fetch golds in your bank.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GoldBankResponseSchema**](GoldBankResponseSchema.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bank_items_my_bank_items_get

> models::DataPageSimpleItemSchema get_bank_items_my_bank_items_get(item_code, page, size)
Get Bank Items

Fetch all items in your bank.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_code** | Option<**String**> | Item to search in your bank. |  |
**page** | Option<**i32**> | Page number |  |[default to 1]
**size** | Option<**i32**> | Page size |  |[default to 50]

### Return type

[**models::DataPageSimpleItemSchema**](DataPage_SimpleItemSchema_.md)

### Authorization

[JWTBearer](../README.md#JWTBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

