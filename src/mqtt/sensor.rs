use serde_derive::Serialize;

use super::{
    common::{Availability, Device, EntityCategory, Origin, SensorStateClass},
    units::Unit,
};

/// Sensors are a basic integration in Home Assistant.
/// They monitor the states and conditions of a variety of entities. An entity can be many things. This can include a physical device like a motion sensor that reports the battery level, a web service that retrieves the weather temperature, a built-in function that calculates the sun’s elevation relative to your GPS position, or even a custom sensor you may have created to report the free space on your laptop. These are all things reporting different types of information.
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct Sensor {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~")]
    pub topic_prefix: Option<String>,
    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,
    /// Information about the device this sensor is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/device_registry_index/). Only works when `unique_id` is set. At least one of identifiers or connections must be present to identify the device.
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
    /// The [type/class](https://www.home-assistant.io/integrations/sensor/#device-class) of the sensor to set the icon in the frontend. The `device_class` can be `null`.
    #[serde(rename = "dev_cla", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<SensorDeviceClass>,
    /// Sends update events even if the value hasn’t changed.
    /// Useful if you want to have meaningful value graphs in history.
    #[serde(rename = "frc_upd", skip_serializing_if = "Option::is_none")]
    pub force_update: Option<bool>,
    /// Defines a [template](https://www.home-assistant.io/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the `last_reset`.
    /// Available variables: `entity_id`. The `entity_id` can be used to reference the entity’s attributes.
    #[serde(rename = "lrst_val_tpl", skip_serializing_if = "Option::is_none")]
    pub last_reset_value_template: Option<String>,
    /// The name of the MQTT sensor. Can be set to null if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The number of decimals which should be used in the sensor’s state after rounding.
    #[serde(rename = "sug_dsp_prc", skip_serializing_if = "Option::is_none")]
    pub suggested_display_precision: Option<u8>,
    /// The state_class of the sensor.
    #[serde(rename = "stat_cla", skip_serializing_if = "Option::is_none")]
    pub state_class: Option<SensorStateClass>,
    /// Defines the units of measurement of the sensor, if any.
    #[serde(rename = "unit_of_meas", skip_serializing_if = "Option::is_none")]
    pub unit_of_measurement: Option<Unit>,
}

impl Sensor {
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

    /// The [type/class](https://www.home-assistant.io/integrations/sensor/#device-class) of the sensor to set the icon in the frontend. The `device_class` can be `null`.
    pub fn device_class(mut self, device_class: SensorDeviceClass) -> Self {
        self.device_class = Some(device_class);
        self
    }

    /// Sends update events even if the value hasn’t changed.
    /// Useful if you want to have meaningful value graphs in history.
    pub fn force_update(mut self, force_update: bool) -> Self {
        self.force_update = Some(force_update);
        self
    }

    /// Defines a [template](https://www.home-assistant.io/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the `last_reset`.
    /// Available variables: `entity_id`. The `entity_id` can be used to reference the entity’s attributes.
    pub fn last_reset_value_template<S: Into<String>>(
        mut self,
        last_reset_value_template: S,
    ) -> Self {
        self.last_reset_value_template = Some(last_reset_value_template.into());
        self
    }

    /// The name of the MQTT sensor. Can be set to null if only the device name is relevant.
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    /// The number of decimals which should be used in the sensor’s state after rounding.
    pub fn suggested_display_precision(mut self, suggested_display_precision: u8) -> Self {
        self.suggested_display_precision = Some(suggested_display_precision);
        self
    }

    /// The state_class of the sensor.
    pub fn state_class(mut self, state_class: SensorStateClass) -> Self {
        self.state_class = Some(state_class);
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
pub enum SensorDeviceClass {
    /// Apparent power in VA.
    #[serde(rename = "apparent_power")]
    ApparentPower,
    /// Air Quality Index (unitless).
    #[serde(rename = "aqi")]
    AirQualityIndex,
    /// Atmospheric pressure in cbar, bar, hPa, mmHg, inHg, kPa, mbar, Pa or psi
    #[serde(rename = "atmospheric_pressure")]
    AtmosphericPressure,
    /// Percentage of battery that is left in %
    #[serde(rename = "battery")]
    Battery,
    /// Carbon Dioxide in CO2 (Smoke) in ppm
    #[serde(rename = "carbon_dioxide")]
    CarbonDioxide,
    /// Carbon Monoxide in CO (Gas CNG/LPG) in ppm
    #[serde(rename = "carbon_monoxide")]
    CarbonMonoxide,
    /// Current in A, mA
    #[serde(rename = "current")]
    Current,
    /// Data rate in bit/s, kbit/s, Mbit/s, Gbit/s, B/s, kB/s, MB/s, GB/s, KiB/s, MiB/s or GiB/s
    #[serde(rename = "data_rate")]
    DataRate,
    /// Data size in bit, kbit, Mbit, Gbit, B, kB, MB, GB, TB, PB, EB, ZB, YB, KiB, MiB, GiB, TiB, PiB, EiB, ZiB or YiB
    #[serde(rename = "data_size")]
    DataSize,
    /// Date string (ISO 8601)
    #[serde(rename = "date")]
    Date,
    /// Generic distance in km, m, cm, mm, mi, yd, or in
    #[serde(rename = "distance")]
    Distance,
    /// Duration in d, h, min, or s
    #[serde(rename = "duration")]
    Duration,
    /// Energy in Wh, kWh, MWh, MJ, or GJ
    #[serde(rename = "energy")]
    Energy,
    /// Stored energy in Wh, kWh, MWh, MJ, or GJ
    #[serde(rename = "energy_storage")]
    EnergyStorage,
    /// Has a limited set of (non-numeric) states
    #[serde(rename = "enum")]
    Enum,
    /// Frequency in Hz, kHz, MHz, or GHz
    #[serde(rename = "frequency")]
    Frequency,
    /// Gasvolume in m³, ft³ or CCF
    #[serde(rename = "gas")]
    Gas,
    /// Percentage of humidity in the air in %
    #[serde(rename = "humidity")]
    Humidity,
    /// The current light level in lx
    #[serde(rename = "illuminance")]
    Illuminance,
    /// Irradiance in W/m² or BTU/(h⋅ft²)
    #[serde(rename = "irradiance")]
    Irradiance,
    /// Percentage of water in a substance in %
    #[serde(rename = "moisture")]
    Moisture,
    /// The monetary value (ISO 4217)
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
    /// Concentration of particulate matter less than 2.5 micrometers in µg/m³
    #[serde(rename = "pm25")]
    Pm25,
    /// Concentration of particulate matter less than 10 micrometers in µg/m³
    #[serde(rename = "pm10")]
    Pm10,
    /// Power factor (unitless), unit may be None or %
    #[serde(rename = "power_factor")]
    PowerFactor,
    /// Power in W or kW
    #[serde(rename = "power")]
    Power,
    /// Accumulated precipitation in cm, in or mm
    #[serde(rename = "precipitation")]
    Precipitation,
    /// Precipitation intensity in in/d, in/h, mm/d or mm/h
    #[serde(rename = "precipitation_intensity")]
    PrecipitationIntensity,
    /// Pressure in Pa, kPa, hPa, bar, cbar, mbar, mmHg, inHg or psi
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
    /// Generic speed in ft/s, in/d, in/h, km/h, kn, m/s, mph or mm/d
    #[serde(rename = "speed")]
    Speed,
    /// Concentration of sulphur dioxide in µg/m³
    #[serde(rename = "sulphur_dioxide")]
    SulphurDioxide,
    /// Temperature in °C, °F or K
    #[serde(rename = "temperature")]
    Temperature,
    /// Datetime object or timestamp string (ISO 8601)
    #[serde(rename = "timestamp")]
    Timestamp,
    /// Concentration of volatile organic compounds in µg/m³
    #[serde(rename = "volatile_organic_compounds")]
    VolatileOrganicCompounds,
    /// Ratio of volatile organic compounds in ppm or ppb
    #[serde(rename = "volatile_organic_compounds_parts")]
    VolatileOrganicCompoundsParts,
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
        units::TempUnit,
    };

    use super::*;

    #[test]
    fn can_serialize_sensor() {
        let sensor = Sensor {
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
            device_class: Some(SensorDeviceClass::Temperature),
            force_update: Some(true),
            last_reset_value_template: Some("{{ json_value.last_reset }}".to_string()),
            name: Some("sensor name".to_string()),
            suggested_display_precision: Some(1),
            state_class: Some(SensorStateClass::Measurement),
            unit_of_measurement: Some(Unit::Temperature(TempUnit::Celsius)),
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
              "dev_cla": "temperature",
              "exp_aft": 60,
              "frc_upd": true,
              "lrst_val_tpl": "{{ json_value.last_reset }}",
              "name": "sensor name",
              "sug_dsp_prc": 1,
              "stat_cla": "measurement",
              "unit_of_meas": "°C"
            }),
            serde_json::to_value(&sensor).unwrap()
        );
    }

    #[test]
    fn can_use_builder_with_defaults() {
        let sensor = Sensor::default()
            .topic_prefix("topic/prefix")
            .origin(Origin::new("application name"))
            .object_id("object-id")
            .unique_id("unique-id")
            .device(
                Device::default()
                    .name("device name")
                    .add_identifier("device id"),
            )
            .availability(Availability::single_topic("~/availability").expire_after(60))
            .enabled_by_default(true)
            .state_topic("~/state")
            .value_template("{{ value }}")
            .device_class(SensorDeviceClass::Temperature)
            .force_update(true)
            .last_reset_value_template("{{ json_value.last_reset }}")
            .name("sensor name".to_string())
            .suggested_display_precision(1)
            .state_class(SensorStateClass::Measurement)
            .unit_of_measurement(Unit::Temperature(TempUnit::Celsius));
        assert_json_eq!(
            json! (
            {
              "~": "topic/prefix",
              "o": {
                "name": "application name"
              },
              "dev": {
                "name": "device name",
                "ids": [
                    "device id"
                ]
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
              "dev_cla": "temperature",
              "exp_aft": 60,
              "frc_upd": true,
              "lrst_val_tpl": "{{ json_value.last_reset }}",
              "name": "sensor name",
              "sug_dsp_prc": 1,
              "stat_cla": "measurement",
              "unit_of_meas": "°C"
            }),
            serde_json::to_value(&sensor).unwrap()
        );
    }
}
