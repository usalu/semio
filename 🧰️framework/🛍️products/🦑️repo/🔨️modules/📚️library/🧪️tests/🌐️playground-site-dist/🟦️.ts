import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import { resolvePlaygroundDistDir } from "../../../../../💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📍️output/🟦️.ts";
import { generatePlaygroundRegistry } from "../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts";

/** @emoji 🌐️ Language-agnostic contract for plugin CDN dist directory defaults. */
export async function testPlaygroundSiteDistDefaults(): Promise<void> {
  const fixture = [
    { pluginId: "energy", variant: "energy", distDir: undefined },
    { pluginId: "fem", variant: "fem2d", distDir: undefined },
    { pluginId: "fem", variant: "fem3d", distDir: "✏️s/🔌️plugins/🏗️fem/dist" },
  ];
  assert.equal(
    resolvePlaygroundDistDir({ pluginId: "energy", variant: "energy", cratePath: "✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust" }, fixture),
    "✏️s/🔌️plugins/🔋️energy/dist",
  );
  assert.equal(
    resolvePlaygroundDistDir({ pluginId: "fem", variant: "fem2d", cratePath: "✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust" }, fixture),
    "✏️s/🔌️plugins/🏗️fem/dist/fem2d",
  );
  assert.equal(
    resolvePlaygroundDistDir({ pluginId: "fem", variant: "fem3d", distDir: "✏️s/🔌️plugins/🏗️fem/dist", cratePath: "✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust" }, fixture),
    "✏️s/🔌️plugins/🏗️fem/dist",
  );
  const catalog = generatePlaygroundRegistry();
  const withoutDist = catalog.filter((row) => !row.distDir?.startsWith("✏️s/🔌️plugins/"));
  assert.equal(withoutDist.length, 0, `every playground needs a plugin distDir: ${withoutDist.map((row) => row.variant).join(", ")}`);
  console.log(`[DEBUG] ${catalog.length} playground variants resolve plugin CDN distDir PASS`);
  console.log("[DEBUG] Playground CDN dist defaults match energy.semio-tech.com and 3d.fem.semio-tech.com layout PASS");
}

/** @emoji 🏗️ Every plugin playground crate gets Nx `build` / `build-<variant>-site` targets wired to framework-os-dev release builds. */
export async function testPluginSiteNxTargets(workspace: string): Promise<void> {
  const registry = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry";
  const paths = JSON.parse(readFileSync(join(workspace, registry, "🤖️generated/🔌️plugins.json"), "utf8")).map((row: { cratePath: string }) => `${row.cratePath}/Cargo.toml`);
  const { cacheInternals } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
  const catalog = cacheInternals.collectPlaygroundCatalog(paths, workspace);
  const crates = [...new Set(catalog.map((row: { cratePath: string }) => row.cratePath))].sort();
  for (const cratePath of crates) {
    const targets = cacheInternals.pluginSiteTargetsForCrate(cratePath, catalog);
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
  console.log(`[DEBUG] ${crates.length} plugin playground crates declare CDN site Nx targets PASS`);
  console.log(`[DEBUG] ${catalog.length} playground CDN release targets: build-<variant>-react-release PASS`);
}

/** @emoji 🌐️ Asserts a built plugin tree is CDN-deployable (GitHub Pages / static host conventions). */
export function assertCdnDeploySurface(distRoot: string): void {
  for (const file of ["index.html", "404.html", "favicon.ico"]) {
    assert.ok(existsSync(join(distRoot, file)), `${distRoot}: missing ${file}`);
  }
  assert.ok(existsSync(join(distRoot, "🧶️bundles")), `${distRoot}: missing 🧶️bundles/`);
}
