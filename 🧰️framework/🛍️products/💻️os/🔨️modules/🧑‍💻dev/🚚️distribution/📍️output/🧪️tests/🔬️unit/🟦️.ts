import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";

import { playgroundReactReleaseNxOutput, playgroundReactReleaseOutputPath } from "../../🟦️.ts";

/** 📦️ Proves authored distribution destinations with independent JSON schema and Node target producers. */
export async function testDistributionOutputContract(workspace: string, artifacts: string): Promise<void> {
  const root = resolve(import.meta.dir, "../..");
  const corpus = JSON.parse(readFileSync(join(root, "🧫️fixtures/🔣️.json"), "utf8"));
  
  const { cacheInternals } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
  mkdirSync(artifacts, { recursive: true });
  const temporary = mkdtempSync(join(artifacts, "distribution-output-"));
  try {
    for (const vector of corpus.cases) {
      const actual = (): string => playgroundReactReleaseOutputPath(temporary, join(temporary, corpus.stagingRoot), vector.row);
      const nx = (): string => playgroundReactReleaseNxOutput(corpus.stagingRoot, vector.row);
      if (!vector.valid) {
        assert.throws(actual, undefined, vector.id); assert.throws(nx, undefined, vector.id);
        assert.throws(() => cacheInternals.pluginSiteTargetsForCrate("owners/component/package", [{ ...vector.row, cratePath: "owners/component/package" }], corpus.stagingRoot), undefined, vector.id);
        continue;
      }
      assert.equal(relative(temporary, actual()).replaceAll("\\", "/"), vector.path, vector.id);
      assert.equal(new URL(vector.path, pathToFileURL(`${temporary}/`)).pathname, pathToFileURL(actual()).pathname, vector.id);
      assert.equal(nx(), `{workspaceRoot}/${vector.path}`, vector.id);
      const target = cacheInternals.pluginSiteTargetsForCrate("owners/component/package", [{ ...vector.row, cratePath: "owners/component/package", pluginId: "neutral" }], corpus.stagingRoot);
      assert.deepEqual(target[`build-${vector.row.variant}-site`].outputs, [nx()], vector.id);
    }
    const configs: string[] = [];
    for (const owner of ["owners/neutral", "✏️s/🔌️plugins/🌿️neutral"]) {
      const file = `${owner}/Cargo.toml`;
      mkdirSync(dirname(join(temporary, file)), { recursive: true });
      writeFileSync(join(temporary, file), '[package]\nname="neutral"\n[package.metadata.component]\npackage="semio:neutral-' + configs.length + '"\n[package.metadata.semio]\ncomponent-kind="plugin"\n[[package.metadata.semio.playground]]\nvariant="counter-' + configs.length + '"\n');
      configs.push(file);
    }
    const rows = cacheInternals.collectPlaygroundCatalog(configs, temporary);
    assert.equal(rows.length, 2);
    assert.ok(rows.every((row: any) => row.distDir === undefined));
    for (const row of rows) assert.equal(playgroundReactReleaseOutputPath(temporary, join(temporary, corpus.stagingRoot), row), join(temporary, corpus.stagingRoot, "dist", `build-${row.variant}-react-release`));
    const nodeSource = `import { readFileSync } from 'node:fs';
      import { pathToFileURL } from 'node:url';
      const { cacheInternals } = await import(pathToFileURL(process.argv[1]).href);
      const corpus = JSON.parse(readFileSync(process.argv[2], 'utf8'));
      const result = corpus.cases.map(v => {
        try { const rows = cacheInternals.pluginSiteTargetsForCrate('owners/component/package', [{ ...v.row, cratePath: 'owners/component/package' }], corpus.stagingRoot); return { id: v.id, valid: true, outputs: rows['build-' + v.row.variant + '-site'].outputs }; }
        catch { return { id: v.id, valid: false }; }
      });
      process.stdout.write(JSON.stringify(result));`;
    const nodeRows = JSON.parse(execFileSync("node", ["--input-type=module", "-e", nodeSource, join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs"), join(root, "🧫️fixtures/🔣️.json")], { encoding: "utf8" }));
    assert.deepEqual(nodeRows, corpus.cases.map((v: any) => ({ id: v.id, valid: v.valid, ...(v.valid ? { outputs: [`{workspaceRoot}/${v.path}`] } : {}) })));
    console.log(`Distribution output: ${corpus.cases.length} portable vectors, 2 owner layouts, independent Ajv/URL/Node producer passed`);
  } finally { rmSync(temporary, { recursive: true, force: true }); }
}
