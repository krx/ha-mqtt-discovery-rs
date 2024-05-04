import { readFileSync, writeFileSync, readdirSync } from "fs";
import YAML from "yaml";
import Handlebars from "handlebars";
import { toPascalCase } from "./strings";

const IGNORED_ATTRS = [
  "availability_mode",
  "availability_template",
  "availability_topic",
  "expire_after",
  "device",
  "entity_category",
  "",
];

type FieldAttributes = {
  description: string;
  required: boolean;
  type:
    | "template"
    | "string"
    | "icon"
    | "float"
    | "integer"
    | "boolean"
    | "list"
    | "map"
    | ["string", "list"];
  rustType?: string;
  import?: string;
  useInto?: boolean;
  rustSafeName?: string;
  keys?: any;
};

type MqttEntity = {
  entityName: string;
  entityDoc: string;
  properties: object;
};

export function generateMqttEntityModel(
  name: string,
  docFile: string
): MqttEntity {
  console.log(name, docFile);
  const docContent = readFileSync(docFile).toString();

  const modelDescriptorYaml =
    /{% configuration %}([^]*){% endconfiguration %}/gm.exec(docContent);
  try {
    const modelDescriptor = YAML.parse(modelDescriptorYaml!![1]);
    const entries = Object.entries(modelDescriptor)
      .filter(([name, attrs]) => !["list", "map"].includes(attrs.type))
      .filter(([name, attrs]) => !IGNORED_ATTRS.includes(name));
    for (const [name, attrs] of entries) {
      appendRustType(name, attrs as FieldAttributes);
    }

    return {
      entityName: name,
      entityDoc: docContent,
      properties: Object.fromEntries(entries),
    };
  } catch (e) {
    console.error(modelDescriptorYaml!![1]);
    throw e;
  }
}

function appendRustType(name: string, attrs: FieldAttributes) {
  if (name === "type") {
    attrs.rustSafeName = `r#${name}`;
  } else {
    attrs.rustSafeName = name;
  }
  switch (attrs.type) {
    case "template":
    case "string":
    case "icon":
      attrs.rustType = "String";
      attrs.useInto = true;
      break;
    case "float":
      attrs.rustType = "f32";
      break;
    case "integer":
      attrs.rustType = "i32";
      break;
    case "boolean":
      attrs.rustType = "bool";
      break;
    default:
      if (attrs.type.includes("list")) {
        attrs.rustType = "Vec<String>";
      }
  }
  switch (name) {
    case "device_class":
      const entityName = new RegExp(
        "/integrations/(?<name>[^/]*)/#device-class"
      ).exec(attrs.description)?.groups.name;
      if (entityName) {
        const deviceClassType = `${toPascalCase(entityName)}DeviceClass`;
        attrs.rustType = deviceClassType;
        attrs.import = `use super::device_classes::${deviceClassType}`;
      }
      break;
    case "unit_of_measurement":
      attrs.rustType = "Unit";
      attrs.import = `use super::units::Unit`;
      break;
      case "state_class":
        attrs.rustType = "SensorStateClass";
        attrs.import = `use super::common::SensorStateClass`;
        break;
    case "qos":
      attrs.rustType = "Qos";
      attrs.import = `use super::common::Qos`;
      break;
  }
}
