use serde_derive::Serialize;

use super::{
    common::{MqttCommon, ReadOnlyEntity, SensorStateClass},
    units::Unit,
};

/// Sensors are a basic integration in Home Assistant.
/// They monitor the states and conditions of a variety of entities. An entity can be many things. This can include a physical device like a motion sensor that reports the battery level, a web service that retrieves the weather temperature, a built-in function that calculates the sun’s elevation relative to your GPS position, or even a custom sensor you may have created to report the free space on your laptop. These are all things reporting different types of information.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Sensor {
    #[serde(flatten)]
    pub common: MqttCommon,
    #[serde(flatten)]
    pub ro_entity: ReadOnlyEntity,
    /// The [type/class](https://www.home-assistant.io/integrations/sensor/#device-class) of the sensor to set the icon in the frontend. The `device_class` can be `null`.
    #[serde(rename = "dev_cla", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<SensorDeviceClass>,
    /// If set, it defines the number of seconds after the sensor’s state expires, if it’s not updated.
    /// After expiry, the sensor’s state becomes unavailable. Default the sensors state never expires.
    /// (optional, default: 0)
    #[serde(rename = "exp_aft", skip_serializing_if = "Option::is_none")]
    pub expire_after: Option<u64>,
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
            device_class: Some(SensorDeviceClass::Temperature),
            expire_after: Some(60),
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
}
