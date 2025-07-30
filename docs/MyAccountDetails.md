# MyAccountDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**username** | **String** | Username. | 
**email** | **String** | Email. | 
**member** | **bool** | Member status. | 
**member_expiration** | Option<**String**> |  | [optional]
**status** | [**models::AccountStatus**](AccountStatus.md) | Account status. | 
**badges** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Account badges. | [optional]
**skins** | [**Vec<serde_json::Value>**](serde_json::Value.md) | Skins owned. | 
**gems** | **i32** | Gems. | 
**achievements_points** | **i32** | Achievement points. | 
**banned** | **bool** | Banned. | 
**ban_reason** | Option<**String**> | Ban reason. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


