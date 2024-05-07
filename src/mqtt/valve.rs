use super::common::Qos;
use super::common::{Availability, Device, EntityCategory, Origin};
use crate::Entity;
use serde_derive::Serialize;

/// ---
/// title: "MQTT Valve"
/// description: "Instructions on how to integrate MQTT valves into Home Assistant."
/// ha_category:
///   - Valve
/// ha_iot_class: Configurable
/// ha_release: 2024.1
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` valve platform allows you to control an MQTT valve (such a gas or water valve).
///
/// ## Configuration
///
/// A valve entity can be have the following states: `open`, `opening`, `closed` or `closing`.
///
/// ### Valve controlled by states
///
/// If a `state_topic` is configured, the entity's state will be updated only after an MQTT message is received on `state_topic` matching `state_open`, `state_opening`, `state_closed` or `state_closing`. Commands configured through `payload_open`, `payload_closed`, and `payload_stop` will be published to `command_topic` to control the valve.
///
/// To use your MQTT valve in your installation, add the following to your `configuration.yaml` file:
///
/// ```yaml
/// # Example configuration.yaml entry for a value that is set by open or close command
/// mqtt:
///   - valve:
///       command_topic: "home-assistant/valve/set"
///       state_topic: "home-assistant/valve/state"
/// ```
///
/// ### Valve controlled by position
///
/// If the valve supports reporting its position (the `reports_position` config option is set to `true`), a numeric state is expected on `state_topic`, but state updates are still allowed for `state_opening` and `state_closing`. Also, a JSON format is supported. It allows both `state` and `position` to be reported together.
///
/// Example of a JSON state update:
///
/// ```json
/// {"state": "opening", "position": 10}
/// ```
///
/// The wanted position value or `payload_stop` will be published to `command_topic` to control the valve when the services `valve.open`, `value.close`, or `value.set_position` are called.
///
/// To use your MQTT valve in your installation, add the following to your `configuration.yaml` file:
///
/// ```yaml
/// # Example configuration.yaml entry for a valve that reports position
/// mqtt:
///   - valve:
///       command_topic: "home-assistant/valve/set"
///       state_topic: "home-assistant/valve/state"
///       reports_position: true
/// ```
///
/// ### Optimistic operation
///
/// If a `state_topic` is not defined, the valve will work in optimistic mode. In this mode, the valve will immediately change state (`open` or `closed`) after every command sent by Home Assistant. It won't wait for an update from the device. Optimistic mode can be forced by setting `optimistic` to `true`, even if a `state_topic` is defined. Try to enable it if you are experiencing an incorrect valve operation.
///
///
/// {% configuration %}
/// availability:
///   description: "A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`."
///   required: false
///   type: list
///   keys:
///     payload_available:
///       description: The payload that represents the available state.
///       required: false
///       type: string
///       default: online
///     payload_not_available:
///       description: The payload that represents the unavailable state.
///       required: false
///       type: string
///       default: offline
///     topic:
///       description: An MQTT topic subscribed to receive availability (online/offline) updates.
///       required: true
///       type: string
///     value_template:
///       description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the device's availability from the `topic`. To determine the devices's availability, the result of this template will be compared to `payload_available` and `payload_not_available`."
///       required: false
///       type: template
/// availability_mode:
///   description: When `availability` is configured, this controls the conditions needed to set the entity to `available`. Valid entries are `all`, `any`, and `latest`. If set to `all`, `payload_available` must be received on all configured availability topics before the entity is marked as online. If set to `any`, `payload_available` must be received on at least one configured availability topic before the entity is marked as online. If set to `latest`, the last `payload_available` or `payload_not_available` received on any configured availability topic controls the availability.
///   required: false
///   type: string
///   default: latest
/// availability_template:
///   description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the device's availability from the `availability_topic`. To determine the devices's availability, the result of this template will be compared to `payload_available` and `payload_not_available`."
///   required: false
///   type: template
/// availability_topic:
///   description: "The MQTT topic subscribed to receive birth and LWT messages from the MQTT valve device. If an `availability` topic is not defined, the valve availability state will always be `available`. If an `availability` topic is defined, the valve availability state will be `unavailable` by default. Must not be used together with `availability`."
///   required: false
///   type: string
/// command_template:
///   description: Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `command_topic`.
///   required: false
///   type: template
/// command_topic:
///   description: The MQTT topic to publish commands to control the valve. The value sent can be a value defined by `payload_open`, `payload_close` or `payload_stop`. If `reports_position` is set to `true`, a numeric value will be published instead.
///   required: false
///   type: string
/// device:
///   description: "Information about the device this valve is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of the identifiers or connections must be present to identify the device."
///   required: false
///   type: map
///   keys:
///     configuration_url:
///       description: "A link to the webpage that can manage the configuration of this device. Can be either an `http://`, `https://` or an internal `homeassistant://` URL."
///       required: false
///       type: string
///     connections:
///       description: 'A list of connections of the device to the outside world as a list of tuples `[connection_type, connection_identifier]`. For example, the MAC address of a network interface: `"connections": [["mac", "02:5b:26:a8:dc:12"]]`.'
///       required: false
///       type: list
///     hw_version:
///       description: The hardware version of the device.
///       required: false
///       type: string
///     identifiers:
///       description: A list of IDs that uniquely identify the device. For example, a serial number.
///       required: false
///       type: [list, string]
///     manufacturer:
///       description: The manufacturer of the device.
///       required: false
///       type: string
///     model:
///       description: The model of the device.
///       required: false
///       type: string
///     name:
///       description: The name of the device.
///       required: false
///       type: string
///     serial_number:
///       description: "The serial number of the device."
///       required: false
///       type: string
///     suggested_area:
///       description: Suggest an area if the device isn’t in one yet.
///       required: false
///       type: string
///     sw_version:
///       description: The firmware version of the device.
///       required: false
///       type: string
///     via_device:
///       description: Identifier of a device that routes messages between this device and Home Assistant. Examples of such devices are hubs, or parent devices of a sub-device. This is used to show device topology in Home Assistant.
///       required: false
///       type: string
/// device_class:
///   description: Sets the [class of the device](/integrations/valve/), changing the device state and icon that is displayed on the frontend. The `device_class` can be `null`.
///   required: false
///   type: string
/// enabled_by_default:
///   description: Flag which defines if the entity should be enabled when first added.
///   required: false
///   type: boolean
///   default: true
/// encoding:
///   description: The encoding of the payloads received and published messages. Set to `""` to disable decoding of incoming payload.
///   required: false
///   type: string
///   default: "utf-8"
/// entity_category:
///   description: "The [category](https://developers.home-assistant.io/docs/core/entity#generic-properties) of the entity."
///   required: false
///   type: string
/// icon:
///   description: "[Icon](/docs/configuration/customizing-devices/#icon) for the entity."
///   required: false
///   type: icon
/// json_attributes_template:
///   description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. A usage example can be found in the [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation."
///   required: false
///   type: template
/// json_attributes_topic:
///   description: The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. A usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
///   required: false
///   type: string
/// name:
///   description: The name of the valve. Can be set to `null` if only the device name is relevant.
///   required: false
///   type: string
///   default: MQTT valve
/// object_id:
///   description: Used instead of `name` to have the `entity_id` generated automatically.
///   required: false
///   type: string
/// optimistic:
///   description: Flag that defines if a switch works in optimistic mode.
///   required: false
///   type: boolean
///   default: "`false` if the state or position topic is defined; `true` otherwise."
/// payload_available:
///   description: The payload that represents the online state.
///   required: false
///   type: string
///   default: online
/// payload_close:
///   description: The command payload that closes the valve. Is only used when `reports_position` is set to `false` (default). The `payload_close` is not allowed if `reports_position` is set to `true`. Can be set to `null` to disable the valve's close option.
///   required: false
///   type: string
///   default: CLOSE
/// payload_not_available:
///   description: The payload that represents the offline state.
///   required: false
///   type: string
///   default: offline
/// payload_open:
///   description: The command payload that opens the valve. Is only used when `reports_position` is set to `false` (default). The `payload_open` is not allowed if `reports_position` is set to `true`. Can be set to `null` to disable the valve's open option.
///   required: false
///   type: string
///   default: OPEN
/// payload_stop:
///   description: The command payload that stops the valve. When not configured, the valve will not support the `valve.stop` service.
///   required: false
///   type: string
/// position_closed:
///   description: Number which represents closed position. The valve's position will be scaled to the(`position_closed`...`position_open`) range when a service is called and scaled back when a value is received.
///   required: false
///   type: integer
///   default: 0
/// position_open:
///   description: Number which represents open position. The valve's position will be scaled to (`position_closed`...`position_open`) range when a service is called and scaled back when a value is received.
///   required: false
///   type: integer
///   default: 100
/// qos:
///   description: The maximum QoS level to be used when receiving and publishing messages.
///   required: false
///   type: integer
///   default: 0
/// reports_position:
///   description: "Set to `true` if the value reports the position or supports setting the position. Enabling the `reports_position` option will cause the position to be published instead of a payload defined by `payload_open`, `payload_close` or `payload_stop`. When receiving messages, `state_topic` will accept numeric payloads or one of the following state messages: `open`, `opening`, `closed`, or `closing`."
///   required: false
///   type: boolean
///   default: false
/// retain:
///   description: Defines if published messages should have the retain flag set.
///   required: false
///   type: boolean
///   default: false
/// state_closed:
///   description: The payload that represents the closed state. Is only allowed when `reports_position` is set to `False` (default).
///   required: false
///   type: string
///   default: closed
/// state_closing:
///   description: The payload that represents the closing state.
///   required: false
///   type: string
///   default: closing
/// state_open:
///   description: The payload that represents the open state. Is only allowed when `reports_position` is set to `False` (default).
///   required: false
///   type: string
///   default: open
/// state_opening:
///   description: The payload that represents the opening state.
///   required: false
///   type: string
///   default: opening
/// state_topic:
///   description: The MQTT topic subscribed to receive valve state messages. State topic accepts a state payload (`open`, `opening`, `closed`, or `closing`) or, if `reports_position` is supported, a numeric value representing the position. In a JSON format with variables `state` and `position` both values can received together.
///   required: false
///   type: string
/// unique_id:
///   description: An ID that uniquely identifies this valve. If two valves have the same unique ID, Home Assistant will raise an exception.
///   required: false
///   type: string
/// value_template:
///   description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) that can be used to extract the payload for the `state_topic` topic. The rendered value should be a defined state payload or, if reporting a `position` is supported and `reports_position` is set to `true`, a numeric value is expected representing the position. See also `state_topic`."
///   required: false
///   type: template
/// {% endconfiguration %}
///
/// <div class="note">
///
/// MQTT valve expects position values to be in the range of 0 to 100, where 0 indicates a closed position and 100 indicates a fully open position.
/// If `position_open` or `position_closed` are set to a different range (for example, 40 to 140), when sending a command to the device, the range will be adjusted to the device range. For example, position 0 will send a value of 40 to device. When the device receives a position payload, it will be adjusted back to the 0 to 100 range. In our example, the device value of 40 will report valve position 0.
/// `position_open` and `position_closed` can also be used to reverse the direction of the device: If `position_closed` is set to 100 and `position_open` is set to `0`, the device operation will be inverted. For example, when setting the position to 40, a value of 60 will be sent to the device.
///
/// </div>
///
/// ## Examples
///
/// This section provides some examples showing how you can use this platform.
///
/// ### Full configuration for a value that does not report position
///
/// The example below shows a full configuration for a valve that does not report position.
///
/// {% raw %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - valve:
///       name: "MQTT valve"
///       command_template: '{"x": {{ value }} }'
///       command_topic: "home-assistant/valve/set"
///       state_topic: "home-assistant/valve/state"
///       availability:
///         - topic: "home-assistant/valve/availability"
///       qos: 0
///       reports_position: false
///       retain: true
///       payload_open: "OPEN"
///       payload_close: "CLOSE"
///       payload_stop: "STOP"
///       state_open: "open"
///       state_opening: "opening"
///       state_closed: "closed"
///       state_closing: "closing"
///       payload_available: "online"
///       payload_not_available: "offline"
///       optimistic: false
///       value_template: "{{ value_json.x }}"
/// ```
///
/// {% endraw %}
///
/// ### Sample configuration of a valve that reports the position
///
/// The example below shows a sample configuration for a valve that reports the position using JSON messages.
///
/// {% raw %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - valve:
///       name: "MQTT valve"
///       command_template: '{"x": {{ value }} }'
///       command_topic: "home-assistant/valve/set"
///       state_topic: "home-assistant/valve/state"
///       availability:
///         - topic: "home-assistant/valve/availability"
///       reports_position: true
///       value_template: "{{ value_json.x }}"
/// ```
///
/// {% endraw %}
///
/// ### Configuration for disabling valve commands
///
/// The example below shows a configuration for a valve that does not have a close command.
/// Setting the `payload_close` to empty or to `null` disables the close command and will not show the close button.
///
/// {% raw %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - valve:
///       payload_open: "on"
///       payload_close:
///       payload_stop: "on"
/// ```
///
/// {% endraw %}
///
/// An MQTT valve will support `open` and `close` commands if a `command_topic` is set. The MQTT valve supports `stop` if `payload_stop` is set.
///
/// ### Testing your configuration
///
/// To test, you can use the command line tool `mosquitto_pub` shipped with `mosquitto` or the `mosquitto-clients` package to send MQTT messages. This allows you to operate your valve manually:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t home-assistant/valve/set -m "CLOSE"
/// ```
///
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct Valve {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: Device,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `command_topic`.
    #[serde(rename = "cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,

    /// The MQTT topic to publish commands to control the valve. The value sent can be a value defined by `payload_open`, `payload_close` or `payload_stop`. If `reports_position` is set to `true`, a numeric value will be published instead.
    #[serde(rename = "cmd_t", skip_serializing_if = "Option::is_none")]
    pub command_topic: Option<String>,

    /// Sets the [class of the device](/integrations/valve/), changing the device state and icon that is displayed on the frontend. The `device_class` can be `null`.
    #[serde(rename = "dev_cla", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<String>,

    /// Flag which defines if the entity should be enabled when first added.
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    /// The encoding of the payloads received and published messages. Set to `""` to disable decoding of incoming payload.
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. A usage example can be found in the [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. A usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The name of the valve. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used instead of `name` to have the `entity_id` generated automatically.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Flag that defines if a switch works in optimistic mode.
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    /// The command payload that closes the valve. Is only used when `reports_position` is set to `false` (default). The `payload_close` is not allowed if `reports_position` is set to `true`. Can be set to `null` to disable the valve's close option.
    #[serde(rename = "pl_cls", skip_serializing_if = "Option::is_none")]
    pub payload_close: Option<String>,

    /// The command payload that opens the valve. Is only used when `reports_position` is set to `false` (default). The `payload_open` is not allowed if `reports_position` is set to `true`. Can be set to `null` to disable the valve's open option.
    #[serde(rename = "pl_open", skip_serializing_if = "Option::is_none")]
    pub payload_open: Option<String>,

    /// The command payload that stops the valve. When not configured, the valve will not support the `valve.stop` service.
    #[serde(rename = "pl_stop", skip_serializing_if = "Option::is_none")]
    pub payload_stop: Option<String>,

    /// Number which represents closed position. The valve's position will be scaled to the(`position_closed`...`position_open`) range when a service is called and scaled back when a value is received.
    #[serde(rename = "pos_clsd", skip_serializing_if = "Option::is_none")]
    pub position_closed: Option<i32>,

    /// Number which represents open position. The valve's position will be scaled to (`position_closed`...`position_open`) range when a service is called and scaled back when a value is received.
    #[serde(rename = "pos_open", skip_serializing_if = "Option::is_none")]
    pub position_open: Option<i32>,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// Set to `true` if the value reports the position or supports setting the position. Enabling the `reports_position` option will cause the position to be published instead of a payload defined by `payload_open`, `payload_close` or `payload_stop`. When receiving messages, `state_topic` will accept numeric payloads or one of the following state messages: `open`, `opening`, `closed`, or `closing`.
    #[serde(rename = "pos", skip_serializing_if = "Option::is_none")]
    pub reports_position: Option<bool>,

    /// Defines if published messages should have the retain flag set.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// The payload that represents the closed state. Is only allowed when `reports_position` is set to `False` (default).
    #[serde(rename = "stat_clsd", skip_serializing_if = "Option::is_none")]
    pub state_closed: Option<String>,

    /// The payload that represents the closing state.
    #[serde(rename = "stat_closing", skip_serializing_if = "Option::is_none")]
    pub state_closing: Option<String>,

    /// The payload that represents the open state. Is only allowed when `reports_position` is set to `False` (default).
    #[serde(rename = "stat_open", skip_serializing_if = "Option::is_none")]
    pub state_open: Option<String>,

    /// The payload that represents the opening state.
    #[serde(rename = "stat_opening", skip_serializing_if = "Option::is_none")]
    pub state_opening: Option<String>,

    /// The MQTT topic subscribed to receive valve state messages. State topic accepts a state payload (`open`, `opening`, `closed`, or `closing`) or, if `reports_position` is supported, a numeric value representing the position. In a JSON format with variables `state` and `position` both values can received together.
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// An ID that uniquely identifies this valve. If two valves have the same unique ID, Home Assistant will raise an exception.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) that can be used to extract the payload for the `state_topic` topic. The rendered value should be a defined state payload or, if reporting a `position` is supported and `reports_position` is set to `true`, a numeric value is expected representing the position. See also `state_topic`.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Valve {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    pub fn topic_prefix<S: Into<String>>(mut self, topic_prefix: S) -> Self {
        self.topic_prefix = Some(topic_prefix.into());
        self
    }

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    pub fn origin(mut self, origin: Origin) -> Self {
        self.origin = origin;
        self
    }

    /// Information about the device this sensor is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/device_registry_index/). Only works when `unique_id` is set. At least one of identifiers or connections must be present to identify the device.
    pub fn device(mut self, device: Device) -> Self {
        self.device = device;
        self
    }

    /// The category of the entity. (optional, default: None)
    pub fn entity_category(mut self, entity_category: EntityCategory) -> Self {
        self.entity_category = Some(entity_category);
        self
    }

    /// Defines how HA will check for entity availability.
    pub fn availability(mut self, availability: Availability) -> Self {
        self.availability = availability;
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `command_topic`.
    pub fn command_template<T: Into<String>>(mut self, command_template: T) -> Self {
        self.command_template = Some(command_template.into());
        self
    }

    /// The MQTT topic to publish commands to control the valve. The value sent can be a value defined by `payload_open`, `payload_close` or `payload_stop`. If `reports_position` is set to `true`, a numeric value will be published instead.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = Some(command_topic.into());
        self
    }

    /// Sets the [class of the device](/integrations/valve/), changing the device state and icon that is displayed on the frontend. The `device_class` can be `null`.
    pub fn device_class<T: Into<String>>(mut self, device_class: T) -> Self {
        self.device_class = Some(device_class.into());
        self
    }

    /// Flag which defines if the entity should be enabled when first added.
    pub fn enabled_by_default(mut self, enabled_by_default: bool) -> Self {
        self.enabled_by_default = Some(enabled_by_default);
        self
    }

    /// The encoding of the payloads received and published messages. Set to `""` to disable decoding of incoming payload.
    pub fn encoding<T: Into<String>>(mut self, encoding: T) -> Self {
        self.encoding = Some(encoding.into());
        self
    }

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    pub fn icon<T: Into<String>>(mut self, icon: T) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. A usage example can be found in the [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    pub fn json_attributes_template<T: Into<String>>(
        mut self,
        json_attributes_template: T,
    ) -> Self {
        self.json_attributes_template = Some(json_attributes_template.into());
        self
    }

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. A usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    pub fn json_attributes_topic<T: Into<String>>(mut self, json_attributes_topic: T) -> Self {
        self.json_attributes_topic = Some(json_attributes_topic.into());
        self
    }

    /// The name of the valve. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used instead of `name` to have the `entity_id` generated automatically.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// Flag that defines if a switch works in optimistic mode.
    pub fn optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = Some(optimistic);
        self
    }

    /// The command payload that closes the valve. Is only used when `reports_position` is set to `false` (default). The `payload_close` is not allowed if `reports_position` is set to `true`. Can be set to `null` to disable the valve's close option.
    pub fn payload_close<T: Into<String>>(mut self, payload_close: T) -> Self {
        self.payload_close = Some(payload_close.into());
        self
    }

    /// The command payload that opens the valve. Is only used when `reports_position` is set to `false` (default). The `payload_open` is not allowed if `reports_position` is set to `true`. Can be set to `null` to disable the valve's open option.
    pub fn payload_open<T: Into<String>>(mut self, payload_open: T) -> Self {
        self.payload_open = Some(payload_open.into());
        self
    }

    /// The command payload that stops the valve. When not configured, the valve will not support the `valve.stop` service.
    pub fn payload_stop<T: Into<String>>(mut self, payload_stop: T) -> Self {
        self.payload_stop = Some(payload_stop.into());
        self
    }

    /// Number which represents closed position. The valve's position will be scaled to the(`position_closed`...`position_open`) range when a service is called and scaled back when a value is received.
    pub fn position_closed(mut self, position_closed: i32) -> Self {
        self.position_closed = Some(position_closed);
        self
    }

    /// Number which represents open position. The valve's position will be scaled to (`position_closed`...`position_open`) range when a service is called and scaled back when a value is received.
    pub fn position_open(mut self, position_open: i32) -> Self {
        self.position_open = Some(position_open);
        self
    }

    /// The maximum QoS level to be used when receiving and publishing messages.
    pub fn qos(mut self, qos: Qos) -> Self {
        self.qos = Some(qos);
        self
    }

    /// Set to `true` if the value reports the position or supports setting the position. Enabling the `reports_position` option will cause the position to be published instead of a payload defined by `payload_open`, `payload_close` or `payload_stop`. When receiving messages, `state_topic` will accept numeric payloads or one of the following state messages: `open`, `opening`, `closed`, or `closing`.
    pub fn reports_position(mut self, reports_position: bool) -> Self {
        self.reports_position = Some(reports_position);
        self
    }

    /// Defines if published messages should have the retain flag set.
    pub fn retain(mut self, retain: bool) -> Self {
        self.retain = Some(retain);
        self
    }

    /// The payload that represents the closed state. Is only allowed when `reports_position` is set to `False` (default).
    pub fn state_closed<T: Into<String>>(mut self, state_closed: T) -> Self {
        self.state_closed = Some(state_closed.into());
        self
    }

    /// The payload that represents the closing state.
    pub fn state_closing<T: Into<String>>(mut self, state_closing: T) -> Self {
        self.state_closing = Some(state_closing.into());
        self
    }

    /// The payload that represents the open state. Is only allowed when `reports_position` is set to `False` (default).
    pub fn state_open<T: Into<String>>(mut self, state_open: T) -> Self {
        self.state_open = Some(state_open.into());
        self
    }

    /// The payload that represents the opening state.
    pub fn state_opening<T: Into<String>>(mut self, state_opening: T) -> Self {
        self.state_opening = Some(state_opening.into());
        self
    }

    /// The MQTT topic subscribed to receive valve state messages. State topic accepts a state payload (`open`, `opening`, `closed`, or `closing`) or, if `reports_position` is supported, a numeric value representing the position. In a JSON format with variables `state` and `position` both values can received together.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// An ID that uniquely identifies this valve. If two valves have the same unique ID, Home Assistant will raise an exception.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) that can be used to extract the payload for the `state_topic` topic. The rendered value should be a defined state payload or, if reporting a `position` is supported and `reports_position` is set to `true`, a numeric value is expected representing the position. See also `state_topic`.
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl Into<Entity> for Valve {
    fn into(self) -> Entity {
        Entity::Valve(self)
    }
}
