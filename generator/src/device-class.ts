import { readFileSync } from "fs";

type EnumValue = {
    value: string,
    description: string,
}

type DeviceClassesEnumModel = {
    name: string,
    values: EnumValue[]
}

export function extractDeviceClassesEnums(name: string, docFile: string): DeviceClassesEnumModel {
  console.log("Device class", name, docFile);
  const docContent = readFileSync(docFile).toString();
  const docEnumValuesParagraph = /^###* Device Class(?:es)?([^]*)/gim
    .exec(docContent)[1]
    .replace(/^###*[^]*/gm, "");

  const enumValues = docEnumValuesParagraph.split("\n").map((line) => {
    const match = /- (?<name>.*): (?<description>.*)/gm.exec(line);
    if (match?.groups) {
      return {
        value: match.groups.name.replace(/[^\w]/g,''),
        description: match.groups.description,
      }
    } else{return null;}
  }).filter((value) => !!value);
  return {
    name: name,
    values: enumValues,
  }
}
