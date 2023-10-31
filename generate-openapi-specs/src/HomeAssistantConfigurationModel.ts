import { readFileSync } from "fs";
import yaml from "js-yaml";

const AVAILABILITY_PROPERTIES = [
  "availability_topic",
  "payload_available",
  "payload_not_available",
  "availability_template",
];

export type ConfigurationModel = Map<string, Property>;

export type Property = ObjectProperty | ListProperty | StringProperty;

export type PropertyType =
  | "map"
  | "list"
  | "string"
  | "boolean"
  | "integer"
  | "template"
  | "device_class";

export interface ObjectProperty {
  description: string | undefined;
  required: boolean;
  type: "map" | undefined;
  keys: Array<Property>;
}

export interface ListProperty {
  description: string;
  required: boolean;
  type: "list";
  keys: Array<Property> | undefined;
  default: undefined;
}

export interface StringProperty {
  description: string;
  required: boolean;
  type: "string" | "boolean" | "integer" | "template" | "device_class";
  default: string | undefined;
}
export class HomeAssistantConfiguration {
  readonly name: string;
  readonly description: string | undefined;
  readonly source: string;

  constructor(private readonly path: string) {
    this.name = path.replace(/^.*[/]/,"").replace(/.mqtt.markdown$/,"");
    const source = readFileSync(path, "utf-8");
    this.source = source
      .replace(/.*{% configuration %}/s, "")
      .replace(/{% endconfiguration %}.*/s, "");
    try {
      const altSource = readFileSync(
        path.replace(".mqtt.markdown", ".markdown"),
        "utf-8"
      );
      this.description = altSource
        .replace(/.*\n---\n/s, "")
        .split("\n")
        .filter((line) => line.trim().length !== 0)[0];
    } catch (e) {
      this.description = undefined;
    }
  }

  toOpenapiObjectModel(): Object {
    const config: ConfigurationModel = yaml.load(this.source, {
      json: true,
    });
    return toOpenapiObjectProperty({
      description: this.description,
      required: true,
      type: "map",
      keys: config as unknown as Array<ObjectProperty>,
    });
  }

  getAvailabilityModel(): Object {
    const model = this.toOpenapiObjectModel() as any;
    return {
      oneOf: [
        {
          allOf: [
            model.properties.availability,
            model.properties.availability_mode,
          ],
        },
        Object.fromEntries(
          Object.entries(model.properties).filter(([name, _]) =>{
            return AVAILABILITY_PROPERTIES.includes(name)
        }
          )
        ),
      ],
    };
  }

  getDeviceModel(): Object {
    const model = this.toOpenapiObjectModel() as any;
    return model.properties.device;
  }
}

function toOpenapiObjectProperty(propertyModel: ObjectProperty): Object {
  return {
    type: "object",
    description: describe(propertyModel),
    required: Object.entries(propertyModel.keys)
      .filter(([_, schema]) => schema.required === true)
      .map(([name, _]) => name),
    properties: Object.fromEntries(
      Object.entries(propertyModel.keys).map(([name, model]) => [
        name,
        toOpenapiObjectPropertyA(model),
      ])
    ),
  };
}

function toOpenapiObjectPropertyA(propertyModel: Property): Object {
  switch (propertyModel.type) {
    case undefined:
    case "map":
      return toOpenapiObjectProperty(
        propertyModel as unknown as ObjectProperty
      );
    case "list":
      return toOpenapiListProperty(propertyModel as unknown as ListProperty);
    case "device_class":
    // TODO return {};
    case "boolean":
    case "integer":
    case "template":
    case "string":
      return toOpenapiStringProperty(
        propertyModel as unknown as StringProperty
      );
  }
}

function toOpenapiListProperty(propertyModel: ListProperty): Object {
  return {
    description: describe(propertyModel),
    type: "array",
    items: !!propertyModel.keys
      ? Object.fromEntries(
          Object.entries(propertyModel.keys).map(([name, model]) => [
            name,
            toOpenapiObjectPropertyA(model),
          ])
        )
      : { type: "string" },
  };
}

function toOpenapiStringProperty(propertyModel: StringProperty): Object {
  return {
    description: describe(propertyModel),
    type: "string",
  };
}

function describe(prop: Property): string {
  const def =
    "default" in prop && prop.default
      ? `(Default: ${prop.default})`
      : undefined;
  return [prop.description, def].filter((s) => !!s).join(" ");
}
