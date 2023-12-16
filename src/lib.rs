use std::fmt::Display;

use anyhow::{anyhow, Result};
use mqtt::{binary_sensor::BinarySensor, number::Number, sensor::Sensor};
use rumqttc::v5::{
    mqttbytes::{v5::PublishProperties, QoS::AtLeastOnce},
    AsyncClient,
};
use serde::Serialize;

pub mod mqtt;

const ONE_WEEK_SECONDS: u32 = 60 * 60 * 24 * 7;

#[derive(Clone)]
pub struct HomeAssistantMqtt {
    client: AsyncClient,
    discovery_prefix: String,
}

impl HomeAssistantMqtt {
    pub fn new<S: Into<String>>(client: AsyncClient, discovery_prefix: S) -> Self {
        Self {
            client,
            discovery_prefix: discovery_prefix.into(),
        }
    }

    /// The discovery topic needs to follow a specific format:
    /// `<discovery_prefix>/<component>/[<node_id>/]<object_id>/config`
    ///
    /// - `<discovery_prefix>`: The Discovery Prefix defaults to homeassistant. This prefix can be changed.
    /// - `<component>`: One of the supported MQTT integrations, eg. binary_sensor.
    /// - `<node_id>` (Optional): ID of the node providing the topic, this is not used by Home Assistant but may be used to structure the MQTT topic. The ID of the node must only consist of characters from the character class [a-zA-Z0-9_-] (alphanumerics, underscore and hyphen).
    /// - `<object_id>`: The ID of the device. This is only to allow for separate topics for each device and is not used for the entity_id. The ID of the device must only consist of characters from the character class [a-zA-Z0-9_-] (alphanumerics, underscore and hyphen).
    ///
    /// The `<node_id>` level can be used by clients to only subscribe to their own (command) topics by using one wildcard topic like <discovery_prefix>/+/<node_id>/+/set.
    ///
    /// Best practice for entities with a unique_id is to set `<object_id>` to unique_id and omit the `<node_id>`.
    async fn publish<S: Serialize>(&self, component: MqttComponent, entity: &S) -> Result<()> {
        let attributes = serde_json::to_value(entity)?;
        let object_id = attributes
            .as_object()
            .ok_or(anyhow!("entity configuration should be an object"))?
            .get("uniq_id")
            .ok_or(anyhow!(
                "entity configuration should have an attribute 'uniq_id'"
            ))?
            .as_str()
            .ok_or(anyhow!("'uniq_id' attribute should be a string"))?;
        let prefix = self
            .discovery_prefix
            .strip_suffix("/")
            .unwrap_or(&self.discovery_prefix);
        let topic = format!("{prefix}/{component}/{object_id}/config");
        let payload = serde_json::ser::to_string(entity).unwrap();
        let props = PublishProperties {
            //payload_format_indicator: Some(1),
            message_expiry_interval: Some(ONE_WEEK_SECONDS),
            content_type: Some("application/json".to_string()),
            ..Default::default()
        };
        Ok(self
            .client
            .publish_with_properties(topic, AtLeastOnce, true, payload, props)
            .await?)
    }

    pub async fn publish_binary_sensor(&self, binary_sensor: BinarySensor) -> Result<()> {
        self.publish(MqttComponent::BinarySensor, &binary_sensor)
            .await
    }

    pub async fn publish_number(&self, number: Number) -> Result<()> {
        self.publish(MqttComponent::Number, &number).await
    }

    pub async fn publish_sensor(&self, sensor: Sensor) -> Result<()> {
        self.publish(MqttComponent::Sensor, &sensor).await
    }

    pub async fn publish_data<S: Serialize>(
        &self,
        topic: &String,
        payload: &S,
        message_expiry_interval: Option<u32>,
    ) -> Result<()> {
        let payload = serde_json::ser::to_string(payload).unwrap();
        let props = PublishProperties {
            message_expiry_interval,
            content_type: Some("application/json".to_string()),
            ..Default::default()
        };
        Ok(self
            .client
            .publish_with_properties(topic, AtLeastOnce, true, payload, props)
            .await?)
    }
}

enum MqttComponent {
    BinarySensor,
    Number,
    Sensor,
}

impl Display for MqttComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            MqttComponent::Sensor => "sensor",
            MqttComponent::Number => "number",
            MqttComponent::BinarySensor => "binary_sensor",
        };
        write!(f, "{name}")
    }
}
