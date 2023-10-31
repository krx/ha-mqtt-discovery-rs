# Sensor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device** | Option<[**crate::models::Device**](Device.md)> |  | [optional]
**device_class** | Option<**String**> | The [type/class](/integrations/sensor/#device-class) of the sensor to set the icon in the frontend. The `device_class` can be `null`. (Default: None) | [optional]
**enabled_by_default** | Option<**String**> | Flag which defines if the entity should be enabled when first added. (Default: true) | [optional]
**encoding** | Option<**String**> | The encoding of the payloads received. Set to `\"\"` to disable decoding of incoming payload. (Default: utf-8) | [optional]
**entity_category** | Option<**String**> | The [category](https://developers.home-assistant.io/docs/core/entity#generic-properties) of the entity. (Default: None) | [optional]
**expire_after** | Option<**String**> | If set, it defines the number of seconds after the sensor's state expires, if it's not updated. After expiry, the sensor's state becomes `unavailable`. Default the sensors state never expires. | [optional]
**force_update** | Option<**String**> | Sends update events even if the value hasn't changed. Useful if you want to have meaningful value graphs in history. | [optional]
**json_attributes_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. | [optional]
**json_attributes_topic** | Option<**String**> | The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Implies `force_update` of the current sensor state when a message is received on this topic. | [optional]
**last_reset_value_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the last_reset. Available variables: `entity_id`. The `entity_id` can be used to reference the entity's attributes. | [optional]
**name** | Option<**String**> | The name of the MQTT sensor. Can be set to `null` if only the device name is relevant. (Default: MQTT Sensor) | [optional]
**object_id** | Option<**String**> | Used instead of `name` for automatic generation of `entity_id` | [optional]
**suggested_display_precision** | Option<**String**> | The number of decimals which should be used in the sensor's state after rounding. | [optional]
**qos** | Option<**String**> | The maximum QoS level to be used when receiving and publishing messages. | [optional]
**state_class** | Option<**String**> | The [state_class](https://developers.home-assistant.io/docs/core/entity/sensor#available-state-classes) of the sensor. (Default: None) | [optional]
**state_topic** | **String** | The MQTT topic subscribed to receive sensor values. If `device_class`, `state_class`, `unit_of_measurement` or `suggested_display_precision` is set, and a numeric value is expected, an empty value `''` will be ignored and will not update the state, a `'null'` value will set the sensor to an `unknown` state. The `device_class` can be `null`. (Default: None) | 
**unique_id** | Option<**String**> | An ID that uniquely identifies this sensor. If two sensors have the same unique ID, Home Assistant will raise an exception. | [optional]
**unit_of_measurement** | Option<**String**> | Defines the units of measurement of the sensor, if any. The `unit_of_measurement` can be `null`. (Default: None) | [optional]
**value_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the value. If the template throws an error, the current state will be used instead. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


