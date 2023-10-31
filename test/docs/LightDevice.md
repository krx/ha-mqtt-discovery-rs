# LightDevice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**configuration_url** | Option<**String**> | A link to the webpage that can manage the configuration of this device. Can be either an `http://`, `https://` or an internal `homeassistant://` URL. | [optional]
**connections** | Option<**Vec<String>**> | A list of connections of the device to the outside world as a list of tuples `[connection_type, connection_identifier]`. For example the MAC address of a network interface: `\"connections\": [[\"mac\", \"02:5b:26:a8:dc:12\"]]`. | [optional]
**identifiers** | Option<[**crate::models::ButtonDeviceIdentifiers**](Button_device_identifiers.md)> |  | [optional]
**manufacturer** | Option<**String**> | The manufacturer of the device. | [optional]
**model** | Option<**String**> | The model of the device. | [optional]
**name** | Option<**String**> | The name of the device. | [optional]
**sw_version** | Option<**String**> | The firmware version of the device. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


