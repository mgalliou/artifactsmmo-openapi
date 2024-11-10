# AccountAchievementSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the achievement. | 
**code** | **String** | Code of the achievement.  | 
**description** | **String** | Description of the achievement. | 
**points** | **i32** | Points of the achievement. Used for the leaderboard. | 
**r#type** | [**models::AchievementType**](AchievementType.md) | Type of achievement. | 
**target** | Option<**String**> |  | 
**total** | **i32** | Total to do. | 
**rewards** | [**models::AchievementRewardsSchema**](AchievementRewardsSchema.md) | Rewards. | 
**current** | **i32** | Current progress. | 
**completed_at** | Option<**String**> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


