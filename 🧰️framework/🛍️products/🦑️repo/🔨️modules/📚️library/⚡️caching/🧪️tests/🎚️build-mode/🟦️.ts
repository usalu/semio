import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire, isBuiltin } from "node:module";
import { resolve } from "node:path";
import { resolveNxBuildMode } from "../../🚀️bootstrap/🎚️mode/🟦️.ts";
import { cacheInternals } from "../../../🟨️.mjs";
import { targetBuildModes } from "../../../🔌️nx-plugin/🎚️mode/🟨️.mjs";

/** 🧪️ Portable mode selections agree with the installed Nx task graph and output resolver. */
export async function testNxBuildModes(workspace: string): Promise<void> {
  const cases = JSON.parse(readFileSync(new URL("../../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).buildModes;
  for (const row of cases) {
    const invoke = () => resolveNxBuildMode({ args: row.args, env: {} }, row.environment ?? {});
    if (row.error) assert.throws(invoke);
    else { const selected = invoke(); assert.equal(selected.env.SEMIO_BUILD_MODE, row.mode); assert.ok(selected.args.includes(`--configuration=${row.mode}`)); }
  }
  const cargo = cacheInternals.cargoTargets("🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust", workspace, []);
  assert.equal(cargo.build.defaultConfiguration, "prototype");
  assert.equal(cargo.build.configurations.release.env.SEMIO_BUILD_MODE, "release");
  const require = createRequire(resolve(workspace, ".nx/installation/package.json"));
  const { createTaskGraph } = require("nx/src/tasks-runner/create-task-graph");
  const { getOutputs } = require("nx/src/tasks-runner/utils");
  const targets = { build: targetBuildModes("build", { executor: "nx:run-commands", cache: true, dependsOn: ["leaf:build"], outputs: ["{projectRoot}/dist/build/{options.env.SEMIO_BUILD_MODE}"], options: { command: "bun ./📜️script.ts build" } }) };
  const leaf = { ...targets.build, dependsOn: [] };
  const nodes = { owner: { name: "owner", type: "lib", data: { root: "owner", targets } }, leaf: { name: "leaf", type: "lib", data: { root: "leaf", targets: { build: leaf } } } };
  const graph = { nodes, externalNodes: {}, dependencies: { owner: [], leaf: [] } };
  for (const mode of ["prototype", "release"]) {
    const tasks = createTaskGraph(graph, {}, ["owner"], ["build"], mode, {});
    assert.deepEqual(Object.keys(tasks.tasks).sort(), [`leaf:build:${mode}`, `owner:build:${mode}`]);
    assert.deepEqual(getOutputs(nodes, tasks.tasks[`leaf:build:${mode}`].target, {}), [`leaf/dist/build/${mode}`]);
  }
  nodes.owner.data.targets["native-build-release"] = targetBuildModes("native-build-release", targets.build);
  const inherited = createTaskGraph(graph, {}, ["owner"], ["native-build-release"], undefined, {});
  assert.ok(inherited.tasks["leaf:build:release"], "Default fixed release must propagate through dependencies");
  assert.throws(() => createTaskGraph(graph, {}, ["owner"], ["native-build-release"], "prototype", {}), /Conflicting build mode/);
  assert.throws(() => createTaskGraph(graph, {}, ["owner"], ["build"], "dev", {}), /prototype or release/);
  const dependencyCase = JSON.parse(readFileSync(new URL("../../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).commandDependencyOracle;
  const entry = dependencyCase.entry;
  const closure = cacheInternals.genericTargetCommandInputs({ options: { command: `bun ${JSON.stringify(entry)} nx` } }, workspace, []);
  const lock = Bun.JSONC.parse(readFileSync(resolve(workspace, "bun.lock"), "utf8"));
  const patches = Object.fromEntries(Object.values(lock.patchedDependencies ?? {}).map(path => [path, readFileSync(resolve(workspace, String(path)))]));
  const model = cacheInternals.bunLockGraph(lock, patches), expected = new Set<string>();
  const oracle = await createRequire(resolve(workspace, "package.json"))("esbuild").build({ entryPoints: [entry], absWorkingDir: workspace, bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
  for (const [file, input] of Object.entries(oracle.metafile.inputs)) for (const dependency of input.imports) {
    if (!dependency.external || dependency.path.startsWith(".") || dependency.path.startsWith("/") || isBuiltin(dependency.path) || dependency.path.startsWith("bun:")) continue;
    const name = dependency.path.startsWith("@") ? dependency.path.split("/").slice(0, 2).join("/") : dependency.path.split("/")[0];
    const key = model.resolveImport(file, name);
    if (key && model.externalNodes[`npm:${key}`]) expected.add(`npm:${key}`);
  }
  for (const dependency of dependencyCase.dynamicExternalDependencies) { assert.ok(model.externalNodes[dependency]); expected.add(dependency); }
  assert.deepEqual(closure.find(input => input.externalDependencies)?.externalDependencies, [...expected].sort(), "Command package scope must match esbuild's independent import oracle");
  assert.ok(!closure.includes("{workspaceRoot}/package.json") && !closure.includes("{workspaceRoot}/bun.lock"));
  const routing = readFileSync(resolve(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🧩️contributions/🟦️.ts"), "utf8");
  assert.ok(!routing.includes("--skip-nx-cache"), "Owned command routing must honor declared target cache policy");
  console.log(`[DEBUG] ${cases.length} Nx mode vectors and installed task graph/output oracle passed`);
}
