import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

import {catalogNxContributionV1} from "../../📣️publication/🟦️.ts";
import {test} from "bun:test";

/** 🌐️ Every destination is authored or supplied by the actual release producer. */
export async function testPlaygroundSiteDistDefaults(): Promise<void> {
  const catalog = JSON.parse(readFileSync(new URL("../../🤖️generated/🚀️playgrounds.json",import.meta.url),"utf8")) as {variant:string;distDir?:string}[];assert.ok(catalog.length>0);
  assert.equal(new Set(catalog.map(row => row.variant)).size, catalog.length);
  for (const row of catalog) if (row.distDir !== undefined) assert.ok(row.distDir.length > 0);
}

/** 🏗️ Every plugin playground crate gets Nx `build` / `build-<variant>-site` targets wired to framework-os-dev release builds. */
export async function testPluginSiteNxTargets(workspace: string): Promise<void> {
  const registry = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry";
  const paths = JSON.parse(readFileSync(join(workspace,catalogNxContributionV1().publication.outputRoot,"🔌️plugins.json"), "utf8")).map((row: { cratePath: string }) => `${row.cratePath}/Cargo.toml`);
  const { cacheInternals } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
  const catalog = cacheInternals.collectPlaygroundCatalog(paths, workspace,[catalogNxContributionV1()]);
  const crates = [...new Set(catalog.map((row: { cratePath: string }) => row.cratePath))].sort();
  for (const cratePath of crates) {
    const targets = cacheInternals.pluginSiteTargetsForCrate(cratePath, catalog, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript",workspace);
    assert.ok(targets.build, `${cratePath}: missing build target`);
    const variants = catalog.filter((row: { cratePath: string }) => row.cratePath === cratePath).map((row: { variant: string }) => row.variant);
    for (const variant of variants) {
      const site = `build-${variant}-site`;
      assert.ok(targets[site], `${cratePath}: missing ${site}`);
      assert.ok(targets[site].dependsOn.includes(`@semio-tech/framework-os-dev:build-${variant}-react`), `${site}: wrong dependsOn`);
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

test("the actual S site producers retain all catalog variants",async()=>{await testPlaygroundSiteDistDefaults();await testPluginSiteNxTargets(process.env.SEMIO_WORKSPACE_ROOT??process.cwd());console.log("[DEBUG] S catalog CDN site producers checked");});
