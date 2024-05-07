use super::common::Qos;
use super::common::{Availability, Device, EntityCategory, Origin};
use crate::Entity;
use serde_derive::Serialize;

/// ---
/// title: "MQTT lawn mower"
/// description: "Instructions on how to integrate MQTT lawn mowers into Home Assistant."
/// ha_category:
///   - Lawn mower
/// ha_release: 2023.9
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` `lawn_mower` platform allows controlling a lawn mower over MQTT.
///
/// ## Configuration
///
/// To enable MQTT lawn mower in your installation, add the following to your `configuration.yaml` file:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - lawn_mower:
///       command_topic: topic
///       name: "Test Lawn Mower"
/// ```
///
/// {% configuration %}
/// activity_state_topic:
///   description: The MQTT topic subscribed to receive an update of the activity. Valid activities are `mowing`, `paused`, `docked`, and `error`. Use `value_template` to extract the activity state from a custom payload. When payload `none` is received, the activity state will be reset to `unknown`.  
///   required: false
///   type: string
/// activity_value_template:
///   description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the value."
///   required: false
///   type: template
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
///       description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the device's availability from the `topic`. To determine the device's availability, the result of this template will be compared to `payload_available` and `payload_not_available`."
///       required: false
///       type: template
/// availability_topic:
///   description: The MQTT topic subscribed to receive availability (online/offline) updates. Must not be used together with `availability`.
///   required: false
///   type: string
/// availability_mode:
///    description: When `availability` is configured, this controls the conditions needed to set the entity to `available`. Valid entries are `all`, `any`, and `latest`. If set to `all`, `payload_available` must be received on all configured availability topics before the entity is marked as online. If set to `any`, `payload_available` must be received on at least one configured availability topic before the entity is marked as online. If set to `latest`, the last `payload_available` or `payload_not_available` received on any configured availability topic controls the availability.
///    required: false
///    type: string
///    default: latest
/// availability_template:
///   description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's availability from the `availability_topic`. To determine the devices's availability, the result of this template will be compared to `payload_available` and `payload_not_available`."
///   required: false
///   type: template
/// device:
///   description: "Information about the device this lawn mower is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when the [`unique_id`](#unique_id) is set. At least one of the identifiers or connections must be present to identify the device."
///   required: false
///   type: map
///   keys:
///     configuration_url:
///       description: 'A link to the webpage that can manage the configuration of this device. Can be either an `http://`, `https://`, or an internal `homeassistant://` URL.'
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
///       description: 'A list of IDs that uniquely identify the device. For example, a serial number.'
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
///       description: 'Suggest an area if the device isn’t in one yet.'
///       required: false
///       type: string
///     sw_version:
///       description: The firmware version of the device.
///       required: false
///       type: string
///     via_device:
///       description: 'Identifier of a device that routes messages between this device and Home Assistant. Examples of such devices are hubs or parent devices of a sub-device. This is used to show the device topology in Home Assistant.'
///       required: false
///       type: string
/// dock_command_template:
///   description: Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `dock_command_topic`. The `value` parameter in the template will be set to `dock`.
///   required: false
///   type: template
/// dock_command_topic:
///   description: The MQTT topic that publishes commands when the service `lawn_mower.dock` service call is executed. The value `dock` is published when the service is called. Use a `dock_command_template` to publish a custom format.
///   required: false
///   type: string
/// enabled_by_default:
///   description: Flag which defines if the entity should be enabled when first added.
///   required: false
///   type: boolean
///   default: true
/// encoding:
///   description: The encoding of the payloads received and published messages. Set to `""` to disable decoding of the incoming payload.
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
///   description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`."
///   required: false
///   type: template
/// json_attributes_topic:
///   description: The MQTT topic subscribed to receive a JSON dictionary payload and then set as entity attributes. Implies `force_update` of the current activity state when a message is received on this topic.
///   required: false
///   type: string
/// name:
///   description: The name of the lawn mower. Can be set to `null` if only the device name is relevant.
///   required: false
///   type: string
/// object_id:
///   description: Used instead of `name` for automatic generation of `entity_id`
///   required: false
///   type: string
/// optimistic:
///   description: Flag that defines if the lawn mower works in optimistic mode.
///   required: false
///   type: boolean
///   default: "`true` if no `activity_state_topic` defined, else `false`."
/// pause_command_template:
///   description: Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `pause_command_topic`. The `value` parameter in the template will be set to `pause`.
///   required: false
///   type: template
/// pause_command_topic:
///   description: The MQTT topic that publishes commands when the service `lawn_mower.pause` service call is executed. The value `pause` is published when the service is called. Use a `pause_command_template` to publish a custom format.
///   required: false
///   type: string
/// qos:
///   description: The maximum QoS level to be used when receiving and publishing messages.
///   required: false
///   type: integer
///   default: 0
/// start_mowing_template:
///   description: Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `start_mowing_command_topic`. The `value` parameter in the template will be set to `start_mowing`.
///   required: false
///   type: template
/// start_mowing_command_topic:
///   description: The MQTT topic that publishes commands when the service `lawn_mower.start_mowing` service call is executed. The value `start_mowing` is published when the service is called. Use a `start_mowing_command_template` to publish a custom format.
///   required: false
///   type: string
/// retain:
///   description: If the published message should have the retain flag on or not.
///   required: false
///   type: boolean
///   default: false
/// unique_id:
///   description: An ID that uniquely identifies this lawn mower. If two lawn mowers have the same unique ID, Home Assistant will raise an exception.
///   required: false
///   type: string
/// {% endconfiguration %}
///
/// <div class='note warning'>
///
/// Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.
///
/// </div>
///
/// ## Example
///
/// The example below shows how to use a single command topic with a command template.
///
/// {% raw %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - lawn_mower:
///       name: "Lawn Mower Plus"
///       activity_state_topic: "lawn_mower_plus/state"
///       activity_value_template: "{{ value_json.activity }}"
///       pause_command_topic: "lawn_mower_plus/set"
///       pause_command_template: '{"activity": "{{ value }}"}'
///       dock_command_topic: "lawn_mower_plus/set"
///       dock_command_template: '{"activity": "{{ value }}"}'
///       start_mowing_command_topic: "lawn_mower_plus/set"
///       start_mowing_command_template: '{"activity": "{{ value }}"}'
/// ```
///
/// {% endraw %}
///
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct LawnMower {
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

    /// The MQTT topic subscribed to receive an update of the activity. Valid activities are `mowing`, `paused`, `docked`, and `error`. Use `value_template` to extract the activity state from a custom payload. When payload `none` is received, the activity state will be reset to `unknown`.
    #[serde(
        rename = "activity_state_topic",
        skip_serializing_if = "Option::is_none"
    )]
    pub activity_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the value.
    #[serde(
        rename = "activity_value_template",
        skip_serializing_if = "Option::is_none"
    )]
    pub activity_value_template: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `dock_command_topic`. The `value` parameter in the template will be set to `dock`.
    #[serde(
        rename = "dock_command_template",
        skip_serializing_if = "Option::is_none"
    )]
    pub dock_command_template: Option<String>,

    /// The MQTT topic that publishes commands when the service `lawn_mower.dock` service call is executed. The value `dock` is published when the service is called. Use a `dock_command_template` to publish a custom format.
    #[serde(rename = "dock_command_topic", skip_serializing_if = "Option::is_none")]
    pub dock_command_topic: Option<String>,

    /// Flag which defines if the entity should be enabled when first added.
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    /// The encoding of the payloads received and published messages. Set to `""` to disable decoding of the incoming payload.
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as entity attributes. Implies `force_update` of the current activity state when a message is received on this topic.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The name of the lawn mower. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used instead of `name` for automatic generation of `entity_id`
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Flag that defines if the lawn mower works in optimistic mode.
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `pause_command_topic`. The `value` parameter in the template will be set to `pause`.
    #[serde(
        rename = "pause_command_template",
        skip_serializing_if = "Option::is_none"
    )]
    pub pause_command_template: Option<String>,

    /// The MQTT topic that publishes commands when the service `lawn_mower.pause` service call is executed. The value `pause` is published when the service is called. Use a `pause_command_template` to publish a custom format.
    #[serde(
        rename = "pause_command_topic",
        skip_serializing_if = "Option::is_none"
    )]
    pub pause_command_topic: Option<String>,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `start_mowing_command_topic`. The `value` parameter in the template will be set to `start_mowing`.
    #[serde(
        rename = "start_mowing_template",
        skip_serializing_if = "Option::is_none"
    )]
    pub start_mowing_template: Option<String>,

    /// The MQTT topic that publishes commands when the service `lawn_mower.start_mowing` service call is executed. The value `start_mowing` is published when the service is called. Use a `start_mowing_command_template` to publish a custom format.
    #[serde(
        rename = "start_mowing_command_topic",
        skip_serializing_if = "Option::is_none"
    )]
    pub start_mowing_command_topic: Option<String>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// An ID that uniquely identifies this lawn mower. If two lawn mowers have the same unique ID, Home Assistant will raise an exception.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl LawnMower {
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

    /// The MQTT topic subscribed to receive an update of the activity. Valid activities are `mowing`, `paused`, `docked`, and `error`. Use `value_template` to extract the activity state from a custom payload. When payload `none` is received, the activity state will be reset to `unknown`.
    pub fn activity_state_topic<T: Into<String>>(mut self, activity_state_topic: T) -> Self {
        self.activity_state_topic = Some(activity_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the value.
    pub fn activity_value_template<T: Into<String>>(mut self, activity_value_template: T) -> Self {
        self.activity_value_template = Some(activity_value_template.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `dock_command_topic`. The `value` parameter in the template will be set to `dock`.
    pub fn dock_command_template<T: Into<String>>(mut self, dock_command_template: T) -> Self {
        self.dock_command_template = Some(dock_command_template.into());
        self
    }

    /// The MQTT topic that publishes commands when the service `lawn_mower.dock` service call is executed. The value `dock` is published when the service is called. Use a `dock_command_template` to publish a custom format.
    pub fn dock_command_topic<T: Into<String>>(mut self, dock_command_topic: T) -> Self {
        self.dock_command_topic = Some(dock_command_topic.into());
        self
    }

    /// Flag which defines if the entity should be enabled when first added.
    pub fn enabled_by_default(mut self, enabled_by_default: bool) -> Self {
        self.enabled_by_default = Some(enabled_by_default);
        self
    }

    /// The encoding of the payloads received and published messages. Set to `""` to disable decoding of the incoming payload.
    pub fn encoding<T: Into<String>>(mut self, encoding: T) -> Self {
        self.encoding = Some(encoding.into());
        self
    }

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    pub fn icon<T: Into<String>>(mut self, icon: T) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`.
    pub fn json_attributes_template<T: Into<String>>(
        mut self,
        json_attributes_template: T,
    ) -> Self {
        self.json_attributes_template = Some(json_attributes_template.into());
        self
    }

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as entity attributes. Implies `force_update` of the current activity state when a message is received on this topic.
    pub fn json_attributes_topic<T: Into<String>>(mut self, json_attributes_topic: T) -> Self {
        self.json_attributes_topic = Some(json_attributes_topic.into());
        self
    }

    /// The name of the lawn mower. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used instead of `name` for automatic generation of `entity_id`
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// Flag that defines if the lawn mower works in optimistic mode.
    pub fn optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = Some(optimistic);
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `pause_command_topic`. The `value` parameter in the template will be set to `pause`.
    pub fn pause_command_template<T: Into<String>>(mut self, pause_command_template: T) -> Self {
        self.pause_command_template = Some(pause_command_template.into());
        self
    }

    /// The MQTT topic that publishes commands when the service `lawn_mower.pause` service call is executed. The value `pause` is published when the service is called. Use a `pause_command_template` to publish a custom format.
    pub fn pause_command_topic<T: Into<String>>(mut self, pause_command_topic: T) -> Self {
        self.pause_command_topic = Some(pause_command_topic.into());
        self
    }

    /// The maximum QoS level to be used when receiving and publishing messages.
    pub fn qos(mut self, qos: Qos) -> Self {
        self.qos = Some(qos);
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `start_mowing_command_topic`. The `value` parameter in the template will be set to `start_mowing`.
    pub fn start_mowing_template<T: Into<String>>(mut self, start_mowing_template: T) -> Self {
        self.start_mowing_template = Some(start_mowing_template.into());
        self
    }

    /// The MQTT topic that publishes commands when the service `lawn_mower.start_mowing` service call is executed. The value `start_mowing` is published when the service is called. Use a `start_mowing_command_template` to publish a custom format.
    pub fn start_mowing_command_topic<T: Into<String>>(
        mut self,
        start_mowing_command_topic: T,
    ) -> Self {
        self.start_mowing_command_topic = Some(start_mowing_command_topic.into());
        self
    }

    /// If the published message should have the retain flag on or not.
    pub fn retain(mut self, retain: bool) -> Self {
        self.retain = Some(retain);
        self
    }

    /// An ID that uniquely identifies this lawn mower. If two lawn mowers have the same unique ID, Home Assistant will raise an exception.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }
}

impl Into<Entity> for LawnMower {
    fn into(self) -> Entity {
        Entity::LawnMower(self)
    }
}
