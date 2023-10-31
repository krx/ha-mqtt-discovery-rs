# AlarmControlPanelAvailabilityInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**payload_available** | Option<**String**> | The payload that represents the available state. (Default: `online)` | [optional]
**payload_not_available** | Option<**String**> | The payload that represents the unavailable state. (Default: `offline)` | [optional]
**topic** | **String** | An MQTT topic subscribed to receive availability (online/offline) updates. | 
**value_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's availability from the `topic`. To determine the devices's availability result of this template will be compared to `payload_available` and `payload_not_available`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


