use super::common::Qos;
use super::common::{Availability, Device, EntityCategory, Origin};
use crate::Entity;
use serde_derive::Serialize;

///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Cover {
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

    /// The MQTT topic to publish commands to control the cover.
    #[serde(rename = "cmd_t", skip_serializing_if = "Option::is_none")]
    pub command_topic: Option<String>,

    /// Sets the [class of the device](/integrations/cover/), changing the device state and icon that is displayed on the frontend. The `device_class` can be `null`.
    #[serde(rename = "dev_cla", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<String>,

    /// Flag which defines if the entity should be enabled when first added.
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    /// The encoding of the payloads received and published messages. Set to `""` to disable decoding of incoming payload.
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    /// Picture URL for the entity.
    #[serde(rename = "ent_pic", skip_serializing_if = "Option::is_none")]
    pub entity_picture: Option<String>,

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The name of the cover. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used instead of `name` for automatic generation of `entity_id`
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Flag that defines if switch works in optimistic mode.
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    /// The command payload that closes the cover.
    #[serde(rename = "pl_cls", skip_serializing_if = "Option::is_none")]
    pub payload_close: Option<String>,

    /// The command payload that opens the cover.
    #[serde(rename = "pl_open", skip_serializing_if = "Option::is_none")]
    pub payload_open: Option<String>,

    /// The command payload that stops the cover.
    #[serde(rename = "pl_stop", skip_serializing_if = "Option::is_none")]
    pub payload_stop: Option<String>,

    /// Must be `cover`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "platform")]
    pub platform: String,

    /// Number which represents closed position.
    #[serde(rename = "pos_clsd", skip_serializing_if = "Option::is_none")]
    pub position_closed: Option<i32>,

    /// Number which represents open position.
    #[serde(rename = "pos_open", skip_serializing_if = "Option::is_none")]
    pub position_open: Option<i32>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) that can be used to extract the payload for the `position_topic` topic. Within the template the following variables are available: `entity_id`, `position_open`; `position_closed`; `tilt_min`; `tilt_max`. The `entity_id` can be used to reference the entity's attributes with help of the [states](/docs/configuration/templating/#states) template function;
    #[serde(rename = "pos_tpl", skip_serializing_if = "Option::is_none")]
    pub position_template: Option<String>,

    /// The MQTT topic subscribed to receive cover position messages.
    #[serde(rename = "pos_t", skip_serializing_if = "Option::is_none")]
    pub position_topic: Option<String>,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// Defines if published messages should have the retain flag set.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to define the position to be sent to the `set_position_topic` topic. Incoming position value is available for use in the template `{% raw %}{{ position }}{% endraw %}`. Within the template the following variables are available: `entity_id`, `position`, the target position in percent; `position_open`; `position_closed`; `tilt_min`; `tilt_max`. The `entity_id` can be used to reference the entity's attributes with help of the [states](/docs/configuration/templating/#states) template function;
    #[serde(rename = "set_pos_tpl", skip_serializing_if = "Option::is_none")]
    pub set_position_template: Option<String>,

    /// The MQTT topic to publish position commands to. You need to set position_topic as well if you want to use position topic. Use template if position topic wants different values than within range `position_closed` - `position_open`. If template is not defined and `position_closed != 100` and `position_open != 0` then proper position value is calculated from percentage position.
    #[serde(rename = "set_pos_t", skip_serializing_if = "Option::is_none")]
    pub set_position_topic: Option<String>,

    /// The payload that represents the closed state.
    #[serde(rename = "stat_clsd", skip_serializing_if = "Option::is_none")]
    pub state_closed: Option<String>,

    /// The payload that represents the closing state.
    #[serde(rename = "stat_closing", skip_serializing_if = "Option::is_none")]
    pub state_closing: Option<String>,

    /// The payload that represents the open state.
    #[serde(rename = "stat_open", skip_serializing_if = "Option::is_none")]
    pub state_open: Option<String>,

    /// The payload that represents the opening state.
    #[serde(rename = "stat_opening", skip_serializing_if = "Option::is_none")]
    pub state_opening: Option<String>,

    /// The payload that represents the stopped state (for covers that do not report `open`/`closed` state).
    #[serde(rename = "stat_stopped", skip_serializing_if = "Option::is_none")]
    pub state_stopped: Option<String>,

    /// The MQTT topic subscribed to receive cover state messages. State topic can only read a (`open`, `opening`, `closed`, `closing` or `stopped`) state.  A "None" payload resets to an `unknown` state. An empty payload is ignored.
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// The value that will be sent on a `close_cover_tilt` command.
    #[serde(rename = "tilt_clsd_val", skip_serializing_if = "Option::is_none")]
    pub tilt_closed_value: Option<i32>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) that can be used to extract the payload for the `tilt_command_topic` topic. Within the template the following variables are available: `entity_id`, `tilt_position`, the target tilt position in percent; `position_open`; `position_closed`; `tilt_min`; `tilt_max`. The `entity_id` can be used to reference the entity's attributes with help of the [states](/docs/configuration/templating/#states) template function;
    #[serde(rename = "tilt_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub tilt_command_template: Option<String>,

    /// The MQTT topic to publish commands to control the cover tilt.
    #[serde(rename = "tilt_cmd_t", skip_serializing_if = "Option::is_none")]
    pub tilt_command_topic: Option<String>,

    /// The maximum tilt value.
    #[serde(rename = "tilt_max", skip_serializing_if = "Option::is_none")]
    pub tilt_max: Option<i32>,

    /// The minimum tilt value.
    #[serde(rename = "tilt_min", skip_serializing_if = "Option::is_none")]
    pub tilt_min: Option<i32>,

    /// The value that will be sent on an `open_cover_tilt` command.
    #[serde(rename = "tilt_opnd_val", skip_serializing_if = "Option::is_none")]
    pub tilt_opened_value: Option<i32>,

    /// Flag that determines if tilt works in optimistic mode.
    #[serde(rename = "tilt_opt", skip_serializing_if = "Option::is_none")]
    pub tilt_optimistic: Option<bool>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) that can be used to extract the payload for the `tilt_status_topic` topic. Within the template the following variables are available: `entity_id`, `position_open`; `position_closed`; `tilt_min`; `tilt_max`. The `entity_id` can be used to reference the entity's attributes with help of the [states](/docs/configuration/templating/#states) template function;
    #[serde(rename = "tilt_status_tpl", skip_serializing_if = "Option::is_none")]
    pub tilt_status_template: Option<String>,

    /// The MQTT topic subscribed to receive tilt status update values.
    #[serde(rename = "tilt_status_t", skip_serializing_if = "Option::is_none")]
    pub tilt_status_topic: Option<String>,

    /// An ID that uniquely identifies this cover. If two covers have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) that can be used to extract the payload for the `state_topic` topic.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Cover {
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

    /// The MQTT topic to publish commands to control the cover.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = Some(command_topic.into());
        self
    }

    /// Sets the [class of the device](/integrations/cover/), changing the device state and icon that is displayed on the frontend. The `device_class` can be `null`.
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

    /// Picture URL for the entity.
    pub fn entity_picture<T: Into<String>>(mut self, entity_picture: T) -> Self {
        self.entity_picture = Some(entity_picture.into());
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

    /// The name of the cover. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used instead of `name` for automatic generation of `entity_id`
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// Flag that defines if switch works in optimistic mode.
    pub fn optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = Some(optimistic);
        self
    }

    /// The command payload that closes the cover.
    pub fn payload_close<T: Into<String>>(mut self, payload_close: T) -> Self {
        self.payload_close = Some(payload_close.into());
        self
    }

    /// The command payload that opens the cover.
    pub fn payload_open<T: Into<String>>(mut self, payload_open: T) -> Self {
        self.payload_open = Some(payload_open.into());
        self
    }

    /// The command payload that stops the cover.
    pub fn payload_stop<T: Into<String>>(mut self, payload_stop: T) -> Self {
        self.payload_stop = Some(payload_stop.into());
        self
    }

    /// Must be `cover`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    pub fn platform<T: Into<String>>(mut self, platform: T) -> Self {
        self.platform = platform.into();
        self
    }

    /// Number which represents closed position.
    pub fn position_closed(mut self, position_closed: i32) -> Self {
        self.position_closed = Some(position_closed);
        self
    }

    /// Number which represents open position.
    pub fn position_open(mut self, position_open: i32) -> Self {
        self.position_open = Some(position_open);
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) that can be used to extract the payload for the `position_topic` topic. Within the template the following variables are available: `entity_id`, `position_open`; `position_closed`; `tilt_min`; `tilt_max`. The `entity_id` can be used to reference the entity's attributes with help of the [states](/docs/configuration/templating/#states) template function;
    pub fn position_template<T: Into<String>>(mut self, position_template: T) -> Self {
        self.position_template = Some(position_template.into());
        self
    }

    /// The MQTT topic subscribed to receive cover position messages.
    pub fn position_topic<T: Into<String>>(mut self, position_topic: T) -> Self {
        self.position_topic = Some(position_topic.into());
        self
    }

    /// The maximum QoS level to be used when receiving and publishing messages.
    pub fn qos(mut self, qos: Qos) -> Self {
        self.qos = Some(qos);
        self
    }

    /// Defines if published messages should have the retain flag set.
    pub fn retain(mut self, retain: bool) -> Self {
        self.retain = Some(retain);
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to define the position to be sent to the `set_position_topic` topic. Incoming position value is available for use in the template `{% raw %}{{ position }}{% endraw %}`. Within the template the following variables are available: `entity_id`, `position`, the target position in percent; `position_open`; `position_closed`; `tilt_min`; `tilt_max`. The `entity_id` can be used to reference the entity's attributes with help of the [states](/docs/configuration/templating/#states) template function;
    pub fn set_position_template<T: Into<String>>(mut self, set_position_template: T) -> Self {
        self.set_position_template = Some(set_position_template.into());
        self
    }

    /// The MQTT topic to publish position commands to. You need to set position_topic as well if you want to use position topic. Use template if position topic wants different values than within range `position_closed` - `position_open`. If template is not defined and `position_closed != 100` and `position_open != 0` then proper position value is calculated from percentage position.
    pub fn set_position_topic<T: Into<String>>(mut self, set_position_topic: T) -> Self {
        self.set_position_topic = Some(set_position_topic.into());
        self
    }

    /// The payload that represents the closed state.
    pub fn state_closed<T: Into<String>>(mut self, state_closed: T) -> Self {
        self.state_closed = Some(state_closed.into());
        self
    }

    /// The payload that represents the closing state.
    pub fn state_closing<T: Into<String>>(mut self, state_closing: T) -> Self {
        self.state_closing = Some(state_closing.into());
        self
    }

    /// The payload that represents the open state.
    pub fn state_open<T: Into<String>>(mut self, state_open: T) -> Self {
        self.state_open = Some(state_open.into());
        self
    }

    /// The payload that represents the opening state.
    pub fn state_opening<T: Into<String>>(mut self, state_opening: T) -> Self {
        self.state_opening = Some(state_opening.into());
        self
    }

    /// The payload that represents the stopped state (for covers that do not report `open`/`closed` state).
    pub fn state_stopped<T: Into<String>>(mut self, state_stopped: T) -> Self {
        self.state_stopped = Some(state_stopped.into());
        self
    }

    /// The MQTT topic subscribed to receive cover state messages. State topic can only read a (`open`, `opening`, `closed`, `closing` or `stopped`) state.  A "None" payload resets to an `unknown` state. An empty payload is ignored.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// The value that will be sent on a `close_cover_tilt` command.
    pub fn tilt_closed_value(mut self, tilt_closed_value: i32) -> Self {
        self.tilt_closed_value = Some(tilt_closed_value);
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) that can be used to extract the payload for the `tilt_command_topic` topic. Within the template the following variables are available: `entity_id`, `tilt_position`, the target tilt position in percent; `position_open`; `position_closed`; `tilt_min`; `tilt_max`. The `entity_id` can be used to reference the entity's attributes with help of the [states](/docs/configuration/templating/#states) template function;
    pub fn tilt_command_template<T: Into<String>>(mut self, tilt_command_template: T) -> Self {
        self.tilt_command_template = Some(tilt_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to control the cover tilt.
    pub fn tilt_command_topic<T: Into<String>>(mut self, tilt_command_topic: T) -> Self {
        self.tilt_command_topic = Some(tilt_command_topic.into());
        self
    }

    /// The maximum tilt value.
    pub fn tilt_max(mut self, tilt_max: i32) -> Self {
        self.tilt_max = Some(tilt_max);
        self
    }

    /// The minimum tilt value.
    pub fn tilt_min(mut self, tilt_min: i32) -> Self {
        self.tilt_min = Some(tilt_min);
        self
    }

    /// The value that will be sent on an `open_cover_tilt` command.
    pub fn tilt_opened_value(mut self, tilt_opened_value: i32) -> Self {
        self.tilt_opened_value = Some(tilt_opened_value);
        self
    }

    /// Flag that determines if tilt works in optimistic mode.
    pub fn tilt_optimistic(mut self, tilt_optimistic: bool) -> Self {
        self.tilt_optimistic = Some(tilt_optimistic);
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) that can be used to extract the payload for the `tilt_status_topic` topic. Within the template the following variables are available: `entity_id`, `position_open`; `position_closed`; `tilt_min`; `tilt_max`. The `entity_id` can be used to reference the entity's attributes with help of the [states](/docs/configuration/templating/#states) template function;
    pub fn tilt_status_template<T: Into<String>>(mut self, tilt_status_template: T) -> Self {
        self.tilt_status_template = Some(tilt_status_template.into());
        self
    }

    /// The MQTT topic subscribed to receive tilt status update values.
    pub fn tilt_status_topic<T: Into<String>>(mut self, tilt_status_topic: T) -> Self {
        self.tilt_status_topic = Some(tilt_status_topic.into());
        self
    }

    /// An ID that uniquely identifies this cover. If two covers have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) that can be used to extract the payload for the `state_topic` topic.
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl Default for Cover {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            command_topic: Default::default(),
            device_class: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            icon: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            optimistic: Default::default(),
            payload_close: Default::default(),
            payload_open: Default::default(),
            payload_stop: Default::default(),
            platform: "cover".to_string(),
            position_closed: Default::default(),
            position_open: Default::default(),
            position_template: Default::default(),
            position_topic: Default::default(),
            qos: Default::default(),
            retain: Default::default(),
            set_position_template: Default::default(),
            set_position_topic: Default::default(),
            state_closed: Default::default(),
            state_closing: Default::default(),
            state_open: Default::default(),
            state_opening: Default::default(),
            state_stopped: Default::default(),
            state_topic: Default::default(),
            tilt_closed_value: Default::default(),
            tilt_command_template: Default::default(),
            tilt_command_topic: Default::default(),
            tilt_max: Default::default(),
            tilt_min: Default::default(),
            tilt_opened_value: Default::default(),
            tilt_optimistic: Default::default(),
            tilt_status_template: Default::default(),
            tilt_status_topic: Default::default(),
            unique_id: Default::default(),
            value_template: Default::default(),
        }
    }
}

impl From<Cover> for Entity {
    fn from(value: Cover) -> Self {
        Entity::Cover(value)
    }
}
