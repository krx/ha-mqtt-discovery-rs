pub use rust_decimal::Decimal;
use serde_derive::Serialize;

use super::common::{Availability, Device, EntityCategory, Origin};

/// The mqtt climate platform lets you control your MQTT enabled HVAC devices.
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct Climate {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,
    #[serde(rename = "o")]
    pub origin: Origin,
    #[serde(rename = "dev")]
    pub device: Device,
    /// Defines how HA will check for entity availability.
    #[serde(flatten)]
    pub availability: Availability,
    /// A template to render the value received on the action_topic with.
    #[serde(rename = "act_tpl", skip_serializing_if = "Option::is_none")]
    pub action_template: Option<String>,
    /// Flag which defines if the entity should be enabled when first added.
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,
    /// The category of the entity.
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,
    /// The MQTT topic to subscribe for changes of the current action. If this is set, the climate graph uses the value received as data source. Valid values: off, heating, cooling, drying, idle, fan.
    #[serde(rename = "act_t", skip_serializing_if = "Option::is_none")]
    pub action_topic: Option<String>,
    /// A template with which the value received on current_humidity_topic will be rendered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_humidity_template: Option<String>,
    /// The MQTT topic on which to listen for the current humidity. A "None" value received will reset the current humidity. Empty values (''') will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_humidity_topic: Option<String>,
    /// A template with which the value received on current_temperature_topic will be rendered.
    #[serde(rename = "curr_temp_tpl", skip_serializing_if = "Option::is_none")]
    pub current_temperature_template: Option<String>,
    /// The MQTT topic on which to listen for the current temperature. A "None" value received will reset the current temperature. Empty values (''') will be ignored.
    #[serde(rename = "curr_temp_t", skip_serializing_if = "Option::is_none")]
    pub current_temperature_topic: Option<String>,
    /// A template to render the value sent to the fan_mode_command_topic with.
    #[serde(rename = "fan_mode_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub fan_mode_command_template: Option<String>,
    /// The MQTT topic to publish commands to change the fan mode.
    #[serde(rename = "fan_mode_cmd_t", skip_serializing_if = "Option::is_none")]
    pub fan_mode_command_topic: Option<String>,
    /// A template to render the value received on the fan_mode_state_topic with.
    #[serde(rename = "fan_mode_stat_tpl", skip_serializing_if = "Option::is_none")]
    pub fan_mode_state_template: Option<String>,
    /// The MQTT topic to subscribe for changes of the HVAC fan mode. If this is not set, the fan mode works in optimistic mode (see below).
    #[serde(rename = "fan_mode_stat_t", skip_serializing_if = "Option::is_none")]
    pub fan_mode_state_topic: Option<String>,
    /// A list of supported fan modes.
    /// Default: [“auto”, “low”, “medium”, “high”]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fan_modes: Option<Vec<String>>,
    /// Set the initial target temperature. The default value depends on the temperature unit and will be 21° or 69.8°F.
    #[serde(rename = "init", skip_serializing_if = "Option::is_none")]
    pub initial: Option<f32>,
    /// Icon for the entity.
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Implies force_update of the current sensor state when a message is received on this topic. (optional)
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,
    /// Defines a template to extract the JSON dictionary from messages received on the `json_attributes_topic`. (optional)
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,
    /// The minimum target humidity percentage that can be set.
    #[serde(rename = "max_hum", skip_serializing_if = "Option::is_none")]
    pub max_humidity: Option<f32>,
    /// Maximum set point available. The default value depends on the temperature unit, and will be 35°C or 95°F.
    #[serde(rename = "max_temp", skip_serializing_if = "Option::is_none")]
    pub max_temp: Option<f32>,
    /// The maximum target humidity percentage that can be set.
    #[serde(rename = "min_hum", skip_serializing_if = "Option::is_none")]
    pub min_humidity: Option<f32>,
    /// Minimum set point available. The default value depends on the temperature unit, and will be 7°C or 44.6°F.
    #[serde(rename = "min_temp", skip_serializing_if = "Option::is_none")]
    pub min_temp: Option<f32>,
    /// A template to render the value sent to the mode_command_topic with.
    #[serde(rename = "mode_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub mode_command_template: Option<String>,
    /// The MQTT topic to publish commands to change the HVAC operation mode.
    #[serde(rename = "mode_cmd_t", skip_serializing_if = "Option::is_none")]
    pub mode_command_topic: Option<String>,
    /// A template to render the value received on the mode_state_topic with.
    #[serde(rename = "mode_stat_tpl", skip_serializing_if = "Option::is_none")]
    pub mode_state_template: Option<String>,
    /// The MQTT topic to subscribe for changes of the HVAC operation mode. If this is not set, the operation mode works in optimistic mode (see below).
    #[serde(rename = "mode_stat_t", skip_serializing_if = "Option::is_none")]
    pub mode_state_topic: Option<String>,
    /// A list of supported modes. Needs to be a subset of the default values.
    /// Default: [“auto”, “off”, “cool”, “heat”, “dry”, “fan_only”]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modes: Option<Vec<String>>,
    /// The name of the HVAC. Can be set to null if only the device name is relevant.
    /// (default: MQTT HVAC)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Used instead of `name` for automatic generation of `entity_id`.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    /// An ID that uniquely identifies this HVAC device.
    /// If two HVAC devices have the same unique ID, Home Assistant will raise an exception.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    /// Flag that defines if the climate works in optimistic mode
    /// Default: true if no state topic defined, else false.
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,
    /// A template to render the value sent to the power_command_topic with. The value parameter is the payload set for payload_on or payload_off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_command_template: Option<String>,
    /// The MQTT topic to publish commands to change the HVAC power state. Sends the payload configured with payload_on if the climate is turned on via the climate.turn_on, or the payload configured with payload_off if the climate is turned off via the climate.turn_off service. Note that optimistic mode is not supported through climate.turn_on and climate.turn_off services. When called, these services will send a power command to the device but will not optimistically update the state of the climate entity. The climate device should report its state back via mode_state_topic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_command_topic: Option<String>,
    /// The desired precision for this device. Can be used to match your actual thermostat’s precision. Supported values are 0.1, 0.5 and 1.0.
    /// Default: 0.1 for Celsius and 1.0 for Fahrenheit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<Decimal>,
    /// Defines a template to generate the payload to send to preset_mode_command_topic.
    #[serde(rename = "pr_mode_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub preset_mode_command_template: Option<String>,
    /// The MQTT topic to publish commands to change the preset mode.
    #[serde(rename = "pr_mode_cmd_t", skip_serializing_if = "Option::is_none")]
    pub preset_mode_command_topic: Option<String>,
    /// The MQTT topic subscribed to receive climate speed based on presets. When preset ‘none’ is received or None the preset_mode will be reset.
    #[serde(rename = "pr_mode_stat_t", skip_serializing_if = "Option::is_none")]
    pub preset_mode_state_topic: Option<String>,
    /// Defines a template to extract the preset_mode value from the payload received on preset_mode_state_topic.
    #[serde(rename = "pr_mode_val_tpl", skip_serializing_if = "Option::is_none")]
    pub preset_mode_value_template: Option<String>,
    /// List of preset modes this climate is supporting. Common examples include eco, away, boost, comfort, home, sleep and activity.
    #[serde(rename = "pr_modes", skip_serializing_if = "Option::is_none")]
    pub preset_modes: Option<Vec<String>>,
    /// Defines if published messages should have the retain flag set.
    /// default: false,
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,
    /// A template to render the value sent to the swing_mode_command_topic with.
    #[serde(rename = "swing_mode_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub swing_mode_command_template: Option<String>,
    /// The MQTT topic to publish commands to change the swing mode.
    #[serde(rename = "swing_mode_cmd_t", skip_serializing_if = "Option::is_none")]
    pub swing_mode_command_topic: Option<String>,
    /// A template to render the value received on the swing_mode_state_topic with.
    #[serde(
        rename = "swing_mode_stat_tpl",
        skip_serializing_if = "Option::is_none"
    )]
    pub swing_mode_state_template: Option<String>,
    /// The MQTT topic to subscribe for changes of the HVAC swing mode. If this is not set, the swing mode works in optimistic mode (see below).
    #[serde(rename = "swing_mode_stat_t", skip_serializing_if = "Option::is_none")]
    pub swing_mode_state_topic: Option<String>,
    /// A list of supported swing modes.
    /// (optional, default: [“on”, “off”])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swing_modes: Option<Vec<String>>,
    /// Defines a template to generate the payload to send to target_humidity_command_topic.
    #[serde(rename = "hum_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub target_humidity_command_template: Option<String>,
    /// The MQTT topic to publish commands to change the target humidity.
    #[serde(rename = "hum_cmd_t", skip_serializing_if = "Option::is_none")]
    pub target_humidity_command_topic: Option<String>,
    /// The MQTT topic subscribed to receive the target humidity. If this is not set, the target humidity works in optimistic mode (see below). A "None" value received will reset the target humidity. Empty values (''') will be ignored.
    #[serde(rename = "hum_stat_t", skip_serializing_if = "Option::is_none")]
    pub target_humidity_state_topic: Option<String>,
    /// Defines a template to extract a value for the climate target_humidity state.
    #[serde(rename = "hum_state_tpl", skip_serializing_if = "Option::is_none")]
    pub target_humidity_state_template: Option<String>,
    /// A template to render the value sent to the temperature_command_topic with.
    #[serde(rename = "temp_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub temperature_command_template: Option<String>,
    /// The MQTT topic to publish commands to change the target temperature.
    #[serde(rename = "temp_cmd_t", skip_serializing_if = "Option::is_none")]
    pub temperature_command_topic: Option<String>,
    /// A template to render the value sent to the temperature_high_command_topic with.
    #[serde(rename = "temp_hi_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub temperature_high_command_template: Option<String>,
    /// The MQTT topic to publish commands to change the high target temperature.
    #[serde(rename = "temp_hi_cmd_t", skip_serializing_if = "Option::is_none")]
    pub temperature_high_command_topic: Option<String>,
    /// A template to render the value received on the temperature_high_state_topic with. A "None" value received will reset the temperature high set point. Empty values (''') will be ignored.
    #[serde(rename = "temp_hi_stat_tpl", skip_serializing_if = "Option::is_none")]
    pub temperature_high_state_template: Option<String>,
    /// The MQTT topic to subscribe for changes in the target high temperature. If this is not set, the target high temperature works in optimistic mode (see below).
    #[serde(rename = "temp_hi_stat_t", skip_serializing_if = "Option::is_none")]
    pub temperature_high_state_topic: Option<String>,
    /// A template to render the value sent to the temperature_low_command_topic with.
    #[serde(rename = "temp_lo_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub temperature_low_command_template: Option<String>,
    /// The MQTT topic to publish commands to change the target low temperature.
    #[serde(rename = "temp_lo_cmd_t", skip_serializing_if = "Option::is_none")]
    pub temperature_low_command_topic: Option<String>,
    /// A template to render the value received on the temperature_low_state_topic with. A "None" value received will reset the temperature low set point. Empty values (''') will be ignored.
    #[serde(rename = "temp_lo_stat_tpl", skip_serializing_if = "Option::is_none")]
    pub temperature_low_state_template: Option<String>,
    /// The MQTT topic to subscribe for changes in the target low temperature. If this is not set, the target low temperature works in optimistic mode (see below).
    #[serde(rename = "temp_lo_stat_t", skip_serializing_if = "Option::is_none")]
    pub temperature_low_state_topic: Option<String>,
    /// A template to render the value received on the temperature_state_topic with.
    #[serde(rename = "temp_stat_tpl", skip_serializing_if = "Option::is_none")]
    pub temperature_state_template: Option<String>,
    /// The MQTT topic to subscribe for changes in the target temperature. If this is not set, the target temperature works in optimistic mode (see below). A "None" value received will reset the temperature set point. Empty values (''') will be ignored.
    #[serde(rename = "temp_stat_t", skip_serializing_if = "Option::is_none")]
    pub temperature_state_topic: Option<String>,
    /// Defines the temperature unit of the device, C or F. If this is not set, the temperature unit is set to the system temperature unit.
    #[serde(rename = "temp_unit", skip_serializing_if = "Option::is_none")]
    pub temperature_unit: Option<TemperatureUnit>,
    /// Step size for temperature set point.
    /// (optional, default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_step: Option<Decimal>,
    /// Default template to render the payloads on all *_state_topics with.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum TemperatureUnit {
    #[serde(rename = "C")]
    Celsius,
    #[serde(rename = "F")]
    Fahrenheit,
}

#[cfg(test)]
mod tests {

    use std::vec;

    use assert_json_diff::assert_json_eq;
    use rust_decimal_macros::dec;
    use serde_json::json;

    use crate::mqtt::{
        climate::{Climate, TemperatureUnit},
        common::{Availability, Device, Origin},
    };

    #[test]
    fn can_serialize_sensor() {
        let sensor = Climate {
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
            action_template: Some("action".to_string()),
            action_topic: Some("~/action".to_string()),
            current_humidity_template: Some("humidity".to_string()),
            current_humidity_topic: Some("~/humidity".to_string()),
            current_temperature_template: Some("temperature".to_string()),
            current_temperature_topic: Some("~/temperature".to_string()),
            fan_mode_command_template: Some("fan-mode-command".to_string()),
            fan_mode_command_topic: Some("~/fan-mode-command".to_string()),
            fan_mode_state_template: Some("fan-mode-state".to_string()),
            fan_mode_state_topic: Some("~/fan-mode-state".to_string()),
            fan_modes: Some(vec!["on".to_string(), "off".to_string()]),
            initial: Some(20.0),
            max_humidity: Some(70.0),
            max_temp: Some(28.0),
            min_humidity: Some(30.0),
            min_temp: Some(18.0),
            mode_command_template: Some("mode-command".to_string()),
            mode_command_topic: Some("~/mode-command".to_string()),
            mode_state_template: Some("mode-state".to_string()),
            mode_state_topic: Some("~/mode-state".to_string()),
            modes: Some(vec![
                "manual".to_string(),
                "auto".to_string(),
                "comfort".to_string(),
            ]),
            optimistic: Some(false),
            power_command_template: Some("power-command".to_string()),
            power_command_topic: Some("~/power-command".to_string()),
            precision: Some(dec!(0.1)),
            preset_mode_command_template: Some("preset-mode-command".to_string()),
            preset_mode_command_topic: Some("~/preset-mode-command".to_string()),
            preset_mode_state_topic: Some("~/preset-mode-state".to_string()),
            preset_mode_value_template: Some("preset-mode-value".to_string()),
            preset_modes: Some(vec!["eco".to_string(), "away".to_string()]),
            retain: Some(false),
            swing_mode_command_template: Some("swing-mode-command".to_string()),
            swing_mode_command_topic: Some("~/swing-mode-command".to_string()),
            swing_mode_state_template: Some("swing-mode-state".to_string()),
            swing_mode_state_topic: Some("~/swing-mode-state".to_string()),
            swing_modes: Some(vec!["on".to_string(), "off".to_string()]),
            target_humidity_command_template: Some("target-humidity-command".to_string()),
            target_humidity_command_topic: Some("~/target-humidity-command".to_string()),
            target_humidity_state_topic: Some("~/target-humidity-state".to_string()),
            target_humidity_state_template: Some("target-humidity-state".to_string()),
            temperature_command_template: Some("temperature-command".to_string()),
            temperature_command_topic: Some("~/temperature-command".to_string()),
            temperature_high_command_template: Some("temperature-high-command".to_string()),
            temperature_high_command_topic: Some("~/temperature-high-command".to_string()),
            temperature_high_state_template: Some("temperature-high-state".to_string()),
            temperature_high_state_topic: Some("~/temperature-high-state".to_string()),
            temperature_low_command_template: Some("temperature-low-command".to_string()),
            temperature_low_command_topic: Some("~/temperature-low-command".to_string()),
            temperature_low_state_template: Some("temperature-low-state".to_string()),
            temperature_low_state_topic: Some("~/temperature-low-state".to_string()),
            temperature_state_template: Some("temperature-state".to_string()),
            temperature_state_topic: Some("~/temperature-state".to_string()),
            temperature_unit: Some(TemperatureUnit::Celsius),
            temp_step: Some(dec!(0.5)),
            value_template: Some("{{ value }}".to_string()),
            name: Some("climate name".to_string()),
        };
        assert_json_eq!(
            json! ({
              "~": "topic/prefix",
              "o": {
                "name": "application name"
              },
              "dev": {
                "name": "device name"
              },
              "exp_aft": 60,
              "obj_id": "object-id",
              "uniq_id": "unique-id",
              "avty_mode": "all",
              "avty": [
                {
                  "t": "~/availability"
                }
              ],
              "en": true,
              "act_tpl": "action",
              "act_t": "~/action",
              "current_humidity_template": "humidity",
              "current_humidity_topic": "~/humidity",
              "curr_temp_tpl": "temperature",
              "curr_temp_t": "~/temperature",
              "fan_mode_cmd_tpl": "fan-mode-command",
              "fan_mode_cmd_t": "~/fan-mode-command",
              "fan_mode_stat_tpl": "fan-mode-state",
              "fan_mode_stat_t": "~/fan-mode-state",
              "fan_modes": ["on", "off"],
              "init": 20.0,
              "max_hum": 70.0,
              "max_temp": 28.0,
              "min_hum": 30.0,
              "min_temp": 18.0,
              "mode_cmd_tpl": "mode-command",
              "mode_cmd_t": "~/mode-command",
              "mode_stat_tpl": "mode-state",
              "mode_stat_t": "~/mode-state",
              "modes": ["manual","auto","comfort"],
              "opt": false,
              "power_command_template": "power-command",
              "power_command_topic": "~/power-command",
              "precision": 0.1,
              "pr_mode_cmd_tpl": "preset-mode-command",
              "pr_mode_cmd_t": "~/preset-mode-command",
              "pr_mode_stat_t": "~/preset-mode-state",
              "pr_mode_val_tpl": "preset-mode-value",
              "pr_modes": ["eco", "away"],
              "ret": false,
              "swing_mode_cmd_tpl": "swing-mode-command",
              "swing_mode_cmd_t": "~/swing-mode-command",
              "swing_mode_stat_tpl": "swing-mode-state",
              "swing_mode_stat_t": "~/swing-mode-state",
              "swing_modes": ["on", "off"],
              "hum_cmd_tpl": "target-humidity-command",
              "hum_cmd_t": "~/target-humidity-command",
              "hum_stat_t": "~/target-humidity-state",
              "hum_state_tpl": "target-humidity-state",
              "temp_cmd_tpl": "temperature-command",
              "temp_cmd_t": "~/temperature-command",
              "temp_hi_cmd_tpl": "temperature-high-command",
              "temp_hi_cmd_t": "~/temperature-high-command",
              "temp_hi_stat_tpl": "temperature-high-state",
              "temp_hi_stat_t": "~/temperature-high-state",
              "temp_lo_cmd_tpl": "temperature-low-command",
              "temp_lo_cmd_t": "~/temperature-low-command",
              "temp_lo_stat_tpl": "temperature-low-state",
              "temp_lo_stat_t": "~/temperature-low-state",
              "temp_stat_tpl": "temperature-state",
              "temp_stat_t": "~/temperature-state",
              "temp_unit": "C",
              "temp_step": 0.5,
              "val_tpl": "{{ value }}",
              "name": "climate name"
            }),
            serde_json::to_value(&sensor).unwrap()
        );
    }
}
