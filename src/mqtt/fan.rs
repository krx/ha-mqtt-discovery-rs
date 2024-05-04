use super::common::{Availability, Device, EntityCategory, Origin};
use serde_derive::Serialize;

use super::common::Qos;

/// ---
/// title: "MQTT Fan"
/// description: "Instructions on how to integrate MQTT fans into Home Assistant."
/// ha_category:
///   - Fan
/// ha_release: 0.27
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` fan platform lets you control your MQTT enabled fans.
///
/// ## Configuration
///
/// In an ideal scenario, the MQTT device will have a `state_topic` to publish state changes. If these messages are published with a `RETAIN` flag, the MQTT fan will receive an instant state update after subscription and will start with the correct state. Otherwise, the initial state of the fan will be `unknown`. A MQTT device can reset the current state to `unknown` using a `None` payload.
///
/// When a `state_topic` is not available, the fan will work in optimistic mode. In this mode, the fan will immediately change state after every command. Otherwise, the fan will wait for state confirmation from the device (message from `state_topic`).  The initial state is set to `False` / `off` in optimistic mode.
///
/// Optimistic mode can be forced even if a `state_topic` is available. Try to enable it if you are experiencing incorrect fan operation.
///
/// To enable MQTT fans in your installation, add the following to your `configuration.yaml` file:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - fan:
///       command_topic: "bedroom_fan/on/set"
/// ```
///
/// {% configuration %}
/// availability:
///   description: A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
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
///       description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's availability from the `topic`. To determine the devices's availability result of this template will be compared to `payload_available` and `payload_not_available`."
///       required: false
///       type: template
/// availability_mode:
///   description: When `availability` is configured, this controls the conditions needed to set the entity to `available`. Valid entries are `all`, `any`, and `latest`. If set to `all`, `payload_available` must be received on all configured availability topics before the entity is marked as online. If set to `any`, `payload_available` must be received on at least one configured availability topic before the entity is marked as online. If set to `latest`, the last `payload_available` or `payload_not_available` received on any configured availability topic controls the availability.
///   required: false
///   type: string
///   default: latest
/// availability_template:
///   description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's availability from the `availability_topic`. To determine the devices's availability result of this template will be compared to `payload_available` and `payload_not_available`."
///   required: false
///   type: template
/// availability_topic:
///   description: The MQTT topic subscribed to receive availability (online/offline) updates. Must not be used together with `availability`.
///   required: false
///   type: string
/// command_template:
///   description: Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `command_topic`.
///   required: false
///   type: template
/// command_topic:
///   description: The MQTT topic to publish commands to change the fan state.
///   required: true
///   type: string
/// device:
///   description: "Information about the device this fan is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device."
///   required: false
///   type: map
///   keys:
///     configuration_url:
///       description: 'A link to the webpage that can manage the configuration of this device. Can be either an `http://`, `https://` or an internal `homeassistant://` URL.'
///       required: false
///       type: string
///     connections:
///       description: 'A list of connections of the device to the outside world as a list of tuples `[connection_type, connection_identifier]`. For example the MAC address of a network interface: `"connections": [["mac", "02:5b:26:a8:dc:12"]]`.'
///       required: false
///       type: list
///     hw_version:
///       description: The hardware version of the device.
///       required: false
///       type: string
///     identifiers:
///       description: A list of IDs that uniquely identify the device. For example a serial number.
///       required: false
///       type: [string, list]
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
///       description: 'Suggest an area if the device isn’t in one yet.'
///       required: false
///       type: string
///     sw_version:
///       description: The firmware version of the device.
///       required: false
///       type: string
///     via_device:
///       description: 'Identifier of a device that routes messages between this device and Home Assistant. Examples of such devices are hubs, or parent devices of a sub-device. This is used to show device topology in Home Assistant.'
///       required: false
///       type: string
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
///   description: The [category](https://developers.home-assistant.io/docs/core/entity#generic-properties) of the entity.
///   required: false
///   type: string
/// icon:
///   description: "[Icon](/docs/configuration/customizing-devices/#icon) for the entity."
///   required: false
///   type: icon
/// json_attributes_template:
///   description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation."
///   required: false
///   type: template
/// json_attributes_topic:
///   description: The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
///   required: false
///   type: string
/// name:
///   description: The name of the fan. Can be set to `null` if only the device name is relevant.
///   required: false
///   type: string
///   default: MQTT Fan
/// object_id:
///   description: Used instead of `name` for automatic generation of `entity_id`
///   required: false
///   type: string
/// optimistic:
///   description: Flag that defines if fan works in optimistic mode
///   required: false
///   type: boolean
///   default: "`true` if no state topic defined, else `false`."
/// direction_command_template:
///   description: Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `direction_command_topic`.
///   required: false
///   type: template
/// direction_command_topic:
///   description: The MQTT topic to publish commands to change the direction state.
///   required: false
///   type: string
/// direction_state_topic:
///   description: The MQTT topic subscribed to receive direction state updates.
///   required: false
///   type: string
/// direction_value_template:
///   description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a value from the direction."
///   required: false
///   type: template
/// oscillation_command_template:
///   description: Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `oscillation_command_topic`.
///   required: false
///   type: template
/// oscillation_command_topic:
///   description: The MQTT topic to publish commands to change the oscillation state.
///   required: false
///   type: string
/// oscillation_state_topic:
///   description: The MQTT topic subscribed to receive oscillation state updates.
///   required: false
///   type: string
/// oscillation_value_template:
///   description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a value from the oscillation."
///   required: false
///   type: template
/// payload_available:
///   description: The payload that represents the available state.
///   required: false
///   type: string
///   default: online
/// payload_not_available:
///   description: The payload that represents the unavailable state.
///   required: false
///   type: string
///   default: offline
/// payload_off:
///   description: The payload that represents the stop state.
///   required: false
///   type: string
///   default: "OFF"
/// payload_on:
///   description: The payload that represents the running state.
///   required: false
///   type: string
///   default: "ON"
/// payload_oscillation_off:
///   description: The payload that represents the oscillation off state.
///   required: false
///   type: string
///   default: oscillate_off
/// payload_oscillation_on:
///   description: The payload that represents the oscillation on state.
///   required: false
///   type: string
///   default: oscillate_on
/// payload_reset_percentage:
///   description: A special payload that resets the `percentage` state attribute to `unknown` when received at the `percentage_state_topic`.
///   required: false
///   type: string
///   default: '"None"'
/// payload_reset_preset_mode:
///   description: A special payload that resets the `preset_mode` state attribute to `unknown` when received at the `preset_mode_state_topic`.
///   required: false
///   type: string
///   default: '"None"'
/// percentage_command_template:
///   description: Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `percentage_command_topic`.
///   required: false
///   type: template
/// percentage_command_topic:
///   description: The MQTT topic to publish commands to change the fan speed state based on a percentage.
///   required: false
///   type: string
/// percentage_state_topic:
///   description: The MQTT topic subscribed to receive fan speed based on percentage.
///   required: false
///   type: string
/// percentage_value_template:
///   description: Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the `percentage` value from the payload received on `percentage_state_topic`.
///   required: false
///   type: template
/// preset_mode_command_template:
///   description: Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `preset_mode_command_topic`.
///   required: false
///   type: template
/// preset_mode_command_topic:
///   description: The MQTT topic to publish commands to change the preset mode.
///   required: false
///   type: string
/// preset_mode_state_topic:
///   description: The MQTT topic subscribed to receive fan speed based on presets.
///   required: false
///   type: string
/// preset_mode_value_template:
///   description: Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the `preset_mode` value from the payload received on `preset_mode_state_topic`.
///   required: false
///   type: template
/// preset_modes:
///   description: List of preset modes this fan is capable of running at. Common examples include `auto`, `smart`, `whoosh`, `eco` and `breeze`.
///   required: false
///   type: [list]
///   default: []
/// qos:
///   description: The maximum QoS level to be used when receiving and publishing messages.
///   required: false
///   type: integer
///   default: 0
/// retain:
///   description: If the published message should have the retain flag on or not.
///   required: false
///   type: boolean
///   default: true
/// speed_range_max:
///   description: The maximum of numeric output range (representing 100 %). The number of speeds within the `speed_range` / `100` will determine the `percentage_step`.
///   required: false
///   type: integer
///   default: 100
/// speed_range_min:
///   description: The minimum of numeric output range (`off` not included, so `speed_range_min` - `1` represents 0 %). The number of speeds within the speed_range / 100 will determine the `percentage_step`.
///   required: false
///   type: integer
///   default: 1
/// state_topic:
///   description: The MQTT topic subscribed to receive state updates.
///   required: false
///   type: string
/// state_value_template:
///   description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a value from the state."
///   required: false
///   type: template
/// unique_id:
///   description: An ID that uniquely identifies this fan. If two fans have the same unique ID, Home Assistant will raise an exception.
///   required: false
///   type: string
/// {% endconfiguration %}
///
/// <div class='note warning'>
///
/// Make sure that your topics match exactly. `some-topic/` and `some-topic` are different topics.
///
/// </div>
///
/// ## Examples
///
/// In this section you find some real-life examples of how to use this fan.
///
/// ### Full configuration
///
/// The example below shows a full configuration for a MQTT fan using percentage and preset modes.
/// There are 10 speeds within the speed range, so  `percentage_step` = 100 / 10 steps = 10.0 %.
///
/// ```yaml
/// # Example using percentage based speeds with preset modes configuration.yaml
/// mqtt:
///   - fan:
///       name: "Bedroom Fan"
///       state_topic: "bedroom_fan/on/state"
///       command_topic: "bedroom_fan/on/set"
///       direction_state_topic: "bedroom_fan/direction/state"
///       direction_command_topic: "bedroom_fan/direction/set"
///       oscillation_state_topic: "bedroom_fan/oscillation/state"
///       oscillation_command_topic: "bedroom_fan/oscillation/set"
///       percentage_state_topic: "bedroom_fan/speed/percentage_state"
///       percentage_command_topic: "bedroom_fan/speed/percentage"
///       preset_mode_state_topic: "bedroom_fan/preset/preset_mode_state"
///       preset_mode_command_topic: "bedroom_fan/preset/preset_mode"
///       preset_modes:
///         -  "auto"
///         -  "smart"
///         -  "whoosh"
///         -  "eco"
///         -  "breeze"
///       qos: 0
///       payload_on: "true"
///       payload_off: "false"
///       payload_oscillation_on: "true"
///       payload_oscillation_off: "false"
///       speed_range_min: 1
///       speed_range_max: 10
/// ```
///
/// ### Configuration using command templates
///
/// This example demonstrates how to use command templates with JSON output.
///
/// {% raw %}
///
/// ```yaml
/// # Example configuration.yaml with command templates
/// mqtt:
///   - fan:
///       name: "Bedroom Fan"
///       command_topic: "bedroom_fan/on/set"
///       command_template: "{ state: '{{ value }}'}"
///       direction_command_template: "{{ iif(value == 'forward', 'fwd', 'rev') }}"
///       direction_value_template: "{{ iif(value == 'fwd', 'forward', 'reverse') }}"
///       oscillation_command_topic: "bedroom_fan/oscillation/set"
///       oscillation_command_template: "{ oscillation: '{{ value }}'}"
///       percentage_command_topic: "bedroom_fan/speed/percentage"
///       percentage_command_template: "{ percentage: '{{ value }}'}"
///       preset_mode_command_topic: "bedroom_fan/preset/preset_mode"
///       preset_mode_command_template: "{ preset_mode: '{{ value }}'}"
///       preset_modes:
///         -  "auto"
///         -  "smart"
///         -  "whoosh"
///         -  "eco"
///         -  "breeze"
/// ```
///
/// {% endraw %}
///
/// This example shows how to configure a fan that doesn't use `forward` and `backward` as directions.
///
/// {% raw %}
///
/// ```yaml
/// # Example configuration.yaml with direction templates
/// mqtt:
///   - fan:
///       name: "Bedroom Fan"
///       direction_command_template: "{{ iif(value == 'forward', 'fwd', 'rev') }}"
///       direction_value_template: "{{ iif(value == 'fwd', 'forward', 'reverse') }}"
/// ```
///
/// {% endraw %}
///
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct Fan {
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

    /// The MQTT topic to publish commands to change the fan state.
    #[serde(rename = "cmd_t")]
    pub command_topic: String,

    /// Flag which defines if the entity should be enabled when first added.
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    /// The encoding of the payloads received and published messages. Set to `""` to disable decoding of incoming payload.
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The name of the fan. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used instead of `name` for automatic generation of `entity_id`
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Flag that defines if fan works in optimistic mode
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `direction_command_topic`.
    #[serde(rename = "dir_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub direction_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the direction state.
    #[serde(rename = "dir_cmd_t", skip_serializing_if = "Option::is_none")]
    pub direction_command_topic: Option<String>,

    /// The MQTT topic subscribed to receive direction state updates.
    #[serde(rename = "dir_stat_t", skip_serializing_if = "Option::is_none")]
    pub direction_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a value from the direction.
    #[serde(rename = "dir_val_tpl", skip_serializing_if = "Option::is_none")]
    pub direction_value_template: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `oscillation_command_topic`.
    #[serde(rename = "osc_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub oscillation_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the oscillation state.
    #[serde(rename = "osc_cmd_t", skip_serializing_if = "Option::is_none")]
    pub oscillation_command_topic: Option<String>,

    /// The MQTT topic subscribed to receive oscillation state updates.
    #[serde(rename = "osc_stat_t", skip_serializing_if = "Option::is_none")]
    pub oscillation_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a value from the oscillation.
    #[serde(rename = "osc_val_tpl", skip_serializing_if = "Option::is_none")]
    pub oscillation_value_template: Option<String>,

    /// The payload that represents the available state.
    #[serde(rename = "pl_avail", skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,

    /// The payload that represents the unavailable state.
    #[serde(rename = "pl_not_avail", skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,

    /// The payload that represents the stop state.
    #[serde(rename = "pl_off", skip_serializing_if = "Option::is_none")]
    pub payload_off: Option<String>,

    /// The payload that represents the running state.
    #[serde(rename = "pl_on", skip_serializing_if = "Option::is_none")]
    pub payload_on: Option<String>,

    /// The payload that represents the oscillation off state.
    #[serde(rename = "pl_osc_off", skip_serializing_if = "Option::is_none")]
    pub payload_oscillation_off: Option<String>,

    /// The payload that represents the oscillation on state.
    #[serde(rename = "pl_osc_on", skip_serializing_if = "Option::is_none")]
    pub payload_oscillation_on: Option<String>,

    /// A special payload that resets the `percentage` state attribute to `unknown` when received at the `percentage_state_topic`.
    #[serde(rename = "pl_rst_pct", skip_serializing_if = "Option::is_none")]
    pub payload_reset_percentage: Option<String>,

    /// A special payload that resets the `preset_mode` state attribute to `unknown` when received at the `preset_mode_state_topic`.
    #[serde(rename = "pl_rst_pr_mode", skip_serializing_if = "Option::is_none")]
    pub payload_reset_preset_mode: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `percentage_command_topic`.
    #[serde(rename = "pct_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub percentage_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the fan speed state based on a percentage.
    #[serde(rename = "pct_cmd_t", skip_serializing_if = "Option::is_none")]
    pub percentage_command_topic: Option<String>,

    /// The MQTT topic subscribed to receive fan speed based on percentage.
    #[serde(rename = "pct_stat_t", skip_serializing_if = "Option::is_none")]
    pub percentage_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the `percentage` value from the payload received on `percentage_state_topic`.
    #[serde(rename = "pct_val_tpl", skip_serializing_if = "Option::is_none")]
    pub percentage_value_template: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `preset_mode_command_topic`.
    #[serde(rename = "pr_mode_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub preset_mode_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the preset mode.
    #[serde(rename = "pr_mode_cmd_t", skip_serializing_if = "Option::is_none")]
    pub preset_mode_command_topic: Option<String>,

    /// The MQTT topic subscribed to receive fan speed based on presets.
    #[serde(rename = "pr_mode_stat_t", skip_serializing_if = "Option::is_none")]
    pub preset_mode_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the `preset_mode` value from the payload received on `preset_mode_state_topic`.
    #[serde(rename = "pr_mode_val_tpl", skip_serializing_if = "Option::is_none")]
    pub preset_mode_value_template: Option<String>,

    /// List of preset modes this fan is capable of running at. Common examples include `auto`, `smart`, `whoosh`, `eco` and `breeze`.
    #[serde(rename = "pr_modes", skip_serializing_if = "Option::is_none")]
    pub preset_modes: Option<Vec<String>>,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// The maximum of numeric output range (representing 100 %). The number of speeds within the `speed_range` / `100` will determine the `percentage_step`.
    #[serde(rename = "spd_rng_max", skip_serializing_if = "Option::is_none")]
    pub speed_range_max: Option<i32>,

    /// The minimum of numeric output range (`off` not included, so `speed_range_min` - `1` represents 0 %). The number of speeds within the speed_range / 100 will determine the `percentage_step`.
    #[serde(rename = "spd_rng_min", skip_serializing_if = "Option::is_none")]
    pub speed_range_min: Option<i32>,

    /// The MQTT topic subscribed to receive state updates.
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a value from the state.
    #[serde(rename = "stat_val_tpl", skip_serializing_if = "Option::is_none")]
    pub state_value_template: Option<String>,

    /// An ID that uniquely identifies this fan. If two fans have the same unique ID, Home Assistant will raise an exception.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl Fan {
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

    /// The MQTT topic to publish commands to change the fan state.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = command_topic.into();
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

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    pub fn json_attributes_template<T: Into<String>>(
        mut self,
        json_attributes_template: T,
    ) -> Self {
        self.json_attributes_template = Some(json_attributes_template.into());
        self
    }

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    pub fn json_attributes_topic<T: Into<String>>(mut self, json_attributes_topic: T) -> Self {
        self.json_attributes_topic = Some(json_attributes_topic.into());
        self
    }

    /// The name of the fan. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used instead of `name` for automatic generation of `entity_id`
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// Flag that defines if fan works in optimistic mode
    pub fn optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = Some(optimistic);
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `direction_command_topic`.
    pub fn direction_command_template<T: Into<String>>(
        mut self,
        direction_command_template: T,
    ) -> Self {
        self.direction_command_template = Some(direction_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the direction state.
    pub fn direction_command_topic<T: Into<String>>(mut self, direction_command_topic: T) -> Self {
        self.direction_command_topic = Some(direction_command_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive direction state updates.
    pub fn direction_state_topic<T: Into<String>>(mut self, direction_state_topic: T) -> Self {
        self.direction_state_topic = Some(direction_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a value from the direction.
    pub fn direction_value_template<T: Into<String>>(
        mut self,
        direction_value_template: T,
    ) -> Self {
        self.direction_value_template = Some(direction_value_template.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `oscillation_command_topic`.
    pub fn oscillation_command_template<T: Into<String>>(
        mut self,
        oscillation_command_template: T,
    ) -> Self {
        self.oscillation_command_template = Some(oscillation_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the oscillation state.
    pub fn oscillation_command_topic<T: Into<String>>(
        mut self,
        oscillation_command_topic: T,
    ) -> Self {
        self.oscillation_command_topic = Some(oscillation_command_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive oscillation state updates.
    pub fn oscillation_state_topic<T: Into<String>>(mut self, oscillation_state_topic: T) -> Self {
        self.oscillation_state_topic = Some(oscillation_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a value from the oscillation.
    pub fn oscillation_value_template<T: Into<String>>(
        mut self,
        oscillation_value_template: T,
    ) -> Self {
        self.oscillation_value_template = Some(oscillation_value_template.into());
        self
    }

    /// The payload that represents the available state.
    pub fn payload_available<T: Into<String>>(mut self, payload_available: T) -> Self {
        self.payload_available = Some(payload_available.into());
        self
    }

    /// The payload that represents the unavailable state.
    pub fn payload_not_available<T: Into<String>>(mut self, payload_not_available: T) -> Self {
        self.payload_not_available = Some(payload_not_available.into());
        self
    }

    /// The payload that represents the stop state.
    pub fn payload_off<T: Into<String>>(mut self, payload_off: T) -> Self {
        self.payload_off = Some(payload_off.into());
        self
    }

    /// The payload that represents the running state.
    pub fn payload_on<T: Into<String>>(mut self, payload_on: T) -> Self {
        self.payload_on = Some(payload_on.into());
        self
    }

    /// The payload that represents the oscillation off state.
    pub fn payload_oscillation_off<T: Into<String>>(mut self, payload_oscillation_off: T) -> Self {
        self.payload_oscillation_off = Some(payload_oscillation_off.into());
        self
    }

    /// The payload that represents the oscillation on state.
    pub fn payload_oscillation_on<T: Into<String>>(mut self, payload_oscillation_on: T) -> Self {
        self.payload_oscillation_on = Some(payload_oscillation_on.into());
        self
    }

    /// A special payload that resets the `percentage` state attribute to `unknown` when received at the `percentage_state_topic`.
    pub fn payload_reset_percentage<T: Into<String>>(
        mut self,
        payload_reset_percentage: T,
    ) -> Self {
        self.payload_reset_percentage = Some(payload_reset_percentage.into());
        self
    }

    /// A special payload that resets the `preset_mode` state attribute to `unknown` when received at the `preset_mode_state_topic`.
    pub fn payload_reset_preset_mode<T: Into<String>>(
        mut self,
        payload_reset_preset_mode: T,
    ) -> Self {
        self.payload_reset_preset_mode = Some(payload_reset_preset_mode.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `percentage_command_topic`.
    pub fn percentage_command_template<T: Into<String>>(
        mut self,
        percentage_command_template: T,
    ) -> Self {
        self.percentage_command_template = Some(percentage_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the fan speed state based on a percentage.
    pub fn percentage_command_topic<T: Into<String>>(
        mut self,
        percentage_command_topic: T,
    ) -> Self {
        self.percentage_command_topic = Some(percentage_command_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive fan speed based on percentage.
    pub fn percentage_state_topic<T: Into<String>>(mut self, percentage_state_topic: T) -> Self {
        self.percentage_state_topic = Some(percentage_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the `percentage` value from the payload received on `percentage_state_topic`.
    pub fn percentage_value_template<T: Into<String>>(
        mut self,
        percentage_value_template: T,
    ) -> Self {
        self.percentage_value_template = Some(percentage_value_template.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `preset_mode_command_topic`.
    pub fn preset_mode_command_template<T: Into<String>>(
        mut self,
        preset_mode_command_template: T,
    ) -> Self {
        self.preset_mode_command_template = Some(preset_mode_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the preset mode.
    pub fn preset_mode_command_topic<T: Into<String>>(
        mut self,
        preset_mode_command_topic: T,
    ) -> Self {
        self.preset_mode_command_topic = Some(preset_mode_command_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive fan speed based on presets.
    pub fn preset_mode_state_topic<T: Into<String>>(mut self, preset_mode_state_topic: T) -> Self {
        self.preset_mode_state_topic = Some(preset_mode_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the `preset_mode` value from the payload received on `preset_mode_state_topic`.
    pub fn preset_mode_value_template<T: Into<String>>(
        mut self,
        preset_mode_value_template: T,
    ) -> Self {
        self.preset_mode_value_template = Some(preset_mode_value_template.into());
        self
    }

    /// List of preset modes this fan is capable of running at. Common examples include `auto`, `smart`, `whoosh`, `eco` and `breeze`.
    pub fn preset_modes(mut self, preset_modes: Vec<String>) -> Self {
        self.preset_modes = Some(preset_modes);
        self
    }

    /// The maximum QoS level to be used when receiving and publishing messages.
    pub fn qos(mut self, qos: Qos) -> Self {
        self.qos = Some(qos);
        self
    }

    /// If the published message should have the retain flag on or not.
    pub fn retain(mut self, retain: bool) -> Self {
        self.retain = Some(retain);
        self
    }

    /// The maximum of numeric output range (representing 100 %). The number of speeds within the `speed_range` / `100` will determine the `percentage_step`.
    pub fn speed_range_max(mut self, speed_range_max: i32) -> Self {
        self.speed_range_max = Some(speed_range_max);
        self
    }

    /// The minimum of numeric output range (`off` not included, so `speed_range_min` - `1` represents 0 %). The number of speeds within the speed_range / 100 will determine the `percentage_step`.
    pub fn speed_range_min(mut self, speed_range_min: i32) -> Self {
        self.speed_range_min = Some(speed_range_min);
        self
    }

    /// The MQTT topic subscribed to receive state updates.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a value from the state.
    pub fn state_value_template<T: Into<String>>(mut self, state_value_template: T) -> Self {
        self.state_value_template = Some(state_value_template.into());
        self
    }

    /// An ID that uniquely identifies this fan. If two fans have the same unique ID, Home Assistant will raise an exception.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }
}
