/** 🧩️ Reassembles every original mixed type law and corpus after canonical owner separation. */
import { test, expect } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import contract from "../../🧫️fixtures/🏷️type/🔣️.json";
import schema from "../../🧬️schema/🏷️type/🔣️.json";

const root = resolve(import.meta.dir, "../../../../../..");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const hash = (source: string) => createHash("sha256").update(source).digest("hex");
const lower = "🧰️framework/🔨️modules/🌱️value/🏷️type";
const graph = "🧰️framework/🔨️modules/🕸️graph/🛂️manifest";
const neural = "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine";
const original = (path: string) => contract.originals.find((row) => row.path === path)!;

test("the complete preservation witness has independent closed schema admission", () => {
  const validate = new Ajv({strict: true, allErrors: true}).compile(schema);
  expect(validate(contract)).toBe(true);
  for (const row of contract.originals) {expect(hash(row.source)).toBe(row.sha256); expect(Buffer.byteLength(row.source)).toBe(row.bytes);}
  for (const candidate of [{...contract, unknown: true}, {...contract, parts: []}, {...contract, originals: []}, {...contract, ownership: {...contract.ownership, unknown: true}}]) expect(validate(candidate)).toBe(false);
});

test("separated corpora and schemas reproduce every original combined byte", () => {
  const before = original(lower + "/🧫️fixtures/🔣️.json"), beforeSchema = original(lower + "/🧬️schema/🔣️.json");
  const neutral = JSON.parse(read(before.path)), specific = JSON.parse(read(graph + "/🧫️fixtures/🏷️type/🔣️.json"));
  const prior = JSON.parse(before.source);
  expect(specific.types).toEqual(neutral.types);
  const restored = Object.fromEntries(Object.keys(prior).map((key) => [key, key === "ownership" ? contract.ownership : Object.hasOwn(neutral, key) ? neutral[key] : specific[key]]));
  expect(JSON.stringify(restored, null, 2) + "\n").toBe(before.source);
  const neutralSchema = JSON.parse(read(beforeSchema.path)), specificSchema = JSON.parse(read(graph + "/🧬️schema/🏷️type/🔣️.json")), priorSchema = JSON.parse(beforeSchema.source);
  const restoredSchema = {...neutralSchema, properties: Object.fromEntries(Object.keys(priorSchema.properties).map((key) => [key, key === "ownership" ? schema.properties.ownership : Object.hasOwn(neutralSchema.properties, key) ? neutralSchema.properties[key] : specificSchema.properties[key]])), required: priorSchema.required};
  expect(JSON.stringify(restoredSchema, null, 2) + "\n").toBe(beforeSchema.source);
});

test("all five original mixed portable laws and all four native law bodies survive exactly", () => {
  const sources = {value: read(lower + "/🧪️tests/🟦️.ts"), graph: read(graph + "/🧪️tests/🏷️type/🟦️.ts"), neural: read(neural + "/🧪️tests/🏷️type/🟦️.ts")};
  expect(sources.value.startsWith(contract.lowerPrefix)).toBe(true);
  const retained = contract.parts.slice(0, 7).map((row) => {const source = sources[row.owner as keyof typeof sources]; expect(source.split(row.text).length).toBe(2); return source.slice(source.indexOf(row.text), source.indexOf(row.text) + row.text.length);});
  const restored = contract.originalPrefix + contract.mixedHeader + retained.slice(0, 6).join("") + contract.closing + retained[6];
  expect(restored).toBe(original(lower + "/🧪️tests/🟦️.ts").source);
  const lowerNative = original(lower + "/🧪️tests/🦀️.rs");
  expect(read(lowerNative.path)).toBe(lowerNative.source);
  const binding = contract.nativeBinding, graphNative = read(binding.path);
  expect(graphNative.split(binding.current).length).toBe(2);
  expect(graphNative.replace(binding.current, binding.previous)).toBe(original(binding.path).source);
});
