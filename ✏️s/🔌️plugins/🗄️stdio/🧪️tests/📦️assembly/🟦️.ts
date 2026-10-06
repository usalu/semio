import assert from "node:assert/strict";
import { parse } from "@iarna/toml";
import { readFileSync } from "node:fs";

/** 📦️ Validates neutral authored assembly vectors with independent schema and TOML readers. */
export function runAssemblyChecks(): number {
  const load = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const fixture = load("../../🧫️fixtures/📦️assembly/🔣️.json");
  assert.equal(new Set(fixture.cases.map((row: { id: string }) => row.id)).size, fixture.cases.length);
  let checks = 1;
  for (const row of fixture.cases) {
    let selected: string[] = [...row.selected];
    let admitted = true;
    for (const owner of row.remove) {
      const candidate = selected.filter((id) => id !== owner);
      const available = new Set(candidate.map((id) => `s.stdio.${id}`));
      const valid = candidate.every((id) => load(`../../📇️registry/🧬️contract/🧫️fixtures/📇️contributions/${id}.json`).dependencies.every((id: string) => available.has(id)));
      admitted &&= valid;
      if (valid) selected = candidate;
    }
    assert.equal(admitted, row.accepted);
    assert.deepEqual(selected, row.remaining);
    checks++;
  }
  const cargo = readFileSync(new URL("../../📦️packages/🦀️rust/Cargo.toml", import.meta.url), "utf8");
  const manifest = Bun.TOML.parse(cargo) as any;
  assert.deepEqual(manifest, parse(cargo));
  assert.equal(manifest.package.metadata.component, undefined);
  assert.deepEqual(Object.keys(manifest.dependencies).sort(), ["semio-framework-plugin", "semio-s-artifact-stdio-contract"]);
  checks++;
  const source = readFileSync(new URL("../../🦀️.rs", import.meta.url), "utf8");
  assert.equal(/selected_contributions|plugin_exports!|semio_s_artifact_stdio_(?!contract)|include_str!/u.test(source), false);
  checks++;
  const capabilityRoot = "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability";
  const capability = load(`${capabilityRoot}/🧫️fixtures/🔣️.json`);
  for (const row of capability.cases) {
    assert.equal(row.native, true);
    assert.equal(row.sqlite, row.relational);
    checks++;
  }
  return checks;
}
