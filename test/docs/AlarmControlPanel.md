# AlarmControlPanel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**availability** | Option<[**Vec<crate::models::AlarmControlPanelAvailabilityInner>**](AlarmControlPanel_availability_inner.md)> | A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`. | [optional]
**availability_mode** | Option<**String**> | When `availability` is configured, this controls the conditions needed to set the entity to `available`. Valid entries are `all`, `any`, and `latest`. If set to `all`, `payload_available` must be received on all configured availability topics before the entity is marked as online. If set to `any`, `payload_available` must be received on at least one configured availability topic before the entity is marked as online. If set to `latest`, the last `payload_available` or `payload_not_available` received on any configured availability topic controls the availability. (Default: `latest)` | [optional]
**availability_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's availability from the `availability_topic`. To determine the devices's availability result of this template will be compared to `payload_available` and `payload_not_available`. | [optional]
**availability_topic** | Option<**String**> | The MQTT topic subscribed to receive availability (online/offline) updates. Must not be used together with `availability`. | [optional]
**code** | Option<**String**> | If defined, specifies a code to enable or disable the alarm in the frontend. Note that the code is validated locally and blocks sending MQTT messages to the remote device. For remote code validation, the code can be configured to either of the special values `REMOTE_CODE` (numeric code) or `REMOTE_CODE_TEXT` (text code). In this case, local code validation is bypassed but the frontend will still show a numeric or text code dialog. Use `command_template` to send the code to the remote device. Example configurations for remote code validation [can be found here](#configurations-with-remote-code-validation). | [optional]
**code_arm_required** | Option<**bool**> | If true the code is required to arm the alarm. If false the code is not validated. (Default: `true)` | [optional]
**code_disarm_required** | Option<**bool**> | If true the code is required to disarm the alarm. If false the code is not validated. (Default: `true)` | [optional]
**code_trigger_required** | Option<**bool**> | If true the code is required to trigger the alarm. If false the code is not validated. (Default: `true)` | [optional]
**command_template** | Option<**String**> | The [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) used for the command payload. Available variables: `action` and `code`. (Default: `action)` | [optional]
**command_topic** | **String** | The MQTT topic to publish commands to change the alarm state. | 
**device** | Option<[**crate::models::AlarmControlPanelDevice**](AlarmControlPanel_device.md)> |  | [optional]
**enabled_by_default** | Option<**bool**> | Flag which defines if the entity should be enabled when first added. (Default: `true)` | [optional]
**encoding** | Option<**String**> | The encoding of the payloads received and published messages. Set to `\"\"` to disable decoding of incoming payload. (Default: `utf-8)` | [optional]
**entity_category** | Option<**String**> | The [category](https://developers.home-assistant.io/docs/core/entity#generic-properties) of the entity. (Default: `None)` | [optional]
**json_attributes_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation. | [optional]
**json_attributes_topic** | Option<**String**> | The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation. | [optional]
**name** | Option<**String**> | The name of the alarm. Can be set to `null` if only the device name is relevant. (Default: `MQTT Alarm)` | [optional]
**object_id** | Option<**String**> | Used instead of `name` for automatic generation of `entity_id` | [optional]
**payload_arm_away** | Option<**String**> | The payload to set armed-away mode on your Alarm Panel. (Default: `ARM_AWAY)` | [optional]
**payload_arm_home** | Option<**String**> | The payload to set armed-home mode on your Alarm Panel. (Default: `ARM_HOME)` | [optional]
**payload_arm_night** | Option<**String**> | The payload to set armed-night mode on your Alarm Panel. (Default: `ARM_NIGHT)` | [optional]
**payload_arm_vacation** | Option<**String**> | The payload to set armed-vacation mode on your Alarm Panel. (Default: `ARM_VACATION)` | [optional]
**payload_arm_custom_bypass** | Option<**String**> | The payload to set armed-custom-bypass mode on your Alarm Panel. (Default: `ARM_CUSTOM_BYPASS)` | [optional]
**payload_available** | Option<**String**> | The payload that represents the available state. (Default: `online)` | [optional]
**payload_disarm** | Option<**String**> | The payload to disarm your Alarm Panel. (Default: `DISARM)` | [optional]
**payload_not_available** | Option<**String**> | The payload that represents the unavailable state. (Default: `offline)` | [optional]
**payload_trigger** | Option<**String**> | The payload to trigger the alarm on your Alarm Panel. (Default: `TRIGGER)` | [optional]
**qos** | Option<**i32**> | The maximum QoS level to be used when receiving and publishing messages. | [optional]
**retain** | Option<**bool**> | If the published message should have the retain flag on or not. | [optional]
**state_topic** | **String** | The MQTT topic subscribed to receive state updates. | 
**supported_features** | Option<**Vec<String>**> | A list of features that the alarm control panel supports. The available list options are `arm_home`, `arm_away`, `arm_night`, `arm_vacation`, `arm_custom_bypass`, and `trigger`. (Default: `arm_home,arm_away,arm_night,arm_vacation,arm_custom_bypass,trigger)` | [optional]
**unique_id** | Option<**String**> | An ID that uniquely identifies this alarm panel. If two alarm panels have the same unique ID, Home Assistant will raise an exception. | [optional]
**value_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the value. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


