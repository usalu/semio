/** 🌊️ Portable concrete Flow law ownership with an independent JSON Schema oracle. */
import assert from "node:assert/strict";
import { readFileSync, existsSync } from "node:fs";
import { resolve, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
const owner = fileURLToPath(new URL("../..", import.meta.url));
export function flowCompositionLaws() {
  const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true }).addKeyword("x-semio-formats").addSchema(schema);
  assert(ajv.getSchema(`${schema.$id}#/$defs/SFlowCompositionV1`)!(fixture));
  assert(ajv.getSchema(`${schema.$id}#/$defs/SFlowGeometryCompositionV1`)!(JSON.parse(readFileSync(join(owner,"🧫️fixtures/🌐️geometry-composition/🔣️.json"),"utf8"))));
  return fixture.tests as { target: string; laws: string[] }[];
}
export function testFlowCompositionOwnership() {
  const laws = flowCompositionLaws();
  for (const group of laws) {
    const directory = group.target === "flow_catalogue" ? "🌿️catalogue" : "🔌️port-types";
    const source = readFileSync(join(owner, `🧪️tests/${directory}/🦀️.rs`), "utf8");
    assert.deepEqual([...source.matchAll(/#\[test\]\s*fn (\w+)/g)].map(match => match[1]), group.laws);
  }
  let repo = owner;
  while (!existsSync(join(repo, "nx.json"))) repo = dirname(repo);
  const framework = join(repo, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow");
  assert(!readFileSync(join(framework, "📦️packages/🦀️rust/Cargo.toml"), "utf8").includes("semio-s-plugin-flow-extension-"));
  assert(!readFileSync(join(framework, "🖥️host/🧪️tests/🔬️unit/🦀️.rs"), "utf8").includes("install_first_party_light"));
  console.log(`Flow composition ownership: ${laws.reduce((sum, group) => sum + group.laws.length, 0)} exact native laws`);
}
