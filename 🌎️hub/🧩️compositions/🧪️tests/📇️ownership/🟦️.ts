import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import TOML from "@iarna/toml";

/** 🏘️ Authored deployment owners for concrete native components: each a cdylib-only `🧩️component` crate beside its rlib library. */
export function assertConcreteCompositionOwnership(root: string): string[] {
  const fixture = JSON.parse(readFileSync(join(root, "🧪️tests/🧫️fixtures/📇️ownership/🔣️.json"), "utf8")) as { components: { package: string; owner: string; component: string; role: string; componentKind: string }[] };
  const repoRoot = resolve(root, "../..");
  const names = new Set<string>();
  for (const row of fixture.components) {
    assert.ok(!names.has(row.package), row.package);
    names.add(row.package);
    const owner = join(repoRoot, row.owner);
    const source = readFileSync(join(owner, "📦️packages/🦀️rust/Cargo.toml"), "utf8");
    const manifest = Bun.TOML.parse(source) as { package: { name: string; metadata: { component: { package: string }; semio: { role: string; "component-kind": string; sources: { artifacts: string[] } } } }; lib: { "crate-type": string[] } };
    assert.deepEqual(manifest, TOML.parse(source));
    assert.deepEqual(manifest.lib["crate-type"], ["cdylib"], `${row.package} is a component crate: cdylib only`);
    const library = Bun.TOML.parse(readFileSync(join(owner, "../📦️packages/🦀️rust/Cargo.toml"), "utf8")) as { package: { name: string; metadata?: { component?: unknown } }; lib?: Record<string, unknown> };
    assert.equal(`${library.package.name}-component`, row.package, `${row.package} is the component of the library beside it`);
    assert.ok(!library.package.metadata?.component && !library.lib?.["crate-type"], `${library.package.name} is a library: rlib only, no component identity`);
    assert.equal(manifest.package.name, row.package);
    assert.equal(manifest.package.metadata.component.package, row.component);
    assert.equal(manifest.package.metadata.semio.role, row.role);
    assert.equal(manifest.package.metadata.semio["component-kind"], row.componentKind);
    for (const path of manifest.package.metadata.semio.sources.artifacts) assert.ok(existsSync(resolve(owner, "📦️packages/🦀️rust", path)), path);
    assert.ok(existsSync(join(owner, "🛂️.descriptor.semio")), row.component);
  }
  return [...names];
}
