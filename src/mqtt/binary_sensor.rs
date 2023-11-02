use serde_derive::Serialize;

use super::common::{MqttCommon, ReadOnlyEntity};

/// Binary sensors are similar to other sensors in that they monitor the states and conditions of different entities. Where binary sensors differ is they can only return one of two mutually exclusive values. For example, a binary sensor for a window may report a value of open or closed, a switch on or off, a condition true or false.
///
/// This either/or constraint is what makes these sensors binary. They are digital in nature, whereas analog sensors, like temperature and weight sensors, return a range of values.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BinarySensor {
    #[serde(flatten)]
    pub common: MqttCommon,
    #[serde(flatten)]
    pub ro_entity: ReadOnlyEntity,
    /// Sets the [class of the device](https://www.home-assistant.io/integrations/binary_sensor/#device-class), changing the device state and icon that is displayed on the frontend. The `device_class` can be `null`.
    #[serde(rename = "dev_cla", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<BinarySensorDeviceClass>,
    /// If set, it defines the number of seconds after the sensor’s state expires, if it’s not updated.
    /// After expiry, the sensor’s state becomes unavailable. Default the sensors state never expires.
    /// (optional, default: 0)
    #[serde(rename = "exp_aft", skip_serializing_if = "Option::is_none")]
    pub expire_after: Option<u64>,
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
            common: MqttCommon {
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
                availability: Availability::single_topic("~/availability"),
                enabled_by_default: Some(true),
            },
            ro_entity: ReadOnlyEntity {
                state_topic: "~/state".to_string(),
                value_template: Some("{{ value }}".to_string()),
            },
            device_class: Some(BinarySensorDeviceClass::Door),
            expire_after: Some(60),
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
