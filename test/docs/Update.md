# Update

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**availability** | Option<[**Vec<crate::models::AlarmControlPanelAvailabilityInner>**](AlarmControlPanel_availability_inner.md)> | A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`. | [optional]
**availability_topic** | Option<**String**> | The MQTT topic subscribed to receive availability (online/offline) updates. Must not be used together with `availability`. | [optional]
**availability_mode** | Option<**String**> | When `availability` is configured, this controls the conditions needed to set the entity to `available`. Valid entries are `all`, `any`, and `latest`. If set to `all`, `payload_available` must be received on all configured availability topics before the entity is marked as online. If set to `any`, `payload_available` must be received on at least one configured availability topic before the entity is marked as online. If set to `latest`, the last `payload_available` or `payload_not_available` received on any configured availability topic controls the availability. (Default: `latest)` | [optional]
**availability_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's availability from the `availability_topic`. To determine the devices's availability result of this template will be compared to `payload_available` and `payload_not_available`. | [optional]
**command_topic** | Option<**String**> | The MQTT topic to publish `payload_install` to start installing process. | [optional]
**device** | Option<[**crate::models::CameraDevice**](Camera_device.md)> |  | [optional]
**device_class** | Option<**String**> | The [type/class](/integrations/update/#device-classes) of the update to set the icon in the frontend. The `device_class` can be `null`. (Default: `None)` | [optional]
**enabled_by_default** | Option<**bool**> | Flag which defines if the entity should be enabled when first added. (Default: `true)` | [optional]
**encoding** | Option<**String**> | The encoding of the payloads received and published messages. Set to `\"\"` to disable decoding of incoming payload. (Default: `utf-8)` | [optional]
**entity_category** | Option<**String**> | The [category](https://developers.home-assistant.io/docs/core/entity#generic-properties) of the entity. (Default: `None)` | [optional]
**entity_picture** | Option<**String**> | Picture URL for the entity. | [optional]
**json_attributes_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. | [optional]
**json_attributes_topic** | Option<**String**> | The MQTT topic subscribed to receive a JSON dictionary payload and then set as entity attributes. Implies `force_update` of the current select state when a message is received on this topic. | [optional]
**latest_version_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the latest version value. | [optional]
**latest_version_topic** | Option<**String**> | The MQTT topic subscribed to receive an update of the latest version. | [optional]
**name** | Option<**String**> | The name of the Update. Can be set to `null` if only the device name is relevant. | [optional]
**object_id** | Option<**String**> | Used instead of `name` for automatic generation of `entity_id` | [optional]
**payload_install** | Option<**String**> | The MQTT payload to start installing process. | [optional]
**qos** | Option<**i32**> | The maximum QoS level to be used when receiving and publishing messages. | [optional]
**release_summary** | Option<**String**> | Summary of the release notes or changelog. This is suitable a brief update description of max 255 characters. | [optional]
**release_url** | Option<**String**> | URL to the full release notes of the latest version available. | [optional]
**retain** | Option<**bool**> | If the published message should have the retain flag on or not. | [optional]
**state_topic** | Option<**String**> | The MQTT topic subscribed to receive state updates. The state update may be either JSON or a simple string with `installed_version` value. When a JSON payload is detected, the state value of the JSON payload should supply the `installed_version` and can optional supply: `latest_version`, `title`, `release_summary`, `release_url` or an `entity_picture` URL. | [optional]
**title** | Option<**String**> | Title of the software, or firmware update. This helps to differentiate between the device or entity name versus the title of the software installed. | [optional]
**unique_id** | Option<**String**> | An ID that uniquely identifies this Update. If two Updates have the same unique ID Home Assistant will raise an exception. | [optional]
**value_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the `installed_version` state value or to render to a valid JSON payload on from the payload received on `state_topic`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


