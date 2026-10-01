import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";
import TOML from "@iarna/toml";

/** 🏘️ Authored deployment owners for concrete native components. */
export function assertConcreteCompositionOwnership(root: string): string[] {
  const fixture = JSON.parse(readFileSync(join(root, "🧫️fixtures/📇️ownership/🔣️.json"), "utf8")) as { components: { package: string; owner: string; removedOwner: string; component: string; role: string; componentKind: string }[] };
  const schema = JSON.parse(readFileSync(join(root, "🧬️schema/📇️ownership/🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert.ok(validate(fixture), JSON.stringify(validate.errors));
  const repoRoot = resolve(root, "../..");
  const names = new Set<string>();
  for (const row of fixture.components) {
    assert.ok(!names.has(row.package), row.package);
    names.add(row.package);
    const owner = join(repoRoot, row.owner);
    const source = readFileSync(join(owner, "📦️packages/🦀️rust/Cargo.toml"), "utf8");
    const manifest = Bun.TOML.parse(source) as { package: { name: string; metadata: { component: { package: string }; semio: { role: string; "component-kind": string; sources: { artifacts: string[] } } } } };
    assert.deepEqual(manifest, TOML.parse(source));
    assert.equal(manifest.package.name, row.package);
    assert.equal(manifest.package.metadata.component.package, row.component);
    assert.equal(manifest.package.metadata.semio.role, row.role);
    assert.equal(manifest.package.metadata.semio["component-kind"], row.componentKind);
    for (const path of manifest.package.metadata.semio.sources.artifacts) assert.ok(existsSync(resolve(owner, "📦️packages/🦀️rust", path)), path);
    assert.ok(existsSync(join(owner, "🛂️.descriptor.semio")), row.component);
    assert.ok(!existsSync(join(repoRoot, row.removedOwner, "📦️packages/🦀️rust/Cargo.toml")), row.removedOwner);
    assert.ok(!existsSync(join(repoRoot, row.removedOwner, "🦀️.rs")), row.removedOwner);
  }
  return [...names];
}
