import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";
import corpus from "../🧫️fixtures/🔣️.json";
import corpusSchema from "../🧬️schema/🔣️.json";

const base = resolve(import.meta.dir, "../../../..");
const read = (path: string) => readFileSync(resolve(base, path), "utf8");
const transforms = [
  "📐set-shape-position",
  "📸️set-snapshot",
  "➕insert-slide",
  "🔷insert-shape",
].map(kind => JSON.parse(read("🧬️schema/🧬️mutations/" + kind + "/🧬️schema/🔣️.json")).$defs.PptxTransform);
const ajv = new Ajv({ strict: true });
const validateTransform = ajv.compile(transforms[0]);

test("closed transform vectors retain every authored hostile witness", () => {
  const validate = ajv.compile(corpusSchema);
  expect(validate(corpus)).toBe(true);
  const replaced = structuredClone(corpus);
  replaced.vectors[13] = structuredClone(replaced.vectors[0]!);
  expect(validate(replaced)).toBe(false);
  expect(validate({ ...corpus, vectors: corpus.vectors.slice(1) })).toBe(false);
});

test("actual schema syntax and independent signed64 scalar reference preserve exact boundaries", () => {
  for (const transform of transforms) expect(transform).toEqual(transforms[0]);
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
  expect(codec).toContain("value.to_string()!=text");
  expect(codec).toContain("DslValue::String");
  console.info("pptx-transform-wire: 16 closed scalar rows; AJV schema syntax and independent Node signed64 boundaries");
});

test("both original eight-row Examples tables carry only canonical transform scalars", () => {
  const feature = read("🧪️tests/🧱️mutate-pptx-ecma-376/🥒️.feature");
  const rows = feature.split("\n").flatMap(line => {
    const match = line.match(/^\s*\|\s*([^|]+?)\s*\|\s*(\{.*\})\s*\|\s*$/);
    return match ? [{ id: match[1]!.trim(), params: JSON.parse(match[2]!) }] : [];
  });
  expect(rows.map(row => row.id)).toEqual([...corpus.scenarios, ...corpus.scenarios]);
  let positions = 0;
  const inspect = (value: unknown): void => {
    if (!value || typeof value !== "object") return;
    if (Array.isArray(value)) { for (const child of value) inspect(child); return; }
    for (const [key, child] of Object.entries(value)) {
      if (key === "position") {
        expect(validateTransform(child)).toBe(true);
        positions++;
      }
      inspect(child);
    }
  };
  for (const row of rows) inspect(row.params);
  expect(positions).toBe(8);
  console.info("pptx-transform-wire: both original Examples tables, 16 scenarios and 32 exact string scalars");
});
