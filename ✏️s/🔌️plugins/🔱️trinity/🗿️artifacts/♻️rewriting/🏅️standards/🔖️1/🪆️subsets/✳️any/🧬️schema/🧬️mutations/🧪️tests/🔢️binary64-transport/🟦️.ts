/** 🔢️ LAW (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, coordinator P1 of 10-03): every binary64 input of a rewriting mutation
 * leaf is the canonical `framework/value/schema.json#/$defs/Binary64Transport` — its exact word or a plain number — never the
 * word-only `Binary64`, so the plain number a history edit writes validates exactly where the committed word does. ajv is the
 * third-party validator; the Rust twin is `a_history_edit_of_a_binary64_offset_validates_and_replays` (editor unit tests).
 * @see ../../🫳️drag-rule/🧬️schema/🔣️.json
 * @see ../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json */
import {test,expect} from "bun:test";
import {existsSync,readdirSync,readFileSync} from "node:fs";
import {semioSchemaAjvV1} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";

type Json = null | boolean | number | string | Json[] | {[key: string]: Json};
const TRANSPORT = "https://json.schemas.assets.semio-tech.com/framework/value/schema.json#/$defs/Binary64Transport";
const LEAVES = new URL("../../", import.meta.url);
const FIXTURES = new URL("../../../../🧫️fixtures/🧬️mutations/", import.meta.url);
const VALUE = new URL("../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json", import.meta.url);
const read = (url: URL): Json => JSON.parse(readFileSync(url, "utf8"));
const folders = (root: URL) => readdirSync(root, {withFileTypes: true}).filter((entry) => entry.isDirectory()).map((entry) => entry.name);
const leaves = folders(LEAVES).filter((leaf) => existsSync(new URL(`${leaf}/🧬️schema/🔣️.json`, LEAVES))).map((leaf) => ({leaf, schema: read(new URL(`${leaf}/🧬️schema/🔣️.json`, LEAVES)) as {[key: string]: Json}}));

function refs(node: Json, at: string, out: [string, string][]): [string, string][] {
  if (Array.isArray(node)) node.forEach((item, index) => refs(item, `${at}/${index}`, out));
  else if (node && typeof node === "object") for (const [key, value] of Object.entries(node)) key === "$ref" && typeof value === "string" ? out.push([at, value]) : refs(value, `${at}/${key}`, out);
  return out;
}

function sites(schema: Json, payload: Json, at: string, out: string[]): string[] {
  if (!schema || typeof schema !== "object" || Array.isArray(schema)) return out;
  if (schema.$ref === TRANSPORT) {
    out.push(at);
    return out;
  }
  if (schema.properties && payload && typeof payload === "object" && !Array.isArray(payload)) for (const [key, property] of Object.entries(schema.properties as {[key: string]: Json})) if (key in payload) sites(property, payload[key]!, `${at}/${key}`, out);
  if (schema.items && Array.isArray(payload)) payload.forEach((item, index) => sites(schema.items!, item, `${at}/${index}`, out));
  return out;
}

function replaced(payload: Json, pointer: string, value: Json): Json {
  const copy = structuredClone(payload) as {[key: string]: Json};
  const keys = pointer.split("/").slice(1);
  const parent = keys.slice(0, -1).reduce<Json>((node, key) => (node as {[key: string]: Json})[key]!, copy) as {[key: string]: Json};
  parent[keys.at(-1)!] = value;
  return copy;
}

test("every binary64 input of a rewriting leaf is the canonical Binary64Transport, never the word-only Binary64", () => {
  const census = leaves.flatMap(({schema}) => refs(schema, "", []).map(([at, ref]) => ({kind: String(schema.$id).split("/mutation/")[1]!.split("/")[0]!, at, ref})));
  expect(census.filter(({ref}) => /#\/\$defs\/Binary64$/.test(ref))).toEqual([]);
  expect(census.filter(({ref}) => ref === TRANSPORT).map(({kind, at}) => `${kind} ${at}`).sort()).toEqual([
    "change-rule-layout-point /properties/newPoint/properties/x",
    "change-rule-layout-point /properties/newPoint/properties/y",
    "drag-rule-nodes /properties/dx",
    "drag-rule-nodes /properties/dy",
    "set-rule-layout-points /properties/points/items/properties/x",
    "set-rule-layout-points /properties/points/items/properties/y",
  ]);
});

test("a history edit's plain number validates wherever the committed word does; a string or a malformed word never does", () => {
  const ajv = semioSchemaAjvV1({allErrors: true});
  ajv.addSchema(read(VALUE) as object);
  const edited = leaves.filter(({schema}) => refs(schema, "", []).some(([, ref]) => ref === TRANSPORT));
  let checked = 0;
  for (const {leaf, schema} of edited) {
    const validate = ajv.compile(schema);
    const cases = folders(new URL(`${leaf}/`, FIXTURES));
    expect(cases.length).toBeGreaterThan(0);
    for (const fixture of cases) {
      const payload = read(new URL(`${leaf}/${fixture}/🦠️mutation/🔣️.json`, FIXTURES));
      expect(validate(payload)).toBe(true);
      const pointers = sites(schema, payload, "", []);
      expect(pointers.length).toBeGreaterThan(0);
      for (const pointer of pointers) {
        expect(validate(replaced(payload, pointer, 50))).toBe(true);
        expect(validate(replaced(payload, pointer, -0.125))).toBe(true);
        expect(validate(replaced(payload, pointer, {bits: "4049000000000000"}))).toBe(true);
        expect(validate(replaced(payload, pointer, "50"))).toBe(false);
        expect(validate(replaced(payload, pointer, {bits: "50"}))).toBe(false);
        expect(validate(replaced(payload, pointer, {bits: "4049000000000000", value: 50}))).toBe(false);
        checked += 1;
      }
    }
  }
  expect(edited.length).toBe(3);
  expect(checked).toBe(6);
});
