# \TasksApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_tasks_rewards_tasks_rewards_get**](TasksApi.md#get_all_tasks_rewards_tasks_rewards_get) | **GET** /tasks/rewards | Get All Tasks Rewards
[**get_all_tasks_tasks_list_get**](TasksApi.md#get_all_tasks_tasks_list_get) | **GET** /tasks/list | Get All Tasks
[**get_task_tasks_list_code_get**](TasksApi.md#get_task_tasks_list_code_get) | **GET** /tasks/list/{code} | Get Task
[**get_tasks_reward_tasks_rewards_code_get**](TasksApi.md#get_tasks_reward_tasks_rewards_code_get) | **GET** /tasks/rewards/{code} | Get Tasks Reward



## get_all_tasks_rewards_tasks_rewards_get

> models::DataPageDropRateSchema get_all_tasks_rewards_tasks_rewards_get(page, size)
Get All Tasks Rewards

Fetch the list of all tasks rewards. To obtain these rewards, you must exchange 6 task coins with a tasks master.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number |  |[default to 1]
**size** | Option<**i32**> | Page size |  |[default to 50]

### Return type

[**models::DataPageDropRateSchema**](DataPage_DropRateSchema_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_tasks_tasks_list_get

> models::DataPageTaskFullSchema get_all_tasks_tasks_list_get(min_level, max_level, skill, r#type, page, size)
Get All Tasks

Fetch the list of all tasks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**min_level** | Option<**i32**> | Minimum level. |  |
**max_level** | Option<**i32**> | Maximum level. |  |
**skill** | Option<[**models::Skill**](.md)> | The code of the skill. |  |
**r#type** | Option<[**models::TaskType**](.md)> | The type of tasks. |  |
**page** | Option<**i32**> | Page number |  |[default to 1]
**size** | Option<**i32**> | Page size |  |[default to 50]

### Return type

[**models::DataPageTaskFullSchema**](DataPage_TaskFullSchema_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_task_tasks_list_code_get

> models::TaskFullResponseSchema get_task_tasks_list_code_get(code)
Get Task

Retrieve the details of a task.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The code of the task. | [required] |

### Return type

[**models::TaskFullResponseSchema**](TaskFullResponseSchema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tasks_reward_tasks_rewards_code_get

> models::TasksRewardResponseSchema get_tasks_reward_tasks_rewards_code_get(code)
Get Tasks Reward

Retrieve the details of a tasks reward.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The code of the tasks reward. | [required] |

### Return type

[**models::TasksRewardResponseSchema**](TasksRewardResponseSchema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

