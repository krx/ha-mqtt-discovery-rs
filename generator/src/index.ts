import { readFileSync, writeFileSync, readdirSync } from "fs";
import Handlebars from "handlebars";
import { allAbbreviations } from "./abbretiations";
import { extractDeviceClassesEnums } from "./device-class";
import { generateMqttEntityModel } from "./entity";
import { toPascalCase } from "./strings";

const BASEDIR = process.env.DEVENV_ROOT;

const ENTITIES = [
  "alarm_control_panel",
  "binary_sensor",
  "button",
  "camera",
  "climate",
  "cover",
  "device_tracker",
  "device_trigger",
  "event",
  "fan",
  "humidifier",
  "image",
  "lawn_mower",
  //"light",
  "lock",
  "number",
  "scene",
  "select",
  "sensor",
  "siren",
  "switch",
  "tag",
  "text",
  "update",
  "vacuum",
  "valve",
  "water_heater",
];

Handlebars.registerHelper("abbreviation", (name: string) => {
  const abbreviation = Object.entries(allAbbreviations).find(
    ([shortName, fullName]) => name === fullName
  );
  return new Handlebars.SafeString(abbreviation ? abbreviation[0] : name);
});

Handlebars.registerHelper("comment", (text: string) => {
  return text?.replaceAll("\n", "\n/// ");
});

Handlebars.registerHelper("toPascalCase", toPascalCase);

// generate entities types
for (const entityName of ENTITIES) {
  const model = generateMqttEntityModel(
    entityName,
    `${BASEDIR}/generator/input/${entityName}.mqtt.markdown`
  );
  const template = readFileSync(
    `${BASEDIR}/generator/src/rust_model.mustache`
  ).toString();
  const output = Handlebars.compile(template)(model);
  writeFileSync(`${BASEDIR}/src/mqtt/${entityName}.rs`, output);
}

// generate device class types
const enumsModels = readdirSync(
  `${BASEDIR}/generator/input/device_classes`
).map((deviceDocFile) => {
  const name = deviceDocFile.replace(".markdown", "");
  return extractDeviceClassesEnums(
    name,
    `${BASEDIR}/generator/input/device_classes/${deviceDocFile}`
  );
});
const template = readFileSync(
  `${BASEDIR}/generator/src/rust_device_classes.mustache`
).toString();
const output = Handlebars.compile(template)(enumsModels);
writeFileSync(`${BASEDIR}/src/mqtt/device_classes.rs`, output);

// generate mod.rs
const templateMod = readFileSync(
  `${BASEDIR}/generator/src/rust_mod.mustache`
).toString();
const outputMod = Handlebars.compile(templateMod)(ENTITIES);
writeFileSync(`${BASEDIR}/src/mqtt/mod.rs`, outputMod);