# CameraDevice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**configuration_url** | Option<**String**> | A link to the webpage that can manage the configuration of this device. Can be either an `http://`, `https://` or an internal `homeassistant://` URL. | [optional]
**connections** | Option<**Vec<String>**> | A list of connections of the device to the outside world as a list of tuples `[connection_type, connection_identifier]`. For example the MAC address of a network interface: `\"connections\": [\"mac\", \"02:5b:26:a8:dc:12\"]`. | [optional]
**hw_version** | Option<**String**> | The hardware version of the device. | [optional]
**identifiers** | Option<[**crate::models::AlarmControlPanelDeviceIdentifiers**](AlarmControlPanel_device_identifiers.md)> |  | [optional]
**manufacturer** | Option<**String**> | The manufacturer of the device. | [optional]
**model** | Option<**String**> | The model of the device. | [optional]
**name** | Option<**String**> | The name of the device. | [optional]
**suggested_area** | Option<**String**> | Suggest an area if the device isn’t in one yet. | [optional]
**sw_version** | Option<**String**> | The firmware version of the device. | [optional]
**via_device** | Option<**String**> | Identifier of a device that routes messages between this device and Home Assistant. Examples of such devices are hubs, or parent devices of a sub-device. This is used to show device topology in Home Assistant. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


