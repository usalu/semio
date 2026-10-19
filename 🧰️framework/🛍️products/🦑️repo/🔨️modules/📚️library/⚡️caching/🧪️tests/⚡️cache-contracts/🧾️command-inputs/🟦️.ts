import assert from "node:assert/strict";
import { delimiter, dirname, join, relative, resolve } from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath, pathToFileURL } from "node:url";
import { chmodSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import type { ScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type { ResourceLeaseControl } from "@semio-tech/process/leases";
import { testFixtureValuesV1, testMethodV1, testRecordV1 } from "../../📋️values/🟦️.ts";
import { testOwnedFixtureWireBindingsV1 } from "../../🎚️build-mode/📥️contracts/🟦️.ts";
import { testArtifactReleaseCleanupV1, testCoreExecutablePinControlV1, testExecutableDigestCleanupV1, testNativeExecutableControlV1, testNativeFractionV1, testNativeSessionControlV1, testRepositoryExecutablePinV1 } from "../../../📦️artifacts/📌️executable/🧪️tests/🟦️.ts";
import { testPlaygroundPreparationBindingsV1, testPlaygroundPreparationModelV1 } from "../../🧊️preparation/🧪️tests/🟦️.ts";
import { testGraphRevision } from "../../🔁️graph-revision/🟦️.ts";
import { testWasmOptimizer } from "../../🕸️wasm/🟦️.ts";
import { testBinaryenToolchain } from "@semio-tech/nx-bootstrap-tools/wasm/testing/binaryen-toolchain";
import { testWasmToolFingerprint } from "@semio-tech/nx-bootstrap-tools/wasm/testing/tool-fingerprint";
import { testCiBaseline } from "../../../🚦️ci/🧭️baseline/🧪️tests/🧭️baseline-selection/🟦️.ts";
import { testGithubHistory } from "../../../🚦️ci/🐙️github/🧪️tests/🐙️workflow-history/🟦️.ts";
import { testCiEnvironment } from "../../../🚦️ci/🧭️baseline/🌿️environment/🧪️tests/🌿️workflow-context/🟦️.ts";
import { testCiResolution } from "../../../🚦️ci/🧭️baseline/🏃️resolve/🧪️tests/🧭️baseline-resolution/🟦️.ts";
import { testCiBaselineCommand } from "../../../🚦️ci/🧪️tests/🚦️baseline-command/🟦️.ts";
import { testDevcontainerContext } from "../../../📦️artifacts/🐳️containers/🧪️tests/🐳️devcontainer-context/🟦️.ts";
import { testContainerRuntimeBootstrap } from "../../../📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts";
import { testDevcontainerLifecycle } from "../../../📦️artifacts/🐳️containers/🧪️tests/🔁️lifecycle/🟦️.ts";
import { testDerivedConfig } from "../../../📦️artifacts/📝️derived-config/🧪️tests/📝️derived-config/🟦️.ts";
import { testCommandImportClosure } from "../../🔗️command-imports/🟦️.ts";
import { testImportEdgeEquality } from "../../🔗️import-edges/🟦️.ts";
import { testTrunkLockfile } from "../../🔒️trunk-lockfile/🟦️.ts";
import { testNativeDependencies } from "../../📦️native-dependencies/🟦️.ts";
import { testCargoCleanupBoundary } from "../../🦀️cleanup-boundary/🟦️.ts";
import { testWgpuBrowserServing } from "../../🧊️browser-serving/🟦️.ts";
import { testGraphCoalescing, testWatcherReadiness, testWorkspaceWatchIgnores } from "../../🕸️daemon/🟦️.ts";
import { testContinuousServices } from "../../🖥️services/🟦️.ts";
import { runProcessTestOwner } from "../../🧭️ownership/🟦️.ts";
import { testCachePrune } from "../../🧹️cache-prune/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { testBrowserDistribution } from "../../🌐️browser/🟦️.ts";
import { testServiceReadiness } from "../../🌐️service-readiness/🟦️.ts";

/** 🧪️ Verifies native source and command ownership against compiler and bundler input oracles. */
export async function testCommandInputs(workspace: string, output: string, invocation: ScriptInvocation, phaseControl: ResourceLeaseControl): Promise<void> {
  testFixtureValuesV1();testOwnedFixtureWireBindingsV1(workspace,invocation,phaseControl);await testNativeExecutableControlV1(workspace,invocation,phaseControl);await testRepositoryExecutablePinV1(invocation,phaseControl);await testCoreExecutablePinControlV1(workspace,invocation,phaseControl);await testNativeSessionControlV1(workspace,invocation,phaseControl);await testExecutableDigestCleanupV1(workspace,invocation,phaseControl);await testArtifactReleaseCleanupV1(workspace,invocation,phaseControl);await testNativeFractionV1(workspace,invocation,phaseControl);
  await testPlaygroundPreparationBindingsV1();
  await testPlaygroundPreparationModelV1(workspace,invocation,phaseControl);
  await testGraphRevision(workspace, output);
  await testWasmOptimizer(workspace, output);
  await testBinaryenToolchain(workspace, output);
  await testWasmToolFingerprint(workspace, output);
  await testCiBaseline(workspace);
  await testGithubHistory();
  await testCiEnvironment();
  await testCiResolution();
  await testCiBaselineCommand(workspace, output);
  testDevcontainerContext(workspace);
  const catalogApi=await import("../../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs"),contributions=await testMethodV1(catalogApi.cacheInternals,"workspaceCatalogBuildContributionsV1",workspace);assert.ok(Array.isArray(contributions));const contribution=contributions[0],catalogOutput=testRecordV1(contribution,"publication").outputRoot;assert.equal(typeof catalogOutput,"string");if(typeof catalogOutput!=="string")throw Error("Actual catalog output DATA required");testContainerRuntimeBootstrap(workspace,join(workspace,catalogOutput));
  await testDevcontainerLifecycle(workspace, output);
  await testDerivedConfig(workspace);
  const { testExtensionHostBuild } = await import("repo-vscode/testing/host-build");
  await testExtensionHostBuild(output);
  const { testExtensionPackage } = await import("repo-vscode/testing/package");
  await testExtensionPackage(output);
  await testCommandImportClosure(workspace, output);
  await testImportEdgeEquality(workspace, output);
  await testTrunkLockfile(workspace,invocation,phaseControl);
  await testNativeDependencies(workspace, output,invocation,phaseControl);
  await testCargoCleanupBoundary(workspace, output);
  const { testWgpuWasmOutputs } = await import("../../🧊️wasm-outputs/🟦️.ts");
  await testWgpuWasmOutputs(workspace, output, invocation);
  await testWgpuBrowserServing(workspace, output, invocation);
  const { testWgpuBootInputs } = await import("@semio-tech/framework-os/modules/renderer/engine/testing/wgpu-browser-boot-cache-inputs");
  await testWgpuBootInputs(workspace, output);
  const { testTypeScriptSourceInputs } = await import("../../../../🕸️dependencies/🟦️typescript/🧪️tests/⚡️inputs/🟦️.ts");
  await testTypeScriptSourceInputs(output);
  const { testGeneratorInputReceipt } = await import("../../../🔏️inputs/🧪️tests/🔏️receipt/🟦️.ts");
  await testGeneratorInputReceipt(workspace, output);
  const { testPluginCoreOptimization } = await import("@semio-tech/framework-plugin/testing/native-optimization");
  await testPluginCoreOptimization(workspace, output);
  testNxDaemonTaskEnvironment(workspace);
  testNxDaemonDiagnostics(workspace, output);
  testNxDaemonRetention(workspace, output);
  await testGraphCoalescing(workspace);
  await testWatcherReadiness(workspace);
  testWorkspaceWatchIgnores(workspace, output);
  await testContinuousServices(workspace, output);
  await runProcessTestOwner(invocation, workspace, output, "resource-leases");
  await testCachePrune(output,scriptCompilerAdmissionControlV1(invocation,"cache:pruning:tests",30000));
  testWorkspaceRoots(workspace, output);
  await testRuntimeComponents(workspace);
  const { testPlaygroundPreferences } = await import("../../../../🎮️playground/🔒️preferences/🧪️tests/🔒️playground-preferences/🟦️.ts");
  await testPlaygroundPreferences();
  await testBrowserModuleRelocation(workspace);
  await testBrowserDistribution(workspace, output,invocation);
  const { testProductionBrowserArtifacts } = await import("@semio-tech/framework-os-dev/distribution/components/testing/production-browser-artifacts");
  await testProductionBrowserArtifacts(workspace, output);
  await testServiceReadiness(workspace, output,invocation);
  await testBunDependencies(workspace, output);
  await testNativePreparation(workspace, output, invocation);
  const { testGeneratorOwnership, testWgpuGeneratorOwnership, testWgpuGeneratorPublication, testBrowserTransportOwnership, testBrowserInputClosure } = await import("../../🧬️generator-ownership/🟦️.ts");
  await testGeneratorOwnership(workspace, output);
  await testBrowserTransportOwnership(workspace);
  await testBrowserInputClosure(workspace);
  await testWgpuGeneratorOwnership(workspace);
  await testWgpuGeneratorPublication(workspace, output);
  const { testNativeRendererOutputs } = await import("../../🧊️native-renderer-outputs/🟦️.ts");
  await testNativeRendererOutputs(workspace, output, invocation, phaseControl);
  const { testStylingOutputs } = await import("../../🎨️styling-outputs/🟦️.ts");
  await testStylingOutputs(workspace, output);
  const { testStylingPythonSourceContractV1 } = await import("../../🐍️styling-python-outputs/🟦️.ts");
  await testStylingPythonSourceContractV1(workspace);
  const { testWgpuLiveActivation } = await import("../../🧊️live-activation/🟦️.ts");
  await testWgpuLiveActivation(workspace, output, invocation, phaseControl);
  const { testNativeRuntime } = await import("../../🧊️native-runtime/🟦️.ts");
  await testNativeRuntime(workspace, output, invocation, phaseControl);
  const { testPlaygroundInputView } = await import("../../🎮️playground-input-view/🟦️.ts");
  await testPlaygroundInputView(workspace);
  const { testInferredNativeInputs } = await import("@semio-tech/repo-test/dependencies/testing/inputs");
  await testInferredNativeInputs(workspace, output);
  const require = createRequire(import.meta.url), fixtures = join(dirname(fileURLToPath(import.meta.url)), "../🧫️fixtures");
  const root = mkdtempSync(join(output, "native-inputs-"));
  try {
    const cases = JSON.parse(readFileSync(join(fixtures, "native-inputs/🧫️cases.json"), "utf8"));
    
    for (const [path, content] of Object.entries(cases.files)) { const file = join(root, path); mkdirSync(dirname(file), { recursive: true }); writeFileSync(file, String(content)); }
    const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
    const { cacheInternals } = await import(pathToFileURL(join(workspace, library, "🏗️build/🕸️graph/📥️inference/🟨️.mjs")).href);
    const inputs = await cacheInternals.projectInputs({ name: "fixture", targets: {} }, "domain/📦️packages/🦀️rust", root, new Map());
    assert.ok(Array.isArray(inputs.nativeSources), "Cargo projects need separate native source inputs");
    const match = (path: string) => {
      const positive = inputs.nativeSources.filter((item: unknown) => typeof item === "string" && !item.startsWith("!")).map((s: string) => s.replace("{workspaceRoot}/", "").replace("{projectRoot}/", "domain/📦️packages/🦀️rust/"));
      const negative = inputs.nativeSources.filter((item: unknown) => typeof item === "string" && item.startsWith("!")).map((s: string) => s.slice(1).replace("{workspaceRoot}/", "").replace("{projectRoot}/", "domain/📦️packages/🦀️rust/"));
      return require("minimatch").minimatch(path, "{" + positive.join(",") + "}") && !negative.some((pattern: string) => require("minimatch").minimatch(path, pattern));
    };
    for (const path of cases.included) assert.ok(match(path), `missing native input ${path}`);
    for (const path of cases.excluded) assert.ok(!match(path), `unrelated native input ${path}`);
    const depfile = join(root, "native.d"), rust = Bun.spawnSync(["rustc", "--edition=2021", "--crate-type=lib", "--crate-name=native_inputs", "--emit=dep-info=" + depfile, join(root, "domain/🦀️.rs")], { cwd: root, stdout: "pipe", stderr: "pipe" });
    assert.equal(rust.exitCode, 0, rust.stderr.toString());
    const dependencies = readFileSync(depfile, "utf8");
    for (const path of cases.included.filter((s: string) => !s.endsWith("Cargo.toml"))) assert.ok(dependencies.includes(join(root, path)), `rustc did not consume ${path}`);
    const entry = join(workspace, library, "⚡️caching/🦀️cargo/📜️script.ts"), actual = await cacheInternals.relativeScriptInputs([entry], workspace);
    const built = await require("esbuild").build({ entryPoints: [entry], absWorkingDir: workspace, bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
    const expected = Object.keys(built.metafile.inputs).map((path) => "{workspaceRoot}/" + relative(workspace, resolve(workspace, path))).sort();
    const missing = expected.filter((path: string) => !actual.includes(path)), extra = actual.filter((path: string) => !expected.includes(path));
    assert.deepEqual({ missing: missing.slice(0, 20), extra: extra.slice(0, 20) }, { missing: [], extra: [] }, "Command import ownership must match the runtime compiler");
    for (const [path, content] of Object.entries(cases.commandRouter.files)) { const file = join(root, path); mkdirSync(dirname(file), { recursive: true }); writeFileSync(file, String(content)); }
    assert.equal(typeof cacheInternals.nativeTargetCommandInputs, "function", "Native target normalization must inspect its executable router");
    const routerInputs = await cacheInternals.nativeTargetCommandInputs({ options: { cwd: cases.commandRouter.cwd, command: cases.commandRouter.command } }, root, []);
    const routerPaths = routerInputs.filter((input: unknown) => typeof input === "string").map((input: string) => input.replace("{workspaceRoot}/", ""));
    for (const path of cases.commandRouter.included) assert.ok(routerPaths.includes(path), `missing native command input ${path}`);
    for (const path of cases.commandRouter.excluded) assert.ok(!routerPaths.includes(path), `unrelated native command input ${path}`);
    const routerOracle = await require("esbuild").build({ entryPoints: [join(root, cases.commandRouter.entry)], absWorkingDir: root, bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
    assert.deepEqual(routerPaths.sort(), Object.keys(routerOracle.metafile.inputs).sort(), "Local native router closure must match the independent esbuild oracle");
    
    const boundaries = JSON.parse(readFileSync(join(fixtures, "command-boundaries/🧫️cases.json"), "utf8"));
    
    for (const row of boundaries.cases) {
      const artifact = await require("esbuild").build({ entryPoints: [resolve(workspace, row.entry)], absWorkingDir: workspace, bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
      const paths = Object.keys(artifact.metafile.inputs).sort();
      assert.ok(paths.length <= row.maximumInputs, `${row.name}: command imported ${paths.length} files`);
      assert.ok(paths.every((path) => !path.includes("🔬️") && !path.includes("🧪️") && !path.endsWith("⚡️caching/📜️script.ts") && path !== "📜️script.ts"), row.name);
      const actual = await import(pathToFileURL(resolve(workspace, row.entry)).href);
      for (const name of row.exports) assert.equal(typeof actual[name], "function", `${row.name}: ${name}`);
      
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}

/** 🪢️ Compares generated module relocation with independent JavaScript import spans. */
export async function testBrowserModuleRelocation(workspace: string): Promise<void> {
  const require = createRequire(import.meta.url), directory = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🕸️imports");
  const { rewritePreview2ShimImportSource } = await import(pathToFileURL(join(directory, "🟦️.ts")).href);
  const fixture = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures/🕸️imports/🧫️cases.json"), "utf8"));
  const lexer = await import("es-module-lexer");
  await lexer.init;
  for (const row of fixture.cases) {
    let expected = row.source;
    for (const specifier of [...lexer.parse(row.source)[0]].reverse()) {
      if (!specifier.n?.includes("preview2-shim/")) continue;
      const name = specifier.n.split("/").at(-1)!.replace(/\.js$/, "");
      expected = expected.slice(0, specifier.s) + row.prefix + name + ".js" + expected.slice(specifier.e);
    }
    assert.equal(expected, row.output);
    assert.equal(rewritePreview2ShimImportSource(row.source, row.prefix), expected);
  }
  const bundle = await require("esbuild").build({ entryPoints: [join(directory, "🟦️.ts")], absWorkingDir: workspace, bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
  assert.equal(Object.keys(bundle.metafile.inputs).length, 1);
}

/** 🕸️ Compares runtime selection with an independent directed-graph traversal. */
export async function testRuntimeComponents(workspace: string): Promise<void> {
  const require = createRequire(import.meta.url), fixtures = join(dirname(fileURLToPath(import.meta.url)), "../🧫️fixtures/runtime-components");
  const fixture = JSON.parse(readFileSync(join(fixtures, "🔣️.json"), "utf8"));
  
  const pluginModule = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs")).href);
  await pluginModule.libraryBootstrap;
  const { cacheInternals } = pluginModule;
  assert.equal(typeof cacheInternals.runtimeComponentClosure, "function", "Runtime preparation needs transitive component consumption");
  const oracle = (rows: any[], roots: readonly (string | { readonly id: string; readonly appScoped: boolean })[]) => cacheInternals.runtimeComponentClosure(rows, roots);
  for (const row of fixture.cases) assert.deepEqual(oracle(fixture.components, row.roots), row.expected, row.name);
  for (const row of fixture.invalid) assert.throws(() => cacheInternals.runtimeComponentClosure(row.components, row.roots), /component/i, row.name);

}

/** 🗂️ Keeps producers inside the workspace supplied by Nx's own task environment. */
export function testWorkspaceRoots(workspace: string, output: string): void {
  const require = createRequire(import.meta.url), root = mkdtempSync(join(output, "workspace-roots-"));
  const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../🧫️fixtures/🗂️workspace-roots.json"), "utf8"));
  
  
  try {
    const supplied = require("nx/src/tasks-runner/task-env").getEnvVariablesForBatchProcess(false, true);
    assert.equal(supplied.SEMIO_WORKSPACE_ROOT, workspace);
    const script = join(root, "📜️script.ts"), helper = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts");
    writeFileSync(script, `import { getWorkspaceRoot } from ${JSON.stringify(helper)}; process.stdout.write(getWorkspaceRoot());\n`);
    for (const row of fixture.cases) {
      const env = { ...supplied, SEMIO_WORKSPACE_ROOT: row.nx ? supplied.SEMIO_WORKSPACE_ROOT : undefined, REPO_ROOT: row.hint ? root : undefined };
      const child = Bun.spawnSync([process.execPath, script], { cwd: workspace, env, stdout: "pipe", stderr: "pipe" });
      assert.equal(child.exitCode, 0, child.stderr.toString());
      assert.equal(child.stdout.toString(), row.expected === "workspace" ? workspace : root, row.name);
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}

/** 🧶️ Validates location-sensitive locked dependencies against Nx's native hash planner. */
export async function testBunDependencies(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url), fixtures = join(dirname(fileURLToPath(import.meta.url)), "../🧫️fixtures/bun-dependencies");
  const cases = JSON.parse(readFileSync(join(fixtures, "🔣️.json"), "utf8"));
  
  const { cacheInternals } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs")).href);
  assert.equal(typeof cacheInternals.bunLockGraph, "function", "Bun inputs require resolved-location dependency identity");
  const graph = cacheInternals.bunLockGraph(cases.lock, cases.patches);
  for (const row of cases.resolutions) assert.equal(graph.resolve(row.from, row.name), row.key);
  for (const row of cases.importers) assert.equal(graph.resolveImport(row.file, row.name), row.key);
  for (const name of cases.unresolvedImports) assert.equal(graph.resolveImport("domain/📜️script.ts", name), undefined);
  const native = await cacheInternals.nativeTargetCommandInputs((await cacheInternals.cargoTargets("🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust", workspace)).build,workspace);
  assert.deepEqual(native.flatMap((input: any) => input.externalDependencies ?? []), cases.nativeInputs.externalDependencies);
  for (const path of cases.nativeInputs.excluded) assert.equal(native.includes(path), false);
  const { HashPlanner, transferProjectGraph } = require("nx/src/native"), { transformProjectGraphForRust } = require("nx/src/native/transform-objects");
  const plan = (model: any, key: string): string[] => {
    const project = { name: "probe", type: "lib", data: { root: "probe", targets: { build: { executor: "nx:run-commands", inputs: [{ externalDependencies: ["npm:" + key] }], options: { command: "bun ./📜️script.ts build" } } } } };
    const graph = { nodes: { probe: project }, externalNodes: model.externalNodes, dependencies: { probe: [], ...Object.fromEntries(Object.keys(model.externalNodes).map((id) => [id, model.dependencies.filter((edge: any) => edge.source === id)])) } };
    const task = { id: "probe:build", target: { project: "probe", target: "build" }, projectRoot: "probe", overrides: {}, outputs: [], cache: true, parallelism: true };
    return new HashPlanner({}, transferProjectGraph(transformProjectGraphForRust(graph))).getPlans([task.id], { roots: [task.id], tasks: { [task.id]: task }, dependencies: { [task.id]: [] }, continuousDependencies: {} })[task.id].filter((value: string) => value.startsWith("npm:")).sort();
  };
  const identity = (model: any, key: string) => JSON.stringify(plan(model, key).map((id) => [id, model.externalNodes[id].data.hash]));
  for (const row of cases.closures) assert.deepEqual(plan(graph, row.root), row.keys.map((key: string) => "npm:" + key).sort());
  for (const row of cases.mutations) {
    const lock = structuredClone(cases.lock); lock.packages[row.package][3] += "-changed";
    assert.equal(identity(graph, row.root) !== identity(cacheInternals.bunLockGraph(lock, cases.patches), row.root), row.changed, row.name);
  }
  const patched = cacheInternals.bunLockGraph(cases.lock, { ...cases.patches, "patches/core.patch": "different patch bytes" });
  assert.notEqual(identity(graph, "api"), identity(patched, "api"));
  assert.equal(identity(graph, "@fixture/document/api"), identity(patched, "@fixture/document/api"));
  assert.throws(() => cacheInternals.bunLockGraph(cases.lock, {}), /patch/i);
  const missing = structuredClone(cases.lock); delete missing.packages["api/core"]; delete missing.packages.core;
  assert.throws(() => cacheInternals.bunLockGraph(missing, cases.patches), /resolve.*core/i);
  assert.throws(() => cacheInternals.bunLockGraph({ ...cases.lock, lockfileVersion: 99 }, cases.patches), /version/i);
  const root = mkdtempSync(join(output, "bun-resolution-"));
  try {
    const location = (key: string) => {
      if (!key) return root;
      const parts = key.match(/(?:@[^/]+\/[^/]+|[^/]+)/g)!;
      const workspacePath = graph.workspacePackages.get(parts[0]);
      return workspacePath ? join(root, workspacePath, ...parts.slice(1).flatMap(part => ["node_modules", part])) : join(root, "node_modules", parts.join("/node_modules/"));
    };
    for (const [key, node] of Object.entries(graph.externalNodes) as [string, any][]) {
      const path = location(key.slice(4)); mkdirSync(path, { recursive: true });
      writeFileSync(join(path, "package.json"), JSON.stringify({ name: node.data.packageName, version: node.data.version, main: "📜️script.ts" }));
      writeFileSync(join(path, "📜️script.ts"), "export default " + JSON.stringify(node.data.version));
    }
    for (const row of cases.resolutions) assert.equal(Bun.resolveSync(row.name, location(row.from)), join(location(row.key), "📜️script.ts"), `Bun resolver: ${row.from} -> ${row.name}`);
    for (const row of cases.importers) { mkdirSync(dirname(join(root, row.file)), { recursive: true }); assert.equal(Bun.resolveSync(row.name, dirname(join(root, row.file))), join(location(row.key), "📜️script.ts"), `Bun importer: ${row.file}`); }
  } finally { rmSync(root, { recursive: true, force: true }); }
}

/** 🧬️ Compares prerequisite closure with Cargo's independent package and dependency model. */
export async function testNativePreparation(workspace: string, output: string, invocation: ScriptInvocation): Promise<void> {
  const require = createRequire(import.meta.url), fixtures = join(dirname(fileURLToPath(import.meta.url)), "../🧫️fixtures/native-preparation");
  const cases = JSON.parse(readFileSync(join(fixtures, "🧫️cases.json"), "utf8"));
  
  const nativeInput = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🎛️native-input/🟦️.ts")).href);
  const nativeBuild = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts")).href);
  assert.equal(typeof nativeInput.validateNativeCargoArguments, "function");
  for (const row of cases.arguments) if (row.valid) assert.doesNotThrow(() => nativeInput.validateNativeCargoArguments(row.operation, row.args)); else assert.throws(() => nativeInput.validateNativeCargoArguments(row.operation, row.args), /input contract/);
  assert.equal(typeof nativeInput.artifactRustCargoArguments, "function");
  for (const row of cases.artifactArguments) if (row.valid) assert.deepEqual(nativeInput.artifactRustCargoArguments(row.operation, row.args).cargoArgs, row.cargoArgs, row.operation); else assert.throws(() => nativeInput.artifactRustCargoArguments(row.operation, row.args), /input contract/);
  const artifactRouterSource = readFileSync(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts"), "utf8");
  for (const witness of cases.artifactTestRunner.required) assert.ok(artifactRouterSource.includes(witness), `artifact test router must use ${witness}`);
  for (const witness of cases.artifactTestRunner.forbidden) assert.ok(!artifactRouterSource.includes(witness), `artifact test router must not use ${witness}`);
  const runnerRoot = mkdtempSync(join(output, "artifact-test-runner-"));
  const capture = join(runnerRoot, "commands.jsonl"), stub = join(runnerRoot, "📜️script.ts");
  const argv = process.argv, path = process.env.PATH, level = process.env.SEMIO_TEST_LEVEL, coverage = process.env.SEMIO_COVERAGE, artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR, testBudget = process.env.SEMIO_TEST_BUDGET_MS, buildBudget = process.env.SEMIO_BUILD_BUDGET_MS;
  try {
    writeFileSync(stub, `#!/usr/bin/env bun
import { appendFileSync } from "node:fs";
const args = process.argv.slice(2);
appendFileSync(process.env.SEMIO_ARTIFACT_TEST_CAPTURE!, JSON.stringify(args) + "\\n");
if (args[0] === "nextest" && args[1] === "list") console.log("{}");
`);
    chmodSync(stub, 0o755);
    if (process.platform === "win32") writeFileSync(join(runnerRoot, "cargo.cmd"), `@bun "%~dp0📜️script.ts" %*\r\n`);
    else symlinkSync("📜️script.ts", join(runnerRoot, "cargo"));
    const artifactDir = join(runnerRoot, "artifacts"); mkdirSync(artifactDir);
    process.env.PATH = runnerRoot + delimiter + (path ?? "");
    process.env.SEMIO_ARTIFACT_TEST_CAPTURE = capture;
    process.env.SEMIO_TEST_ARTIFACT_DIR = artifactDir;
    process.env.SEMIO_TEST_BUDGET_MS = "5000";
    process.env.SEMIO_BUILD_BUDGET_MS = "5000";
    delete process.env.SEMIO_COVERAGE;
    process.argv = [process.execPath, "artifact-test-fixture", "test", ...cases.artifactTestRunner.segments];
    const packageRoot = join(workspace, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust");
    const artifactRunner = await import("../../../📦️artifacts/🦀️rust/🟦️.ts");
    await artifactRunner.runArtifactRustPackageMain(packageRoot, "semio-s-artifact-stdio-pdf");
    assert.equal(process.env.SEMIO_TEST_LEVEL, cases.artifactTestRunner.level);
    const commands = readFileSync(capture, "utf8").trim().split(/\r?\n/u).map(line => JSON.parse(line) as string[]);
    const listed = commands.find(args => args[0] === "nextest" && args[1] === "list"), executed = commands.find(args => args[0] === "nextest" && args[1] === "run");
    assert.ok(listed?.includes("--lib") && listed.includes(cases.artifactTestRunner.level), "artifact test build must preserve selection and level profile");
    assert.ok(cases.artifactTestRunner.runtimeFilters.every((filter: string) => executed?.includes(filter)) && executed?.includes("--nocapture"), "artifact test execution must preserve level-looking runtime filters and libtest filters");
    const skips = executed!.flatMap((argument, index) => argument === "--skip" ? [executed![index + 1]] : []);
    for (const excluded of cases.artifactTestRunner.includedSkips) assert.ok(skips.includes(excluded), `artifact test level must skip ${excluded}`);
    for (const included of cases.artifactTestRunner.excludedSkips) assert.ok(!skips.includes(included), `artifact test level must retain ${included}`);
    assert.ok(commands.every(args => args[0] !== "test"), "artifact test router must not bypass the budgeted Nextest runner");
  } finally {
    process.argv = argv;
    const restore = (name: string, value: string | undefined): void => { if (value === undefined) delete process.env[name]; else process.env[name] = value; };
    restore("PATH", path); restore("SEMIO_TEST_LEVEL", level); restore("SEMIO_COVERAGE", coverage); restore("SEMIO_TEST_ARTIFACT_DIR", artifacts); restore("SEMIO_TEST_BUDGET_MS", testBudget); restore("SEMIO_BUILD_BUDGET_MS", buildBudget); delete process.env.SEMIO_ARTIFACT_TEST_CAPTURE;
    rmSync(runnerRoot, { recursive: true, force: true });
  }
  const captureRoot=mkdtempSync(join(output,"artifact-refusal-"));let consumed=false;
  try{const{repositoryCargoNetworkV1}=await import("../../../../🏃️process/🦀️cargo/📥️dependencies/🟦️.ts"),control=scriptCompilerAdmissionControlV1(invocation,"native:foreign-workspace-refusal",0);await assert.rejects(nativeBuild.buildRepositoryCargoArtifacts("package/Cargo.toml",[],captureRoot,{producer:"artifact",invocation,control,network:repositoryCargoNetworkV1(invocation,workspace),consume:async()=>{consumed=true;throw Error("Foreign workspace must never admit a compiler consumer");}}),/workspace/u);assert.equal(consumed,false);assert.deepEqual(readdirSync(captureRoot),[]);}finally{rmSync(captureRoot,{recursive:true,force:true});}
  const root = mkdtempSync(join(output, "native-preparation-"));
  try {
    for (const [path, content] of Object.entries(cases.files)) { const file = join(root, path); mkdirSync(dirname(file), { recursive: true }); writeFileSync(file, String(content)); }
    const { cacheInternals } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs")).href);
    assert.equal(typeof cacheInternals.nativePreparation, "function", "native codegen needs an explicit transitive prerequisite resolver");
    const cargo = Bun.spawnSync(["cargo", "metadata", "--offline", "--format-version=1"], { cwd: root, stdout: "pipe", stderr: "pipe" });
    assert.equal(cargo.exitCode, 0, cargo.stderr.toString());
    const metadata = JSON.parse(cargo.stdout.toString()), packages = new Map(metadata.packages.map((p: any) => [p.id, p]));
    for (const row of cases.cases) {
      const entry = metadata.packages.find((p: any) => p.name === row.root), visited = new Set<string>();
      const visit = (id: string) => {
        if (visited.has(id)) return;
        visited.add(id);
        for (const dependency of metadata.resolve.nodes.find((n: any) => n.id === id).deps) if (dependency.dep_kinds.some((kind: any) => kind.kind !== "dev" || row.tests && id === entry.id)) visit(dependency.pkg);
      };
      visit(entry.id);
      const oracle = [...visited].map((id) => (packages.get(id) as any).name).filter((name) => name in cases.contracts).map((name) => name + ":generate").sort();
      assert.deepEqual(oracle, row.targets, `${row.name}: Cargo oracle`);
      assert.deepEqual(cacheInternals.nativeDependencyRoots(row.root, root, row.tests), [...visited].map((id) => relative(root, dirname((packages.get(id) as any).manifest_path)).replaceAll("\\", "/")).sort(), `${row.name}: Cargo source owners`);
      assert.deepEqual(cacheInternals.nativePreparation(row.root, root, cases.contracts, row.tests).map((c: any) => c.target), row.targets, row.name);
      const project = { name: row.root, root: row.root, targets: { [row.tests ? "test" : "build"]: { inputs: [row.tests ? "nativeTestSources" : "nativeSources", "^nativeSources"] } } };
      cacheInternals.withNativePreparation(project, root, cases.contracts, new Map(), new Map(metadata.packages.map((p: any) => [p.name, p.name])));
      const target = Object.values(project.targets)[0] as any;
      assert.deepEqual(target.dependsOn, row.targets);
      assert.deepEqual(target.inputs.find((input: any) => input.projects)?.projects ?? [], [...visited].map((id) => (packages.get(id) as any).name).filter((name) => name !== row.root).sort());
    }
    const declared = await cacheInternals.projectInputs({ name: "fixture", namedInputs: { nativeSources: ["{workspaceRoot}/external.bin"] }, targets: {} }, "app", root, new Map());
    assert.ok(declared.nativeTestSources.includes("{workspaceRoot}/external.bin"), "tests inherit declared production native inputs");
  } finally { rmSync(root, { recursive: true, force: true }); }
}

/** 🪵️ Exercises Nx's installed error formatter against a bounded diagnostic contract. */
export function testNxDaemonDiagnostics(workspace: string, output: string): void {
  const require = createRequire(join(workspace, "package.json")), root = mkdtempSync(join(output, "daemon-tail-"));
  const vector = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../🧫️fixtures/nx-contract/🔣️.json"), "utf8")).daemonLog;
  const source = `
    const fs = require("node:fs"), assert = require("node:assert/strict");
    const client = require(${JSON.stringify(require.resolve("nx/src/daemon/client/client.js"))});
    const path = require(${JSON.stringify(require.resolve("nx/src/daemon/tmp-dir.js"))}).DAEMON_OUTPUT_LOG_FILE;
    const vector = ${JSON.stringify(vector)};
    fs.mkdirSync(require("node:path").dirname(path), { recursive: true });
    fs.writeFileSync(path, "x".repeat(vector.size) + "\\n" + vector.marker);
    const originalReadFile = fs.readFileSync, originalRead = fs.readSync;
    let wholeReads = 0, maxRead = 0;
    fs.readFileSync = (file, ...args) => { if (file === path) wholeReads++; return originalReadFile(file, ...args); };
    fs.readSync = (fd, buffer, offset, length, position) => { maxRead = Math.max(maxRead, length); return originalRead(fd, buffer, offset, length, position); };
    const error = client.daemonProcessException("fixture failure");
    assert.equal(error.internalDaemonError, true);
    assert.ok(error.message.includes(vector.marker));
    assert.equal(wholeReads, 0, "diagnostic must never read the whole log");
    assert.ok(maxRead > 0 && maxRead <= vector.limit);
    fs.rmSync(path);
    assert.match(client.daemonProcessException("missing log").message, /missing log/);
  `;
  try {
    const child = Bun.spawnSync(["node", "--eval", source], { cwd: workspace, env: { ...process.env, NX_WORKSPACE_DATA_DIRECTORY: join(root, "workspace-data") }, stdout: "pipe", stderr: "pipe", timeout: 10000 });
    assert.equal(child.exitCode, 0, child.stdout.toString() + child.stderr.toString());
  } finally { rmSync(root, { recursive: true, force: true }); }
}

/** 🧭️ Keeps task-scoped reporting and selection out of daemon graph identity while preserving caller values. */
export function testNxDaemonTaskEnvironment(workspace: string): void {
  const require = createRequire(join(workspace, "package.json"));
  const vector = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../🧫️fixtures/nx-contract/🔣️.json"), "utf8")).daemonTaskEnvironment;
  const source = `
    const assert = require("node:assert/strict"), nx = require(${JSON.stringify(require.resolve("nx/src/daemon/client/daemon-environment.js"))});
    const vector = ${JSON.stringify(vector)};
    for (const key of Object.keys(vector)) delete process.env[key];
    const hash = nx.hashDaemonClientEnv();
    for (const [key, value] of Object.entries(vector)) {
      process.env[key] = value;
      assert.equal(nx.hashDaemonClientEnv(), hash, key + " must not invalidate the graph");
      assert.equal(nx.getDaemonEnv()[key], undefined, key);
      assert.equal(process.env[key], value, key + " must remain available to task children");
    }
    process.env.NX_LOAD_DOT_ENV_FILES = "false";
    assert.notEqual(nx.hashDaemonClientEnv(), hash);
  `;
  const env = { ...process.env, NX_LOAD_DOT_ENV_FILES: "true" };
  const child = Bun.spawnSync(["node", "--eval", source], { cwd: workspace, env, stdout: "pipe", stderr: "pipe", timeout: 10000 });
  assert.equal(child.exitCode, 0, child.stdout.toString() + child.stderr.toString());
}

/** 📓️ Checks installed Nx retention against native append descriptors and unowned path boundaries. */
export function testNxDaemonRetention(workspace: string, output: string): void {
  const require = createRequire(join(workspace, "package.json")), directory = mkdtempSync(join(output, "daemon-retention-"));
  const original = require.resolve("nx/src/daemon/logger.js"), source = original;
  const vector = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../🧫️fixtures/📓️daemon-retention.json"), "utf8"));
  const code = `
  const fs = require("node:fs"), assert = require("node:assert/strict"), path = require("node:path"), logger = {};
  const load = require("node:module").createRequire(${JSON.stringify(original)});
  new Function("require", "exports", fs.readFileSync(${JSON.stringify(source)}, "utf8"))(load, logger);
  const filename = load("./tmp-dir").DAEMON_OUTPUT_LOG_FILE, vector = ${JSON.stringify(vector)};
  fs.mkdirSync(path.dirname(filename), {recursive:true});
  const descriptor = fs.openSync(filename, "a"), inode = fs.fstatSync(descriptor).ino;
  let maximumRead = 0;
  const read = fs.readSync;
  fs.readSync = (fd, buffer, offset, length, position) => { maximumRead = Math.max(maximumRead, length); return read(fd, buffer, offset, length, position); };
  try {
    for (let cycle = 0; cycle < vector.cycles; cycle++) {
      fs.ftruncateSync(descriptor, vector.limitBytes * 2);
      fs.writeSync(descriptor, "\\n" + vector.tail);
      logger.clientLogger.writeToFile("retention cycle " + cycle);
      assert.ok(fs.statSync(filename).size <= vector.limitBytes, "Nx daemon log remains unbounded");
      assert.equal(fs.statSync(filename).ino, inode, "The daemon's open descriptor must keep its inode");
      fs.writeSync(descriptor, vector.append);
      const content = fs.readFileSync(filename, "utf8");
      assert.ok(content.includes(vector.tail)); assert.ok(content.endsWith(vector.append));
    }
    assert.ok(maximumRead > 0 && maximumRead <= vector.retainBytes);
  } finally { fs.closeSync(descriptor); }
  fs.rmSync(filename); fs.mkdirSync(filename); logger.pruneDaemonLog(); assert.ok(fs.statSync(filename).isDirectory()); fs.rmdirSync(filename);
  logger.pruneDaemonLog();
  const external = path.join(path.dirname(filename), "outside.log");
  fs.writeFileSync(external, "unowned bytes"); const externalFd = fs.openSync(external,"r+"); fs.ftruncateSync(externalFd,vector.limitBytes*2); fs.closeSync(externalFd);
  fs.linkSync(external,filename); logger.pruneDaemonLog(); assert.equal(fs.statSync(external).size,vector.limitBytes*2); fs.rmSync(filename);
  if (process.platform !== "win32") { fs.symlinkSync(external,filename); logger.pruneDaemonLog(); assert.equal(fs.statSync(external).size,vector.limitBytes*2); fs.rmSync(filename); }
  const actualDirectory = path.dirname(filename), savedDirectory = actualDirectory+"-original";
  fs.renameSync(actualDirectory,savedDirectory); fs.symlinkSync(savedDirectory,actualDirectory,process.platform === "win32" ? "junction" : "dir");
  fs.copyFileSync(path.join(savedDirectory,"outside.log"),filename); logger.pruneDaemonLog(); assert.equal(fs.statSync(filename).size,vector.limitBytes*2);
  fs.rmSync(actualDirectory); fs.renameSync(savedDirectory,actualDirectory);
`;
  try {
    const child = Bun.spawnSync(["node", "--eval", code], { cwd: workspace, env: { ...process.env, NX_WORKSPACE_DATA_DIRECTORY: join(directory, "workspace-data") }, stdout: "pipe", stderr: "pipe", timeout: 10000 });
    assert.equal(child.exitCode, 0, child.stdout.toString() + child.stderr.toString());
    process.stdout.write(child.stdout);
  } finally { rmSync(directory, { recursive: true, force: true }); }
}
