use serde_derive::Serialize;

use super::{
    common::{Availability, Device, EntityCategory, Origin},
    units::Unit,
};

/// Keeps track on number entities in your environment, their state, and allows you to control them. This integration allows other integrations to get a value input from user within a range.
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct Number {
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
    /// Sets the [class of the device](https://www.home-assistant.io/integrations/number/#device-class), changing the device state and icon that is displayed on the frontend. The `device_class` can be `null`.
    #[serde(rename = "dev_cla", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<NumberDeviceClass>,
    /// The name of the Number. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Minimum value. (optional, default: 1)
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    /// Maximum value. (optional, default: 100)
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    /// Control how the number sh0.001ould be displayed in the UI. Can be set to `box` or `slider` to force a display mode. (optional, default: `auto`)
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<DisplayMode>,
    /// A special payload that resets the state to unknown when received on the `state_topic`. (optional, default: `None`)
    #[serde(rename = "pl_rst", skip_serializing_if = "Option::is_none")]
    pub payload_reset: Option<String>,
    /// Step value. Smallest value `0.001`. (optional, default: 1)
    #[serde(rename = "step", skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    /// Defines the unit of measurement of the sensor, if any. The `unit_of_measurement` can be `null`. (optional)
    #[serde(rename = "unit_of_meas", skip_serializing_if = "Option::is_none")]
    pub unit_of_measurement: Option<Unit>,
}

impl Number {
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

    /// The [type/class](https://www.home-assistant.io/integrations/number/#device-class) of the sensor to set the icon in the frontend. The `device_class` can be `null`.
    pub fn device_class(mut self, device_class: NumberDeviceClass) -> Self {
        self.device_class = Some(device_class);
        self
    }

    /// The name of the MQTT sensor. Can be set to null if only the device name is relevant.
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Minimum value. (optional, default: 1)
    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    /// Maximum value. (optional, default: 100)
    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    /// Control how the number sh0.001ould be displayed in the UI. Can be set to `box` or `slider` to force a display mode. (optional, default: `auto`)
    pub fn mode(mut self, mode: DisplayMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// A special payload that resets the state to unknown when received on the `state_topic`. (optional, default: `None`)
    pub fn payload_reset<S: Into<String>>(mut self, payload_reset: S) -> Self {
        self.payload_reset = Some(payload_reset.into());
        self
    }

    /// Step value. Smallest value `0.001`. (optional, default: 1)
    pub fn step(mut self, step: f64) -> Self {
        self.step = Some(step);
        self
    }

    /// Defines the units of measurement of the sensor, if any.
    pub fn unit_of_measurement(mut self, unit_of_measurement: Unit) -> Self {
        self.unit_of_measurement = Some(unit_of_measurement);
        self
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum DisplayMode {
    #[serde(rename = "box")]
    Box,
    #[serde(rename = "slider")]
    Slider,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum NumberDeviceClass {
    /// Apparent power in VA.
    #[serde(rename = "apparent_power")]
    ApparentPower,
    /// Air Quality Index (unitless).
    #[serde(rename = "aqi")]
    Aqi,
    /// Atmospheric pressure in cbar, bar, hPa, inHg, kPa, mbar, Pa, psi
    #[serde(rename = "atmospheric_pressure")]
    AtmosphericPressure,
    /// Percentage of battery that is left
    #[serde(rename = "battery")]
    Battery,
    /// Carbon Dioxide in CO2 (Smoke)
    #[serde(rename = "carbon_dioxide")]
    CarbonDioxide,
    /// Carbon Monoxide in CO (Gas CNG/LPG)
    #[serde(rename = "carbon_monoxide")]
    CarbonMonoxide,
    /// Current in A, mA
    #[serde(rename = "current")]
    Current,
    /// Data rate in bit/s, kbit/s, Mbit/s, Gbit/s, B/s, kB/s, MB/s, GB/s, KiB/s, MiB/s, or GiB/s
    #[serde(rename = "data_rate")]
    DataRate,
    /// Data size in bit, kbit, Mbit, Gbit, B, kB, MB, GB, TB, PB, EB, ZB, YB, KiB, MiB, GiB, TiB, PiB, EiB, ZiB, or YiB
    #[serde(rename = "data_size")]
    DataSize,
    /// Generic distance in km, m, cm, mm, mi, yd, or in
    #[serde(rename = "distance")]
    Distance,
    /// Energy in Wh, kWh, MWh, MJ, or GJ
    #[serde(rename = "energy")]
    Energy,
    /// Stored energy in Wh, kWh, MWh, MJ, or GJ
    #[serde(rename = "energy_storage")]
    EnergyStorage,
    /// Frequency in Hz, kHz, MHz, or GHz
    #[serde(rename = "frequency")]
    Frequency,
    /// Gasvolume in m³, ft³, or CCF
    #[serde(rename = "gas")]
    Gas,
    /// Percentage of humidity in the air
    #[serde(rename = "humidity")]
    Humidity,
    /// The current light level in lx
    #[serde(rename = "illuminance")]
    Illuminance,
    /// Irradiance in W/m² or BTU/(h⋅ft²)
    #[serde(rename = "irradiance")]
    Irradiance,
    /// Percentage of water in a substance
    #[serde(rename = "moisture")]
    Moisture,
    /// The monetary value
    #[serde(rename = "monetary")]
    Monetary,
    /// Concentration of Nitrogen Dioxide in µg/m³
    #[serde(rename = "nitrogen_dioxide")]
    NitrogenDioxide,
    /// Concentration of Nitrogen Monoxide in µg/m³
    #[serde(rename = "nitrogen_monoxide")]
    NitrogenMonoxide,
    /// Concentration of Nitrous Oxide in µg/m³
    #[serde(rename = "nitrous_oxide")]
    NitrousOxide,
    /// Concentration of Ozone in µg/m³
    #[serde(rename = "ozone")]
    Ozone,
    /// Potential hydrogen (pH) value of a water solution
    #[serde(rename = "ph")]
    Ph,
    /// Concentration of particulate matter less than 1 micrometer in µg/m³
    #[serde(rename = "pm1")]
    Pm1,
    /// Concentration of particulate matter less than 10 micrometers in µg/m³
    #[serde(rename = "pm10")]
    Pm10,
    /// Concentration of particulate matter less than 2.5 micrometers in µg/m³
    #[serde(rename = "pm25")]
    Pm25,
    /// Power factor(unitless), unit may be None or %
    #[serde(rename = "power_factor")]
    PowerFactor,
    /// Power in W or kW
    #[serde(rename = "power")]
    Power,
    /// Accumulated precipitation in cm, in or mm
    #[serde(rename = "precipitation")]
    Precipitation,
    /// Precipitation intensity in in/d, in/h, mm/d, or mm/h
    #[serde(rename = "precipitation_intensity")]
    PrecipitationIntensity,
    /// Pressure in Pa, kPa, hPa, bar, cbar, mbar, mmHg, inHg, or psi
    #[serde(rename = "pressure")]
    Pressure,
    /// Reactive power in var
    #[serde(rename = "reactive_power")]
    ReactivePower,
    /// Signal strength in dB or dBm
    #[serde(rename = "signal_strength")]
    SignalStrength,
    /// Sound pressure in dB or dBA
    #[serde(rename = "sound_pressure")]
    SoundPressure,
    /// Generic speed in ft/s, in/d, in/h, km/h, kn, m/s, mph, or mm/d
    #[serde(rename = "speed")]
    Speed,
    /// Concentration of sulphur dioxide in µg/m³
    #[serde(rename = "sulphur_dioxide")]
    SulphurDioxide,
    /// Temperature in °C, °F or K
    #[serde(rename = "temperature")]
    Temperature,
    /// Concentration of volatile organic compounds in µg/m³
    #[serde(rename = "volatile_organic_compounds")]
    VolatileOrganicCompounds,
    /// Voltage in V, mV
    #[serde(rename = "voltage")]
    Voltage,
    /// Generic volume in L, mL, gal, fl. oz., m³, ft³, or CCF
    #[serde(rename = "volume")]
    Volume,
    /// Generic stored volume in L, mL, gal, fl. oz., m³, ft³, or CCF
    #[serde(rename = "volume_storage")]
    VolumeStorage,
    /// Water consumption in L, gal, m³, ft³, or CCF
    #[serde(rename = "water")]
    Water,
    /// Generic mass in kg, g, mg, µg, oz, lb, or st
    #[serde(rename = "weight")]
    Weight,
    /// Wind speed in ft/s, km/h, kn, m/s, or mph
    #[serde(rename = "wind_speed")]
    WindSpeed,
}

#[cfg(test)]
mod tests {
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

    use crate::mqtt::{
        common::{Availability, Device, Origin},
        units::{PercentageUnit::Percentage, Unit},
    };

    use super::*;

    #[test]
    fn can_serialize_sensor() {
        let number = Number {
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
            command_topic: "~/command".to_string(),
            command_template: Some("{{ command_value }}".to_string()),
            optimistic: Some(false),
            retain: Some(true),
            device_class: Some(NumberDeviceClass::Battery),
            name: Some("number name".to_string()),
            min: Some(1.0),
            max: Some(100.0),
            mode: Some(DisplayMode::Slider),
            payload_reset: Some("NaN".to_string()),
            step: Some(0.02),
            unit_of_measurement: Some(Unit::Percentage(Percentage)),
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
              "cmd_t": "~/command",
              "cmd_tpl": "{{ command_value }}",
              "opt" : false,
              "ret": true,
              "dev_cla": "battery",
              "exp_aft": 60,
              "name": "number name",
              "mode": "slider",
              "min": 1.0,
              "max": 100.0,
              "step": 0.02,
              "pl_rst": "NaN",
              "unit_of_meas": "%"
            }),
            serde_json::to_value(&number).unwrap()
        );
    }
}
