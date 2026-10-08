/** 🔢️ Genuine reusable testing mutation payloads use native scalar contracts and independent Ajv admission. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const contract = () => read("../../🧪️testing/🧬️schema/🔣️.json");
const examples = read("../../🧪️testing/🧫️fixtures/🔢️payload-contracts/🔣️.json");

test("native testing scalar contracts admit only actual signed, nullable and unsigned values", () => {
  const schema = contract(), ajv = new Ajv({ strict: true }).addSchema(schema);
  for (const row of examples.scalars) expect(ajv.compile({ $ref: schema.$id + "#/$defs/" + row.export })(row.value)).toBe(row.accepted);
});

test("testing mutation leaves resolve actual semantic scalar owners", () => {
  const ajv = new Ajv({ strict: true }).addSchema(contract());
  const prefix = "../../🧪️testing/🧬️mutations/";
  const schemas = [
    read(prefix + "🧮️demo/🧬️mutations/🔢️set-n/🧬️schema/🔣️.json"),
    read(prefix + "🧮️demo/🧬️mutations/↩️restore-n/🧬️schema/🔣️.json"),
    read(prefix + "⏱️timestamped/🧬️mutations/🔢️set-n/🧬️schema/🔣️.json")
  ];
  for (let index = 0; index < examples.mutations.length; index++) {
    const row = examples.mutations[index], schema = Object.hasOwn(row, "physicalMs") ? schemas[2] : row.operation === "restoreN" ? schemas[1] : schemas[0];
    expect(ajv.compile(schema)(row)).toBe(examples.mutationValidity[index]);
  }
});
