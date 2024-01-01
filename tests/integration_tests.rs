use assert_json_diff::assert_json_eq;
use hass_mqtt_autodiscovery::{
    mqtt::{
        binary_sensor::{BinarySensor, BinarySensorDeviceClass},
        common::{Availability, Device, DeviceConnection, Origin, SensorStateClass},
        number::{DisplayMode, Number, NumberDeviceClass},
        sensor::{Sensor, SensorDeviceClass},
        units::{TempUnit::Celsius, Unit},
    },
    HomeAssistantMqtt,
};
use rumqttc::v5::{
    mqttbytes::{
        v5::{Packet, Publish},
        QoS::ExactlyOnce,
    },
    AsyncClient,
    Event::Incoming,
    MqttOptions,
};
use serde_json::{json, Value};
use std::time::Duration;
use testcontainers::{clients, core::WaitFor, Container, GenericImage};
use tokio::task;

fn start_mosquitto_container(docker: &clients::Cli) -> Container<'_, GenericImage> {
    let image = GenericImage::new("eclipse-mosquitto", "latest")
        .with_exposed_port(1883)
        .with_volume(
            "./tests/resources/mosquitto.conf",
            "/mosquitto/config/mosquitto.conf",
        )
        .with_wait_for(WaitFor::message_on_stderr(
            "Opening ipv4 listen socket on port 1883.",
        ));
    docker.run(image)
}

fn some_string(s: &str) -> Option<String> {
    Some(s.to_string())
}
fn origin() -> Origin {
    Origin {
        name: "Integration test".to_string(),
        sw_version: some_string("0.0.1"),
        support_url: some_string("https://www.github.com"),
    }
}

fn device() -> Device {
    Device {
        name: some_string("Barometer"),
        identifiers: vec!["barometer-09AF".to_string()],
        connections: vec![DeviceConnection::mac("09:AF:A4:54:F0:9D")],
        configuration_url: some_string("https://barometer.home/admin"),
        manufacturer: some_string("Awesome corp"),
        model: some_string("Awesome model"),
        suggested_area: some_string("kitchen"),
        sw_version: some_string("0.1a"),
        hw_version: some_string("rev B"),
        via_device: some_string("manufacturer cloud"),
    }
}

async fn do_with_mosquitto(callback: fn(AsyncClient) -> ()) -> (Publish, Value) {
    // start a mosquitto container
    let docker = clients::Cli::default();
    let mosquitto_container = start_mosquitto_container(&docker);

    // open a client to communicate with mosquitto container
    let mqtt_options = MqttOptions::new(
        "test",
        "127.0.0.1",
        mosquitto_container.get_host_port_ipv4(1883),
    );
    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

    // listen all topics
    client
        .subscribe("#", ExactlyOnce)
        .await
        .expect("successful subscription to all topics");

    // executes callback
    callback(client.clone());

    // disconnect client in 2 seconds to avoid waiting indefinitely
    task::spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await;
        client
            .disconnect()
            .await
            .expect("client to disconnect from mosquitto");
    });
    // wait for a text message
    while let Ok(message) = eventloop.poll().await {
        match message {
            Incoming(Packet::Publish(content)) => {
                let payload_string =
                    String::from_utf8(content.payload.to_vec()).expect("a valid UTF-8 string");
                return (
                    content,
                    serde_json::from_str(&payload_string).expect("a valid json"),
                );
            }
            _ => {}
        }
    }
    // or panic
    panic!("timeout waiting for expected message");
}

#[tokio::test]
async fn can_publish_a_binary_sensor_configuration() {
    let (packet, json) = do_with_mosquitto(|client| {
        let registry = HomeAssistantMqtt::new(client, "homeassistant/");
        tokio::spawn(async move {
            registry
                .publish_binary_sensor(BinarySensor {
                    topic_prefix: some_string("temperature_devices/barometer-09AF"),
                    origin: origin(),
                    device: device(),
                    entity_category: None,
                    icon: None,
                    json_attributes_topic: None,
                    json_attributes_template: None,
                    object_id: some_string("barometer-09AF"),
                    unique_id: some_string("barometer-09AF_state"),
                    availability: Availability::single_topic("~/availability").expire_after(120),
                    enabled_by_default: None,
                    state_topic: "~/state".to_string(),
                    value_template: some_string("{{ json_value.state }}"),
                    device_class: Some(BinarySensorDeviceClass::Door),
                    force_update: Some(true),
                    name: some_string("Barometer state"),
                    off_delay: Some(10),
                    payload_off: some_string("Off"),
                    payload_on: some_string("On"),
                })
                .await
                .expect("message to be published");
        });
    })
    .await;

    assert_eq!(
        packet.topic,
        "homeassistant/binary_sensor/barometer-09AF_state/config"
    );
    assert_json_eq!(
        json,
        json!(
            {
                "~": "temperature_devices/barometer-09AF",
                "o": {
                  "name": "Integration test",
                  "sw": "0.0.1",
                  "support_url": "https://www.github.com"
                },
                "dev": {
                  "name": "Barometer",
                  "ids": [
                    "barometer-09AF"
                  ],
                  "cns": [
                    [
                      "mac",
                      "09:AF:A4:54:F0:9D"
                    ]
                  ],
                  "cu": "https://barometer.home/admin",
                  "mf": "Awesome corp",
                  "mdl": "Awesome model",
                  "sa": "kitchen",
                  "sw": "0.1a",
                  "hw": "rev B",
                  "via_device": "manufacturer cloud"
                },
                "obj_id": "barometer-09AF",
                "uniq_id": "barometer-09AF_state",
                "avty_mode": "all",
                "avty": [
                  {
                    "t": "~/availability"
                  }
                ],
                "stat_t": "~/state",
                "val_tpl": "{{ json_value.state }}",
                "dev_cla": "door",
                "exp_aft": 120,
                "frc_upd": true,
                "name": "Barometer state",
                "off_dly": 10,
                "pl_off": "Off",
                "pl_on": "On"
              }
        )
    );
}

#[tokio::test]
async fn can_publish_a_number_configuration() {
    let (packet, json) = do_with_mosquitto(|client| {
        let registry = HomeAssistantMqtt::new(client, "homeassistant/");
        tokio::spawn(async move {
            registry
                .publish_number(Number {
                    topic_prefix: some_string("temperature_devices/barometer-09AF"),
                    origin: origin(),
                    device: device(),
                    entity_category: None,
                    icon: None,
                    json_attributes_topic: None,
                    json_attributes_template: None,
                    object_id: some_string("barometer-09AF"),
                    unique_id: some_string("barometer-09AF_temperature_drift"),
                    availability: Availability::single_topic("~/availability").expire_after(120),
                    enabled_by_default: None,
                    state_topic: "~/state".to_string(),
                    value_template: some_string("{{ json_value.temperature }}"),
                    command_topic: "~/command".to_string(),
                    command_template: Some("{{ json_value.command }}".to_string()),
                    optimistic: Some(false),
                    retain: Some(true),
                    device_class: Some(NumberDeviceClass::Temperature),
                    name: some_string("Temperature drift"),
                    min: Some(-10.0),
                    max: Some(10.0),
                    mode: Some(DisplayMode::Slider),
                    payload_reset: Some("NaN".to_string()),
                    step: Some(0.1),
                    unit_of_measurement: Some(Unit::Temperature(Celsius)),
                })
                .await
                .expect("message to be published");
        });
    })
    .await;

    assert_eq!(
        packet.topic,
        "homeassistant/number/barometer-09AF_temperature_drift/config"
    );
    assert_json_eq!(
        json,
        json!(
            {
                "~": "temperature_devices/barometer-09AF",
                "o": {
                  "name": "Integration test",
                  "sw": "0.0.1",
                  "support_url": "https://www.github.com"
                },
                "dev": {
                  "name": "Barometer",
                  "ids": [
                    "barometer-09AF"
                  ],
                  "cns": [
                    [
                      "mac",
                      "09:AF:A4:54:F0:9D"
                    ]
                  ],
                  "cu": "https://barometer.home/admin",
                  "mf": "Awesome corp",
                  "mdl": "Awesome model",
                  "sa": "kitchen",
                  "sw": "0.1a",
                  "hw": "rev B",
                  "via_device": "manufacturer cloud"
                },
                "obj_id": "barometer-09AF",
                "uniq_id": "barometer-09AF_temperature_drift",
                "avty_mode": "all",
                "avty": [
                  {
                    "t": "~/availability"
                  }
                ],
                "stat_t": "~/state",
                "val_tpl": "{{ json_value.temperature }}",
                "cmd_t": "~/command",
                "cmd_tpl": "{{ json_value.command }}",
                "dev_cla": "temperature",
                "exp_aft": 120,
                "name": "Temperature drift",
                "min": -10.0,
                "max": 10.0,
                "mode": "slider",
                "pl_rst": "NaN",
                "step": 0.1,
                "opt": false,
                "ret": true,
                "unit_of_meas": "°C",
              }
        )
    );
}

#[tokio::test]
async fn can_publish_a_sensor_configuration() {
    let (packet, json) = do_with_mosquitto(|client| {
        let registry = HomeAssistantMqtt::new(client, "homeassistant/");
        tokio::spawn(async move {
            registry
                .publish_sensor(Sensor {
                    topic_prefix: some_string("temperature_devices/barometer-09AF"),
                    origin: origin(),
                    device: device(),
                    entity_category: None,
                    icon: None,
                    json_attributes_topic: None,
                    json_attributes_template: None,
                    object_id: some_string("barometer-09AF"),
                    unique_id: some_string("barometer-09AF_temperature"),
                    availability: Availability::single_topic("~/availability").expire_after(120),
                    enabled_by_default: None,
                    state_topic: "~/state".to_string(),
                    value_template: some_string("{{ json_value.temperature }}"),
                    device_class: Some(SensorDeviceClass::Temperature),
                    force_update: Some(true),
                    last_reset_value_template: None,
                    name: some_string("Temperature"),
                    suggested_display_precision: Some(1),
                    state_class: Some(SensorStateClass::Measurement),
                    unit_of_measurement: Some(Unit::Temperature(Celsius)),
                })
                .await
                .expect("message to be published");
        });
    })
    .await;

    assert_eq!(
        packet.topic,
        "homeassistant/sensor/barometer-09AF_temperature/config"
    );
    assert_json_eq!(
        json,
        json!(
            {
                "~": "temperature_devices/barometer-09AF",
                "o": {
                  "name": "Integration test",
                  "sw": "0.0.1",
                  "support_url": "https://www.github.com"
                },
                "dev": {
                  "name": "Barometer",
                  "ids": [
                    "barometer-09AF"
                  ],
                  "cns": [
                    [
                      "mac",
                      "09:AF:A4:54:F0:9D"
                    ]
                  ],
                  "cu": "https://barometer.home/admin",
                  "mf": "Awesome corp",
                  "mdl": "Awesome model",
                  "sa": "kitchen",
                  "sw": "0.1a",
                  "hw": "rev B",
                  "via_device": "manufacturer cloud"
                },
                "obj_id": "barometer-09AF",
                "uniq_id": "barometer-09AF_temperature",
                "avty_mode": "all",
                "avty": [
                  {
                    "t": "~/availability"
                  }
                ],
                "stat_t": "~/state",
                "val_tpl": "{{ json_value.temperature }}",
                "dev_cla": "temperature",
                "exp_aft": 120,
                "frc_upd": true,
                "name": "Temperature",
                "sug_dsp_prc": 1,
                "stat_cla": "measurement",
                "unit_of_meas": "°C"
              }
        )
    );
}
