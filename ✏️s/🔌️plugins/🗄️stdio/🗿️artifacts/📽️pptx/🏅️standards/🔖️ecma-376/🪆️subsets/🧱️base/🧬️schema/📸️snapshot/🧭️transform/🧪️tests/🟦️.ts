import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";
import corpus from "../🧫️fixtures/🔣️.json";

const base = resolve(import.meta.dir, "../../../..");
const read = (path: string) => readFileSync(resolve(base, path), "utf8");
const snapshot = JSON.parse(read("🧬️schema/📸️snapshot/🔣️.json"));
const mutationSchemas = ["📐set-shape-position", "📸️set-snapshot", "➕insert-slide", "🔷insert-shape"].map(kind => JSON.parse(read("🧬️schema/🧬️mutations/" + kind + "/🧬️schema/🔣️.json")));
const ajv = new Ajv({ strict: true }).addKeyword({ keyword: "x-semio-ui", schemaType: "object", valid: true });
const validateTransform = ajv.compile(snapshot.$defs.PptxTransform);
const validateScalars = Object.fromEntries(corpus.keys.map(key => [key, ajv.compile(snapshot.$defs.PptxTransform.properties[key])]));



test("actual schema syntax and independent signed64 scalar reference preserve exact boundaries", () => {
  expect(mutationSchemas[0].properties.position.$ref).toBe(snapshot.$id + "#/$defs/PptxTransform");
  expect(mutationSchemas[1].properties.snapshot.$ref).toBe(snapshot.$id);
  expect(mutationSchemas[2].properties.entry.$ref).toBe("https://json.schemas.assets.semio-tech.com/s/stdio/xml/1.0/base/snapshot.json#/$defs/XmlNode");
  expect(mutationSchemas[3].properties.shape.$ref).toBe(mutationSchemas[2].properties.entry.$ref);
  const child = spawnSync(process.env.SEMIO_NODE_EXECUTABLE ?? "node", ["--input-type=module", "-e", `let input="";for await(const chunk of process.stdin)input+=chunk;console.log(JSON.stringify(JSON.parse(input).map(({wire})=>{if(typeof wire!=="string"||!/^(-?[1-9][0-9]*|0)$/.test(wire)||wire.length>20)return false;const n=BigInt(wire);return n>=-(1n<<63n)&&n<(1n<<63n)&&String(n)===wire;})));`], { input: JSON.stringify(corpus.vectors), encoding: "utf8", timeout: 4000 });
  expect(child.status).toBe(0);
  const reference: boolean[] = JSON.parse(child.stdout);
  for (const [index, row] of corpus.vectors.entries()) {
    const position = Object.fromEntries(corpus.keys.map(key => [key, row.wire]));
    expect(validateTransform(position)).toBe(row.schemaAccepted);
    expect(reference[index]).toBe(row.codecAccepted);
  }
  const codec = read("🧬️schema/📸️snapshot/🧭️transform/🦀️.rs");
  expect(codec).toContain("parse::<i64>()");
  expect(codec).toMatch(/value\.to_string\(\)\s*!=\s*text/);
  expect(codec).toContain("DslValue::String");
  console.info("pptx-transform-wire: 16 closed scalar rows; AJV schema syntax and independent Node signed64 boundaries");
});

test("both current mutation Examples tables carry only canonical typed and XML transform scalars", () => {
  const feature = read("🧪️tests/🧱️mutate-pptx-ecma-376/🥒️.feature");
  const rows = feature.split("\n").flatMap(line => {
    const match = line.match(/^\s*\|\s*([^|]+?)\s*\|\s*(\{.*\})\s*\|\s*$/);
    return match ? [{ id: match[1]!.trim(), params: JSON.parse(match[2]!) }] : [];
  });
  const scenarios = [...corpus.scenarios, "patch-snapshot"];
  expect(rows.map(row => row.id)).toEqual([...scenarios, ...scenarios]);
  let positions = 0, scalars = 0;
  const inspect = (value: unknown): void => {
    if (!value || typeof value !== "object") return;
    if (Array.isArray(value)) { for (const child of value) inspect(child); return; }
    const node = value as { name?: string; attrs?: { name: string; value: unknown }[] };
    if (["a:off", "a:ext"].includes(node.name ?? "")) for (const attribute of node.attrs ?? []) {
      expect(validateScalars[attribute.name]).toBeDefined();
      expect(validateScalars[attribute.name]!(attribute.value)).toBe(true);
      scalars++;
    }
    for (const [key, child] of Object.entries(value)) {
      if (key === "position") {
        expect(validateTransform(child)).toBe(true);
        positions++;
        scalars += corpus.keys.length;
      }
      inspect(child);
    }
  };
  for (const row of rows) inspect(row.params);
  expect(positions).toBe(2);
  expect(scalars).toBe(24);
  console.info("pptx-transform-wire: both original Examples tables, 18 scenarios and 24 exact typed/XML string scalars");
});
