# DeviceTriggermqtt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**automation_type** | **String** | The type of automation, must be 'trigger'. | 
**payload** | Option<**String**> | Optional payload to match the payload being sent over the topic. | [optional]
**qos** | Option<**i32**> | The maximum QoS level to be used when receiving and publishing messages. | [optional]
**topic** | **String** | The MQTT topic subscribed to receive trigger events. | 
**r#type** | **String** | The type of the trigger, e.g. `button_short_press`. Entries supported by the frontend: `button_short_press`, `button_short_release`, `button_long_press`, `button_long_release`, `button_double_press`, `button_triple_press`, `button_quadruple_press`, `button_quintuple_press`. If set to an unsupported value, will render as `subtype type`, e.g. `button_1 spammed` with `type` set to `spammed` and `subtype` set to `button_1` | 
**subtype** | **String** | The subtype of the trigger, e.g. `button_1`. Entries supported by the frontend: `turn_on`, `turn_off`, `button_1`, `button_2`, `button_3`, `button_4`, `button_5`, `button_6`. If set to an unsupported value, will render as `subtype type`, e.g. `left_button pressed` with `type` set to `button_short_press` and `subtype` set to `left_button` | 
**device** | [**crate::models::DeviceTriggerDevice**](DeviceTrigger_device.md) |  | 
**value_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the value. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


