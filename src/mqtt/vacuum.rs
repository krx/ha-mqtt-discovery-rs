use super::common::{Availability, Device, EntityCategory, Origin};
use serde_derive::Serialize;

use super::common::Qos;

/// ---
/// title: "MQTT Vacuum"
/// description: "Instructions on how to integrate your MQTT enabled Vacuum within Home Assistant."
/// ha_category:
///   - Vacuum
/// ha_release: 0.54
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` vacuum {% term integration %} allows you to control your MQTT-enabled vacuum.
/// The initial state of the MQTT vacuum {% term entity %} will set to `unknown` and can be reset by a device by sending a `null` payload as state.
///
/// ## Configuration
///
/// MQTT vacuum configuration section.
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
/// command_topic:
///   description: The MQTT topic to publish commands to control the vacuum.
///   required: false
///   type: string
/// device:
///   description: "Information about the device this switch is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device."
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
/// encoding:
///   description: The encoding of the payloads received and published messages. Set to `""` to disable decoding of incoming payload.
///   required: false
///   type: string
///   default: "utf-8"
/// fan_speed_list:
///   description: List of possible fan speeds for the vacuum.
///   required: false
///   type: [string, list]
/// json_attributes_template:
///   description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation."
///   required: false
///   type: template
/// json_attributes_topic:
///   description: The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
///   required: false
///   type: string
/// name:
///   description: The name of the vacuum. Can be set to `null` if only the device name is relevant.
///   required: false
///   type: string
///   default: MQTT Vacuum
/// object_id:
///   description: Used instead of `name` for automatic generation of `entity_id`
///   required: false
///   type: string
/// payload_available:
///   description: The payload that represents the available state.
///   required: false
///   type: string
///   default: online
/// payload_clean_spot:
///   description: The payload to send to the `command_topic` to begin a spot cleaning cycle.
///   required: false
///   type: string
///   default: clean_spot
/// payload_locate:
///   description: The payload to send to the `command_topic` to locate the vacuum (typically plays a song).
///   required: false
///   type: string
///   default: locate
/// payload_not_available:
///   description: The payload that represents the unavailable state.
///   required: false
///   type: string
///   default: offline
/// payload_pause:
///   description: The payload to send to the `command_topic` to pause the vacuum.
///   required: false
///   type: string
///   default: pause
/// payload_return_to_base:
///   description: The payload to send to the `command_topic` to tell the vacuum to return to base.
///   required: false
///   type: string
///   default: return_to_base
/// payload_start:
///   description: "The payload to send to the `command_topic` to begin the cleaning cycle."
///   required: false
///   type: string
///   default: start
/// payload_stop:
///   description: "The payload to send to the `command_topic` to stop cleaning."
///   required: false
///   type: string
///   default: stop
/// qos:
///   description: The maximum QoS level to be used when receiving and publishing messages.
///   required: false
///   type: integer
///   default: 0
/// retain:
///   description: If the published message should have the retain flag on or not.
///   required: false
///   type: boolean
///   default: false
/// send_command_topic:
///   description: The MQTT topic to publish custom commands to the vacuum.
///   required: false
///   type: string
/// set_fan_speed_topic:
///   description: The MQTT topic to publish commands to control the vacuum's fan speed.
///   required: false
///   type: string
/// state_topic:
///   description: "The MQTT topic subscribed to receive state messages from the vacuum. Messages received on the `state_topic` must be a valid JSON dictionary, with a mandatory `state` key and optionally `battery_level` and `fan_speed` keys as shown in the [example](#configuration-example)."
///   required: false
///   type: string
/// supported_features:
///   description: "List of features that the vacuum supports (possible values are `start`, `stop`, `pause`, `return_home`, `battery`, `status`, `locate`, `clean_spot`, `fan_speed`, `send_command`)."
///   required: false
///   type: [string, list]
///   default: "`start`, `stop`, `return_home`, `status`, `battery`, `clean_spot`"
/// unique_id:
///    description: An ID that uniquely identifies this vacuum. If two vacuums have the same unique ID, Home Assistant will raise an exception.
///    required: false
///    type: string
/// {% endconfiguration %}
///
/// ## Configuration example
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - vacuum:
///       name: "MQTT Vacuum"
///       schema: state
///       supported_features:
///         - start
///         - pause
///         - stop
///         - return_home
///         - battery
///         - status
///         - locate
///         - clean_spot
///         - fan_speed
///         - send_command
///       command_topic: "vacuum/command"
///       state_topic: "vacuum/state"
///       set_fan_speed_topic: "vacuum/set_fan_speed"
///       fan_speed_list:
///         - min
///         - medium
///         - high
///         - max
///       send_command_topic: "vacuum/send_command"
/// ```
///
/// ## MQTT Protocol
///
/// The  configuration for this integration expects an MQTT protocol like the following.
///
/// ### Basic Commands
///
/// MQTT topic: `vacuum/command`
///
/// Possible MQTT payloads:
///
/// - `start` - Start cleaning
/// - `pause` - Pause cleaning
/// - `return_to_base` - Return to base/dock
/// - `stop` - Stop the vacuum.
/// - `clean_spot` - Initialize a spot cleaning cycle
/// - `locate` - Locate the vacuum (typically by playing a song)
///
/// ### Send custom command
///
/// Vacuum send_command allows three parameters:
///
/// - entity_id
/// - command
/// - params - optional
///
/// If params are not provided it sends command as payload to MQTT send_command topic.
/// If params are provided service sends JSON as payload with such structure:
///
/// ```json
/// {
///   'command': 'command',
///   'param1-key': 'param1-value'
/// }
/// ```
///
/// Service trigger example:
///
/// ```yaml
/// - alias: "Push command based on sensor"
///     trigger:
///       - platform: state
///         entity_id: sensor.sensor
///     action:
///       service: vacuum.send_command
///       target:
///         entity_id: vacuum.vacuum_entity
///       data:
///         command: "custom_command"
///         params:
///           - key: value
/// ```
///
/// MQTT topic: `vacuum/send_command`
///
/// ### Status/Sensor Updates
///
/// MQTT topic: `vacuum/state`
///
/// MQTT payload:
///
/// ```json
/// {
///     "battery_level": 61,
///     "state": "docked",
///     "fan_speed": "off"
/// }
/// ```
///
/// State has to be one of vacuum states supported by Home Assistant:
///
/// - cleaning,
/// - docked,
/// - paused,
/// - idle,
/// - returning,
/// - error.
///
/// ### Set Fan Speed
///
/// MQTT topic: `vacuum/set_fan_speed`
///
/// Possible MQTT payloads:
///
/// - `min` - Minimum fan speed
/// - `medium` - Medium fan speed
/// - `high` - High fan speed
/// - `max` - Max fan speed
///
/// ## Usage examples
///
/// ### Usage with cloudless Xiaomi vacuums
///
/// This integration is supported by the cloud-free Xiaomi Vacuum Webinterface [Valetudo](https://github.com/Hypfer/Valetudo).
///
/// ### Retrofitting non-wifi vacuums
///
/// - Retrofitting your old Roomba with an ESP8266. [This repository](https://github.com/johnboiles/esp-roomba-mqtt) provides MQTT client firmware.
/// - If you own a non-wifi Neato, you can refer to [this repository](https://github.com/jeroenterheerdt/neato-serial) that uses a Raspberry Pi to retrofit an old Neato.
///
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct Vacuum {
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

    /// The MQTT topic to publish commands to control the vacuum.
    #[serde(rename = "cmd_t", skip_serializing_if = "Option::is_none")]
    pub command_topic: Option<String>,

    /// The encoding of the payloads received and published messages. Set to `""` to disable decoding of incoming payload.
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    /// List of possible fan speeds for the vacuum.
    #[serde(rename = "fanspd_lst", skip_serializing_if = "Option::is_none")]
    pub fan_speed_list: Option<Vec<String>>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The name of the vacuum. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used instead of `name` for automatic generation of `entity_id`
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// The payload that represents the available state.
    #[serde(rename = "pl_avail", skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,

    /// The payload to send to the `command_topic` to begin a spot cleaning cycle.
    #[serde(rename = "pl_cln_sp", skip_serializing_if = "Option::is_none")]
    pub payload_clean_spot: Option<String>,

    /// The payload to send to the `command_topic` to locate the vacuum (typically plays a song).
    #[serde(rename = "pl_loc", skip_serializing_if = "Option::is_none")]
    pub payload_locate: Option<String>,

    /// The payload that represents the unavailable state.
    #[serde(rename = "pl_not_avail", skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,

    /// The payload to send to the `command_topic` to pause the vacuum.
    #[serde(rename = "pl_paus", skip_serializing_if = "Option::is_none")]
    pub payload_pause: Option<String>,

    /// The payload to send to the `command_topic` to tell the vacuum to return to base.
    #[serde(rename = "pl_ret", skip_serializing_if = "Option::is_none")]
    pub payload_return_to_base: Option<String>,

    /// The payload to send to the `command_topic` to begin the cleaning cycle.
    #[serde(rename = "pl_strt", skip_serializing_if = "Option::is_none")]
    pub payload_start: Option<String>,

    /// The payload to send to the `command_topic` to stop cleaning.
    #[serde(rename = "pl_stop", skip_serializing_if = "Option::is_none")]
    pub payload_stop: Option<String>,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// The MQTT topic to publish custom commands to the vacuum.
    #[serde(rename = "send_cmd_t", skip_serializing_if = "Option::is_none")]
    pub send_command_topic: Option<String>,

    /// The MQTT topic to publish commands to control the vacuum's fan speed.
    #[serde(rename = "set_fan_spd_t", skip_serializing_if = "Option::is_none")]
    pub set_fan_speed_topic: Option<String>,

    /// The MQTT topic subscribed to receive state messages from the vacuum. Messages received on the `state_topic` must be a valid JSON dictionary, with a mandatory `state` key and optionally `battery_level` and `fan_speed` keys as shown in the [example](#configuration-example).
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// List of features that the vacuum supports (possible values are `start`, `stop`, `pause`, `return_home`, `battery`, `status`, `locate`, `clean_spot`, `fan_speed`, `send_command`).
    #[serde(rename = "sup_feat", skip_serializing_if = "Option::is_none")]
    pub supported_features: Option<Vec<String>>,

    /// An ID that uniquely identifies this vacuum. If two vacuums have the same unique ID, Home Assistant will raise an exception.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl Vacuum {
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

    /// The MQTT topic to publish commands to control the vacuum.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = Some(command_topic.into());
        self
    }

    /// The encoding of the payloads received and published messages. Set to `""` to disable decoding of incoming payload.
    pub fn encoding<T: Into<String>>(mut self, encoding: T) -> Self {
        self.encoding = Some(encoding.into());
        self
    }

    /// List of possible fan speeds for the vacuum.
    pub fn fan_speed_list(mut self, fan_speed_list: Vec<String>) -> Self {
        self.fan_speed_list = Some(fan_speed_list);
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

    /// The name of the vacuum. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used instead of `name` for automatic generation of `entity_id`
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// The payload that represents the available state.
    pub fn payload_available<T: Into<String>>(mut self, payload_available: T) -> Self {
        self.payload_available = Some(payload_available.into());
        self
    }

    /// The payload to send to the `command_topic` to begin a spot cleaning cycle.
    pub fn payload_clean_spot<T: Into<String>>(mut self, payload_clean_spot: T) -> Self {
        self.payload_clean_spot = Some(payload_clean_spot.into());
        self
    }

    /// The payload to send to the `command_topic` to locate the vacuum (typically plays a song).
    pub fn payload_locate<T: Into<String>>(mut self, payload_locate: T) -> Self {
        self.payload_locate = Some(payload_locate.into());
        self
    }

    /// The payload that represents the unavailable state.
    pub fn payload_not_available<T: Into<String>>(mut self, payload_not_available: T) -> Self {
        self.payload_not_available = Some(payload_not_available.into());
        self
    }

    /// The payload to send to the `command_topic` to pause the vacuum.
    pub fn payload_pause<T: Into<String>>(mut self, payload_pause: T) -> Self {
        self.payload_pause = Some(payload_pause.into());
        self
    }

    /// The payload to send to the `command_topic` to tell the vacuum to return to base.
    pub fn payload_return_to_base<T: Into<String>>(mut self, payload_return_to_base: T) -> Self {
        self.payload_return_to_base = Some(payload_return_to_base.into());
        self
    }

    /// The payload to send to the `command_topic` to begin the cleaning cycle.
    pub fn payload_start<T: Into<String>>(mut self, payload_start: T) -> Self {
        self.payload_start = Some(payload_start.into());
        self
    }

    /// The payload to send to the `command_topic` to stop cleaning.
    pub fn payload_stop<T: Into<String>>(mut self, payload_stop: T) -> Self {
        self.payload_stop = Some(payload_stop.into());
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

    /// The MQTT topic to publish custom commands to the vacuum.
    pub fn send_command_topic<T: Into<String>>(mut self, send_command_topic: T) -> Self {
        self.send_command_topic = Some(send_command_topic.into());
        self
    }

    /// The MQTT topic to publish commands to control the vacuum's fan speed.
    pub fn set_fan_speed_topic<T: Into<String>>(mut self, set_fan_speed_topic: T) -> Self {
        self.set_fan_speed_topic = Some(set_fan_speed_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive state messages from the vacuum. Messages received on the `state_topic` must be a valid JSON dictionary, with a mandatory `state` key and optionally `battery_level` and `fan_speed` keys as shown in the [example](#configuration-example).
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// List of features that the vacuum supports (possible values are `start`, `stop`, `pause`, `return_home`, `battery`, `status`, `locate`, `clean_spot`, `fan_speed`, `send_command`).
    pub fn supported_features(mut self, supported_features: Vec<String>) -> Self {
        self.supported_features = Some(supported_features);
        self
    }

    /// An ID that uniquely identifies this vacuum. If two vacuums have the same unique ID, Home Assistant will raise an exception.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }
}
