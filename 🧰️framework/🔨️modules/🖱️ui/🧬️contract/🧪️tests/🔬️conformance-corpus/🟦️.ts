/** 🔬️ Canonical conformanceCorpusSelfTests fixture and oracle checks. */
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { readFileSync, readdirSync } from "node:fs";

const packageRoot = fileURLToPath(new URL("../../📦️packages/🦀️rust/", import.meta.url));

/** 🔍️ Validates the language-neutral corpus catalog with Ajv and exact filesystem ownership. */
export function conformanceCorpusSelfTests(): number {
  const root = join(packageRoot, "../../🧫️fixtures/🧪️conformance");
  const catalog = JSON.parse(readFileSync(join(root, "📇️catalog.json"), "utf8")) as { version: number; roles: { snapshot: string; expect: string; patch: string }; groups: Record<string, { patch: boolean; cases: Record<string, string> }> };
  const Ajv = createRequire(import.meta.url)("ajv");
  const contractModule = JSON.parse(readFileSync(join(packageRoot, "../../🧬️schema/🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", minItems: 1, items: { type: "string" } } }).addSchema(contractModule).getSchema(`${contractModule.$id}#/$defs/ConformanceCatalogFixture`);
  assert(validate(catalog), JSON.stringify(validate.errors));
  assert(!validate({ ...catalog, roles: { ...catalog.roles, snapshot: "snapshot.json" } }));
  assert.deepEqual(readdirSync(root).sort(), [...Object.keys(catalog.groups), "📇️catalog.json"].sort());
  let count = 0;
  for (const [group, definition] of Object.entries(catalog.groups)) {
    assert.equal(new Set(Object.values(definition.cases)).size, Object.keys(definition.cases).length);
    assert.deepEqual(readdirSync(join(root, group)).sort(), Object.values(definition.cases).sort());
    for (const [id, directory] of Object.entries(definition.cases)) {
      const roles = [catalog.roles.snapshot, catalog.roles.expect, ...(definition.patch ? [catalog.roles.patch] : [])];
      assert.deepEqual(readdirSync(join(root, group, directory)).sort(), roles.sort());
      assert.equal(JSON.parse(readFileSync(join(root, group, directory, catalog.roles.expect), "utf8")).case, id);
      count++;
    }
  }
  assert.equal(count, 62);
  return count;
}
