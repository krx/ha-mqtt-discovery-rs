use super::common::Qos;
use super::common::{Availability, Device, EntityCategory, Origin};
use crate::Entity;
use serde_derive::Serialize;

/// ---
/// title: "MQTT device tracker"
/// description: "Instructions on how to use MQTT to track devices in Home Assistant."
/// ha_category:
///   - Presence detection
/// ha_iot_class: Configurable
/// ha_release: 0.7.3
/// ha_domain: mqtt
/// ---
///
///
/// The `mqtt` device tracker platform allows you to define new device_trackers through [manual YAML configuration](#yaml-configuration) in `configuration.yaml` and also to automatically discover device_trackers [using the MQTT Discovery protocol](#using-the-discovery-protocol).
///
/// ## Configuration
///
/// To use this device tracker in your installation, add the following to your `configuration.yaml` file:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - device_tracker:
///       name: "annetherese_n4"
///       state_topic: "location/annetherese"
///   - device_tracker:
///       name: "paulus_oneplus"
///       state_topic: "location/paulus"
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
/// device:
///   description: "Information about the device this device tracker is a part of that ties it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). At least one of identifiers or connections must be present to identify the device."
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
/// icon:
///   description: "[Icon](/docs/configuration/customizing-devices/#icon) for the entity."
///   required: false
///   type: icon
/// json_attributes_template:
///   description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation."
///   required: false
///   type: template
/// json_attributes_topic:
///   description: |
///     The MQTT topic subscribed to receive a JSON dictionary message containing device tracker attributes.
///     This topic can be used to set the location of the device tracker under the following conditions:
///     - If the attributes in the JSON message include `longitude`, `latitude`, and `gps_accuracy` (optional).\n
///     - If the device tracker is within a configured [zone](/integrations/zone/).\n
///     If these conditions are met, it is not required to configure `state_topic`.\n\n
///     Be aware that any location message received at `state_topic`  overrides the location received via `json_attributes_topic` until a message configured with `payload_reset` is received at `state_topic`. For a more generic usage example of the `json_attributes_topic`, refer to the [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
///   required: false
///   type: string
/// name:
///   description: The name of the MQTT device_tracker.
///   required: false
///   type: string
/// object_id:
///   description: Used instead of `name` for automatic generation of `entity_id`
///   required: false
///   type: string
/// payload_available:
///   description: The payload that represents the available state.
///   required: false
///   type: string
///   default: online
/// payload_home:
///   description: The payload value that represents the 'home' state for the device.
///   required: false
///   type: string
///   default: home
/// payload_not_available:
///   description: The payload that represents the unavailable state.
///   required: false
///   type: string
///   default: offline
/// payload_not_home:
///   description: The payload value that represents the 'not_home' state for the device.
///   required: false
///   type: string
///   default: not_home
/// payload_reset:
///   description: The payload value that will have the device's location automatically derived from Home Assistant's zones.
///   required: false
///   type: string
///   default: '"None"'
/// qos:
///   description: The maximum QoS level to be used when receiving and publishing messages.
///   required: false
///   type: integer
///   default: 0
/// source_type:
///   description: Attribute of a device tracker that affects state when being used to track a [person](/integrations/person/). Valid options are `gps`, `router`, `bluetooth`, or `bluetooth_le`.
///   required: false
///   type: string
/// state_topic:
///   description: The MQTT topic subscribed to receive device tracker state changes. The states defined in `state_topic` override the location states defined by the `json_attributes_topic`. This state override is turned inactive if the `state_topic` receives a message containing `payload_reset`. The `state_topic` can only be omitted if `json_attributes_topic` is used.
///   required: false
///   type: string
/// unique_id:
///   description: "An ID that uniquely identifies this device_tracker. If two device_trackers have the same unique ID, Home Assistant will raise an exception."
///   required: false
///   type: string
/// value_template:
///   description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) that returns a device tracker state."
///   required: false
///   type: template
/// {% endconfiguration %}
///
/// ## Examples
///
/// ### Using the discovery protocol
///
/// The device_tracker can be created via publishing to a discovery topic that follows the following [MQTT Discovery](/integrations/mqtt/#mqtt-discovery#discovery-topic) topic name format: `<discovery_prefix>/device_tracker/[<node_id>/]<object_id>/config`.
///
/// You can use the command line tool `mosquitto_pub` shipped with `mosquitto` or the `mosquitto-clients` package to send MQTT messages.
///
/// To create the device_tracker:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t homeassistant/device_tracker/a4567d663eaf/config -m '{"state_topic": "a4567d663eaf/state", "name": "My Tracker", "payload_home": "home", "payload_not_home": "not_home"}'
/// ```
///
/// To set the state of the device tracker to "home":
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t a4567d663eaf/state -m 'home'
/// ```
///
/// To set the state of the device tracker to a named location:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t a4567d663eaf/state -m 'location_name'
/// ```
///
/// If the device supports GPS coordinates then they can be sent to Home Assistant by specifying an attributes topic (i.e. "json_attributes_topic") in the configuration payload:
///
/// - Attributes topic: `a4567d663eaf/attributes`
/// - Example attributes payload:
///
/// Example message to be received at topic `a4567d663eaf/attributes`:
///
/// ```json
/// {
///   "latitude": 32.87336,
///   "longitude": -117.22743,
///   "gps_accuracy": 1.2
///  }
/// ```
///
/// To create the device_tracker with GPS coordinates support:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t homeassistant/device_tracker/a4567d663eaf/config -m '{"json_attributes_topic": "a4567d663eaf/attributes", "name": "My Tracker"}'
/// ```
///
/// <div class='note info'>
///
/// Using `state_topic` is optional when using `json_attributes_topic` to determine the state of the device tracker.
///
/// </div>
///
/// To set the state of the device tracker to specific coordinates:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t a4567d663eaf/attributes -m '{"latitude": 32.87336, "longitude": -117.22743, "gps_accuracy": 1.2}'
/// ```
///
///
/// ### YAML configuration
///
/// The following example shows how to configure the same device tracker through configuration.yaml
///
/// {% raw %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - device_tracker:
///       name: "My Tracker"
///       state_topic: "a4567d663eaf/state"
///       payload_home: "home"
///       payload_not_home: "not_home"
/// ```
///
/// {% endraw %}
///
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct DeviceTracker {
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

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary message containing device tracker attributes.
    /// This topic can be used to set the location of the device tracker under the following conditions:
    /// - If the attributes in the JSON message include `longitude`, `latitude`, and `gps_accuracy` (optional).\n
    /// - If the device tracker is within a configured [zone](/integrations/zone/).\n
    /// If these conditions are met, it is not required to configure `state_topic`.\n\n
    /// Be aware that any location message received at `state_topic`  overrides the location received via `json_attributes_topic` until a message configured with `payload_reset` is received at `state_topic`. For a more generic usage example of the `json_attributes_topic`, refer to the [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    ///
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The name of the MQTT device_tracker.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used instead of `name` for automatic generation of `entity_id`
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// The payload value that represents the 'home' state for the device.
    #[serde(rename = "pl_home", skip_serializing_if = "Option::is_none")]
    pub payload_home: Option<String>,

    /// The payload value that represents the 'not_home' state for the device.
    #[serde(rename = "pl_not_home", skip_serializing_if = "Option::is_none")]
    pub payload_not_home: Option<String>,

    /// The payload value that will have the device's location automatically derived from Home Assistant's zones.
    #[serde(rename = "pl_rst", skip_serializing_if = "Option::is_none")]
    pub payload_reset: Option<String>,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// Attribute of a device tracker that affects state when being used to track a [person](/integrations/person/). Valid options are `gps`, `router`, `bluetooth`, or `bluetooth_le`.
    #[serde(rename = "src_type", skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,

    /// The MQTT topic subscribed to receive device tracker state changes. The states defined in `state_topic` override the location states defined by the `json_attributes_topic`. This state override is turned inactive if the `state_topic` receives a message containing `payload_reset`. The `state_topic` can only be omitted if `json_attributes_topic` is used.
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// An ID that uniquely identifies this device_tracker. If two device_trackers have the same unique ID, Home Assistant will raise an exception.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) that returns a device tracker state.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl DeviceTracker {
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

    /// The MQTT topic subscribed to receive a JSON dictionary message containing device tracker attributes.
    /// This topic can be used to set the location of the device tracker under the following conditions:
    /// - If the attributes in the JSON message include `longitude`, `latitude`, and `gps_accuracy` (optional).\n
    /// - If the device tracker is within a configured [zone](/integrations/zone/).\n
    /// If these conditions are met, it is not required to configure `state_topic`.\n\n
    /// Be aware that any location message received at `state_topic`  overrides the location received via `json_attributes_topic` until a message configured with `payload_reset` is received at `state_topic`. For a more generic usage example of the `json_attributes_topic`, refer to the [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    ///
    pub fn json_attributes_topic<T: Into<String>>(mut self, json_attributes_topic: T) -> Self {
        self.json_attributes_topic = Some(json_attributes_topic.into());
        self
    }

    /// The name of the MQTT device_tracker.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used instead of `name` for automatic generation of `entity_id`
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// The payload value that represents the 'home' state for the device.
    pub fn payload_home<T: Into<String>>(mut self, payload_home: T) -> Self {
        self.payload_home = Some(payload_home.into());
        self
    }

    /// The payload value that represents the 'not_home' state for the device.
    pub fn payload_not_home<T: Into<String>>(mut self, payload_not_home: T) -> Self {
        self.payload_not_home = Some(payload_not_home.into());
        self
    }

    /// The payload value that will have the device's location automatically derived from Home Assistant's zones.
    pub fn payload_reset<T: Into<String>>(mut self, payload_reset: T) -> Self {
        self.payload_reset = Some(payload_reset.into());
        self
    }

    /// The maximum QoS level to be used when receiving and publishing messages.
    pub fn qos(mut self, qos: Qos) -> Self {
        self.qos = Some(qos);
        self
    }

    /// Attribute of a device tracker that affects state when being used to track a [person](/integrations/person/). Valid options are `gps`, `router`, `bluetooth`, or `bluetooth_le`.
    pub fn source_type<T: Into<String>>(mut self, source_type: T) -> Self {
        self.source_type = Some(source_type.into());
        self
    }

    /// The MQTT topic subscribed to receive device tracker state changes. The states defined in `state_topic` override the location states defined by the `json_attributes_topic`. This state override is turned inactive if the `state_topic` receives a message containing `payload_reset`. The `state_topic` can only be omitted if `json_attributes_topic` is used.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// An ID that uniquely identifies this device_tracker. If two device_trackers have the same unique ID, Home Assistant will raise an exception.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) that returns a device tracker state.
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl From<DeviceTracker> for Entity {
    fn from(value: DeviceTracker) -> Self {
        Entity::DeviceTracker(value)
    }
}
