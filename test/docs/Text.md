# Text

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**availability** | Option<[**Vec<crate::models::AlarmControlPanelAvailabilityInner>**](AlarmControlPanel_availability_inner.md)> | A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`. | [optional]
**availability_topic** | Option<**String**> | The MQTT topic subscribed to receive availability (online/offline) updates. Must not be used together with `availability`. | [optional]
**availability_mode** | Option<**String**> | When `availability` is configured, this controls the conditions needed to set the entity to `available`. Valid entries are `all`, `any`, and `latest`. If set to `all`, `payload_available` must be received on all configured availability topics before the entity is marked as online. If set to `any`, `payload_available` must be received on at least one configured availability topic before the entity is marked as online. If set to `latest`, the last `payload_available` or `payload_not_available` received on any configured availability topic controls the availability. (Default: `latest)` | [optional]
**availability_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's availability from the `availability_topic`. To determine the devices's availability result of this template will be compared to `payload_available` and `payload_not_available`. | [optional]
**command_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `command_topic`. | [optional]
**command_topic** | **String** | The MQTT topic to publish the text value that is set. | 
**device** | Option<[**crate::models::CameraDevice**](Camera_device.md)> |  | [optional]
**enabled_by_default** | Option<**bool**> | Flag which defines if the entity should be enabled when first added. (Default: `true)` | [optional]
**encoding** | Option<**String**> | The encoding of the payloads received and published messages. Set to `\"\"` to disable decoding of incoming payload. (Default: `utf-8)` | [optional]
**entity_category** | Option<**String**> | The [category](https://developers.home-assistant.io/docs/core/entity#generic-properties) of the entity. (Default: `None)` | [optional]
**json_attributes_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. | [optional]
**json_attributes_topic** | Option<**String**> | The MQTT topic subscribed to receive a JSON dictionary payload and then set as entity attributes. Implies `force_update` of the current select state when a message is received on this topic. | [optional]
**max** | Option<**i32**> | The maximum size of a text being set or received (maximum is 255). (Default: `255)` | [optional]
**min** | Option<**i32**> | The minimum size of a text being set or received. | [optional]
**mode** | Option<**String**> | The mode off the text entity. Must be either `text` or `password`. (Default: `text)` | [optional]
**name** | Option<**String**> | The name of the text entity. Can be set to `null` if only the device name is relevant. (Default: `MQTT Text)` | [optional]
**object_id** | Option<**String**> | Used instead of `name` for automatic generation of `entity_id` | [optional]
**pattern** | Option<**String**> | A valid regular expression the text being set or received must match with. | [optional]
**qos** | Option<**i32**> | The maximum QoS level to be used when receiving and publishing messages. | [optional]
**retain** | Option<**bool**> | If the published message should have the retain flag on or not. | [optional]
**state_topic** | Option<**String**> | The MQTT topic subscribed to receive text state updates. Text state updates should match the `pattern` (if set) and meet the size constraints `min` and `max`. Can be used with `value_template` to render the incoming payload to a text update. | [optional]
**unique_id** | Option<**String**> | An ID that uniquely identifies this Select. If two Selects have the same unique ID Home Assistant will raise an exception. | [optional]
**value_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the text state value from the payload received on `state_topic`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


