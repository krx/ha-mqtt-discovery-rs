# LawnMower

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity_state_topic** | Option<**String**> | The MQTT topic subscribed to receive an update of the activity. Valid activities are `mowing`, `paused`, `docked`, and `error`. Use `value_template` to extract the activity state from a custom payload. When payload `none` is received, the activity state will be reset to `unknown`. | [optional]
**activity_value_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the value. | [optional]
**availability** | Option<[**Vec<crate::models::LawnMowerAvailabilityInner>**](LawnMower_availability_inner.md)> | A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`. | [optional]
**availability_topic** | Option<**String**> | The MQTT topic subscribed to receive availability (online/offline) updates. Must not be used together with `availability`. | [optional]
**availability_mode** | Option<**String**> | When `availability` is configured, this controls the conditions needed to set the entity to `available`. Valid entries are `all`, `any`, and `latest`. If set to `all`, `payload_available` must be received on all configured availability topics before the entity is marked as online. If set to `any`, `payload_available` must be received on at least one configured availability topic before the entity is marked as online. If set to `latest`, the last `payload_available` or `payload_not_available` received on any configured availability topic controls the availability. (Default: `latest)` | [optional]
**availability_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's availability from the `availability_topic`. To determine the devices's availability, the result of this template will be compared to `payload_available` and `payload_not_available`. | [optional]
**device** | Option<[**crate::models::LawnMowerDevice**](LawnMower_device.md)> |  | [optional]
**dock_command_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `dock_command_topic`. The `value` parameter in the template will be set to `dock`. | [optional]
**dock_command_topic** | Option<**String**> | The MQTT topic that publishes commands when the service `lawn_mower.dock` service call is executed. The value `dock` is published when the service is called. Use a `dock_command_template` to publish a custom format. | [optional]
**enabled_by_default** | Option<**bool**> | Flag which defines if the entity should be enabled when first added. (Default: `true)` | [optional]
**encoding** | Option<**String**> | The encoding of the payloads received and published messages. Set to `\"\"` to disable decoding of the incoming payload. (Default: `utf-8)` | [optional]
**entity_category** | Option<**String**> | The [category](https://developers.home-assistant.io/docs/core/entity#generic-properties) of the entity. (Default: `None)` | [optional]
**json_attributes_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. | [optional]
**json_attributes_topic** | Option<**String**> | The MQTT topic subscribed to receive a JSON dictionary payload and then set as entity attributes. Implies `force_update` of the current activity state when a message is received on this topic. | [optional]
**name** | Option<**String**> | The name of the lawn mower. Can be set to `null` if only the device name is relevant. | [optional]
**object_id** | Option<**String**> | Used instead of `name` for automatic generation of `entity_id` | [optional]
**optimistic** | Option<**bool**> | Flag that defines if the lawn mower works in optimistic mode. (Default: ``true` if no `activity_state_topic` defined, else `false`.)` | [optional]
**pause_command_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `pause_command_topic`. The `value` parameter in the template will be set to `pause`. | [optional]
**pause_command_topic** | Option<**String**> | The MQTT topic that publishes commands when the service `lawn_mower.pause` service call is executed. The value `pause` is published when the service is called. Use a `pause_command_template` to publish a custom format. | [optional]
**qos** | Option<**i32**> | The maximum QoS level to be used when receiving and publishing messages. | [optional]
**start_mowing_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `dock_command_topic`. The `value` parameter in the template will be set to `dock`. | [optional]
**start_mowing_command_topic** | Option<**String**> | The MQTT topic that publishes commands when the service `lawn_mower.start_mowing` service call is executed. The value `start_mowing` is published when the service is called. Use a `start_mowing_command_template` to publish a custom format. | [optional]
**retain** | Option<**bool**> | If the published message should have the retain flag on or not. | [optional]
**unique_id** | Option<**String**> | An ID that uniquely identifies this lawn mower. If two lawn mowers have the same unique ID, Home Assistant will raise an exception. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


