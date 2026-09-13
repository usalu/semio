import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";

/** 🧩️ Preserves package imports and stable task identities against native Nx inference. */
export function testSelectedPackageIdentities(workspace: string): void {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
  const { buildProjectConfigurationFromPackageJson } = require("nx/src/plugins/package-json/create-nodes");
  const manager = require("nx/src/utils/package-manager").getPackageManagerCommand("bun", workspace);
  for (const row of fixture.packageProjects) {
    const path = row.root + "/package.json", manifest = JSON.parse(readFileSync(join(workspace, path), "utf8"));
    const authored = JSON.parse(readFileSync(join(workspace, row.root, "📋️project.json"), "utf8"));
    assert.equal(manifest.name, row.package);
    assert.equal(authored.name, row.project);
    const native = buildProjectConfigurationFromPackageJson(manifest, workspace, path, {}, true, manager);
    assert.equal(native.name, row.project, path + ": native Nx must preserve task identity");
    const unassigned = structuredClone(manifest);
    delete unassigned.nx?.name;
    assert.equal(buildProjectConfigurationFromPackageJson(unassigned, workspace, path, {}, true, manager).name, row.package);
  }
  console.log("[DEBUG] Selected package imports and installed Nx project identities PASS");
}

/** 🎯️ Checks selected app prerequisites against neutral cases and the installed Nx scheduler. */
export function testSelectedRuntimeDependencies(targets: Record<string, any>, graph?: any): void {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
  const validate = new (require("ajv"))().compile(JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8")));
  assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
  const owner = "@semio-tech/framework-os-dev", rendererProject = "@semio-tech/framework-renderer-wgpu";
  for (const row of fixture.variants) for (const profile of fixture.profiles) for (const renderer of fixture.renderers) {
    const name = `prepare-${row.variant}-${renderer}-${profile}`, target = targets[name];
    assert.ok(target, name);
    const engines = target.dependsOn.filter((id: string) => /:wasm(?:-release)?$/.test(id));
    const expected = renderer === "react" ? row.engines : [`${rendererProject}:${profile === "release" ? "wasm-release" : "wasm"}`];
    assert.deepEqual(require("lodash").sortBy(engines), require("lodash").sortBy(expected), name + ": only declared runtime engines");
    if (!graph) continue;
    const tasks = require("nx/src/tasks-runner/create-task-graph").createTaskGraph(graph, {}, [owner], [`dev-${row.variant}-${renderer}-${profile}`], undefined, {}, false);
    const ids = Object.keys(tasks.tasks);
    if (renderer === "react") assert.ok(!ids.some(id => id.startsWith(rendererProject + ":")), name + ": React must not compile the WGPU renderer");
    const fontTask = fixture.fontTool.project + ":build";
    assert.ok(ids.includes(fontTask), name + ": standalone font prerequisite");
    assert.ok(!ids.some(id => /^semio-framework-os-infinite:(?:build|font-tool)$/.test(id)), name + ": fonts must not compile Infinite");
    for (const forbidden of row.forbiddenProjects) assert.ok(!ids.some(id => id === forbidden + ":wasm" || id === forbidden + ":wasm-release"), name + ": unwanted engine " + forbidden);
  }
  console.log("[DEBUG] Selected runtime prerequisites match neutral app/renderer cases" + (graph ? " and installed Nx task closures" : "") + " PASS");
}

/** 🔤️ Verifies the font producer's complete Cargo closure independently of the renderer graph. */
export async function testSelectedFontDependencies(workspace: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).fontTool;
  const manifest = require("@iarna/toml").parse(readFileSync(join(workspace, fixture.root, "Cargo.toml"), "utf8"));
  assert.equal(manifest.package.name, fixture.project);
  const { runTool } = await import("../../🚀️bootstrap/📦️dependencies/📜️script.ts");
  const result = await runTool("cargo", ["tree", "--locked", "--offline", "-p", fixture.project, "--edges", "normal,build", "--prefix", "none", "--format", "{p}"], workspace, AbortSignal.timeout(60_000), true);
  const crates = [...new Set(result.trim().split("\n").map(line => line.split(" ")[0]))].sort();
  assert.deepEqual(crates, fixture.crates);
  const { cacheInternals } = await import("../../../🟨️.mjs");
  const inputs = cacheInternals.relativeScriptInputs(fixture.commands.map((path: string) => join(workspace, path)), workspace);
  const { build } = require("esbuild");
  const bundled = await build({ entryPoints: fixture.commands, absWorkingDir: workspace, outdir: "dist", bundle: true, packages: "external", platform: "node", format: "esm", write: false, metafile: true, logLevel: "silent" });
  const oracle = Object.keys(bundled.metafile.inputs);
  for (const forbidden of fixture.forbiddenInputs) {
    assert.ok(!inputs.some((path: string) => path.includes(forbidden)), "Font command hashes unrelated source: " + forbidden);
    assert.ok(!oracle.some(path => path.includes(forbidden)), "Font command loads unrelated source: " + forbidden);
  }
  console.log("[DEBUG] Native Cargo font producer contains only the asset dumper and pinned font assets PASS");
}
