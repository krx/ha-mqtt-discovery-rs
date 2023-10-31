# Humidifier

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action_template** | Option<**String**> | A template to render the value received on the `action_topic` with. | [optional]
**action_topic** | Option<**String**> | The MQTT topic to subscribe for changes of the current action. Valid values: `off`, `humidifying`, `drying`, `idle` | [optional]
**availability** | Option<[**Vec<crate::models::AlarmControlPanelAvailabilityInner>**](AlarmControlPanel_availability_inner.md)> | A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`. | [optional]
**availability_mode** | Option<**String**> | When `availability` is configured, this controls the conditions needed to set the entity to `available`. Valid entries are `all`, `any`, and `latest`. If set to `all`, `payload_available` must be received on all configured availability topics before the entity is marked as online. If set to `any`, `payload_available` must be received on at least one configured availability topic before the entity is marked as online. If set to `latest`, the last `payload_available` or `payload_not_available` received on any configured availability topic controls the availability. (Default: `latest)` | [optional]
**availability_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's availability from the `availability_topic`. To determine the devices's availability result of this template will be compared to `payload_available` and `payload_not_available`. | [optional]
**availability_topic** | Option<**String**> | The MQTT topic subscribed to receive availability (online/offline) updates. Must not be used together with `availability`. | [optional]
**current_humidity_template** | Option<**String**> | A template with which the value received on `current_humidity_topic` will be rendered. | [optional]
**current_humidity_topic** | Option<**String**> | The MQTT topic on which to listen for the current humidity. A `\"None\"` value received will reset the current humidity. Empty values (`'''`) will be ignored. | [optional]
**command_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `command_topic`. | [optional]
**command_topic** | **String** | The MQTT topic to publish commands to change the humidifier state. | 
**device** | Option<[**crate::models::FanDevice**](Fan_device.md)> |  | [optional]
**device_class** | Option<**String**> | The device class of the MQTT device. Must be either `humidifier`, `dehumidifier` or `null`. (Default: `humidifier)` | [optional]
**enabled_by_default** | Option<**bool**> | Flag which defines if the entity should be enabled when first added. (Default: `true)` | [optional]
**encoding** | Option<**String**> | The encoding of the payloads received and published messages. Set to `\"\"` to disable decoding of incoming payload. (Default: `utf-8)` | [optional]
**entity_category** | Option<**String**> | The [category](https://developers.home-assistant.io/docs/core/entity#generic-properties) of the entity. (Default: `None)` | [optional]
**json_attributes_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation. | [optional]
**json_attributes_topic** | Option<**String**> | The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation. | [optional]
**max_humidity** | Option<**i32**> | The minimum target humidity percentage that can be set. (Default: `100)` | [optional]
**min_humidity** | Option<**i32**> | The maximum target humidity percentage that can be set. | [optional]
**name** | Option<**String**> | The name of the humidifier. Can be set to `null` if only the device name is relevant. (Default: `MQTT humidifier)` | [optional]
**object_id** | Option<**String**> | Used instead of `name` for automatic generation of `entity_id` | [optional]
**optimistic** | Option<**bool**> | Flag that defines if humidifier works in optimistic mode (Default: ``true` if no state topic defined, else `false`.)` | [optional]
**payload_available** | Option<**String**> | The payload that represents the available state. (Default: `online)` | [optional]
**payload_not_available** | Option<**String**> | The payload that represents the unavailable state. (Default: `offline)` | [optional]
**payload_off** | Option<**String**> | The payload that represents the stop state. (Default: `OFF)` | [optional]
**payload_on** | Option<**String**> | The payload that represents the running state. (Default: `ON)` | [optional]
**payload_reset_humidity** | Option<**String**> | A special payload that resets the `target_humidity` state attribute to an `unknown` state when received at the `target_humidity_state_topic`. When received at `current_humidity_topic` it will reset the current humidity state. (Default: `None)` | [optional]
**payload_reset_mode** | Option<**String**> | A special payload that resets the `mode` state attribute to an `unknown` state when received at the `mode_state_topic`. (Default: `None)` | [optional]
**target_humidity_command_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `target_humidity_command_topic`. | [optional]
**target_humidity_command_topic** | **String** | The MQTT topic to publish commands to change the humidifier target humidity state based on a percentage. | 
**target_humidity_state_topic** | Option<**String**> | The MQTT topic subscribed to receive humidifier target humidity. | [optional]
**target_humidity_state_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a value for the humidifier `target_humidity` state. | [optional]
**mode_command_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `mode_command_topic`. | [optional]
**mode_command_topic** | Option<**String**> | The MQTT topic to publish commands to change the `mode` on the humidifier. This attribute ust be configured together with the `modes` attribute. | [optional]
**mode_state_topic** | Option<**String**> | The MQTT topic subscribed to receive the humidifier `mode`. | [optional]
**mode_state_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a value for the humidifier `mode` state. | [optional]
**modes** | Option<[**crate::models::FanPresetModes**](Fan_preset_modes.md)> |  | [optional]
**qos** | Option<**i32**> | The maximum QoS level to be used when receiving and publishing messages. | [optional]
**retain** | Option<**bool**> | If the published message should have the retain flag on or not. (Default: `true)` | [optional]
**state_topic** | Option<**String**> | The MQTT topic subscribed to receive state updates. | [optional]
**state_value_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a value from the state. | [optional]
**unique_id** | Option<**String**> | An ID that uniquely identifies this humidifier. If two humidifiers have the same unique ID, Home Assistant will raise an exception. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


