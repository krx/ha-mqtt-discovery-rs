# Lock

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**availability** | Option<[**Vec<crate::models::AlarmControlPanelAvailabilityInner>**](AlarmControlPanel_availability_inner.md)> | A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`. | [optional]
**availability_mode** | Option<**String**> | When `availability` is configured, this controls the conditions needed to set the entity to `available`. Valid entries are `all`, `any`, and `latest`. If set to `all`, `payload_available` must be received on all configured availability topics before the entity is marked as online. If set to `any`, `payload_available` must be received on at least one configured availability topic before the entity is marked as online. If set to `latest`, the last `payload_available` or `payload_not_available` received on any configured availability topic controls the availability. (Default: `latest)` | [optional]
**availability_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's availability from the `availability_topic`. To determine the devices's availability result of this template will be compared to `payload_available` and `payload_not_available`. | [optional]
**availability_topic** | Option<**String**> | The MQTT topic subscribed to receive availability (online/offline) updates. Must not be used together with `availability`. | [optional]
**code_format** | Option<**String**> | A regular expression to validate a supplied code when it is set during the service call to `open`, `lock` or `unlock` the MQTT lock. | [optional]
**command_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `command_topic`. The lock command template accepts the parameters `value` and `code`. The `value` parameter will contain the configured value for either `payload_open`, `payload_lock` or `payload_unlock`. The `code` parameter is set during the service call to `open`, `lock` or `unlock` the MQTT lock and will be set `None` if no code was passed. | [optional]
**command_topic** | **String** | The MQTT topic to publish commands to change the lock state. | 
**device** | Option<[**crate::models::ButtonDevice**](Button_device.md)> |  | [optional]
**enabled_by_default** | Option<**bool**> | Flag which defines if the entity should be enabled when first added. (Default: `true)` | [optional]
**encoding** | Option<**String**> | The encoding of the payloads received and published messages. Set to `\"\"` to disable decoding of incoming payload. (Default: `utf-8)` | [optional]
**entity_category** | Option<**String**> | The [category](https://developers.home-assistant.io/docs/core/entity#generic-properties) of the entity. (Default: `None)` | [optional]
**json_attributes_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation. | [optional]
**json_attributes_topic** | Option<**String**> | The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation. | [optional]
**name** | Option<**String**> | The name of the lock. Can be set to `null` if only the device name is relevant. (Default: `MQTT Lock)` | [optional]
**object_id** | Option<**String**> | Used instead of `name` for automatic generation of `entity_id` | [optional]
**optimistic** | Option<**bool**> | Flag that defines if lock works in optimistic mode. (Default: ``true` if no `state_topic` defined, else `false`.)` | [optional]
**payload_available** | Option<**String**> | The payload that represents the available state. (Default: `online)` | [optional]
**payload_lock** | Option<**String**> | The payload sent to the lock to lock it. (Default: `LOCK)` | [optional]
**payload_not_available** | Option<**String**> | The payload that represents the unavailable state. (Default: `offline)` | [optional]
**payload_unlock** | Option<**String**> | The payload sent to the lock to unlock it. (Default: `UNLOCK)` | [optional]
**payload_open** | Option<**String**> | The payload sent to the lock to open it. (Default: `OPEN)` | [optional]
**payload_reset** | Option<**String**> | A special payload that resets the state to `unknown` when received on the `state_topic`. (Default: `\"None\")` | [optional]
**qos** | Option<**i32**> | The maximum QoS level to be used when receiving and publishing messages. | [optional]
**retain** | Option<**bool**> | If the published message should have the retain flag on or not. | [optional]
**state_jammed** | Option<**String**> | The payload sent to `state_topic` by the lock when it's jammed. (Default: `JAMMED)` | [optional]
**state_locked** | Option<**String**> | The payload sent to `state_topic` by the lock when it's locked. (Default: `LOCKED)` | [optional]
**state_locking** | Option<**String**> | The payload sent to `state_topic` by the lock when it's locking. (Default: `LOCKING)` | [optional]
**state_topic** | Option<**String**> | The MQTT topic subscribed to receive state updates. It accepts states configured with `state_jammed`, `state_locked`, `state_unlocked`, `state_locking` or `state_unlocking`. | [optional]
**state_unlocked** | Option<**String**> | The payload sent to `state_topic` by the lock when it's unlocked. (Default: `UNLOCKED)` | [optional]
**state_unlocking** | Option<**String**> | The payload sent to `state_topic` by the lock when it's unlocking. (Default: `UNLOCKING)` | [optional]
**unique_id** | Option<**String**> | An ID that uniquely identifies this lock. If two locks have the same unique ID, Home Assistant will raise an exception. | [optional]
**value_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a state value from the payload. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


