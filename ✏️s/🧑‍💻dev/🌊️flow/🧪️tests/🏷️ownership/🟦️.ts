/** 🌊️ Portable concrete Flow law ownership checked against actual native law declarations. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
const owner = fileURLToPath(new URL("../..", import.meta.url));
export function flowCompositionLaws() {
  const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8"));
  return fixture.tests as { target: string; laws: string[] }[];
}
export function testFlowCompositionOwnership() {
  const laws = flowCompositionLaws();
  for (const group of laws) {
    const directory = group.target === "flow_catalogue" ? "🌿️catalogue" : "🔌️port-types";
    const source = readFileSync(join(owner, `🧪️tests/${directory}/🦀️.rs`), "utf8");
    assert.deepEqual([...source.matchAll(/#\[test\]\s*fn (\w+)/g)].map(match => match[1]), group.laws);
  }
  const repo = findWorkspaceRoot(owner);
  const framework = join(repo, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow");
  assert(!readFileSync(join(framework, "📦️packages/🦀️rust/Cargo.toml"), "utf8").includes("semio-s-plugin-flow-extension-"));
  assert(!readModuleTreeSource(join(framework, "🖥️host/🧪️tests/🔬️unit/🦀️.rs")).includes("install_first_party_light"));
  console.log(`Flow composition ownership: ${laws.reduce((sum, group) => sum + group.laws.length, 0)} exact native laws`);
}
