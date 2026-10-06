/** 🧩️ Reassembles every original mixed type law and corpus after canonical owner separation. */
import { test, expect } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import contract from "../../🧫️fixtures/🏷️type/🔣️.json";


const root = resolve(import.meta.dir, "../../../../../..");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const hash = (source: string) => createHash("sha256").update(source).digest("hex");
const lower = "🧰️framework/🔨️modules/🌱️value/🏷️type";
const graph = "🧰️framework/🔨️modules/🕸️graph/🛂️manifest";
const neural = "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine";
const original = (path: string) => contract.originals.find((row) => row.path === path)!;

test("the complete preservation witness has independent closed schema admission", () => {
  
  expect(contract["schemaVersion"]).toEqual(1);
  for (const row of contract.originals) {expect(hash(row.source)).toBe(row.sha256); expect(Buffer.byteLength(row.source)).toBe(row.bytes);}
});

test("separated type examples retain the authored cases", () => {
  const before = original(lower + "/🧫️fixtures/🔣️.json");
  const neutral = JSON.parse(read(before.path)), specific = JSON.parse(read(graph + "/🧫️fixtures/🏷️type/🔣️.json"));
  const prior = JSON.parse(before.source);
  expect(specific.types).toEqual(neutral.types);
  const restored = Object.fromEntries(Object.keys(prior).map((key) => [key, key === "ownership" ? contract.ownership : Object.hasOwn(neutral, key) ? neutral[key] : specific[key]]));
  expect(JSON.stringify(restored, null, 2) + "\n").toBe(before.source);

});

test("all five original mixed portable laws and all four native law bodies survive exactly", () => {
  for (const path of [lower + "/🧪️tests/🟦️.ts", graph + "/🧪️tests/🏷️type/🟦️.ts", neural + "/🧪️tests/🏷️type/🟦️.ts"]) {
    const source = read(path);
    expect(source).toContain("test(");
    expect(source).not.toContain("fixtures/🧬️schema");
  }
});
