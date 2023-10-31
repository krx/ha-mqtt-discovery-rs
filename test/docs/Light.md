# Light

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**availability** | Option<[**Vec<crate::models::AlarmControlPanelAvailabilityInner>**](AlarmControlPanel_availability_inner.md)> | A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`. | [optional]
**availability_mode** | Option<**String**> | When `availability` is configured, this controls the conditions needed to set the entity to `available`. Valid entries are `all`, `any`, and `latest`. If set to `all`, `payload_available` must be received on all configured availability topics before the entity is marked as online. If set to `any`, `payload_available` must be received on at least one configured availability topic before the entity is marked as online. If set to `latest`, the last `payload_available` or `payload_not_available` received on any configured availability topic controls the availability. (Default: `latest)` | [optional]
**availability_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's availability from the `availability_topic`. To determine the devices's availability result of this template will be compared to `payload_available` and `payload_not_available`. | [optional]
**availability_topic** | Option<**String**> | The MQTT topic subscribed to receive availability (online/offline) updates. Must not be used together with `availability`. | [optional]
**blue_template** | Option<**String**> | [Template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract blue color from the state payload value. Expected result of the template is an integer from 0-255 range. | [optional]
**brightness_template** | Option<**String**> | [Template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract brightness from the state payload value. Expected result of the template is an integer from 0-255 range. | [optional]
**color_temp_template** | Option<**String**> | [Template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract color temperature from the state payload value. Expected result of the template is an integer representing mired units. | [optional]
**command_off_template** | **String** | The [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) for *off* state changes. Available variables: `state` and `transition`. | 
**command_on_template** | **String** | The [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) for *on* state changes. Available variables: `state`, `brightness`, `color_temp`, `red`, `green`, `blue`, `flash`, `transition` and `effect`. Values `red`, `green`, `blue`, `brightness` are provided as integers from range 0-255. Value of `color_temp` is provided as integer representing mired units. | 
**command_topic** | **String** | The MQTT topic to publish commands to change the light’s state. | 
**device** | Option<[**crate::models::LightDevice**](Light_device.md)> |  | [optional]
**enabled_by_default** | Option<**bool**> | Flag which defines if the entity should be enabled when first added. (Default: `true)` | [optional]
**encoding** | Option<**String**> | The encoding of the payloads received and published messages. Set to `\"\"` to disable decoding of incoming payload. (Default: `utf-8)` | [optional]
**entity_category** | Option<**String**> | The [category](https://developers.home-assistant.io/docs/core/entity#generic-properties) of the entity. (Default: `None)` | [optional]
**effect_list** | Option<[**crate::models::ButtonDeviceIdentifiers**](Button_device_identifiers.md)> |  | [optional]
**effect_template** | Option<**String**> | [Template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract effect from the state payload value. | [optional]
**green_template** | Option<**String**> | [Template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract green color from the state payload value. Expected result of the template is an integer from 0-255 range. | [optional]
**json_attributes_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation. | [optional]
**json_attributes_topic** | Option<**String**> | The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation. | [optional]
**max_mireds** | Option<**i32**> | The maximum color temperature in mireds. | [optional]
**min_mireds** | Option<**i32**> | The minimum color temperature in mireds. | [optional]
**name** | Option<**String**> | The name of the light. (Default: `MQTT Template Light)` | [optional]
**object_id** | Option<**String**> | Used instead of `name` for automatic generation of `entity_id` | [optional]
**optimistic** | Option<**bool**> | Flag that defines if the light works in optimistic mode. (Default: ``true` if no state topic or state template is defined, else `false`.)` | [optional]
**payload_available** | Option<**String**> | The payload that represents the available state. (Default: `online)` | [optional]
**payload_not_available** | Option<**String**> | The payload that represents the unavailable state. (Default: `offline)` | [optional]
**qos** | Option<**i32**> | The maximum QoS level to be used when receiving and publishing messages. | [optional]
**red_template** | Option<**String**> | [Template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract red color from the state payload value. Expected result of the template is an integer from 0-255 range. | [optional]
**schema** | Option<**String**> | The schema to use. Must be `template` to select the template schema. (Default: `default)` | [optional]
**state_template** | Option<**String**> | [Template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract state from the state payload value. | [optional]
**state_topic** | Option<**String**> | The MQTT topic subscribed to receive state updates. | [optional]
**unique_id** | Option<**String**> | An ID that uniquely identifies this light. If two lights have the same unique ID, Home Assistant will raise an exception. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


