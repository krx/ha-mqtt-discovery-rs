use serde_derive::Serialize;

use super::common::{Availability, Device, EntityCategory, Origin};

/// Binary sensors are similar to other sensors in that they monitor the states and conditions of different entities. Where binary sensors differ is they can only return one of two mutually exclusive values. For example, a binary sensor for a window may report a value of open or closed, a switch on or off, a condition true or false.
///
/// This either/or constraint is what makes these sensors binary. They are digital in nature, whereas analog sensors, like temperature and weight sensors, return a range of values.
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct BinarySensor {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~")]
    pub topic_prefix: Option<String>,
    #[serde(rename = "o")]
    pub origin: Origin,
    #[serde(rename = "dev")]
    pub device: Device,
    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,
    /// Icon for the entity.
    /// Any icon from [MaterialDesignIcons.com](https://materialdesignicons.com/). Prefix name with `mdi:`, ie `mdi:home`.
    /// Note: Newer icons may not yet be available in the current Home Assistant release. You can check when an icon was added to MaterialDesignIcons.com at [MDI History](https://materialdesignicons.com/history).
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Implies force_update of the current sensor state when a message is received on this topic. (optional)
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,
    /// Defines a template to extract the JSON dictionary from messages received on the `json_attributes_topic`. (optional)
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,
    /// Used instead of `name` for automatic generation of `entity_id`.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    /// An ID that uniquely identifies this sensor.
    /// If two sensors have the same unique ID, Home Assistant will raise an exception.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    /// Defines how HA will check for entity availability.
    #[serde(flatten)]
    pub availability: Availability,
    /// Flag which defines if the entity should be enabled when first added.
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,
    /// The MQTT topic subscribed to receive sensor values.
    /// If `device_class`, `state_class`, `unit_of_measurement` or `suggested_display_precision` is set, and a numeric value is expected, an empty value `''` will be ignored and will not update the state, a `'null'` value will set the sensor to an `unknown` state. The `device_class` can be `null`.
    #[serde(rename = "stat_t")]
    pub state_topic: String,
    /// Defines a [template](https://www.home-assistant.io/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the value.
    /// If the template throws an error, the current state will be used instead.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
    /// Sets the [class of the device](https://www.home-assistant.io/integrations/binary_sensor/#device-class), changing the device state and icon that is displayed on the frontend. The `device_class` can be `null`.
    #[serde(rename = "dev_cla", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<BinarySensorDeviceClass>,
    /// Sends update events even if the value hasn’t changed.
    /// Useful if you want to have meaningful value graphs in history.
    #[serde(rename = "frc_upd", skip_serializing_if = "Option::is_none")]
    pub force_update: Option<bool>,
    /// The name of the MQTT sensor. Can be set to null if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// For sensors that only send `on` state updates (like PIRs), this variable sets a delay in seconds after which the sensor’s state will be updated back to `off`. (optional)
    #[serde(rename = "off_dly", skip_serializing_if = "Option::is_none")]
    pub off_delay: Option<u32>,
    /// The string that represents the `off` state. It will be compared to the message in the `state_topic` (see `value_template` for details). (optional, default: OFF)
    #[serde(rename = "pl_off", skip_serializing_if = "Option::is_none")]
    pub payload_off: Option<String>,
    /// The string that represents the `on` state. It will be compared to the message in the `state_topic` (see `value_template` for details). (optional, default: ON)
    #[serde(rename = "pl_on", skip_serializing_if = "Option::is_none")]
    pub payload_on: Option<String>,
}

impl BinarySensor {
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

    /// Icon for the entity.
    /// Any icon from [MaterialDesignIcons.com](https://materialdesignicons.com/). Prefix name with `mdi:`, ie `mdi:home`.
    /// Note: Newer icons may not yet be available in the current Home Assistant release. You can check when an icon was added to MaterialDesignIcons.com at [MDI History](https://materialdesignicons.com/history).
    pub fn icon<S: Into<String>>(mut self, icon: S) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Used instead of `name` for automatic generation of `entity_id`.
    pub fn object_id<S: Into<String>>(mut self, id: S) -> Self {
        self.object_id = Some(id.into());
        self
    }
    /// An ID that uniquely identifies this sensor.
    /// If two sensors have the same unique ID, Home Assistant will raise an exception.
    pub fn unique_id<S: Into<String>>(mut self, id: S) -> Self {
        self.unique_id = Some(id.into());
        self
    }

    /// Defines how HA will check for entity availability.
    pub fn availability(mut self, availability: Availability) -> Self {
        self.availability = availability;
        self
    }

    /// Flag which defines if the entity should be enabled when first added.
    pub fn enabled_by_default(mut self, enabled_by_default: bool) -> Self {
        self.enabled_by_default = Some(enabled_by_default);
        self
    }

    /// The MQTT topic subscribed to receive sensor values.
    /// If `device_class`, `state_class`, `unit_of_measurement` or `suggested_display_precision` is set, and a numeric value is expected, an empty value `''` will be ignored and will not update the state, a `'null'` value will set the sensor to an `unknown` state. The `device_class` can be `null`.
    pub fn state_topic<S: Into<String>>(mut self, state_topic: S) -> Self {
        self.state_topic = state_topic.into();
        self
    }

    /// Defines a [template](https://www.home-assistant.io/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the value.
    /// If the template throws an error, the current state will be used instead.
    pub fn value_template<S: Into<String>>(mut self, value_template: S) -> Self {
        self.value_template = Some(value_template.into());
        self
    }

    /// The [type/class](https://www.home-assistant.io/integrations/binary_sensor/#device-class) of the sensor to set the icon in the frontend. The `device_class` can be `null`.
    pub fn device_class(mut self, device_class: BinarySensorDeviceClass) -> Self {
        self.device_class = Some(device_class);
        self
    }

    /// Sends update events even if the value hasn’t changed.
    /// Useful if you want to have meaningful value graphs in history.
    pub fn force_update(mut self, force_update: bool) -> Self {
        self.force_update = Some(force_update);
        self
    }

    /// The name of the MQTT sensor. Can be set to null if only the device name is relevant.
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    /// For sensors that only send `on` state updates (like PIRs), this variable sets a delay in seconds after which the sensor’s state will be updated back to `off`. (optional)
    pub fn off_delay(mut self, off_delay: u32) -> Self {
        self.off_delay = Some(off_delay);
        self
    }

    /// The string that represents the `off` state. It will be compared to the message in the `state_topic` (see `value_template` for details). (optional, default: OFF)
    pub fn payload_off<S: Into<String>>(mut self, payload_off: S) -> Self {
        self.payload_off = Some(payload_off.into());
        self
    }

    /// The string that represents the `on` state. It will be compared to the message in the `state_topic` (see `value_template` for details). (optional, default: ON)
    pub fn payload_on<S: Into<String>>(mut self, payload_on: S) -> Self {
        self.payload_on = Some(payload_on.into());
        self
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum BinarySensorDeviceClass {
    ///  `on` means low, `off` means normal
    #[serde(rename = "battery")]
    Battery,
    ///  `on` means charging, `off` means not charging
    #[serde(rename = "battery_charging")]
    BatteryCharging,
    ///  `on` means carbon monoxide detected, `off` no carbon monoxide (clear)
    #[serde(rename = "carbon_monoxide")]
    CarbonMonoxide,
    ///  `on` means cold, `off` means normal
    #[serde(rename = "cold")]
    Cold,
    ///  `on` means connected, `off` means disconnected
    #[serde(rename = "connectivity")]
    Connectivity,
    ///  `on` means open, `off` means closed
    #[serde(rename = "door")]
    Door,
    ///  `on` means open, `off` means closed
    #[serde(rename = "garage_door")]
    GarageDoor,
    ///  `on` means gas detected, `off` means no gas (clear)
    #[serde(rename = "gas")]
    Gas,
    ///  `on` means hot, `off` means normal
    #[serde(rename = "heat")]
    Heat,
    ///  `on` means light detected, `off` means no light
    #[serde(rename = "light")]
    Light,
    ///  `on` means open (unlocked), `off` means closed (locked)
    #[serde(rename = "lock")]
    Lock,
    ///  `on` means moisture detected (wet), `off` means no moisture (dry)
    #[serde(rename = "moisture")]
    Moisture,
    ///  `on` means motion detected, `off` means no motion (clear)
    #[serde(rename = "motion")]
    Motion,
    ///  `on` means moving, `off` means not moving (stopped)
    #[serde(rename = "moving")]
    Moving,
    ///  `on` means occupied (detected), `off` means not occupied (clear)
    #[serde(rename = "occupancy")]
    Occupancy,
    ///  `on` means open, `off` means closed
    #[serde(rename = "opening")]
    Opening,
    ///  `on` means device is plugged in, `off` means device is unplugged
    #[serde(rename = "plug")]
    Plug,
    ///  `on` means power detected, `off` means no power
    #[serde(rename = "power")]
    Power,
    ///  `on` means home, `off` means away
    #[serde(rename = "presence")]
    Presence,
    ///  `on` means problem detected, `off` means no problem (OK)
    #[serde(rename = "problem")]
    Problem,
    ///  `on` means running, `off` means not running
    #[serde(rename = "running")]
    Running,
    ///  `on` means unsafe, `off` means safe
    #[serde(rename = "safety")]
    Safety,
    ///  `on` means smoke detected, `off` means no smoke (clear)
    #[serde(rename = "smoke")]
    Smoke,
    ///  `on` means sound detected, `off` means no sound (clear)
    #[serde(rename = "sound")]
    Sound,
    ///  `on` means tampering detected, `off` means no tampering (clear)
    #[serde(rename = "tamper")]
    Tamper,
    ///  `on` means update available, `off` means up-to-date
    #[serde(rename = "update")]
    Update,
    ///  `on` means vibration detected, `off` means no vibration (clear)
    #[serde(rename = "vibration")]
    Vibration,
    ///  `on` means open, `off` means closed
    #[serde(rename = "window")]
    Window,
}

#[cfg(test)]
mod tests {
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

    use crate::mqtt::common::{Availability, Device, Origin};

    use super::*;

    #[test]
    fn can_serialize_sensor() {
        let sensor = BinarySensor {
            topic_prefix: Some("topic/prefix".to_string()),
            origin: Origin {
                name: "application name".to_string(),
                sw_version: None,
                support_url: None,
            },
            device: Device {
                name: Some("device name".to_string()),
                identifiers: vec![],
                connections: vec![],
                configuration_url: None,
                manufacturer: None,
                model: None,
                suggested_area: None,
                sw_version: None,
                hw_version: None,
                via_device: None,
            },
            entity_category: None,
            icon: None,
            json_attributes_topic: None,
            json_attributes_template: None,
            object_id: Some("object-id".to_string()),
            unique_id: Some("unique-id".to_string()),
            availability: Availability::single_topic("~/availability").expire_after(60),
            enabled_by_default: Some(true),
            state_topic: "~/state".to_string(),
            value_template: Some("{{ value }}".to_string()),
            device_class: Some(BinarySensorDeviceClass::Door),
            force_update: Some(true),
            name: Some("sensor name".to_string()),
            off_delay: Some(10),
            payload_off: Some("OFF".to_string()),
            payload_on: Some("ON".to_string()),
        };
        assert_json_eq!(
            json! (
            {
              "~": "topic/prefix",
              "o": {
                "name": "application name"
              },
              "dev": {
                "name": "device name"
              },
              "obj_id": "object-id",
              "uniq_id": "unique-id",
              "avty_mode": "all",
              "avty": [
                {
                  "t": "~/availability"
                }
              ],
              "en": true,
              "stat_t": "~/state",
              "val_tpl": "{{ value }}",
              "dev_cla": "door",
              "exp_aft": 60,
              "frc_upd": true,
              "name": "sensor name",
              "off_dly": 10,
              "pl_off": "OFF",
              "pl_on": "ON"
            }),
            serde_json::to_value(&sensor).unwrap()
        );
    }
}
