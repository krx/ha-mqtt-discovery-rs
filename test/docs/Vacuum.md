# Vacuum

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**availability** | Option<[**Vec<crate::models::AlarmControlPanelAvailabilityInner>**](AlarmControlPanel_availability_inner.md)> | A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`. | [optional]
**availability_mode** | Option<**String**> | When `availability` is configured, this controls the conditions needed to set the entity to `available`. Valid entries are `all`, `any`, and `latest`. If set to `all`, `payload_available` must be received on all configured availability topics before the entity is marked as online. If set to `any`, `payload_available` must be received on at least one configured availability topic before the entity is marked as online. If set to `latest`, the last `payload_available` or `payload_not_available` received on any configured availability topic controls the availability. (Default: `latest)` | [optional]
**availability_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's availability from the `availability_topic`. To determine the devices's availability result of this template will be compared to `payload_available` and `payload_not_available`. | [optional]
**availability_topic** | Option<**String**> | The MQTT topic subscribed to receive availability (online/offline) updates. Must not be used together with `availability`. | [optional]
**command_topic** | Option<**String**> | The MQTT topic to publish commands to control the vacuum. | [optional]
**device** | Option<[**crate::models::ButtonDevice**](Button_device.md)> |  | [optional]
**encoding** | Option<**String**> | The encoding of the payloads received and published messages. Set to `\"\"` to disable decoding of incoming payload. (Default: `utf-8)` | [optional]
**fan_speed_list** | Option<[**crate::models::ButtonDeviceIdentifiers**](Button_device_identifiers.md)> |  | [optional]
**json_attributes_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation. | [optional]
**json_attributes_topic** | Option<**String**> | The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation. | [optional]
**name** | Option<**String**> | The name of the vacuum. Can be set to `null` if only the device name is relevant. (Default: `MQTT Vacuum)` | [optional]
**object_id** | Option<**String**> | Used instead of `name` for automatic generation of `entity_id` | [optional]
**payload_available** | Option<**String**> | The payload that represents the available state. (Default: `online)` | [optional]
**payload_clean_spot** | Option<**String**> | The payload to send to the `command_topic` to begin a spot cleaning cycle. (Default: `clean_spot)` | [optional]
**payload_locate** | Option<**String**> | The payload to send to the `command_topic` to locate the vacuum (typically plays a song). (Default: `locate)` | [optional]
**payload_not_available** | Option<**String**> | The payload that represents the unavailable state. (Default: `offline)` | [optional]
**payload_pause** | Option<**String**> | The payload to send to the `command_topic` to pause the vacuum. (Default: `pause)` | [optional]
**payload_return_to_base** | Option<**String**> | The payload to send to the `command_topic` to tell the vacuum to return to base. (Default: `return_to_base)` | [optional]
**payload_start** | Option<**String**> | The payload to send to the `command_topic` to begin the cleaning cycle. (Default: `start)` | [optional]
**payload_stop** | Option<**String**> | The payload to send to the `command_topic` to stop cleaning. (Default: `stop)` | [optional]
**qos** | Option<**i32**> | The maximum QoS level to be used when receiving and publishing messages. | [optional]
**retain** | Option<**bool**> | If the published message should have the retain flag on or not. | [optional]
**schema** | Option<**String**> | The schema to use. Must be `state`. (Default: `legacy)` | [optional]
**send_command_topic** | Option<**String**> | The MQTT topic to publish custom commands to the vacuum. | [optional]
**set_fan_speed_topic** | Option<**String**> | The MQTT topic to publish commands to control the vacuum's fan speed. | [optional]
**state_topic** | Option<**String**> | The MQTT topic subscribed to receive state messages from the vacuum. Messages received on the `state_topic` must be a valid JSON dictionary, with a mandatory `state` key and optionally `battery_level` and `fan_speed` keys as shown in the [example](#state-mqtt-protocol). | [optional]
**supported_features** | Option<[**crate::models::ButtonDeviceIdentifiers**](Button_device_identifiers.md)> |  | [optional]
**unique_id** | Option<**String**> | An ID that uniquely identifies this vacuum. If two vacuums have the same unique ID, Home Assistant will raise an exception. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


