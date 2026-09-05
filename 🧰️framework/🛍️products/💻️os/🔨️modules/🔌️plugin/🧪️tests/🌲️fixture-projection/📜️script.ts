/** 🌲️ Neutral observation outcomes and exact retained-tree integration guards. */
import Ajv from "ajv";
import { strict as assert } from "node:assert";
import { readFileSync } from "node:fs";
import stableStringify from "fast-json-stable-stringify";

export function testFixtureProjectionRetirement(): void {
  const fixture = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(new URL("./🧬️schema.json", import.meta.url), "utf8")));
  assert(validate(fixture), JSON.stringify(validate.errors));
  for (const invalid of [{ ...fixture, terminalBeforeOutcome: false }, { ...fixture, cases: ["success"] }, { ...fixture, reservedPages: 383 }]) assert(!validate(invalid));
  assert.deepEqual(JSON.parse(stableStringify(fixture.foreign)), fixture.foreign);
  assert.equal(new TextEncoder().encode(fixture.foreign.foreign[0]).length, Buffer.byteLength(fixture.foreign.foreign[0]));
  const source = readFileSync(new URL("../../🦀️.rs", import.meta.url), "utf8");
  assert(source.includes("BuiltTreeRetirement::new(tree.root)"), "fixture observation must retain the exact complete typed tree");
  const start = source.indexOf("fn observe_and_retire_fixture_tree");
  assert(start >= 0);
  const body = source.slice(start, source.indexOf("/// 🧪️ Mounts", start));
  assert(body.includes("catch_unwind") && body.includes("terminal_is_empty()") && body.includes("resume_unwind"));
  assert(!body.includes("close_built_node_page_one") && !body.includes("close_ui_value_page"));
  console.log("[DEBUG] fixture observation source:3 outcomes,3 denials,384 exact pages; native panic retirement remains separate");
}
