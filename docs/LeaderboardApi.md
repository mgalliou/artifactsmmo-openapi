# \LeaderboardApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_leaderboard_leaderboard_get**](LeaderboardApi.md#get_leaderboard_leaderboard_get) | **GET** /leaderboard | Get Leaderboard



## get_leaderboard_leaderboard_get

> models::DataPageCharacterLeaderboardSchema get_leaderboard_leaderboard_get(sort, page, size)
Get Leaderboard

Fetch leaderboard details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Default sort by combat total XP. |  |
**page** | Option<**i32**> | Page number |  |[default to 1]
**size** | Option<**i32**> | Page size |  |[default to 50]

### Return type

[**models::DataPageCharacterLeaderboardSchema**](DataPage_CharacterLeaderboardSchema_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

