# Fan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**availability** | Option<[**Vec<crate::models::AlarmControlPanelAvailabilityInner>**](AlarmControlPanel_availability_inner.md)> | A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`. | [optional]
**availability_mode** | Option<**String**> | When `availability` is configured, this controls the conditions needed to set the entity to `available`. Valid entries are `all`, `any`, and `latest`. If set to `all`, `payload_available` must be received on all configured availability topics before the entity is marked as online. If set to `any`, `payload_available` must be received on at least one configured availability topic before the entity is marked as online. If set to `latest`, the last `payload_available` or `payload_not_available` received on any configured availability topic controls the availability. (Default: `latest)` | [optional]
**availability_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's availability from the `availability_topic`. To determine the devices's availability result of this template will be compared to `payload_available` and `payload_not_available`. | [optional]
**availability_topic** | Option<**String**> | The MQTT topic subscribed to receive availability (online/offline) updates. Must not be used together with `availability`. | [optional]
**command_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `command_topic`. | [optional]
**command_topic** | **String** | The MQTT topic to publish commands to change the fan state. | 
**device** | Option<[**crate::models::FanDevice**](Fan_device.md)> |  | [optional]
**enabled_by_default** | Option<**bool**> | Flag which defines if the entity should be enabled when first added. (Default: `true)` | [optional]
**encoding** | Option<**String**> | The encoding of the payloads received and published messages. Set to `\"\"` to disable decoding of incoming payload. (Default: `utf-8)` | [optional]
**entity_category** | Option<**String**> | The [category](https://developers.home-assistant.io/docs/core/entity#generic-properties) of the entity. (Default: `None)` | [optional]
**json_attributes_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation. | [optional]
**json_attributes_topic** | Option<**String**> | The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation. | [optional]
**name** | Option<**String**> | The name of the fan. Can be set to `null` if only the device name is relevant. (Default: `MQTT Fan)` | [optional]
**object_id** | Option<**String**> | Used instead of `name` for automatic generation of `entity_id` | [optional]
**optimistic** | Option<**bool**> | Flag that defines if fan works in optimistic mode (Default: ``true` if no state topic defined, else `false`.)` | [optional]
**direction_command_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `direction_command_topic`. | [optional]
**direction_command_topic** | Option<**String**> | The MQTT topic to publish commands to change the direction state. | [optional]
**direction_state_topic** | Option<**String**> | The MQTT topic subscribed to receive direction state updates. | [optional]
**direction_value_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a value from the direction. | [optional]
**oscillation_command_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `oscillation_command_topic`. | [optional]
**oscillation_command_topic** | Option<**String**> | The MQTT topic to publish commands to change the oscillation state. | [optional]
**oscillation_state_topic** | Option<**String**> | The MQTT topic subscribed to receive oscillation state updates. | [optional]
**oscillation_value_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a value from the oscillation. | [optional]
**payload_available** | Option<**String**> | The payload that represents the available state. (Default: `online)` | [optional]
**payload_not_available** | Option<**String**> | The payload that represents the unavailable state. (Default: `offline)` | [optional]
**payload_off** | Option<**String**> | The payload that represents the stop state. (Default: `OFF)` | [optional]
**payload_on** | Option<**String**> | The payload that represents the running state. (Default: `ON)` | [optional]
**payload_oscillation_off** | Option<**String**> | The payload that represents the oscillation off state. (Default: `oscillate_off)` | [optional]
**payload_oscillation_on** | Option<**String**> | The payload that represents the oscillation on state. (Default: `oscillate_on)` | [optional]
**payload_reset_percentage** | Option<**String**> | A special payload that resets the `percentage` state attribute to `unknown` when received at the `percentage_state_topic`. (Default: `None)` | [optional]
**payload_reset_preset_mode** | Option<**String**> | A special payload that resets the `preset_mode` state attribute to `unknown` when received at the `preset_mode_state_topic`. (Default: `None)` | [optional]
**percentage_command_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `percentage_command_topic`. | [optional]
**percentage_command_topic** | Option<**String**> | The MQTT topic to publish commands to change the fan speed state based on a percentage. | [optional]
**percentage_state_topic** | Option<**String**> | The MQTT topic subscribed to receive fan speed based on percentage. | [optional]
**percentage_value_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the `percentage` value from the payload received on `percentage_state_topic`. | [optional]
**preset_mode_command_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `preset_mode_command_topic`. | [optional]
**preset_mode_command_topic** | Option<**String**> | The MQTT topic to publish commands to change the preset mode. | [optional]
**preset_mode_state_topic** | Option<**String**> | The MQTT topic subscribed to receive fan speed based on presets. | [optional]
**preset_mode_value_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the `preset_mode` value from the payload received on `preset_mode_state_topic`. | [optional]
**preset_modes** | Option<[**crate::models::FanPresetModes**](Fan_preset_modes.md)> |  | [optional]
**qos** | Option<**i32**> | The maximum QoS level to be used when receiving and publishing messages. | [optional]
**retain** | Option<**bool**> | If the published message should have the retain flag on or not. (Default: `true)` | [optional]
**speed_range_max** | Option<**i32**> | The maximum of numeric output range (representing 100 %). The number of speeds within the `speed_range` / `100` will determine the `percentage_step`. (Default: `100)` | [optional]
**speed_range_min** | Option<**i32**> | The minimum of numeric output range (`off` not included, so `speed_range_min` - `1` represents 0 %). The number of speeds within the speed_range / 100 will determine the `percentage_step`. (Default: `1)` | [optional]
**state_topic** | Option<**String**> | The MQTT topic subscribed to receive state updates. | [optional]
**state_value_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a value from the state. | [optional]
**unique_id** | Option<**String**> | An ID that uniquely identifies this fan. If two fans have the same unique ID, Home Assistant will raise an exception. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


