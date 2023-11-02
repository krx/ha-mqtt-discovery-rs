use serde::ser::SerializeSeq;
use serde_derive::Serialize;

/// Common attributes for all MQTT entities.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MqttCommon {
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
}

/// Common attributes for read only entities.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReadOnlyEntity {
    /// The MQTT topic subscribed to receive sensor values.
    /// If `device_class`, `state_class`, `unit_of_measurement` or `suggested_display_precision` is set, and a numeric value is expected, an empty value `''` will be ignored and will not update the state, a `'null'` value will set the sensor to an `unknown` state. The `device_class` can be `null`.
    #[serde(rename = "stat_t")]
    pub state_topic: String,
    /// Defines a [template](https://www.home-assistant.io/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the value.
    /// If the template throws an error, the current state will be used instead.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

/// Common attributes for read/write entities.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReadWriteEntity {
    /// The MQTT topic subscribed to receive sensor values.
    /// If `device_class`, `state_class`, `unit_of_measurement` or `suggested_display_precision` is set, and a numeric value is expected, an empty value `''` will be ignored and will not update the state, a `'null'` value will set the sensor to an `unknown` state. The `device_class` can be `null`.
    #[serde(rename = "stat_t")]
    pub state_topic: String,
    /// Defines a [template](https://www.home-assistant.io/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the value.
    /// If the template throws an error, the current state will be used instead.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
    /// The MQTT topic to publish commands to change the number.
    #[serde(rename = "cmd_t")]
    pub command_topic: String,
    /// Defines a template to generate the payload to send to command_topic.
    #[serde(rename = "cmd_tpl")]
    pub command_template: Option<String>,
    /// Flag that defines if number works in optimistic mode. Default: `true` if no `state_topic` defined, else `false`.
    #[serde(rename = "opt")]
    pub optimistic: Option<bool>,
    /// If the published message should have the retain flag on or not. (optional, default: `false`)
    #[serde(rename = "ret")]
    pub retain: Option<bool>,
}

/// Classification of a non-primary entity.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum EntityCategory {
    /// The entity allows changing the configuration of a device,
    /// for example a switch entity making it possible to turn the background illumination of a switch on and off.
    #[serde(rename = "config")]
    Config,

    /// THe entity exposes some configuration parameter or diagnostics of a device but does not allow changing it,
    /// for example a sensor showing RSSI or MAC-address.
    #[serde(rename = "diagnostic")]
    Diagnostic,
}

/// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Origin {
    /// The name of the application that is the origin the discovered MQTT item. This option is required.
    #[serde(rename = "name")]
    pub name: String,
    /// Software version of the application that supplies the discovered MQTT item.
    #[serde(rename = "sw", skip_serializing_if = "Option::is_none")]
    pub sw_version: Option<String>,
    /// Support URL of the application that supplies the discovered MQTT item.
    #[serde(rename = "support_url", skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
}

/// Information about the device this sensor is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/device_registry_index/). Only works when `unique_id` is set. At least one of identifiers or connections must be present to identify the device.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Device {
    /// The name of the device.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A list of IDs that uniquely identify the device. For example a serial number.
    #[serde(rename = "ids", skip_serializing_if = "Vec::is_empty")]
    pub identifiers: Vec<String>,
    /// A list of connections of the device to the outside world as a list of tuples `[connection_type, connection_identifier]`. For example the MAC address of a network interface: `"connections": [["mac", "02:5b:26:a8:dc:12"]]`.
    #[serde(rename = "cns", skip_serializing_if = "Vec::is_empty")]
    pub connections: Vec<DeviceConnection>,
    /// A link to the webpage that can manage the configuration of this device. Can be either an `http://`, `https://` or an internal `homeassistant://` URL.
    #[serde(rename = "cu", skip_serializing_if = "Option::is_none")]
    pub configuration_url: Option<String>,
    /// The manufacturer of the device.
    #[serde(rename = "mf", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// The model of the device.
    #[serde(rename = "mdl", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Suggest an area if the device isn’t in one yet.
    #[serde(rename = "sa", skip_serializing_if = "Option::is_none")]
    pub suggested_area: Option<String>,
    /// The firmware version of the device.
    #[serde(rename = "sw", skip_serializing_if = "Option::is_none")]
    pub sw_version: Option<String>,
    /// The hardware version of the device.
    #[serde(rename = "hw", skip_serializing_if = "Option::is_none")]
    pub hw_version: Option<String>,
    /// Identifier of a device that routes messages between this device and Home Assistant. Examples of such devices are hubs, or parent devices of a sub-device. This is used to show device topology in Home Assistant.
    #[serde(rename = "via_device", skip_serializing_if = "Option::is_none")]
    pub via_device: Option<String>,
}

/// A tuple `[connection_type, connection_identifier]`.
/// For example the MAC address of a network interface: `["mac", "02:5b:26:a8:dc:12"]`.
#[derive(Clone, Debug, PartialEq)]
pub struct DeviceConnection {
    pub r#type: String,
    pub identifier: String,
}

impl DeviceConnection {
    pub fn mac<S: Into<String>>(mac_address: S) -> Self {
        DeviceConnection {
            r#type: "mac".to_string(),
            identifier: mac_address.into(),
        }
    }
}

impl serde::ser::Serialize for DeviceConnection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.r#type)?;
        seq.serialize_element(&self.identifier)?;
        seq.end()
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum SensorStateClass {
    /// The state represents a measurement in present time, not a historical aggregation such as statistics or a prediction of the future.
    ///
    /// Examples of what should be classified measurement are: current temperature, humidify or electric power.
    /// Examples of what should not be classified as measurement: Forecasted temperature for tomorrow, yesterday's energy consumption or anything else that doesn't include the current measurement.
    /// For supported sensors, statistics of hourly min, max and average sensor readings is updated every 5 minutes.
    #[serde(rename = "measurement")]
    Measurement,

    /// The state represents a total amount that can both increase and decrease, e.g. a net energy meter.
    ///
    /// Statistics of the accumulated growth or decline of the sensor's value since it was first added is updated every 5 minutes.
    /// This state class should not be used for sensors where the absolute value is interesting instead of the accumulated growth or decline, for example remaining battery capacity or CPU load; in such cases state class measurement should be used instead.
    #[serde(rename = "total")]
    Total,

    /// Similar to total, with the restriction that the state represents a monotonically increasing positive total which periodically restarts counting from 0, e.g. a daily amount of consumed gas, weekly water consumption or lifetime energy consumption.
    ///
    /// Statistics of the accumulated growth of the sensor's value since it was first added is updated every 5 minutes.
    /// A decreasing value is interpreted as the start of a new meter cycle or the replacement of the meter.
    #[serde(rename = "total_increasing")]
    TotalIncreasing,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Availability {
    /// Controls the conditions needed to set the entity to `available`.
    #[serde(rename = "avty_mode")]
    pub mode: AvailabilityMode,
    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(rename = "avty")]
    pub availability: Vec<AvailabilityCheck>,
}

#[allow(dead_code)]
impl Availability {
    pub fn single_topic(topic: &str) -> Self {
        Self::single(AvailabilityCheck {
            payload_available: None,
            payload_not_available: None,
            topic: topic.to_string(),
            value_template: None,
        })
    }

    pub fn single(availability: AvailabilityCheck) -> Self {
        Self {
            mode: AvailabilityMode::All,
            availability: vec![availability],
        }
    }

    pub fn all(checks: Vec<AvailabilityCheck>) -> Self {
        Self {
            mode: AvailabilityMode::All,
            availability: checks,
        }
    }

    pub fn any(checks: Vec<AvailabilityCheck>) -> Self {
        Self {
            mode: AvailabilityMode::Any,
            availability: checks,
        }
    }

    pub fn latest(checks: Vec<AvailabilityCheck>) -> Self {
        Self {
            mode: AvailabilityMode::Latest,
            availability: checks,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum AvailabilityMode {
    /// `payload_available` must be received on all configured availability topics before the entity is marked as online.
    #[serde(rename = "all")]
    All,
    /// `payload_available` must be received on at least one configured availability topic before the entity is marked as online.
    #[serde(rename = "any")]
    Any,
    /// the last `payload_available` or `payload_not_available` received on any configured availability topic controls the availability.
    #[serde(rename = "latest")]
    Latest,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AvailabilityCheck {
    /// The payload that represents the available state. (optional, default: `online`)
    #[serde(rename = "pl_avail", skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,
    /// The payload that represents the unavailable state. (optional, default: `offline`)
    #[serde(rename = "pl_not_avail", skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,
    /// An MQTT topic subscribed to receive availability (online/offline) updates.
    #[serde(rename = "t")]
    pub topic: String,
    /// Defines a template to extract device’s availability from the topic. To determine the devices’s availability result of this template will be compared to payload_available and payload_not_available.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

#[cfg(test)]
mod tests {
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn can_serialize_common() {
        let common = MqttCommon {
            topic_prefix: Some("topic/prefix".to_string()),
            origin: Origin {
                name: "application name".to_string(),
                sw_version: Some("software version".to_string()),
                support_url: Some("https://github.com".to_string()),
            },
            device: Device {
                name: Some("device name".to_string()),
                identifiers: vec!["device id".to_string()],
                connections: vec![DeviceConnection::mac("connection id")],
                configuration_url: Some("http://config.url".to_string()),
                manufacturer: Some("device manufacturer".to_string()),
                model: Some("device model".to_string()),
                suggested_area: Some("area".to_string()),
                sw_version: Some("sw_v".to_string()),
                hw_version: Some("hw_v".to_string()),
                via_device: Some("via".to_string()),
            },
            entity_category: Some(EntityCategory::Config),
            icon: Some("mdi:home".to_string()),
            json_attributes_topic: Some("~/attrs_topic".to_string()),
            json_attributes_template: Some("{{ json_value.attrs }}".to_string()),
            object_id: Some("object-id".to_string()),
            unique_id: Some("unique-id".to_string()),
            availability: Availability::single(AvailabilityCheck {
                payload_available: Some("online".to_string()),
                payload_not_available: Some("offline".to_string()),
                topic: "~/availability".to_string(),
                value_template: Some("{{ value }}".to_string()),
            }),
            enabled_by_default: Some(true),
        };
        assert_json_eq!(
            json! ({
              "~": "topic/prefix",
              "o": {
                "name": "application name",
                "sw": "software version",
                "support_url": "https://github.com"
              },
              "dev": {
                "name": "device name",
                "ids": [
                  "device id"
                ],
                "cns": [
                  [
                    "mac",
                    "connection id"
                  ]
                ],
                "cu": "http://config.url",
                "mf": "device manufacturer",
                "mdl": "device model",
                "sa": "area",
                "sw": "sw_v",
                "hw": "hw_v",
                "via_device": "via"
              },
              "ent_cat": "config",
              "ic": "mdi:home",
              "json_attr_t": "~/attrs_topic",
              "json_attr_tpl": "{{ json_value.attrs }}",
              "obj_id": "object-id",
              "uniq_id": "unique-id",
              "avty_mode": "all",
              "avty": [
                {
                  "pl_avail": "online",
                  "pl_not_avail": "offline",
                  "t": "~/availability",
                  "val_tpl": "{{ value }}"
                }
              ],
              "en": true
            }),
            serde_json::to_value(&common).unwrap()
        );
    }

    #[test]
    fn can_serialize_origin() {
        let origin = Origin {
            name: "application name".to_string(),
            sw_version: Some("software version".to_string()),
            support_url: Some("https://github.com".to_string()),
        };
        assert_json_eq!(
            json! ({
              "name": "application name",
              "sw": "software version",
              "support_url": "https://github.com"
            }),
            serde_json::to_value(&origin).unwrap()
        );
    }

    #[test]
    fn can_serialize_device() {
        let device = Device {
            name: Some("device name".to_string()),
            identifiers: vec!["device id".to_string()],
            connections: vec![DeviceConnection::mac("connection id")],
            configuration_url: Some("http://config.url".to_string()),
            manufacturer: Some("device manufacturer".to_string()),
            model: Some("device model".to_string()),
            suggested_area: Some("area".to_string()),
            sw_version: Some("sw_v".to_string()),
            hw_version: Some("hw_v".to_string()),
            via_device: Some("via".to_string()),
        };
        assert_json_eq!(
            json! (
            {
              "name": "device name",
              "ids": [
                "device id"
              ],
              "cns": [
                [
                  "mac",
                  "connection id"
                ]
              ],
              "cu": "http://config.url",
              "mf": "device manufacturer",
              "mdl": "device model",
              "sa": "area",
              "sw": "sw_v",
              "hw": "hw_v",
              "via_device": "via"
            }),
            serde_json::to_value(&device).unwrap()
        );
    }
}
