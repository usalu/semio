import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

import { generatePlaygroundRegistry } from "../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts";

/** 🌐️ Every destination is authored or supplied by the actual release producer. */
export async function testPlaygroundSiteDistDefaults(): Promise<void> {
  const catalog = generatePlaygroundRegistry();
  assert.equal(new Set(catalog.map(row => row.variant)).size, catalog.length);
  for (const row of catalog) if (row.distDir !== undefined) assert.ok(row.distDir.length > 0);
}

/** 🏗️ Every plugin playground crate gets Nx `build` / `build-<variant>-site` targets wired to framework-os-dev release builds. */
export async function testPluginSiteNxTargets(workspace: string): Promise<void> {
  const registry = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry";
  const paths = JSON.parse(readFileSync(join(workspace, registry, "🤖️generated/🔌️plugins.json"), "utf8")).map((row: { cratePath: string }) => `${row.cratePath}/Cargo.toml`);
  const { cacheInternals } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
  const catalog = cacheInternals.collectPlaygroundCatalog(paths, workspace);
  const crates = [...new Set(catalog.map((row: { cratePath: string }) => row.cratePath))].sort();
  for (const cratePath of crates) {
    const targets = cacheInternals.pluginSiteTargetsForCrate(cratePath, catalog, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript");
    assert.ok(targets.build, `${cratePath}: missing build target`);
    const variants = catalog.filter((row: { cratePath: string }) => row.cratePath === cratePath).map((row: { variant: string }) => row.variant);
    for (const variant of variants) {
      const site = `build-${variant}-site`;
      assert.ok(targets[site], `${cratePath}: missing ${site}`);
      assert.ok(targets[site].dependsOn.includes(`@semio-tech/framework-os-dev:build-${variant}-react-release`), `${site}: wrong dependsOn`);
    }
    assert.deepEqual(targets.build.dependsOn.sort(), variants.map((variant: string) => `build-${variant}-site`).sort(), `${cratePath}: build aggregates variant site targets`);
  }
  const releaseNames = catalog.map((row: { variant: string }) => `build-${row.variant}-react-release`);
  assert.equal(new Set(releaseNames).size, catalog.length, "each playground variant must map to one CDN release Nx target on framework-os-dev");
}

/** 🌐️ Asserts a built plugin tree is CDN-deployable (GitHub Pages / static host conventions). */
export function assertCdnDeploySurface(distRoot: string): void {
  for (const file of ["index.html", "404.html", "favicon.ico"]) {
    assert.ok(existsSync(join(distRoot, file)), `${distRoot}: missing ${file}`);
  }
  assert.ok(existsSync(join(distRoot, "🧶️bundles")), `${distRoot}: missing 🧶️bundles/`);
}
