import {testNativeExecutableControlV1,testRepositoryExecutablePinV1,testCoreExecutablePinControlV1,testNativeSessionControlV1,testExecutableDigestCleanupV1,testArtifactReleaseCleanupV1,testNativeFractionV1} from "../../📦️artifacts/📌️executable/🧪️tests/🟦️.ts";
import {testFixtureValuesV1,testMethodV1,testRecordV1,testStringsV1} from "../📋️values/🟦️.ts";
import {testOwnedFixtureWireBindingsV1} from "../🎚️build-mode/📥️contracts/🟦️.ts";
import {testPlaygroundPreparationBindingsV1,testPlaygroundPreparationModelV1} from "../🧊️preparation/🧪️tests/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {resolveBuildSelectionV1} from "../../../🏗️build/🧭️invocation/🟦️.ts";
import {resolveNxBuildMode} from "../../🚀️bootstrap/🎚️mode/🟦️.ts";
import {testCacheContractBindingsV1} from "./📋️bindings/🟦️.ts";
import { testWgpuBrowserServing } from "../🧊️browser-serving/🟦️.ts";
import { inventory } from "../../📇️inventory/🧮️composition/🟦️.ts";
import { slash } from "../../🔍️discovery/📂️source/🟦️.ts";
import { ticketOutput } from "../../🎫️output/🟦️.ts";
import assert from "node:assert/strict";
import { chmodSync, lstatSync, readFileSync, mkdirSync, writeFileSync, mkdtempSync, readdirSync, rmSync, symlinkSync, utimesSync } from "node:fs";
import { delimiter, dirname, join, resolve, relative } from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath, pathToFileURL } from "node:url";
import { testGraphCoalescing, testWatcherReadiness, testWorkspaceWatchIgnores } from "../🕸️daemon/🟦️.ts";
import { testBrowserDistribution } from "../🌐️browser/🟦️.ts";
import { testContinuousServices } from "../🖥️services/🟦️.ts";
import { testServiceReadiness } from "../🌐️service-readiness/🟦️.ts";
import { testDependencyBootstrap, testNxTooling, testDependencyCancellation } from "../📦️dependencies/🟦️.ts";
import { type ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { readBuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { runProcessTestOwner } from "../🧭️ownership/🟦️.ts";
import { testCachePrune } from "../🧹️cache-prune/🟦️.ts";
import { testWasmOptimizer } from "../🕸️wasm/🟦️.ts";
import { testCiBaseline } from "../../🚦️ci/🧭️baseline/🧪️tests/🧭️baseline-selection/🟦️.ts";
import { testGithubHistory } from "../../🚦️ci/🐙️github/🧪️tests/🐙️workflow-history/🟦️.ts";
import { testCiEnvironment } from "../../🚦️ci/🧭️baseline/🌿️environment/🧪️tests/🌿️workflow-context/🟦️.ts";
import { testCiResolution } from "../../🚦️ci/🧭️baseline/🏃️resolve/🧪️tests/🧭️baseline-resolution/🟦️.ts";
import { testCiBaselineCommand } from "../../🚦️ci/🧪️tests/🚦️baseline-command/🟦️.ts";
import { testDevcontainerContext } from "../../📦️artifacts/🐳️containers/🧪️tests/🐳️devcontainer-context/🟦️.ts";
import { testContainerRuntimeBootstrap } from "../../📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts";
import { testCommandImportClosure } from "../🔗️command-imports/🟦️.ts";
import { testImportEdgeEquality } from "../🔗️import-edges/🟦️.ts";
import { testTrunkLockfile } from "../🔒️trunk-lockfile/🟦️.ts";
import { testNativeDependencies } from "../📦️native-dependencies/🟦️.ts";
import { testCargoCleanupBoundary } from "../🦀️cleanup-boundary/🟦️.ts";
import { testGraphRevision } from "../🔁️graph-revision/🟦️.ts";
import { testDevcontainerLifecycle } from "../../📦️artifacts/🐳️containers/🧪️tests/🔁️lifecycle/🟦️.ts";
import { testDerivedConfig } from "../../📦️artifacts/📝️derived-config/🧪️tests/📝️derived-config/🟦️.ts";
import { testBinaryenToolchain } from "../../🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🛠️binaryen-toolchain/🟦️.ts";
import { testWasmToolFingerprint } from "../../🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🔏️tool-fingerprint/🟦️.ts";

/** 🛡️ Proves explicit owner cache refusal against the portable policy corpus and installed Nx task semantics. */
export function proveCachePolicy(workspace: string, internals: { targetPolicy(name: string, target: object, policy: object): { cache: boolean; continuous?: boolean } }): void {
  const root = resolve(import.meta.dir, "../.."), require = createRequire(join(workspace, "package.json"));
  const vectors = JSON.parse(readFileSync(join(root, "🧫️fixtures/nx-contract/🔣️.json"), "utf8"));
  
  
  const policy = JSON.parse(readFileSync(join(root, "🔣️policy.json"), "utf8"));
  const { isCacheableTask } = require("nx/src/tasks-runner/utils");
  for (const row of vectors.policies) for (const authored of [true, false]) {
    const target = internals.targetPolicy(row.target, { cache: authored, options: { command: row.command ?? `bun ./📜️script.ts ${row.target}` } }, policy);
    const expected = authored ? row.cache : row.disabledCache;
    assert.deepEqual({ cache: target.cache, continuous: target.continuous ?? false }, { cache: expected, continuous: row.continuous }, `${row.target}: authored cache ${authored}`);
    assert.equal(isCacheableTask({ cache: target.cache, continuous: target.continuous === true, target: { project: "probe", target: row.target }, overrides: {} }), expected, `${row.target}: installed Nx cache semantics`);
  }
  console.log(`[cache-policy] ${vectors.policies.length} portable owner-cache laws agree with installed Nx`);
}

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
  const catalogApi=await import("../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs"),contributions=await testMethodV1(catalogApi.cacheInternals,"workspaceCatalogBuildContributionsV1",workspace);assert.ok(Array.isArray(contributions));const contribution=contributions[0],catalogOutput=testRecordV1(contribution,"publication").outputRoot;assert.equal(typeof catalogOutput,"string");if(typeof catalogOutput!=="string")throw Error("Actual catalog output DATA required");testContainerRuntimeBootstrap(workspace,join(workspace,catalogOutput));
  await testDevcontainerLifecycle(workspace, output);
  await testDerivedConfig(workspace);
  const { testExtensionHostBuild } = await import("../../../../💻️client/🧩️vscode/🧪️tests/🧩️host-build/🟦️.ts");
  await testExtensionHostBuild(output);
  const { testExtensionPackage } = await import("../../../../💻️client/🧩️vscode/🧪️tests/📦️package/🟦️.ts");
  await testExtensionPackage(output);
  await testCommandImportClosure(workspace, output);
  await testImportEdgeEquality(workspace, output);
  await testTrunkLockfile(workspace,invocation,phaseControl);
  await testNativeDependencies(workspace, output,invocation,phaseControl);
  await testCargoCleanupBoundary(workspace, output);
  const { testWgpuWasmOutputs } = await import("../🧊️wasm-outputs/🟦️.ts");
  await testWgpuWasmOutputs(workspace, output, invocation);
  await testWgpuBrowserServing(workspace, output, invocation);
  const { testWgpuBootInputs } = await import("../../../../../../💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts");
  await testWgpuBootInputs(workspace, output);
  const { testTypeScriptSourceInputs } = await import("../../../🕸️dependencies/🟦️typescript/🧪️tests/⚡️inputs/🟦️.ts");
  await testTypeScriptSourceInputs(output);
  const { testGeneratorInputReceipt } = await import("../../🔏️inputs/🧪️tests/🔏️receipt/🟦️.ts");
  await testGeneratorInputReceipt(workspace, output);
  const { testPluginCoreOptimization } = await import("../../../../../../💻️os/🔨️modules/🔌️plugin/🧪️tests/🕸️native-optimization/🟦️.ts");
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
  const { testPlaygroundPreferences } = await import("../../../🎮️playground/🔒️preferences/🧪️tests/🔒️playground-preferences/🟦️.ts");
  await testPlaygroundPreferences();
  await testBrowserModuleRelocation(workspace);
  await testBrowserDistribution(workspace, output,invocation);
  const { testProductionBrowserArtifacts } = await import("../../../../../../💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts");
  await testProductionBrowserArtifacts(workspace, output);
  await testServiceReadiness(workspace, output,invocation);
  await testBunDependencies(workspace, output);
  await testNativePreparation(workspace, output, invocation);
  const { testGeneratorOwnership, testWgpuGeneratorOwnership, testWgpuGeneratorPublication, testBrowserTransportOwnership, testBrowserInputClosure } = await import("../🧬️generator-ownership/🟦️.ts");
  await testGeneratorOwnership(workspace, output);
  await testBrowserTransportOwnership(workspace);
  await testBrowserInputClosure(workspace);
  await testWgpuGeneratorOwnership(workspace);
  await testWgpuGeneratorPublication(workspace, output);
  const { testNativeRendererOutputs } = await import("../🧊️native-renderer-outputs/🟦️.ts");
  await testNativeRendererOutputs(workspace, output, invocation, phaseControl);
  const { testStylingOutputs } = await import("../🎨️styling-outputs/🟦️.ts");
  await testStylingOutputs(workspace, output);
  const { testStylingPythonSourceContractV1 } = await import("../🐍️styling-python-outputs/🟦️.ts");
  await testStylingPythonSourceContractV1(workspace);
  const { testWgpuLiveActivation } = await import("../🧊️live-activation/🟦️.ts");
  await testWgpuLiveActivation(workspace, output, invocation, phaseControl);
  const { testNativeRuntime } = await import("../🧊️native-runtime/🟦️.ts");
  await testNativeRuntime(workspace, output, invocation, phaseControl);
  const { testPlaygroundInputView } = await import("../🎮️playground-input-view/🟦️.ts");
  await testPlaygroundInputView(workspace);
  const { testInferredNativeInputs } = await import("../../../../🧪️test/🕸️dependencies/🧪️tests/🦀️inputs/🟦️.ts");
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
    const artifactRunner = await import("../../📦️artifacts/🦀️rust/🟦️.ts");
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
  try{const{repositoryCargoNetworkV1}=await import("../../../🏃️process/🦀️cargo/📥️dependencies/🟦️.ts"),control=scriptCompilerAdmissionControlV1(invocation,"native:foreign-workspace-refusal",0);await assert.rejects(nativeBuild.buildRepositoryCargoArtifacts("package/Cargo.toml",[],captureRoot,{producer:"artifact",invocation,control,network:repositoryCargoNetworkV1(invocation,workspace),consume:async()=>{consumed=true;throw Error("Foreign workspace must never admit a compiler consumer");}}),/workspace/u);assert.equal(consumed,false);assert.deepEqual(readdirSync(captureRoot),[]);}finally{rmSync(captureRoot,{recursive:true,force:true});}
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

export function createCachePolicyTests(dependencies: Record<string, any>, testSource: { directory: string; url: string }) {
  const { assert, cacheInternals, chmodSync, copyFileSync, createRequire, devToolingEnv, dirname, EventEmitter, existsSync, getWorkspaceRoot, join, lstatSync, mkdirSync, mkdtempSync, plugin, readFileSync, relative, resolve, rmSync, SCRIPT_ROOT, spawn, stageRepositoryArtifacts, utimesSync, wasmBindgenVersion, wasmBuildArguments, wasmBuildEnvironment, writeFileSync } = dependencies;

  /** 🧪️ Executes language-neutral policy examples against the Nx project plugin. */
  async function testCacheContracts(invocation: ScriptInvocation, phaseControl: ResourceLeaseControl): Promise<void> {
    testCacheContractBindingsV1();
    const { validate } = createRequire(testSource.url)("jsonschema");
    const policy = JSON.parse(readFileSync(join(SCRIPT_ROOT, "🔣️policy.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(SCRIPT_ROOT, "🧬️schema/🔣️.json"), "utf8"));
    assert.equal(validate(policy, schema).valid, true);
    assert.ok(cacheInternals, "cache policy must be exposed for contract verification");
    const authoredWorkspace = JSON.parse(readFileSync(join(getWorkspaceRoot(), "📋️project.json"), "utf8"));
    const { availableParallelism } = await import("node:os");
    const { semioBuildParallel } = await import("../../../📦️packages/🟦️typescript/🟦️.ts");
    const previousParallel = process.env.SEMIO_BUILD_PARALLEL;
    delete process.env.SEMIO_BUILD_PARALLEL;
    try {
      assert.equal(semioBuildParallel(), Math.max(1, availableParallelism()));
      process.env.SEMIO_BUILD_PARALLEL = "5";
      assert.equal(semioBuildParallel(), 5);
    } finally {
      if (previousParallel === undefined) delete process.env.SEMIO_BUILD_PARALLEL;
      else process.env.SEMIO_BUILD_PARALLEL = previousParallel;
    }
    assert.equal(devToolingEnv().NX_PARALLEL, String(semioBuildParallel()));
    const { semioBuildParallelFlag } = await import("../../../📦️packages/🟦️typescript/🟦️.ts");
    assert.deepEqual(semioBuildParallelFlag(), ["--parallel", String(semioBuildParallel())]);
    assert.equal(JSON.parse(readFileSync(join(getWorkspaceRoot(), "nx.json"), "utf8")).parallel, undefined);
    for (const name of policy.serialTargets) {
      const target = authoredWorkspace.targets[name];
      if (target?.parallelism !== undefined) assert.equal(target.parallelism, false, name);
    }
    for (const name of ["test", "lint", "build", "verify"]) assert.equal(authoredWorkspace.targets[name].cache, true, name);
    for (const name of ["format", "setup", "publish"]) assert.equal(authoredWorkspace.targets[name].cache, false, name);
    const root = getWorkspaceRoot();
    assert.equal(Object.keys(JSON.parse(readFileSync(join(root, "nx.json"), "utf8")).targetDefaults ?? {}).length, 0, "Native Nx defaults override custom project metadata; apply defaults inside the repository plugin");
    assert.equal(wasmBuildEnvironment(root, {}).CARGO_TARGET_DIR, undefined);
    assert.equal(wasmBuildEnvironment(root, { CARGO_TARGET_DIR: "chosen-cache" }).CARGO_TARGET_DIR, "chosen-cache");
    const vectors = JSON.parse(readFileSync(join(SCRIPT_ROOT, "🧫️fixtures/nx-contract/🔣️.json"), "utf8"));
    
    const loggingKeys = Object.keys(vectors.daemonEnvironment), savedLogging = Object.fromEntries(loggingKeys.map((key) => [key, process.env[key]]));
    const quiet = devToolingEnv(Object.fromEntries(loggingKeys.map((key) => [key, undefined])));
    try {
      for (const [key, value] of Object.entries(vectors.daemonEnvironment)) { assert.equal(quiet[key], value); process.env[key] = quiet[key]; }
      const oracle = createRequire(testSource.url)("nx/src/daemon/client/daemon-environment.js").getDaemonSpawnEnv();
      for (const [key, value] of Object.entries(vectors.daemonEnvironment)) assert.equal(oracle[key], value, key);
      assert.equal(devToolingEnv({ NX_NATIVE_LOGGING: "nx=debug" }).NX_NATIVE_LOGGING, "nx=debug");
    } finally { for (const [key, value] of Object.entries(savedLogging)) if (value === undefined) delete process.env[key]; else process.env[key] = value; }
    for (const row of vectors.wasmProfiles) {
      const args = wasmBuildArguments(row.profile);
      assert.deepEqual(args, { pack: row.pack, cargo: row.cargo });
      for (const [tool, flags] of [["cargo", args.cargo], ["wasm-pack", args.pack]] as const) {
        const help = Bun.spawnSync([tool, "build", ...flags, "--help"], { stdout: "pipe", stderr: "pipe" });
        assert.equal(help.exitCode, 0, `${tool} rejected ${flags.join(" ")}: ${help.stderr.toString()}`);
      }
    }
    const socketFrames = createRequire(testSource.url)("nx/src/utils/consume-messages-from-socket.js");
    assert.equal(typeof socketFrames.writeMessage, "function", "Nx must preserve Unicode across socket frame boundaries");
    for (const size of vectors.socketFrames.chunkSizes) {
      const frames: Buffer[] = [], received: unknown[] = [];
      for (const message of vectors.socketFrames.messages) socketFrames.writeMessage({ write: (chunk: Buffer) => frames.push(chunk) }, Buffer.from(JSON.stringify(message)));
      const consume = socketFrames.consumeMessagesFromSocket((message: Buffer) => received.push(socketFrames.parseMessage(message)));
      const bytes = Buffer.concat(frames);
      for (let offset = 0; offset < bytes.length; offset += size) consume(bytes.subarray(offset, offset + size));
      assert.deepEqual(received, vectors.socketFrames.messages, `Unicode socket frame size ${size}`);
    }
    const { isCacheableTask } = createRequire(testSource.url)("nx/src/tasks-runner/utils");
    assert.equal(typeof cacheInternals.matchesUncached, "function");
    assert.equal(typeof cacheInternals.cacheableFamily, "function");
    assert.equal(cacheInternals.matchesUncached("format-check", policy), false);
    assert.equal(cacheInternals.matchesUncached("format", policy), true);
    assert.equal(cacheInternals.matchesUncached("commit", policy), true);
    assert.equal(cacheInternals.matchesUncached("os", policy), true);
    assert.equal(cacheInternals.matchesUncached("os-dev", policy), false);
    assert.equal(cacheInternals.matchesUncached("setup-git", policy), true);
    assert.equal(cacheInternals.matchesUncached("cache-report", policy), true);
    assert.equal(cacheInternals.cacheableFamily("format-check"), true);
    assert.equal(cacheInternals.cacheableFamily("generator-inputs"), false);
    assert.equal(cacheInternals.mutatingName("build-report"), false, "a document id that happens to be named 'report' must not be treated as a mutating report writer");
    assert.equal(cacheInternals.mutatingName("verify-dependencies-freeze-write-baseline"), false, "a deterministic baseline writer must not be blanket-matched by 'write-baseline'");
    assert.equal(cacheInternals.mutatingName("reset-document-ownership"), false, "a cargo test proving reset behavior is not itself a mutating reset");
    assert.equal(cacheInternals.mutatingName("cache-prune"), true);
    assert.equal(cacheInternals.mutatingName("stdio-fuzz"), true);
    assert.equal(cacheInternals.liveName("hub-live-catalog-check"), true, "the name heuristic stays domain-neutral");
    assert.equal(cacheInternals.targetPolicy("hub-live-catalog-check", { cache: false }, policy).cache, false, "owner cache refusal precedes cachedExact");
    assert.equal(cacheInternals.liveName("admin-live-journey-check"), true, "a genuine live-browser/live-server journey stays uncached");
    assert.equal(cacheInternals.liveName("collab-e2e"), true);
    for (const row of vectors.policies) {
      const enabled = cacheInternals.targetPolicy(row.target, { cache: true, options: { command: row.command ?? `bun ./📜️script.ts ${row.target}` } }, policy);
      const disabled = cacheInternals.targetPolicy(row.target, { cache: false, options: { command: row.command ?? `bun ./📜️script.ts ${row.target}` } }, policy);
      assert.deepEqual({ cache: enabled.cache, continuous: enabled.continuous ?? false }, { cache: row.cache, continuous: row.continuous }, row.target);
      assert.equal(disabled.cache, row.disabledCache, `${row.target}: owner cache refusal precedes automatic cache families`);
      assert.equal(isCacheableTask({ cache: enabled.cache, continuous: enabled.continuous === true, target: { project: "probe", target: row.target }, overrides: {} }), row.cache, `${row.target}: nx isCacheableTask`);
    }
    await testCommandInputs(root, ticketOutput(root, []), invocation, phaseControl);
    await (await import("../🚀️bootstrap/🟦️.ts")).testNxBootstrap(root, ticketOutput(root, []));
    await runProcessTestOwner(invocation, root, ticketOutput(root, []), "artifact-publication");
    await runProcessTestOwner(invocation, root, ticketOutput(root, []), "native-artifact-publication");
    await (await import("../📇️artifacts/🟦️.ts")).testArtifactRegistry(root, ticketOutput(root, []));
    await (await import("../🌎️hub/🟦️.ts")).testHubBuild(root);
    await testDependencyBootstrap(root, ticketOutput(root, []));
    await testNxTooling(root, ticketOutput(root, []));
    await testDependencyCancellation(root, ticketOutput(root, []));
    const result = await (await import("../../📇️inventory/🧪️tests/🕸️coverage/🟦️.ts")).testNativeInventory(root, inventory), contracts = result.projects;
    const toml = createRequire(testSource.url)("@iarna/toml");
    let componentPackages = 0;
    const componentLaunchers: { project: string; pluginId: string }[] = [];
    for (const project of contracts) {
      const path = join(root, project.root, "Cargo.toml");
      if (!existsSync(path)) continue;
      const manifest = toml.parse(readFileSync(path, "utf8"));
      if (!manifest.package?.metadata?.component?.package || !["plugin", "extension"].includes(manifest.package?.metadata?.semio?.["component-kind"])) continue;
      componentPackages++;
      componentLaunchers.push({ project: project.name, pluginId: manifest.package.metadata.component.package.slice("semio:".length) });
      for (const removed of vectors.materialization.removedTargets) assert.equal(project.targets[removed], undefined, `${project.name}:${removed} is a driver stage, never a per-component Nx target`);
      const component = project.targets[vectors.componentTarget.target];
      assert.equal(component?.cache, false, `${project.name}:component answers to Cargo freshness and the stage cache`);
      assert.equal(component.metadata?.semio?.orchestrator, true, `${project.name}:component owns its driver without an owner-command deadline`);
      for (const dependency of vectors.componentTarget.dependsOn) assert.ok(component.dependsOn.includes(dependency), `${project.name}:component follows ${dependency}`);
      assert.equal(component.outputs[0], vectors.componentTarget.output);
      assert.ok(component.options.command.includes(vectors.componentTarget.command));
    }
    {
      const row = vectors.materialization, shared = contracts.find((project) => project.name === row.project)?.targets[row.support];
      assert.equal(shared?.cache, row.supportCache);
      const supportRoot = `{projectRoot}/${row.modules}`;
      assert.deepEqual(shared.outputs, [supportRoot, `{projectRoot}/${row.canonicalModules}`].flatMap(base => [`${base}/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim`, `${base}/🪞️vendor/🔤️guestslim-typst-fonts.bin`, `${base}/🧵️shard`]), "browser support owns the preview2 shims, the typst font asset every served module root needs, and the shard worker");
      assert.deepEqual(shared.dependsOn, ["semio-framework-os-infinite:fonts"], "browser support stages the font the fonts target publishes");
    }
    for (const row of vectors.releaseOrchestrators) {
      const target = contracts.find((project) => project.name === row.project)?.targets[row.target];
      assert.ok(target, `${row.project}:${row.target} must be a release orchestrator`);
      assert.equal(target.cache, false);
      assert.equal(target.metadata?.semio?.orchestrator, true);
      assert.equal(target.executor, "nx:run-commands", `${row.project}:${row.target} must not be wrapped by an owner command deadline`);
      assert.deepEqual(target.metadata.semio.release.jobs.map((job: any) => job.id), row.jobs);
      for (const prerequisite of row.prerequisites) assert.ok(target.dependsOn.includes(prerequisite), `${row.project}:${row.target} needs ${prerequisite}`);
      const generators = new Set(Object.values(JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")).generatorContracts ?? {}).map((contract: any) => contract.target));
      assert.ok(target.dependsOn.every((dependency: string) => row.prerequisites.includes(dependency) || generators.has(dependency)), `${row.project}:${row.target} depends on generators and prerequisites only: ${target.dependsOn.filter((dependency: string) => !row.prerequisites.includes(dependency) && !generators.has(dependency)).join(", ")}`);
    }
    assert.ok(componentPackages > 0, "Component metadata must discover plugin producers");
    const fontProject = contracts.find((project) => project.name === vectors.fontAssets.project)!;
    const fontToolProject = contracts.find((project) => project.name === vectors.fontAssets.toolProject)!;
    for (const [project, name, output] of [[fontToolProject, vectors.fontAssets.tool, vectors.fontAssets.toolOutput], [fontProject, vectors.fontAssets.producer, vectors.fontAssets.output]] as const) {
      assert.equal(project.targets[name]?.cache, true, `${name} must be separately cacheable`);
      assert.deepEqual(project.targets[name].outputs, [output]);
    }
    assert.deepEqual(fontProject.targets[vectors.fontAssets.producer].dependsOn, [`${vectors.fontAssets.toolProject}:${vectors.fontAssets.tool}`]);
    const { testSelectedFontDependencies, testSelectedPackageIdentities } = await import("../🎯️selected-runtime/🟦️.ts");
    testSelectedPackageIdentities(root);
    await testSelectedFontDependencies(root);
    assert.ok(fontProject.targets[vectors.fontAssets.producer].inputs.some((input: any) => input.dependentTasksOutputFiles === "**/*"));
    const activationRoot = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation");
    const activation = await import(join(activationRoot, "🟦️.ts"));
    const activationCases = JSON.parse(readFileSync(join(activationRoot, "🧫️cases.json"), "utf8"));
    const ActivationAjv = createRequire(testSource.url)("ajv");
    const activationSchema = JSON.parse(readFileSync(join(activationRoot, "🧬️schema/🔣️.json"), "utf8"));
    const validateActivation = new ActivationAjv().addSchema(activationSchema).getSchema(`${activationSchema.$id}#/$defs/DevActivationV1`)!;
    const { keyBy, sortBy } = createRequire(testSource.url)("lodash");
    let activationReceipt: any;
    for (const cycle of activationCases.cycles) {
      const completed = cycle.plugins.map((row:unknown) => {const[pluginId,digest]=testStringsV1(row);if(typeof pluginId!=="string"||typeof digest!=="string")throw Error("Actual activation plugin/digest pair required");return{pluginId,artifactSha256:digest.repeat(64)};});
      const prior = keyBy(activationReceipt?.plugins ?? [], "pluginId");
      const nextTimestamp = Math.max(cycle.now, 1, ...Object.values(prior).map((row: any) => row.rebuiltAt + 1));
      const oracle = sortBy(completed, "pluginId").map((row: any) => ({ ...row, rebuiltAt: prior[row.pluginId]?.artifactSha256 === row.artifactSha256 ? prior[row.pluginId].rebuiltAt : nextTimestamp }));
      const next = activation.nextActivationReceipt(activationCases.variant, activationCases.profile, completed, activationReceipt, cycle.now);
      assert.equal(validateActivation(next), true, JSON.stringify(validateActivation.errors));
      assert.deepEqual(next.plugins, oracle);
      assert.deepEqual(next.plugins.map((row: any) => [row.pluginId, row.artifactSha256[0], row.rebuiltAt]), cycle.expected);
      assert.deepEqual(activation.parseActivationReceipt(JSON.parse(JSON.stringify(next))), next);
      activationReceipt = next;
    }
    assert.throws(() => activation.parseActivationReceipt({ ...activationReceipt, plugins: [...activationReceipt.plugins, ...activationReceipt.plugins] }), /Duplicate/);
    assert.throws(() => activation.parseActivationReceipt({ ...activationReceipt, unknown: true }), /Invalid/);
    assert.throws(() => activation.nextActivationReceipt("other", "prototype", [], activationReceipt, 500), /identity/);
    assert.throws(() => activation.nextActivationReceipt("../note", "prototype", [], undefined, 500), /Invalid/);
    const sessionProject = contracts.find((project) => project.name === vectors.playgroundSessions.project)!;
    for (const variant of vectors.playgroundSessions.variants) {
      const session = sessionProject.targets[`session-${variant}`];
      assert.equal(session?.cache, true, `${variant} needs its own cacheable playground session`);
      assert.ok(session.dependsOn.includes((await cacheInternals.workspaceCatalogBuildContributionsV1(root))[0].project+":"+(await cacheInternals.workspaceCatalogBuildContributionsV1(root))[0].generateTarget));
      assert.deepEqual(session.dependsOn, [(await cacheInternals.workspaceCatalogBuildContributionsV1(root))[0].project+":"+(await cacheInternals.workspaceCatalogBuildContributionsV1(root))[0].generateTarget], `${variant} session needs no component build`);
      const descriptors = session.inputs.filter((input: unknown) => typeof input === "string" && input.startsWith("{workspaceRoot}/"));
      assert.ok(descriptors.length > 0 && descriptors.every((input: string) => vectors.playgroundSessions.descriptorInputs.some((name: string) => input.endsWith(`/${name}`))), `${variant} session hashes the committed semantic descriptors of its closure`);
      assert.deepEqual(session.outputs, [`{projectRoot}/dist/sessions/${variant}`]);
      assert.ok(session.inputs.some((input: any) => input.dependentTasksOutputFiles === "**/*"));
    }
    const registryRoot = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry");
    const { generatePlaygroundRegistry } = await import(join(registryRoot, "🎮️playground/🔎️discovery/🟦️.ts"));
    const { buildPlaygroundSession } = await import(join(registryRoot, "🎮️playground/🧭️session/🟦️.ts"));
    const playgrounds = generatePlaygroundRegistry(root);
    for (const project of contracts) for (const [name, target] of Object.entries(project.targets) as [string, any][]) if (target.options?.command?.includes("⚡️caching/🦀️cargo/📜️script.ts") && ["build", "check", "test"].includes(name)) assert.ok(project.targets[name], `Missing native editor target ${project.name}:${name}`);
    const preparationProject = contracts.find((project) => project.name === vectors.playgroundPreparation.project)!;
    for (const playground of playgrounds) {
      for (const engine of playground.engines) assert.ok(contracts.some((project) => resolve(root, project.root) === resolve(root, engine) && project.targets.wasm), `Engine ${engine} must name an authored producer`);
      for (const profile of vectors.playgroundPreparation.profiles) {
        for (const command of ["prepare", "run", "smoke"]) assert.ok(preparationProject.targets[`${command}-${playground.variant}-native-${profile}`], "Missing native variant editor target");

        const targetName = `prepare-${playground.variant}-react-${profile}`, preparation = preparationProject.targets[targetName];
        assert.ok(preparation, `${targetName} needs declared prerequisites`);
        assert.equal(preparation.cache, false, `${targetName} builds its closure through the driver`);
        assert.equal(preparation.metadata?.semio?.orchestrator, true);
        const wgpuTargetName = `prepare-${playground.variant}-wgpu-${profile}`, wgpuPreparation = preparationProject.targets[wgpuTargetName];
        assert.ok(wgpuPreparation, `${wgpuTargetName} needs declared prerequisites`);
        assert.ok(wgpuPreparation.dependsOn.includes(`@semio-tech/framework-renderer-wgpu:wasm`));
        assert.equal(wgpuPreparation.cache, false, `${wgpuTargetName} builds its closure through the driver`);
        assert.deepEqual(wgpuPreparation.outputs, preparation.outputs, `${wgpuTargetName} publishes the same boot closure modules`);
        const activationName = `activate-${playground.variant}-react-${profile}`, activationTarget = preparationProject.targets[activationName];
        assert.ok(activationTarget, `${activationName} must follow completed preparation`);
        assert.equal(activationTarget.cache, true, `${activationName} publishes into a variant/profile-scoped runtime root and is safe to replay`);
        assert.deepEqual(activationTarget.outputs, [`{projectRoot}/dist/runtime/react/${profile}/${playground.variant}`]);
        assert.deepEqual(activationTarget.dependsOn, [targetName]);
        const wgpuActivationName = `activate-${playground.variant}-wgpu-${profile}`, wgpuActivationTarget = preparationProject.targets[wgpuActivationName];
        assert.ok(wgpuActivationTarget, `${wgpuActivationName} must follow completed preparation`);
        assert.equal(wgpuActivationTarget.cache, false, `${wgpuActivationName} publishes live extensions and reload notifications on every activation`);
        assert.deepEqual(wgpuActivationTarget.outputs, [`{projectRoot}/dist/runtime/wgpu/${profile}/${playground.variant}`]);
        assert.deepEqual(wgpuActivationTarget.dependsOn, [wgpuTargetName]);
        {
          const serverName = `serve-${playground.variant}-react-${profile}`, server = preparationProject.targets[serverName];
          assert.ok(server, `${serverName} needs an Nx server owner`);
          assert.equal(server.continuous, true);
          assert.equal(server.cache, false);
          assert.deepEqual(server.outputs, []);
          assert.deepEqual(server.dependsOn, [`@semio-tech/plugin-registry:session-${playground.variant}`]);
          assert.equal(server.options.command, `bun ./📜️script.ts serve ${playground.variant} react ${profile}`);
        }
        {
          const serverName = `dev-${playground.variant}-react-${profile}`, server = preparationProject.targets[serverName];
          assert.ok(server, `${serverName} needs an Nx server owner`);
          assert.equal(server.continuous, true);
          assert.equal(server.cache, false);
          assert.deepEqual(server.outputs, []);
          assert.deepEqual(server.dependsOn, [activationName]);
          assert.equal(server.options.command, `bun ./📜️script.ts serve ${playground.variant} react ${profile}`);
        }
        const hostLauncher = componentLaunchers.find((entry) => entry.pluginId === playground.pluginId);
        if (!hostLauncher) throw new Error(`${playground.variant}: host plugin must launch`);
        const moduleOf = (pluginId: string) => contracts.find((project) => project.name === componentLaunchers.find((entry) => entry.pluginId === pluginId)?.project);
        const hostDirectory = moduleOf(playground.pluginId) && toml.parse(readFileSync(join(root, moduleOf(playground.pluginId)!.root, "Cargo.toml"), "utf8")).package.metadata.semio["deployment-directory"];
        if (hostDirectory !== undefined) assert.ok(preparation.outputs.some((output: string) => output.endsWith(`/${hostDirectory}`)), `${targetName}: boot prepare must publish the host module`);
        const fullSession = buildPlaygroundSession(playground.variant, (await import("../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts")).readGeneratedCatalogProjection(join(root,(await cacheInternals.workspaceCatalogBuildContributionsV1(root))[0].publication.outputRoot))).plugins;
        assert.ok(preparation.outputs.length <= fullSession.length, `${targetName}: boot prepare must not exceed the full session closure`);
        if (fullSession.length > preparation.outputs.length + 5) assert.ok(preparation.outputs.length < fullSession.length / 2, `${targetName}: host boot prepare must skip the catalog fanout`);
        for (const prerequisite of vectors.closurePreparation.prerequisites) assert.ok(preparation.dependsOn.includes(prerequisite));
        const engines = playground.engines.map((engine: string) => contracts.find((project) => resolve(root, project.root) === resolve(root, engine))!.name + ":wasm");
        for (const engine of engines) assert.ok(preparation.dependsOn.includes(engine));
        assert.ok(preparation.dependsOn.includes(`@semio-tech/plugin-registry:session-${playground.variant}`));
        assert.ok(preparation.dependsOn.includes(`${vectors.materialization.project}:${vectors.materialization.support}`));
        assert.ok(preparation.dependsOn.includes(vectors.playgroundPreparation.fonts));
      }
      const target = `session-${playground.variant}`;
      assert.ok(sessionProject.targets[target]);
    }
    assert.deepEqual(result.violations.filter((finding) => finding.rule === "ORCH-01"), [], "All public commands must use one script entrypoint");
    for (const entry of vectors.entrypoints) assert.ok(contracts.find((project) => project.name === entry.project)?.targets[entry.target].dependsOn?.includes(entry.prerequisite), `${entry.project}:${entry.target} needs ${entry.prerequisite} in Nx`);
    for (const entry of vectors.nativeBinaries) {
      const project = contracts.find((project) => project.name === entry.project)!;
      assert.equal(project.targets[entry.target].cache, true, `${entry.project} binary must be restorable`);
      const oracle=createRequire(testSource.url)("nx/src/tasks-runner/utils").getOutputs({[entry.project]:{data:project}}, {project:entry.project,target:entry.target,configuration:entry.configuration},{});
      assert.deepEqual(oracle,[entry.output.replace("{projectRoot}",project.root)]);
    }
    for (const entry of vectors.components) {
      const project = contracts.find((project) => project.name === entry.project)!;
      const target = project.targets[entry.target];
      assert.ok(project.namedInputs?.default?.includes(entry.schemaInput), `${entry.project} must hash its shared component schema`);
      assert.equal(target.cache, true, `${entry.project}:${entry.target} component must be restorable`);
      assert.deepEqual(target.outputs, [entry.output]);
      assert.ok(contracts.find((project) => project.name === entry.consumerProject)!.targets[entry.consumerTarget].dependsOn.includes(`${entry.project}:${entry.target}`));
    }
    const mcpRoot = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp";
    const mcp = await import(join(root, mcpRoot, "🟦️.ts"));
    const binaryVectors = JSON.parse(readFileSync(join(root, mcpRoot, "🎚️config/🧱️binary-gate.json"), "utf8"));
    const pathOracle = createRequire(testSource.url)("node:path");
    for (const row of binaryVectors.pathCases) {
      assert.equal(mcp.resolveMcpBinaryPath(row.repoRoot, row.environment, row.platform), row.expected);
      const paths = row.platform === "win32" ? pathOracle.win32 : pathOracle.posix;
      assert.equal(paths.resolve(row.repoRoot, row.environment.SEMIO_OS_MCP_BIN ?? binaryVectors.artifactRoot, ...(row.environment.SEMIO_OS_MCP_BIN ? [] : [binaryVectors.cargoBinary + (row.platform === "win32" ? ".exe" : "")])), row.expected);
    }
    const generators = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")).generatorContracts;
    for (const id of vectors.generators) {
      const authority = generators[id], split = authority.target.lastIndexOf(":"), project = contracts.find((project) => project.name === authority.target.slice(0, split))!;
      const target = project.targets[authority.target.slice(split + 1)];
      assert.equal(target.cache, true, `${id} must cache its verified deliverables`);
      assert.deepEqual(target.outputs, authority.outputRoots.filter((output: any) => !output.producer || output.producer.target === authority.target).map((output: any) => `{workspaceRoot}/${output.path}`), id);
      for (const output of authority.outputRoots.filter((output: any) => output.producer && output.producer.target !== authority.target)) {
        const split = output.producer.target.lastIndexOf(":"), producer = contracts.find((project) => project.name === output.producer.target.slice(0, split));
        assert.ok(producer!.targets[output.producer.target.slice(split + 1)].outputs.includes(`{workspaceRoot}/${output.path}`));
        assert.ok(target.dependsOn.includes(output.producer.target));
      }
      for (const path of authority.inputPatterns) assert.ok(target.inputs.includes(`{workspaceRoot}/${path}`), `${id} missing ${path}`);
      if (authority.inputDiscovery) {
        const fingerprint = policy.generatorInputs[authority.inputDiscovery.kind], separator = fingerprint.target.lastIndexOf(":"), owner = contracts.find((project) => project.name === fingerprint.target.slice(0, separator));
        const guard = owner!.targets[fingerprint.target.slice(separator + 1)];
        assert.ok(target.inputs.some((input: any) => input.dependentTasksOutputFiles === fingerprint.output), `${id} must hash the discovered input receipt`);
        assert.ok(!target.inputs.some((input: any) => input.runtime?.includes("generator-inputs")), `${id} must not repeat discovery inside the hasher`);
        assert.ok(target.dependsOn.includes(fingerprint.target)); assert.equal(guard.cache, false, `${id} discovery receipt producer re-digests on every run: its receipt digests bytes its own cache key cannot name`);
        assert.deepEqual(guard.outputs, [`{workspaceRoot}/${fingerprint.output}`]);
      }
      if (authority.checkTarget) {
        const guardName = authority.checkTarget.slice(authority.checkTarget.lastIndexOf(":") + 1), guardTarget = project.targets[guardName];
        assert.equal(guardTarget.cache, true, `${id} freshness checks must be soundly cacheable`);
        for (const path of authority.inputPatterns) assert.ok(guardTarget.inputs.includes(`{workspaceRoot}/${path}`), `${id} check target missing generator input ${path}`);
        for (const output of authority.outputRoots) assert.ok(guardTarget.inputs.some((input: any) => typeof input === "object" && typeof input.runtime === "string" && input.runtime.includes(JSON.stringify(join(root, output.path)))), `${id} check target must hash its own generated output ${output.path} live, since Nx's own fileset walk silently drops anything .gitignore covers`);
        if (authority.inputDiscovery) {
          const fingerprint = policy.generatorInputs[authority.inputDiscovery.kind];
          assert.ok(guardTarget.inputs.some((input: any) => input.dependentTasksOutputFiles === fingerprint.output), `${id} check target must hash the discovered input receipt too`);
          assert.ok(guardTarget.dependsOn?.includes(fingerprint.target), `${id} check target must depend on the discovery fingerprint owner`);
        }
      }
    }
    const [validatedProject, validatedTarget] = vectors.validationPrerequisite.target.split(":");
    assert.ok(contracts.find((project) => project.name === validatedProject)?.targets[validatedTarget].dependsOn.includes(vectors.validationPrerequisite.prerequisite));
    assert.ok(!contracts.find((project) => project.name === validatedProject)?.targets[validatedTarget].inputs.includes("^default"), "Exact catalog discovery must not reintroduce generated outputs through broad dependency defaults");
    const workspace = contracts.find((project) => project.name === "workspace")!;
    assert.deepEqual(workspace.targets.setup.dependsOn, vectors.lifecycle.setupDependencies);
    assert.deepEqual(workspace.targets.prepare.dependsOn, vectors.lifecycle.prepareDependencies);
    for (const target of vectors.lifecycle.setupDependencies) assert.equal(workspace.targets[target]?.cache, false);
    for (const name of ["test", "lint", "build", "verify"]) assert.equal(workspace.targets[name]?.cache, true, name);
    assert.equal(workspace.targets.dev, undefined, "the playground variant switch is dissolved: playgrounds are dashboard commands");
    assert.equal((await cacheInternals.nativeTargetCommandInputs({ options: { command: "bun ./x.ts", cwd: "." } }, root)).some((input: { readonly env?: string }) => input.env && input.env.startsWith("SEMIO_")), true, "native leaves must fingerprint SEMIO toolchain env");
    const hostProject = contracts.find((project) => project.name === vectors.platformEnvironment.project)!;
    for (const target of Object.values(hostProject.targets) as any[]) for (const key of vectors.platformEnvironment.keys) assert.equal(target.options?.env?.[key], undefined, `Shared target metadata cannot force ${key}`);
    const bootstrap = vectors.bootstrap, dotnet = contracts.find((project) => project.name === bootstrap.dotnetProject);
    assert.ok(dotnet, "The current .NET support library needs an Nx owner");
    assert.deepEqual(dotnet!.targets.build.outputs, ["{projectRoot}/dist/build"]);
    assert.ok(dotnet!.targets.build.dependsOn.includes("deps"));
    assert.equal(dotnet!.targets.deps.cache, false);
    assert.ok(workspace.targets["deps-dotnet"].dependsOn.includes(`${bootstrap.dotnetProject}:deps`));
    assert.ok(readFileSync(join(root, "Monorepo.sln"), "utf8").includes(bootstrap.dotnetPath.replaceAll("/", "\\")));
    assert.ok(readFileSync(join(root, bootstrap.dotnetPath, "🧪️Semio.Repo.Test.csproj"), "utf8").includes(`Include="${bootstrap.compile}"`));
    assert.ok(existsSync(join(root, bootstrap.dotnetPath, bootstrap.compile)), "The .NET compile item must resolve to its current source file");
    const python = createRequire(testSource.url)("@iarna/toml").parse(readFileSync(join(root, "pyproject.toml"), "utf8"));
    assert.deepEqual(python.tool.uv.workspace.members, bootstrap.pythonMembers);
    assert.deepEqual(workspace.targets["deps-python"].dependsOn, bootstrap.pythonDependencies);
    const styling = contracts.find((project) => project.name === bootstrap.stylingProject)!;
    assert.ok(styling.targets.generate.dependsOn.includes(bootstrap.stylingGenerator));
    assert.deepEqual(styling.targets.generate.outputs, []);
    const nativeGraph = createRequire(testSource.url)("@nx/devkit").readCachedProjectGraph();
    const { createTaskGraph } = createRequire(testSource.url)("nx/src/tasks-runner/create-task-graph");
    for (const [name, target] of Object.entries(styling.targets) as [string, any][]) if (target.options.command.includes(" test")) {
      const tasks = createTaskGraph(nativeGraph, {}, [styling.name], [name], undefined, {}, false);
      assert.ok(tasks.tasks[bootstrap.stylingGenerator], `${styling.name}:${name} must schedule the styling generator through its prerequisites`);
    }
    assert.ok(!/compose|topologic|vcpkg/i.test(readFileSync(join(root, "CMakeLists.txt"), "utf8") + readFileSync(join(root, "CMakePresets.json"), "utf8")));
    const ts = createRequire(testSource.url)("typescript");
    for (const row of vectors.engineOutputs) {
      const project = contracts.find((project) => project.name === row.project)!;
      const physicalOutputs = (outputs: string[]): string[] => outputs.map(output => cacheInternals.resolveOutputPath(output, project.root, root)).sort();
      assert.deepEqual(physicalOutputs(project.targets.wasm.outputs), physicalOutputs(row.outputs));
      const engineSource = ts.createSourceFile("engine.ts", readFileSync(join(root, project.root, "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
      const outputs: string[] = [];
      const inspect = (node: any): void => {
        if (ts.isCallExpression(node) && ["buildWasmWebV1","buildRepositoryWasmWebV1"].includes(node.expression.getText(engineSource))) {
          const output = node.arguments[0].properties.find((property: any) => property.name?.getText(engineSource) === "outputDirectory");
          outputs.push(output?.initializer.text ?? "pkg");
        }
        ts.forEachChild(node, inspect);
      };
      inspect(engineSource);
      assert.deepEqual(outputs, [row.directory]);
    }
    const source = ts.createSourceFile("setup.ts", readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🔧️setup/🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);
    const nxSource = ts.createSourceFile("nx.ts", readFileSync(join(SCRIPT_ROOT, "🚀️bootstrap/📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
    const { testNxCoordinator } = await import("../🛑️coordinator/🟦️.ts");
    await testNxCoordinator(root);
    const invocationNode = nxSource.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "resolveNxInvocation");
    const route = ts.transpileModule(invocationNode.getText(nxSource).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
    const resolveInvocation = new Function("process", "WORKSPACE_ROOT", "resolveBuildSelectionV1", "resolveNxBuildMode", `${route}; return resolveNxInvocation;`)({ env: {} }, root, resolveBuildSelectionV1, resolveNxBuildMode);
    const resolvePrint = new Function("process", "WORKSPACE_ROOT", "resolveBuildSelectionV1", "resolveNxBuildMode", `${route}; return resolveNxInvocation;`)({ env: {} }, root, resolveBuildSelectionV1, resolveNxBuildMode);
    for (const path of ["🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🧫️invocations.json", "♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🧫️invocations.json", "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️invocations.json"]) {
      const invocations = JSON.parse(readFileSync(join(root, path), "utf8"));
      for (const vector of invocations.valid) {
        const result = resolvePrint(vector.input);
        assert.deepEqual(result.args, vector.args);
        assert.equal(result.watch, vector.watch);
        if (vector.env) assert.deepEqual(result.env, vector.env);
      }
      for (const vector of invocations.invalid) assert.throws(() => resolvePrint(vector));
    }
    for (const command of ["prepare", "activate", "serve", "dev"]) {
      const selected = `@semio-tech/framework-os-dev:${command}-note-react-prototype`;
      const invocation = resolveInvocation(["run", selected]);
      assert.equal(invocation.watch, command === "dev" ? "@semio-tech/framework-os-dev:activate-note-react-prototype" : undefined);
      assert.equal(invocation.env.SEMIO_BUILD_MODE, "prototype");
      assert.equal(resolveInvocation(["run", selected, "--graph=stdout"]).watch, undefined);
    }
    for (const renderer of vectors.benchmarkRenderers) {
      const target = `bench-plugins-${renderer}`;
      const args = resolveInvocation(["run", "workspace:bench", "--", "plugins", `--renderer=${renderer}`, "--count", "3"]).args;
      assert.deepEqual(args, ["run", `@semio-tech/framework-os-dev:${target}`, "--", "--count", "3"]);
      assert.equal(hostProject.targets[target].dependsOn.includes("@semio-tech/framework-os-scale-fixture:build-wasm"), renderer === "native");
      assert.equal(hostProject.targets[target].dependsOn.includes("@semio-tech/framework-renderer-wgpu:native-build"), renderer === "native");
    }
    const hostSource = ts.createSourceFile("host.ts", readFileSync(join(root, vectors.platformEnvironment.source), "utf8"), ts.ScriptTarget.Latest, true);
    const apple = hostSource.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "ensureAppleDeveloperDir");
    assert.ok(apple, `Missing Apple tooling selector in ${vectors.platformEnvironment.source}`);
    const selectApple = ts.transpileModule(apple.getText(hostSource), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
    for (const platform of vectors.platformEnvironment.platforms) {
      const host = { platform, env: {} as Record<string, string> };
      new Function("process", "existsSync", `${selectApple}; ensureAppleDeveloperDir();`)(host, () => true);
      assert.deepEqual(Object.keys(host.env).sort(), platform === "darwin" ? [...vectors.platformEnvironment.keys].sort() : []);
    }
    const setup = source.statements.find((node: any) => ts.isClassDeclaration(node) && node.name?.text === "SetupScript").getText(source);
    assert.ok(!/runWorkspaceCodegen|buildRepoMcpClient|runNx|\["nx"|\["build"/.test(setup), "Setup must only provision dependency environments");
    const lock = readFileSync(join(root, "Cargo.lock"), "utf8");
    const lockOracle = createRequire(testSource.url)("@iarna/toml").parse(lock);
    assert.equal(wasmBindgenVersion(lock), lockOracle.package.find((pkg: any) => pkg.name === "wasm-bindgen").version);
    for (const row of vectors.wasm) {
      const project = contracts.find((project) => project.name === row.project);
      assert.ok(project, row.project);
      const expectedOutput = cacheInternals.resolveOutputPath(row.output, project!.root, root);
      assert.ok(expectedOutput, row.output);
      assert.ok(project!.targets.wasm.outputs?.some((output: string) => cacheInternals.resolveOutputPath(output, project!.root, root) === expectedOutput), `${row.project}:wasm must own ${row.output}`);
      assert.notEqual(project?.targets.wasm.cache, false, `${row.project}:wasm must be cacheable`);
      assert.ok(project?.namedInputs?.default.some((input: any) => input.env === "RUSTFLAGS"), `${row.project} must preserve toolchain inputs`);
      if (row.output.startsWith("{projectRoot}/") && !row.output.includes("..")) assert.ok(project?.namedInputs?.default.includes(`!${row.output}/**/*`), `${row.project} must exclude its output in the project fileset`);
    }
    const ticket = process.env.SEMIO_TICKET_DIR;
    assert.ok(ticket, "SEMIO_TICKET_DIR must point to the active ticket for generated test files");
    const fixture = join(resolve(root, ticket), "🗑️generated", "policy-contract");
    const rustFixture = join(fixture, "rust-inputs");
    for (const [path, contents] of Object.entries(vectors.rust.files)) { mkdirSync(dirname(join(rustFixture, path)), { recursive: true }); writeFileSync(join(rustFixture, path), contents as string); }
    const rustInputs = cacheInternals.rustSourceFiles([join(rustFixture, vectors.rust.entry)], rustFixture).map((path: string) => slash(relative(rustFixture, path))).sort();
    assert.deepEqual(rustInputs, vectors.rust.inputs);
    const rustOracle = Bun.spawnSync(["rustc", "--crate-type=lib", "--emit=dep-info", "-o", join(rustFixture, "oracle.d"), vectors.rust.entry], { cwd: rustFixture, stdout: "pipe", stderr: "pipe" });
    assert.equal(rustOracle.exitCode, 0, rustOracle.stderr.toString());
    const rustDependencies = readFileSync(join(rustFixture, "oracle.d"), "utf8").split("\n")[0]!.split(": ")[1]!.trim().split(/\s+/).map((path: string) => slash(relative(rustFixture, resolve(rustFixture, path)))).sort();
    assert.deepEqual(rustInputs, [...new Set(rustDependencies)]);
    const discovery = vectors.rustDiscoveryCache, sourceCache = cacheInternals.createRustSourceCache(discovery.limitBytes);
    const revisionRoot = join(fixture, "rust-revisions");
    for (const [path, contents] of Object.entries(discovery.files)) { mkdirSync(dirname(join(revisionRoot, path)), { recursive: true }); writeFileSync(join(revisionRoot, path), contents as string); }
    mkdirSync(dirname(join(revisionRoot, discovery.entry)), { recursive: true });
    for (const revision of discovery.revisions) {
      const entry = join(revisionRoot, discovery.entry);
      writeFileSync(entry, revision.source);
      utimesSync(entry, 1_700_000_000, 1_700_000_000);
      const inputs = cacheInternals.rustSourceFiles([entry], revisionRoot, sourceCache).map((path: string) => slash(relative(revisionRoot, path))).sort();
      assert.deepEqual(inputs, revision.inputs);
      const oracle = Bun.spawnSync(["rustc", "--crate-type=lib", "--emit=dep-info", "-o", "oracle.d", discovery.entry], { cwd: revisionRoot, stdout: "pipe", stderr: "pipe" });
      assert.equal(oracle.exitCode, 0, oracle.stderr.toString());
      const dependencies = readFileSync(join(revisionRoot, "oracle.d"), "utf8").split("\n")[0]!.split(": ")[1]!.trim().split(/\s+/).map((path: string) => slash(relative(revisionRoot, resolve(revisionRoot, path)))).sort();
      assert.deepEqual(inputs, [...new Set(dependencies)]);
      const hits = sourceCache.hits;
      assert.deepEqual(cacheInternals.rustSourceFiles([entry], revisionRoot, sourceCache).map((path: string) => slash(relative(revisionRoot, path))).sort(), inputs);
      assert.ok(sourceCache.hits > hits);
      assert.ok(sourceCache.bytes <= discovery.limitBytes);
    }
    for (let index = 0; index < 100; index++) {
      const entry = join(revisionRoot, discovery.entry);
      writeFileSync(entry, `pub const VALUE_${index}: usize = ${index};`);
      cacheInternals.rustSourceFiles([entry], revisionRoot, sourceCache);
      assert.ok(sourceCache.bytes <= discovery.limitBytes);
    }
    assert.ok(sourceCache.entries.size < 100);
    const { captureOwnedCommandV1 } = await import(pathToFileURL(join(root, vectors.materialization.source)).href);
    const materializerSpawn = (command: string, args: readonly string[], cwd: string, signal?: AbortSignal) => captureOwnedCommandV1(command, args, cwd, signal ? { signal } : {});
    const materializerFixture = join(fixture, "materializer-cancel");
    mkdirSync(materializerFixture, { recursive: true });
    const childScript = join(materializerFixture, "📜️script.ts"), pidPath = join(materializerFixture, "pid");
    writeFileSync(childScript, 'import { renameSync, writeFileSync } from "node:fs"; if (process.argv[2] === "fail") { process.stderr.write("x".repeat(200000)); process.exit(1); } process.on("SIGTERM", () => {}); setInterval(() => {}, 1000); const stage = process.argv[2] + ".stage"; writeFileSync(stage, String(process.pid)); renameSync(stage, process.argv[2]);');
    rmSync(pidPath, { force: true });
    const controller = new AbortController();
    const execution = materializerSpawn("bun", [childScript, pidPath], materializerFixture, controller.signal).then(() => undefined, (error: Error) => error);
    let childPid = 0;
    try {
      const startupDeadline = Date.now() + 10000;
      while (!existsSync(pidPath)) { assert.ok(Date.now() < startupDeadline, "Materializer fixture must become ready"); await new Promise((accept) => setTimeout(accept, 25)); }
      childPid = Number(readFileSync(pidPath, "utf8"));
      assert.ok(Number.isSafeInteger(childPid) && childPid > 0, "Materializer readiness must identify its child process");
      const started = Date.now();
      controller.abort(new Error("materializer cancelled"));
      assert.match(String(await execution), /materializer cancelled/);
      assert.ok(Date.now() - started < vectors.materialization.cancellation.shutdownMilliseconds);
      assert.throws(() => process.kill(childPid, 0), "Cancelled code generation must terminate the subprocess");
    } finally {
      controller.abort();
      if (childPid && process.platform !== "win32") try { process.kill(-childPid, "SIGKILL"); } catch {}
    }
    const diagnostic = await materializerSpawn("bun", [childScript, "fail"], materializerFixture).then(() => "", (error: Error) => error.message);
    assert.match(diagnostic, /exited with status 1/);
    assert.ok(diagnostic.length < vectors.materialization.cancellation.maximumDiagnosticCharacters);
    const kernelInputs = contracts.find((project) => project.name === "@semio-tech/framework-os-kernel")?.namedInputs?.default;
    assert.ok(kernelInputs?.includes("{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"));
    assert.ok(!kernelInputs?.includes("{workspaceRoot}/🧰️framework/🛍️products/💻️os/**/*.{rs,toml,json,semio,wit,wgsl,glsl,h,c,cpp}"));
    const graph = vectors.pluginGraph;
    mkdirSync(join(fixture, graph.leaf.directory), { recursive: true });
    mkdirSync(join(fixture, "other"), { recursive: true });
    writeFileSync(join(fixture, graph.leaf.directory, "📋️project.json"), JSON.stringify({ name: graph.leaf.name, targets: { test: { options: { command: graph.leaf.testCommand } } } }));
    writeFileSync(join(fixture, graph.leaf.directory, "📜️script.ts"), graph.leaf.script);
    writeFileSync(join(fixture, "other", "📋️project.json"), JSON.stringify({ name: graph.leaf.name }));
    writeFileSync(join(fixture, "📜️script.ts"), graph.workspaceRoot.script);
    writeFileSync(join(fixture, "📋️project.json"), JSON.stringify({ name: graph.workspaceRoot.name, targets: Object.fromEntries(graph.workspaceRoot.targets.map((name: string) => [name, { options: { command: `bun ./📜️script.ts ${name}` } }])) }));
    const goManifest = vectors.go.manifest;
    writeFileSync(join(fixture, "go.mod"), goManifest);
    const goOracle = Bun.spawnSync(["go", "mod", "edit", "-json", `-modfile=${join(fixture, "go.mod")}`], { cwd: fixture, env: { ...process.env, GOWORK: "off" }, stdout: "pipe", stderr: "pipe" });
    assert.equal(goOracle.exitCode, 0, goOracle.stderr.toString());
    const go = JSON.parse(goOracle.stdout.toString());
    const { manifest: _manifest, ...expectedGo } = vectors.go;
    assert.deepEqual(cacheInternals.goDependencies(goManifest), expectedGo);
    assert.deepEqual(cacheInternals.goDependencies(goManifest), { module: go.Module.Path, requires: go.Require.map((row: any) => row.Path), replacements: go.Replace.map((row: any) => ({ module: row.Old.Path, path: row.New.Path })) });
    await assert.rejects(async () => await plugin.createNodesV2[1]([`${graph.leaf.directory}/📋️project.json`, "other/📋️project.json"], {}, { workspaceRoot: fixture }), new RegExp(`duplicate.*${graph.leaf.name}`, "i"));
    const projects = await plugin.createNodesV2[1]([`${graph.leaf.directory}/📋️project.json`], {}, { workspaceRoot: fixture });
    assert.equal(projects[0][1].projects[graph.leaf.name].targets[graph.leaf.cachedTarget].cache, true);
    const rooted = await plugin.createNodesV2[1](["📋️project.json"], {}, { workspaceRoot: fixture });
    const resolvedRoot = rooted[0][1].projects[graph.workspaceRoot.name].targets;
    for (const name of graph.workspaceRoot.targets) {
      const expected = vectors.policies.find((row: { readonly target: string }) => row.target === name);
      assert.ok(expected, `workspace-root fixture target ${name} needs a policy row`);
      assert.equal(resolvedRoot[name].cache, expected.cache, `workspace-root ${name}`);
      assert.equal(resolvedRoot[name].continuous ?? false, expected.continuous, `workspace-root ${name} continuous`);
    }
    assert.equal(projects[0][1].projects[graph.leaf.name].targets.test.executor, "nx:run-commands");
    assert.deepEqual(cacheInternals.nativeDependencies({ dependencies: { core: { path: "../core" }, serde: "1" } }, {}), [{ name: "core", path: "../core", workspace: false, kind: "dependencies" }]);
    const artifact = join(fixture, "consumer");
    writeFileSync(artifact, "native fixture\n");
    chmodSync(artifact, 0o755);
    const staged = join(fixture, "dist/build");
    await stageRepositoryArtifacts(staged, "leaf/Cargo.toml", new Map([["consumer", artifact], ["obsolete", artifact]]));
    await stageRepositoryArtifacts(staged, "leaf/Cargo.toml", new Map([["consumer", artifact]]));
    assert.deepEqual(readFileSync(join(staged, "consumer")), readFileSync(artifact));
    assert.equal(existsSync(join(staged, "obsolete")), false);
    if (process.platform !== "win32") assert.equal(lstatSync(join(staged, "consumer")).mode & 0o111, 0o111);
    await assert.rejects(() => stageRepositoryArtifacts(staged, "another/Cargo.toml", new Map()), /Unowned/);
    await assert.rejects(() => stageRepositoryArtifacts(staged, "leaf/Cargo.toml", new Map([["../escape", artifact]])), /Invalid artifact/);
    assert.ok(existsSync(join(staged, "consumer")));
    const testApi = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts"));
    const taxonomy = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
    mkdirSync(dirname(join(fixture, taxonomy)), { recursive: true });
    copyFileSync(join(root, taxonomy), join(fixture, taxonomy));
    const results = join(testApi.testCacheRoot(fixture), "tasks", "probe", "test", "results", "run");
    mkdirSync(results, { recursive: true });
    const retainedHash = "sha256:" + "a".repeat(64);
    writeFileSync(join(results, "🗝️run.json"), JSON.stringify({ artifacts: [{ sha256: retainedHash }] }));
    assert.ok(testApi.markReferencedBlobs(fixture, { contributions: [] } as any).has(retainedHash), "Task-scoped restored reports must retain their referenced blobs");
    const past = new Date(Date.now() - 30 * 86400000).toISOString();
    testApi.writeLease(results, { runId: "complete", agentId: "cache-test", pid: process.pid, state: "complete", createdAt: past, heartbeatAt: past, retention: "ephemeral-success" });
    const active = join(dirname(results), "active");
    testApi.writeLease(active, { runId: "active", agentId: "cache-test", pid: process.pid, state: "active", createdAt: past, heartbeatAt: past, retention: "ephemeral-success" });
    const pendingBlob = testApi.installFixtureBlob(fixture, new TextEncoder().encode("active publication"));
    const gc = testApi.collectGarbage(fixture, { contributions: [] } as any, { dry: false, olderThanMs: 7 * 86400000 });
    assert.ok(gc.removed.some((path: string) => path.endsWith("/results/run")), "Completed scoped runs must be collectible");
    assert.ok(existsSync(active), "Active scoped runs must survive collection");
    assert.ok(existsSync(testApi.fixtureBlobPath(fixture, pendingBlob)), "An active run may not yet have published its blob references");
    writeFileSync(join(active, "🗝️run.json"), "incomplete");
    assert.throws(() => testApi.markReferencedBlobs(fixture, { contributions: [] } as any), /Cannot determine/);
    const { resolveNxInvocation } = await import("../../🚀️bootstrap/📜️script.ts");
    assert.deepEqual(resolveNxInvocation(["run", "workspace:build", "--graph=stdout", "--", "assets"]).args, ["run", "@semio-tech/assets:build", "--graph=stdout"]);
    assert.deepEqual(resolveNxInvocation(["show", "projects"]).args, ["show", "projects"]);
    assert.deepEqual(resolveNxInvocation(["run", "workspace:cpp", "--", "build"]).args, ["run", "workspace:cpp-build"]);
    const cppSource = ts.createSourceFile("cmake.ts", readFileSync(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/📜️script.ts"),"utf8"), ts.ScriptTarget.Latest, true);
    const cpp = cppSource.statements.find((node: any) => ts.isClassDeclaration(node) && node.name?.text === "CppScript");
    const configure = cpp.members.find((node: any) => node.name?.getText(cppSource) === "runConfigure").getText(cppSource);
    assert.ok(!configure.includes("runSetup"), "CMake configuration must not reinstall its prerequisites");
    assert.ok(!cpp.getText(cppSource).includes("ensureVcpkg"), "No current project consumes a vcpkg installation");
    assert.deepEqual(resolveNxInvocation(["run", "workspace:test", "--", "quick"]).args, ["run", "workspace:test", "--", "quick"]);
    assert.deepEqual(resolveNxInvocation(["run", "workspace:test"]).args, ["run", "workspace:test", "--", "fundamental"]);
    assert.deepEqual(resolveNxInvocation(["run", "workspace:test", "--", "long", "repo-mcp", "-run", "TestX"]).args, ["run", "repo-mcp:test", "--", "long", "-run", "TestX"]);
    assert.equal(resolveNxInvocation(["run", "workspace:setup", "--", "deps", "wasm"]).args[1], "workspace:deps-wasm");
    const rootTest = source.statements.find((node: any) => ts.isClassDeclaration(node) && node.name?.text === "TestScript").getText(source);
    assert.ok(!rootTest.includes("run-many"), "The root test target cannot schedule a second Nx graph");
    for (const level of ["fundamental", "quick", "long", "exhaustive"]) assert.equal(workspace.targets[`test-${level}`], undefined, level);
    assert.deepEqual(workspace.targets.test.dependsOn.find((row: any) => typeof row === "object" && row.target === "test"), { target: "test", projects: ["*", "!workspace", "!@semio-tech/repo-test-domain"], params: "forward" });
    for (const retired of [["run", "workspace:dev", "--", "s"], ["run", "workspace:dev", "--", "mcp", "http", "os"], ["run", "@semio-tech/framework-os-dev:dev"]]) assert.deepEqual(resolveNxInvocation(retired).args, retired, `${retired.join(" ")}: the bootstrap is no playground or MCP resolver; the dashboard registry resolves playgrounds`);
    const osDevProject = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json"), "utf8")) as { targets: Record<string, { dependsOn?: readonly string[]; options?: { command?: string } }> };
    assert.equal(osDevProject.targets.dev, undefined, "the framework-os-dev dev alias is dissolved: playgrounds are the generated dev-<variant>-<renderer>-<profile> targets");
    assert.ok(!invocationNode.getText(nxSource).includes("PlaygroundSelections") && !invocationNode.getText(nxSource).includes("PlaygroundCatalog"), "the bootstrap resolves no playground variant");
    assert.equal(root.length > 0, true);
    rmSync(fixture, { recursive: true });
    console.log("[cache-contract] schema, graph ownership, source-byte discovery, native dependency oracles and materializer cancellation passed");
  }
  return { testCacheContracts };
}
