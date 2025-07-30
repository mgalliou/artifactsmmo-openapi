# AccountDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**username** | **String** | Username. | 
**member** | **bool** | Member status. | 
**status** | [**models::AccountStatus**](AccountStatus.md) | Account status. | 
**badges** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Account badges. | [optional]
**skins** | [**Vec<serde_json::Value>**](serde_json::Value.md) | Skins owned. | 
**achievements_points** | **i32** | Achievement points. | 
**banned** | **bool** | Banned. | 
**ban_reason** | Option<**String**> | Ban reason. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


