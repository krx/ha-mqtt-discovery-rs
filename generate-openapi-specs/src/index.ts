import { HomeAssistantConfiguration } from "./HomeAssistantConfigurationModel";
import { readdirSync, readFileSync, writeFileSync } from "fs";
import yaml from "js-yaml";

const config = new HomeAssistantConfiguration(
  `${process.env.HOME_ASSISTANT_DOCS}/source/_integrations/sensor.mqtt.markdown`
);

const INTEGRATIONS_DOCS = `${process.env.HOME_ASSISTANT_DOCS}/source/_integrations`;
const INTEGRATIONS_DOCS_MQTT = readdirSync(INTEGRATIONS_DOCS)
  .filter((name) => name.endsWith(".mqtt.markdown"))
  .filter(
    (name) =>
      ![
        "climate.mqtt.markdown",
        "number.mqtt.markdown",
        "water_heater.mqtt.markdown",
      ].includes(name)
  )
  .sort();
const REFERENCE_COMMON = `${INTEGRATIONS_DOCS}/sensor.mqtt.markdown`;

// generate common components
const commonConfig = new HomeAssistantConfiguration(REFERENCE_COMMON);
const common = {
  Availability: commonConfig.getAvailabilityModel(),
  Device: commonConfig.getDeviceModel(),
};
writeFileSync(
  `${process.env.DEVENV_ROOT}/specs/fragments/common.yml`,
  yaml.dump(common)
);

INTEGRATIONS_DOCS_MQTT.forEach((file) => {
  const filePath = `${INTEGRATIONS_DOCS}/${file}`;
  console.info(`converting ${filePath}`);
  const config = new HomeAssistantConfiguration(filePath);

  const openapiModel = config.toOpenapiObjectModel();
  writeFileSync(
    `${process.env.DEVENV_ROOT}/specs/fragments/${config.name}.yml`,
    `# Auto-generated Openapi model from the following content:
${config.source.replace(/^/gm, "# ")}

${yaml.dump(openapiModel)}`
  );
});
