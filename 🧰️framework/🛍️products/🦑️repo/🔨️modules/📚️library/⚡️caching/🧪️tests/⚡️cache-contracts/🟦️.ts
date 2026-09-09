import assert from "node:assert/strict";
import { chmodSync, lstatSync, readFileSync, mkdirSync, writeFileSync, mkdtempSync, readdirSync, rmSync, symlinkSync, utimesSync } from "node:fs";
import { delimiter, dirname, join, resolve, relative } from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath, pathToFileURL } from "node:url";
import { testGraphCoalescing } from "../🕸️daemon/🟦️.ts";
import { testBrowserDistribution } from "../🌐️browser/🟦️.ts";
import { testContinuousServices } from "../🖥️services/🟦️.ts";
import { testServiceReadiness } from "../🌐️service-readiness/🟦️.ts";
import { testDependencyBootstrap, testNxTooling, testDependencyCancellation } from "../📦️dependencies/🟦️.ts";
import { testResourceLeases } from "../../🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts";
import { testWasmOptimizer } from "../🕸️wasm/🟦️.ts";
import { testCiBaseline } from "../../🚦️ci/🧭️baseline/🧪️tests/🧭️baseline-selection/🟦️.ts";
import { testGithubHistory } from "../../🚦️ci/🐙️github/🧪️tests/🐙️workflow-history/🟦️.ts";
import { testCiEnvironment } from "../../🚦️ci/🧭️baseline/🌿️environment/🧪️tests/🌿️workflow-context/🟦️.ts";
import { testCiResolution } from "../../🚦️ci/🧭️baseline/🏃️resolve/🧪️tests/🧭️baseline-resolution/🟦️.ts";
import { testCiBaselineCommand } from "../../🚦️ci/🧪️tests/🚦️baseline-command/🟦️.ts";
import { testDevcontainerContext } from "../../📦️artifacts/🐳️containers/🧪️tests/🐳️devcontainer-context/🟦️.ts";
import { testContainerRuntimeBootstrap } from "../../📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts";
import { testCommandImportClosure } from "../🔗️command-imports/🟦️.ts";
import { testTrunkLockfile } from "../🔒️trunk-lockfile/🟦️.ts";
import { testContainerPersistentState } from "../../📦️artifacts/🐳️containers/🧪️tests/🔒️persistent-state/🟦️.ts";
import { testExtensionAttach } from "../../📦️artifacts/🐳️containers/🧪️tests/🧩️extension-attach/🟦️.ts";
import { testBinaryenToolchain } from "../../🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🛠️binaryen-toolchain/🟦️.ts";
import { testWasmToolFingerprint } from "../../🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🔏️tool-fingerprint/🟦️.ts";

/** 🧪️ Verifies native source and command ownership against compiler and bundler input oracles. */
export async function testCommandInputs(workspace: string, output: string): Promise<void> {
  await testWasmOptimizer(workspace, output);
  await testBinaryenToolchain(workspace, output);
  await testWasmToolFingerprint(workspace, output);
  await testCiBaseline(workspace);
  await testGithubHistory();
  await testCiEnvironment();
  await testCiResolution();
  await testCiBaselineCommand(workspace, output);
  testDevcontainerContext(workspace);
  testContainerRuntimeBootstrap(workspace);
  testContainerPersistentState(workspace);
  testExtensionAttach(workspace, output);
  const { testExtensionHostBuild } = await import("../../../../💻️client/🧩️vscode/🧪️tests/🧩️host-build/🟦️.ts");
  await testExtensionHostBuild(output);
  const { testExtensionPackage } = await import("../../../../💻️client/🧩️vscode/🧪️tests/📦️package/🟦️.ts");
  await testExtensionPackage(output);
  await testCommandImportClosure(workspace, output);
  await testTrunkLockfile(workspace);
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
  await testContinuousServices(workspace, output);
  await testResourceLeases(output);
  testWorkspaceRoots(workspace, output);
  await testRuntimeComponents(workspace);
  const { testPlaygroundPreferences } = await import("../../../🎮️playground/🔒️preferences/🧪️tests/🔒️playground-preferences/🟦️.ts");
  await testPlaygroundPreferences();
  await testDemonstratorRuntime(workspace);
  await testBrowserModuleRelocation(workspace);
  await testBrowserDistribution(workspace, output);
  const { testProductionBrowserArtifacts } = await import("../../../../../../💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts");
  await testProductionBrowserArtifacts(workspace, output);
  await testServiceReadiness(workspace, output);
  await testBunDependencies(workspace, output);
  await testNativePreparation(workspace, output);
  const require = createRequire(import.meta.url), fixtures = join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures");
  const root = mkdtempSync(join(output, "native-inputs-"));
  try {
    const cases = JSON.parse(readFileSync(join(fixtures, "native-inputs/🧫️cases.json"), "utf8"));
    assert.equal(require("jsonschema").validate(cases, JSON.parse(readFileSync(join(fixtures, "native-inputs/🛂️schema/🔣️.json"), "utf8"))).valid, true);
    for (const [path, content] of Object.entries(cases.files)) { const file = join(root, path); mkdirSync(dirname(file), { recursive: true }); writeFileSync(file, String(content)); }
    const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
    const { cacheInternals } = await import(pathToFileURL(join(workspace, library, "🟨️.mjs")).href);
    const inputs = cacheInternals.projectInputs({ name: "fixture", targets: {} }, "domain/📦️packages/🦀️rust", root, new Map());
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
    const entry = join(workspace, library, "⚡️caching/🦀️cargo/📜️script.ts"), actual = cacheInternals.relativeScriptInputs([entry], workspace);
    const built = await require("esbuild").build({ entryPoints: [entry], absWorkingDir: workspace, bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
    const expected = Object.keys(built.metafile.inputs).map((path) => "{workspaceRoot}/" + relative(workspace, resolve(workspace, path))).sort();
    const missing = expected.filter(path => !actual.includes(path)), extra = actual.filter(path => !expected.includes(path));
    assert.deepEqual({ missing: missing.slice(0, 20), extra: extra.slice(0, 20) }, { missing: [], extra: [] }, "Command import ownership must match the runtime compiler");
    for (const [path, content] of Object.entries(cases.commandRouter.files)) { const file = join(root, path); mkdirSync(dirname(file), { recursive: true }); writeFileSync(file, String(content)); }
    assert.equal(typeof cacheInternals.nativeTargetCommandInputs, "function", "Native target normalization must inspect its executable router");
    const routerInputs = cacheInternals.nativeTargetCommandInputs({ options: { cwd: cases.commandRouter.cwd, command: cases.commandRouter.command } }, root, []);
    const routerPaths = routerInputs.filter((input: unknown) => typeof input === "string").map((input: string) => input.replace("{workspaceRoot}/", ""));
    for (const path of cases.commandRouter.included) assert.ok(routerPaths.includes(path), `missing native command input ${path}`);
    for (const path of cases.commandRouter.excluded) assert.ok(!routerPaths.includes(path), `unrelated native command input ${path}`);
    const routerOracle = await require("esbuild").build({ entryPoints: [join(root, cases.commandRouter.entry)], absWorkingDir: root, bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
    assert.deepEqual(routerPaths.sort(), Object.keys(routerOracle.metafile.inputs).sort(), "Local native router closure must match the independent esbuild oracle");
    console.log("[DEBUG] Native input closure preserves rustc-consumed assets, excludes frontend and separate tests, and matches the esbuild command import oracle PASS");
    
    const boundaries = JSON.parse(readFileSync(join(fixtures, "command-boundaries/🧫️cases.json"), "utf8"));
    assert.equal(require("jsonschema").validate(boundaries, JSON.parse(readFileSync(join(fixtures, "command-boundaries/🛂️schema/🔣️.json"), "utf8"))).valid, true);
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
  const require = createRequire(import.meta.url), directory = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🕸️imports");
  const { rewritePreview2ShimImportSource } = await import(pathToFileURL(join(directory, "🟦️.ts")).href);
  const fixture = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🕸️imports/🧫️cases.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(directory, "../../../🌐️browser-bundle/🧬️schema/🔣️.json"), "utf8")), validator = new (require("ajv").default)();
  validator.addSchema(schema);
  assert.ok(validator.validate({ $ref: schema.$id + "#/$defs/Preview2ShimImportRewriteV1" }, fixture));
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
  console.log(`[DEBUG] ${fixture.cases.length} browser relocation vectors match es-module-lexer import spans with a single-file production boundary PASS`);
}

/** 🎪️ Keeps Vite's runtime description independent of build commands and brand implementations. */
export async function testDemonstratorRuntime(workspace: string): Promise<void> {
  const require = createRequire(import.meta.url), directory = join(workspace, "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime");
  const catalog = JSON.parse(readFileSync(join(directory, "🔣️.json"), "utf8")), fixture = JSON.parse(readFileSync(join(directory, "🧫️cases.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(directory, "🧬️schema/🔣️.json"), "utf8")), validator = new (require("ajv").default)();
  validator.addSchema(schema);
  assert.ok(validator.validate(schema.$id, catalog));
  const runtime = await import(pathToFileURL(join(directory, "🟦️.ts")).href);
  const bundle = await require("esbuild").build({ entryPoints: [join(directory, "🟦️.ts")], absWorkingDir: workspace, bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
  const inputs = Object.keys(bundle.metafile.inputs);
  assert.ok(inputs.every(path => !path.endsWith("📜️script.ts") && !path.endsWith("🪧️brand.ts")), "Runtime descriptions must not load task or brand implementations");
  assert.deepEqual(runtime.demonstratorRuntimeBuildVariants(fixture.primary), fixture.additionalBuildVariants);
  assert.equal(new Set(catalog.panes.map((row: any) => row.variant)).size, catalog.panes.length);
  for (const row of catalog.panes) assert.equal(runtime.demonstratorPaneRuntimeVariant(row.variant), row.runtimeVariant);
  for (const variant of fixture.invalidVariants) assert.throws(() => runtime.demonstratorPaneRuntimeVariant(variant), /variant/);
  const moduleCatalog = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json"), "utf8"));
  const names = (ids: string[]) => ids.map(id => moduleCatalog.modules.find((row: any) => row.pluginId === id).directoryName);
  const layout = runtime.demonstratorRuntimeModuleLayout(["demonstrator", "procedural"]);
  assert.deepEqual(layout.pluginModuleDirNames, [...fixture.supportDirectories, ...names(fixture.pluginIds)]);
  assert.deepEqual(layout.extensionModuleDirNames, names(fixture.extensionIds));
  const pipeline = JSON.parse(readFileSync(join(directory, "🧫️pipeline.json"), "utf8"));
  assert.ok(validator.validate({ $ref: schema.$id + "#/$defs/DemonstratorPipelineContract" }, pipeline));
  assert.deepEqual(runtime.DEMONSTRATOR_RUNTIME_TARGETS.map((row: any) => row.variant).sort(), pipeline.variants);
  const project = JSON.parse(readFileSync(join(directory, "../../📋️project.json"), "utf8"));
  assert.deepEqual(project.targets.build.dependsOn, [`prepare-${pipeline.site.profile}`]);
  assert.deepEqual(project.targets.build.outputs, [`{projectRoot}/${pipeline.site.output}`]);
  assert.equal(project.targets.build.options.command, `bun ./${pipeline.site.command} build`);
  assert.deepEqual(project.targets["activate-dev"]?.dependsOn, ["prepare-dev", pipeline.development.activationTarget]);
  for (const name of ["serve", "dev"]) {
    assert.equal(project.targets[name]?.continuous, true);
    assert.equal(project.targets[name]?.cache, false);
    assert.deepEqual(project.targets[name]?.dependsOn, ["activate-dev"]);
    assert.equal(project.targets[name]?.options.command, `bun ./${pipeline.command} serve`);
  }
  assert.deepEqual(project.targets["prepare-e2e"]?.dependsOn, ["activate-dev", pipeline.e2e.browserTarget]);
  assert.deepEqual(project.targets["serve-e2e"]?.dependsOn, ["prepare-e2e"]);
  assert.equal(project.targets["serve-e2e"]?.continuous, true);
  assert.equal(project.targets["serve-e2e"]?.metadata.semio.continuousSharing, false);
  assert.deepEqual(project.targets["test-e2e"]?.dependsOn, ["serve-e2e"]);
  assert.equal(project.targets["test-e2e"]?.cache, false);
  assert.equal(project.targets["test-e2e"]?.options.command, `bun ./${pipeline.e2e.testCommand} test`);
  const testCommand = readFileSync(join(directory, "../..", pipeline.e2e.testCommand), "utf8");
  for (const forbidden of pipeline.forbiddenOrchestration) assert.equal(testCommand.includes(forbidden), false, `E2E hides ${forbidden}`);
  const workspaceProject = JSON.parse(readFileSync(join(workspace, "📋️project.json"), "utf8"));
  assert.ok(!workspaceProject.targets["deps-browsers"].dependsOn?.includes("deps-javascript"), "Browser preparation must not replace dependencies used by running Nx consumers");
  const siteCommand = readFileSync(join(directory, "../..", pipeline.site.command), "utf8");
  for (const forbidden of [...pipeline.forbiddenCommandImports, ...pipeline.forbiddenOrchestration]) assert.equal(siteCommand.includes(forbidden), false, `Site build hides ${forbidden}`);
  for (const profile of pipeline.profiles) {
    const preparation = project.targets[`prepare-${profile}`];
    assert.ok(preparation, `Missing Demonstrator ${profile} preparation`);
    assert.equal(preparation.cache, false);
    assert.deepEqual(preparation.dependsOn, pipeline.variants.map((variant: string) => `${pipeline.preparationProject}:prepare-${variant}-react-${profile}`));
  }
  const { demonstratorRuntimeAssetSources } = await import(pathToFileURL(join(directory, "📦️assets/🟦️.ts")).href);
  for (const profile of pipeline.profiles) {
    const sources = demonstratorRuntimeAssetSources(workspace, profile);
    assert.equal(sources.length, fixture.pluginIds.length + fixture.extensionIds.length + 3);
    assert.equal(new Set(sources.map((row: any) => row.root)).size, sources.length);
    assert.equal(sources.filter((row: any) => row.shimDirectory !== undefined).length, fixture.extensionIds.length);
    for (const row of sources) assert.ok(row.owner === "infinite:fonts" || row.owner.includes(`:${profile}`), row.owner);
    assert.equal(sources.some((row: any) => row.root.includes("/runtime/") || row.root.includes("🏪️store")), false);
  }
  const command = readFileSync(join(directory, "../..", pipeline.command), "utf8");
  for (const forbidden of [...pipeline.forbiddenCommandImports, ...pipeline.forbiddenOrchestration]) assert.equal(command.includes(forbidden), false, `Demonstrator hides ${forbidden}`);
  console.log(`[DEBUG] Demonstrator runtime catalog, full component union and ${inputs.length}-file pure import boundary PASS`);
}

/** 🕸️ Compares runtime selection with an independent directed-graph traversal. */
export async function testRuntimeComponents(workspace: string): Promise<void> {
  const require = createRequire(import.meta.url), fixtures = join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/runtime-components");
  const fixture = JSON.parse(readFileSync(join(fixtures, "🔣️.json"), "utf8"));
  assert.ok(new (require("ajv/dist/2020").default)().validate(JSON.parse(readFileSync(join(fixtures, "🛂️schema/🔣️.json"), "utf8")), fixture));
  const { cacheInternals } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
  assert.equal(typeof cacheInternals.runtimeComponentClosure, "function", "Runtime preparation needs transitive component consumption");
  const oracle = (rows: any[], roots: string[]) => {
    const { Graph, alg } = require("graphlib"), graph = new Graph({ directed: true });
    for (const row of rows) graph.setNode(row.pluginId);
    for (const row of rows) for (const dependency of new Set([...(row.dependsOn ?? []), ...rows.filter(candidate => row.host || (candidate.contributes ?? []).some((topic: string) => (row.consumes ?? []).includes(topic))).map(candidate => candidate.pluginId)])) graph.setEdge(row.pluginId, dependency);
    return [...new Set(roots.flatMap(root => alg.preorder(graph, root)))].sort();
  };
  for (const row of fixture.cases) {
    assert.deepEqual(oracle(fixture.components, row.roots), row.expected, row.name + ": graphlib");
    assert.deepEqual(cacheInternals.runtimeComponentClosure(fixture.components, row.roots), row.expected, row.name);
  }
  for (const row of fixture.invalid) assert.throws(() => cacheInternals.runtimeComponentClosure(row.components, row.roots), /component/i, row.name);
  const registry = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry", dev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript";
  const entries = JSON.parse(readFileSync(join(workspace, registry, "🤖️generated/🔌️plugins.json"), "utf8"));
  const paths = entries.map((row: any) => row.cratePath + "/Cargo.toml");
  const components = paths.map((path: string) => {
    const manifest = require("@iarna/toml").parse(readFileSync(join(workspace, path), "utf8")), metadata = manifest.package.metadata;
    const project = JSON.parse(readFileSync(join(workspace, dirname(path), "📋️project.json"), "utf8")).name;
    return { ...metadata.semio, project, pluginId: metadata.component.package.slice(6), dependsOn: [...(metadata.semio.extends ? [metadata.semio.extends] : []), ...(metadata.semio["depends-on"] ?? [])] };
  });
  const targets = cacheInternals.playgroundPreparationTargets(paths, workspace, dev), projects = new Map(components.map((row: any) => [row.pluginId, row.project]));
  let checked = 0;
  for (const component of components) for (const row of component.playground ?? []) for (const profile of ["dev", "release"]) {
    const expected = oracle(components, [component.pluginId]).map((id: string) => `${projects.get(id)}:materialize-${profile}`).sort();
    const actual = targets[`prepare-${row.variant}-react-${profile}`].dependsOn.filter((id: string) => id.endsWith(`:materialize-${profile}`)).sort();
    assert.deepEqual(actual, expected, `${row.variant} ${profile}: Cargo metadata and graphlib`);
    if (profile === "release") {
      const name = `build-${row.variant}-react-release`, build = targets[name];
      assert.equal(build?.cache, true, `${name}: production needs a cacheable owner`);
      assert.deepEqual(build.outputs, [`{projectRoot}/dist/${name}`]);
      assert.deepEqual(build.dependsOn, [...targets[`prepare-${row.variant}-react-release`].dependsOn, "@semio-tech/assets:build"]);
      assert.ok(!build.dependsOn.some((target: string) => /:?(?:activate|dev|serve)-/.test(target)));
      assert.ok(build.inputs.some((input: any) => input.dependentTasksOutputFiles === "**/*" && input.transitive));
    }
    checked++;
  }
  console.log("[DEBUG] Runtime component closure includes transitive contributions, unions and hosts and matches graphlib PASS");
  console.log(`[DEBUG] ${checked} real playground preparation targets match the independent Cargo metadata runtime closure PASS`);
}

/** 🗂️ Keeps producers inside the workspace supplied by Nx's own task environment. */
export function testWorkspaceRoots(workspace: string, output: string): void {
  const require = createRequire(import.meta.url), root = mkdtempSync(join(output, "workspace-roots-"));
  const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/🗂️workspace-roots.json"), "utf8"));
  const schema = { type: "object", required: ["schemaVersion", "cases"], properties: { schemaVersion: { const: 1 }, cases: { type: "array", minItems: 1, items: { type: "object", required: ["name", "nx", "hint", "expected"], properties: { name: { type: "string" }, nx: { type: "boolean" }, hint: { type: "boolean" }, expected: { enum: ["workspace", "hint"] } } } } } };
  assert.equal(require("jsonschema").validate(fixture, schema).valid, true);
  try {
    const supplied = require("nx/src/tasks-runner/task-env").getEnvVariablesForBatchProcess(false, true);
    assert.equal(supplied.NX_WORKSPACE_ROOT, workspace);
    const script = join(root, "📜️script.ts"), helper = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts");
    writeFileSync(script, `import { getWorkspaceRoot } from ${JSON.stringify(helper)}; process.stdout.write(getWorkspaceRoot());\n`);
    for (const row of fixture.cases) {
      const env = { ...supplied, NX_WORKSPACE_ROOT: row.nx ? supplied.NX_WORKSPACE_ROOT : undefined, REPO_ROOT: row.hint ? root : undefined };
      const child = Bun.spawnSync([process.execPath, script], { cwd: workspace, env, stdout: "pipe", stderr: "pipe" });
      assert.equal(child.exitCode, 0, child.stderr.toString());
      assert.equal(child.stdout.toString(), row.expected === "workspace" ? workspace : root, row.name);
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
  console.log("[DEBUG] Workspace resolution respects Nx's native task environment before standalone hints PASS");
}

/** 🧶️ Validates location-sensitive locked dependencies against Nx's native hash planner. */
export async function testBunDependencies(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url), fixtures = join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/bun-dependencies");
  const cases = JSON.parse(readFileSync(join(fixtures, "🔣️.json"), "utf8"));
  assert.ok(new (require("ajv/dist/2020").default)().validate(JSON.parse(readFileSync(join(fixtures, "🛂️schema/🔣️.json"), "utf8")), cases));
  const { cacheInternals } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
  assert.equal(typeof cacheInternals.bunLockGraph, "function", "Bun inputs require resolved-location dependency identity");
  const graph = cacheInternals.bunLockGraph(cases.lock, cases.patches);
  for (const row of cases.resolutions) assert.equal(graph.resolve(row.from, row.name), row.key);
  for (const row of cases.importers) assert.equal(graph.resolveImport(row.file, row.name), row.key);
  for (const name of cases.unresolvedImports) assert.equal(graph.resolveImport("domain/📜️script.ts", name), undefined);
  const native = cacheInternals.cargoTargets("🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust", workspace).build.inputs;
  assert.deepEqual(native.flatMap((input: any) => input.externalDependencies ?? []), cases.nativeInputs.externalDependencies);
  for (const path of cases.nativeInputs.excluded) assert.equal(native.includes(path), false);
  assert.deepEqual(native.find((input: any) => input.json === "{workspaceRoot}/package.json")?.fields, cases.nativeInputs.manifestFields);
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
  console.log("[DEBUG] Bun dependency locations, cycles, aliases, peers, optional platforms and scoped patch bytes match native Nx hash plans PASS");
}

/** 🧬️ Compares prerequisite closure with Cargo's independent package and dependency model. */
export async function testNativePreparation(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url), fixtures = join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/native-preparation");
  const cases = JSON.parse(readFileSync(join(fixtures, "🧫️cases.json"), "utf8"));
  assert.equal(require("jsonschema").validate(cases, JSON.parse(readFileSync(join(fixtures, "🛂️schema/🔣️.json"), "utf8"))).valid, true);
  const generatedFiles = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🗂️files/🟦️.ts")).href);
  assert.equal(typeof generatedFiles.writeGeneratedFileIfChanged, "function");
  const writeRoot = mkdtempSync(join(output, "generated-write-"));
  try {
    for (const row of cases.generatedWrites) {
      const target = join(writeRoot, `${row.name}.txt`);
      writeFileSync(target, row.initial);
      utimesSync(target, new Date(946684800000), new Date(946684800000));
      const before = lstatSync(target).mtimeMs;
      assert.equal(generatedFiles.writeGeneratedFileIfChanged(target, row.next), row.rewritten, row.name);
      assert.equal(readFileSync(target, "utf8"), row.next, row.name);
      assert.equal(lstatSync(target).mtimeMs === before, !row.rewritten, `${row.name}: mtime identity`);
    }
  } finally { rmSync(writeRoot, { recursive: true, force: true }); }
  const native = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts")).href);
  assert.equal(typeof native.validateNativeCargoArguments, "function");
  for (const row of cases.arguments) if (row.valid) assert.doesNotThrow(() => native.validateNativeCargoArguments(row.operation, row.args)); else assert.throws(() => native.validateNativeCargoArguments(row.operation, row.args), /input contract/);
  assert.equal(typeof native.artifactRustCargoArguments, "function");
  for (const row of cases.artifactArguments) if (row.valid) assert.deepEqual(native.artifactRustCargoArguments(row.operation, row.args).cargoArgs, row.cargoArgs, row.operation); else assert.throws(() => native.artifactRustCargoArguments(row.operation, row.args), /input contract/);
  assert.equal(typeof native.startNativeProgress, "function");
  const progress: string[] = [], stopProgress = native.startNativeProgress(cases.artifactProgress.label, cases.artifactProgress.intervalMs, (line: string) => progress.push(line));
  await Bun.sleep(cases.artifactProgress.intervalMs * 4);
  stopProgress();
  assert.equal(progress.some((line) => new RegExp(cases.artifactProgress.pattern).test(line)), true, "artifact build progress must remain visible while Cargo waits");
  const artifactRouterSource = readFileSync(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/📜️script.ts"), "utf8");
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
    const artifactRunner = await import("../../📦️artifacts/🦀️rust/📜️script.ts");
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
  const captureRoot = mkdtempSync(join(output, "artifact-capture-"));
  const capturePath = process.env.PATH;
  try {
    const packageRoot = join(captureRoot, "package"), dependencyRoot = join(captureRoot, "dependency"), binRoot = join(captureRoot, "bin");
    const dependency = join(captureRoot, "target/deps/libfixture_dependency.rlib"), primary = join(captureRoot, "target/libfixture_primary.rlib"), stub = join(binRoot, "📜️script.ts");
    mkdirSync(packageRoot, { recursive: true }); mkdirSync(dependencyRoot, { recursive: true }); mkdirSync(dirname(dependency), { recursive: true }); mkdirSync(binRoot, { recursive: true });
    writeFileSync(join(packageRoot, "Cargo.toml"), "[package]\nname='fixture-primary'\nversion='0.0.0'\n");
    writeFileSync(stub, `#!/usr/bin/env bun
import { rmSync, writeFileSync } from "node:fs";
const dependency = process.env.SEMIO_CAPTURE_DEPENDENCY!, primary = process.env.SEMIO_CAPTURE_PRIMARY!;
rmSync(dependency, { force: true });
if (process.env.SEMIO_CAPTURE_MISSING !== "1") writeFileSync(dependency, process.env.SEMIO_CAPTURE_DEPENDENCY_BYTES!);
writeFileSync(primary, process.env.SEMIO_CAPTURE_PRIMARY_BYTES!);
console.log(JSON.stringify({ reason: "compiler-artifact", package_id: "path+" + process.env.SEMIO_CAPTURE_DEPENDENCY_URL + "#fixture-dependency@0.0.0", target: { kind: ["lib"], name: "fixture_dependency" }, filenames: [dependency] }));
if (process.env.SEMIO_CAPTURE_MISSING === "1") await Bun.sleep(30000);
console.log(JSON.stringify({ reason: "compiler-artifact", package_id: "path+" + process.env.SEMIO_CAPTURE_PRIMARY_URL + "#fixture-primary@0.0.0", target: { kind: ["lib"], name: "fixture_primary" }, filenames: [primary] }));
await Bun.sleep(Number(process.env.SEMIO_CAPTURE_DELAY_MS));
rmSync(dependency);
writeFileSync(dependency, process.env.SEMIO_CAPTURE_REPLACEMENT_BYTES!);
`);
    chmodSync(stub, 0o755);
    if (process.platform === "win32") writeFileSync(join(binRoot, "cargo.cmd"), `@bun "%~dp0📜️script.ts" %*\r\n`);
    else symlinkSync("📜️script.ts", join(binRoot, "cargo"));
    process.env.PATH = binRoot + delimiter + (capturePath ?? "");
    process.env.SEMIO_CAPTURE_DEPENDENCY = dependency;
    process.env.SEMIO_CAPTURE_PRIMARY = primary;
    process.env.SEMIO_CAPTURE_DEPENDENCY_URL = pathToFileURL(dependencyRoot).href;
    process.env.SEMIO_CAPTURE_PRIMARY_URL = pathToFileURL(packageRoot).href;
    process.env.SEMIO_CAPTURE_DEPENDENCY_BYTES = cases.artifactCapture.dependency;
    process.env.SEMIO_CAPTURE_PRIMARY_BYTES = cases.artifactCapture.primary;
    process.env.SEMIO_CAPTURE_REPLACEMENT_BYTES = cases.artifactCapture.replacement;
    process.env.SEMIO_CAPTURE_DELAY_MS = String(cases.artifactCapture.delayMs);
    await native.buildCargoArtifacts("package/Cargo.toml", [], captureRoot);
    assert.equal(readFileSync(join(packageRoot, "dist/build/deps/libfixture_dependency.rlib"), "utf8"), cases.artifactCapture.dependency, "Cargo dependency must be captured before a successor can replace its shared output");
    assert.equal(readFileSync(join(packageRoot, "dist/build/libfixture_primary.rlib"), "utf8"), cases.artifactCapture.primary, "Cargo primary output must be captured from the same compiler event epoch");
    process.env.SEMIO_CAPTURE_MISSING = "1";
    const failureStarted = Date.now();
    await assert.rejects(native.buildCargoArtifacts("package/Cargo.toml", [], captureRoot), /ENOENT/);
    assert.ok(Date.now() - failureStarted < 5000, "A capture failure must cancel and await Cargo without leaving the build alive");
    assert.equal(readFileSync(join(packageRoot, "dist/build/deps/libfixture_dependency.rlib"), "utf8"), cases.artifactCapture.dependency, "A capture failure must preserve the previous dependency output");
    assert.equal(readFileSync(join(packageRoot, "dist/build/libfixture_primary.rlib"), "utf8"), cases.artifactCapture.primary, "A capture failure must preserve the previous primary output");
    assert.equal(readdirSync(join(packageRoot, "dist")).some((name) => name.startsWith("cargo-artifacts-")), false, "A capture failure must remove its private directory");
  } finally {
    process.env.PATH = capturePath;
    for (const name of ["SEMIO_CAPTURE_DEPENDENCY", "SEMIO_CAPTURE_PRIMARY", "SEMIO_CAPTURE_DEPENDENCY_URL", "SEMIO_CAPTURE_PRIMARY_URL", "SEMIO_CAPTURE_DEPENDENCY_BYTES", "SEMIO_CAPTURE_PRIMARY_BYTES", "SEMIO_CAPTURE_REPLACEMENT_BYTES", "SEMIO_CAPTURE_DELAY_MS", "SEMIO_CAPTURE_MISSING"]) delete process.env[name];
    rmSync(captureRoot, { recursive: true, force: true });
  }
  const root = mkdtempSync(join(output, "native-preparation-"));
  try {
    for (const [path, content] of Object.entries(cases.files)) { const file = join(root, path); mkdirSync(dirname(file), { recursive: true }); writeFileSync(file, String(content)); }
    const { cacheInternals } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
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
    const declared = cacheInternals.projectInputs({ name: "fixture", namedInputs: { nativeSources: ["{workspaceRoot}/external.bin"] }, targets: {} }, "app", root, new Map());
    assert.ok(declared.nativeTestSources.includes("{workspaceRoot}/external.bin"), "tests inherit declared production native inputs");
    console.log("[DEBUG] Native preparation production/test dependency closure matches Cargo metadata and declared inputs propagate to tests PASS");
  } finally { rmSync(root, { recursive: true, force: true }); }
}

/** 🪵️ Exercises Nx's installed error formatter against a bounded diagnostic contract. */
export function testNxDaemonDiagnostics(workspace: string, output: string): void {
  const require = createRequire(join(workspace, "package.json")), root = mkdtempSync(join(output, "daemon-tail-"));
  const vector = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/nx-contract/🔣️.json"), "utf8")).daemonLog;
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
    console.log("[DEBUG] Installed Nx daemon diagnostics preserve the tail using bounded reads PASS");
  } finally { rmSync(root, { recursive: true, force: true }); }
}

/** 🧭️ Keeps task-scoped reporting and selection out of daemon graph identity while preserving caller values. */
export function testNxDaemonTaskEnvironment(workspace: string): void {
  const require = createRequire(join(workspace, "package.json"));
  const vector = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/nx-contract/🔣️.json"), "utf8")).daemonTaskEnvironment;
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
  console.log("[DEBUG] Nx daemon graph identity ignores task-scoped environment and preserves real graph controls PASS");
}

/** 📓️ Checks installed Nx retention against native append descriptors and unowned path boundaries. */
export function testNxDaemonRetention(workspace: string, output: string): void {
  const require = createRequire(join(workspace, "package.json")), directory = mkdtempSync(join(output, "daemon-retention-"));
  const original = require.resolve("nx/src/daemon/logger.js"), source = original;
  const vector = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/📓️daemon-retention.json"), "utf8"));
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
  console.log("[DEBUG] Nx daemon retention: repeated native writes are bounded, tail preserved and existing append descriptor stays live PASS");
`;
  try {
    const child = Bun.spawnSync(["node", "--eval", code], { cwd: workspace, env: { ...process.env, NX_WORKSPACE_DATA_DIRECTORY: join(directory, "workspace-data") }, stdout: "pipe", stderr: "pipe", timeout: 10000 });
    assert.equal(child.exitCode, 0, child.stdout.toString() + child.stderr.toString());
    process.stdout.write(child.stdout);
  } finally { rmSync(directory, { recursive: true, force: true }); }
}

export function createCachePolicyTests(dependencies: Record<string, any>, testSource: { directory: string; url: string }) {
  const { assert, cacheInternals, chmodSync, copyFileSync, createRequire, devToolingEnv, dirname, EventEmitter, existsSync, getWorkspaceRoot, inventory, join, lstatSync, mkdirSync, mkdtempSync, plugin, readFileSync, relative, resolve, rmSync, SCRIPT_ROOT, slash, spawn, stageArtifacts, ticketOutput, utimesSync, wasmBindgenVersion, wasmBuildArguments, wasmBuildEnvironment, writeFileSync } = dependencies;

  /** 🧪️ Executes language-neutral policy examples against the Nx project plugin. */
  async function testCacheContracts(): Promise<void> {
    const { validate } = createRequire(testSource.url)("jsonschema");
    const policy = JSON.parse(readFileSync(join(SCRIPT_ROOT, "🔣️policy.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(SCRIPT_ROOT, "🧬️schema/🔣️.json"), "utf8"));
    assert.equal(validate(policy, schema).valid, true);
    assert.ok(cacheInternals, "cache policy must be exposed for contract verification");
    const authoredWorkspace = JSON.parse(readFileSync(join(getWorkspaceRoot(), "📋️project.json"), "utf8"));
    for (const name of ["test", "lint", "build", "generate", "verify", "test-exhaustive"]) assert.equal(authoredWorkspace.targets[name].cache, true, name);
    for (const name of ["format", "setup", "publish", "dev"]) assert.equal(authoredWorkspace.targets[name].cache, false, name);
    const root = getWorkspaceRoot();
    assert.equal(Object.keys(JSON.parse(readFileSync(join(root, "nx.json"), "utf8")).targetDefaults ?? {}).length, 0, "Native Nx defaults override custom project metadata; apply defaults inside the repository plugin");
    assert.equal(wasmBuildEnvironment(root, {}).CARGO_TARGET_DIR, join(root, ".🧬semio/🦑️repo/⚡️cache/cargo/browser"));
    assert.equal(wasmBuildEnvironment(root, { CARGO_TARGET_DIR: "chosen-cache" }).CARGO_TARGET_DIR, join(root, "chosen-cache"));
    const vectors = JSON.parse(readFileSync(join(SCRIPT_ROOT, "🧫️fixtures/nx-contract/🔣️.json"), "utf8"));
    assert.equal(validate(vectors, JSON.parse(readFileSync(join(SCRIPT_ROOT, "🧫️fixtures/nx-contract/🛂️schema/🔣️.json"), "utf8"))).valid, true);
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
    assert.equal(cacheInternals.matchesUncached("format-check", policy.uncached), false);
    assert.equal(cacheInternals.matchesUncached("format", policy.uncached), true);
    assert.equal(cacheInternals.cacheableFamily("format-check"), true);
    assert.equal(cacheInternals.cacheableFamily("generator-inputs"), false);
    for (const row of vectors.policies) {
      const enabled = cacheInternals.targetPolicy(row.target, { cache: true, options: { command: `bun ./📜️script.ts ${row.target}` } }, policy);
      const disabled = cacheInternals.targetPolicy(row.target, { cache: false, options: { command: `bun ./📜️script.ts ${row.target}` } }, policy);
      assert.deepEqual({ cache: enabled.cache, continuous: enabled.continuous ?? false }, { cache: row.cache, continuous: row.continuous }, row.target);
      assert.equal(disabled.cache, row.cache, `${row.target} must not keep authored cache when policy is authoritative`);
      assert.equal(isCacheableTask({ cache: enabled.cache, continuous: enabled.continuous === true, target: { project: "probe", target: row.target }, overrides: {} }), row.cache, `${row.target}: nx isCacheableTask`);
    }
    const selectionApi = await import("../../../🎮️playground/🟦️.ts");
    assert.equal(typeof selectionApi.loadFrameworkOsPlaygroundSelections, "function", "Development selection must read authored metadata before generation");
    const selectionFixture = mkdtempSync(join(ticketOutput(root, []), "playground-selection-"));
    try {
      const vector = vectors.playgroundSelections, manifest = join(selectionFixture, vector.manifest);
      mkdirSync(dirname(manifest), { recursive: true });
      writeFileSync(manifest, vector.text);
      const originalTime = lstatSync(manifest).mtime;
      for (const port of vector.ports) {
        const text = vector.text.replace(String(vector.ports[0]), String(port));
        writeFileSync(manifest, text);
        utimesSync(manifest, originalTime, originalTime);
        const authored = createRequire(testSource.url)("@iarna/toml").parse(text).package.metadata;
        const rows = selectionApi.loadFrameworkOsPlaygroundSelections(selectionFixture, [vector.manifest]);
        assert.deepEqual(rows, [{ ...authored.semio.playground[0], pluginId: authored.component.package.slice(6), cratePath: slash(dirname(vector.manifest)) }]);
        assert.equal(rows[0].variant, vector.variant);
        assert.deepEqual(rows[0].aliases, [vector.alias]);
        assert.equal(rows[0].ports.react, port);
      }
      const stale = join(selectionFixture, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json");
      mkdirSync(dirname(stale), { recursive: true });
      writeFileSync(stale, "invalid generated catalog");
      assert.equal(selectionApi.loadFrameworkOsPlaygroundSelections(selectionFixture, [vector.manifest])[0].variant, vector.variant);
      assert.throws(() => selectionApi.loadFrameworkOsPlaygroundSelections(selectionFixture, [vector.manifest, vector.manifest]), /duplicate/i);
      writeFileSync(manifest, vector.text.replace(String(vector.ports[0]), "65536"));
      assert.throws(() => selectionApi.loadFrameworkOsPlaygroundSelections(selectionFixture, [vector.manifest]), /port/i);
      assert.throws(() => selectionApi.loadFrameworkOsPlaygroundSelections(selectionFixture, ["../Cargo.toml"]), /outside/i);
    } finally { rmSync(selectionFixture, { recursive: true }); }
    await testCommandInputs(root, ticketOutput(root, []));
    await (await import("../🚀️bootstrap/🟦️.ts")).testNxBootstrap(root, ticketOutput(root, []));
    await (await import("../📇️artifacts/🟦️.ts")).testArtifactRegistry(root, ticketOutput(root, []));
    await (await import("../🌎️hub/🟦️.ts")).testHubBuild(root);
    await testDependencyBootstrap(root, ticketOutput(root, []));
    await testNxTooling(root, ticketOutput(root, []));
    await testDependencyCancellation(root, ticketOutput(root, []));
    console.log("[DEBUG] Native command contracts passed; collecting project inventory");
    const result = await (await import("../../📇️inventory/🧪️tests/🕸️coverage/🟦️.ts")).testNativeInventory(root, inventory), contracts = result.projects;
    console.log(`[DEBUG] Project inventory collected: ${contracts.length} projects`);
    const toml = createRequire(testSource.url)("@iarna/toml");
    let componentPackages = 0;
    const componentLaunchers: { project: string; pluginId: string }[] = [];
    for (const project of contracts) {
      const path = join(root, project.root, "Cargo.toml");
      if (!existsSync(path)) continue;
      const manifest = toml.parse(readFileSync(path, "utf8"));
      if (!manifest.package?.metadata?.component?.package || !["plugin", "extension"].includes(manifest.package?.metadata?.semio?.role)) continue;
      componentPackages++;
      componentLaunchers.push({ project: project.name, pluginId: manifest.package.metadata.component.package.slice("semio:".length) });
      const moduleCatalog = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json"), "utf8"));
      const moduleDirectory = moduleCatalog.modules.find((row: any) => row.pluginId === manifest.package.metadata.component.package.slice("semio:".length))?.directoryName;
      assert.ok(moduleDirectory);
      for (const row of vectors.materialization.profiles) {
        const target = project.targets[row.target], shared = contracts.find((project) => project.name === vectors.materialization.project)?.targets[row.support];
        assert.equal(target?.cache, true, `${project.name}:${row.target} needs a materialization producer`);
        assert.deepEqual(target.dependsOn, [row.component, `${vectors.materialization.project}:${row.support}`, ...row.tooling]);
        assert.deepEqual(target.outputs, [`{workspaceRoot}/${vectors.materialization.root}/dist/${row.profile}/🔌️plugin-modules/${moduleDirectory}`]);
        assert.ok(target.inputs.some((input: any) => input.dependentTasksOutputFiles === "**/*"));
        assert.ok(target.options.command.includes(`materialize ${row.profile} --manifest`));
        assert.equal(shared?.cache, true);
        assert.equal(shared.outputs.length, 2);
      }
      for (const row of vectors.componentProfiles) {
        const target = project.targets[row.target];
        assert.equal(target?.cache, true, `${project.name}:${row.target} needs a component producer`);
        assert.deepEqual(target.outputs, [row.output]);
        assert.equal(target.parallelism, false);
        assert.ok(target.options.command.includes(`native component ${row.profile} --manifest`));
      }
    }
    assert.ok(componentPackages > 0, "Component metadata must discover plugin producers");
    const fontProject = contracts.find((project) => project.name === vectors.fontAssets.project)!;
    for (const [name, output] of [[vectors.fontAssets.tool, vectors.fontAssets.toolOutput], [vectors.fontAssets.producer, vectors.fontAssets.output]]) {
      assert.equal(fontProject.targets[name]?.cache, true, `${name} must be separately cacheable`);
      assert.deepEqual(fontProject.targets[name].outputs, [output]);
    }
    assert.deepEqual(fontProject.targets[vectors.fontAssets.producer].dependsOn, [vectors.fontAssets.tool]);
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
      const completed = cycle.plugins.map(([pluginId, digest]: string[]) => ({ pluginId, artifactSha256: digest.repeat(64) }));
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
    assert.throws(() => activation.nextActivationReceipt("other", "dev", [], activationReceipt, 500), /identity/);
    assert.throws(() => activation.nextActivationReceipt("../note", "dev", [], undefined, 500), /Invalid/);
    const sessionProject = contracts.find((project) => project.name === vectors.playgroundSessions.project)!;
    for (const variant of vectors.playgroundSessions.variants) {
      const session = sessionProject.targets[`session-${variant}`];
      assert.equal(session?.cache, true, `${variant} needs its own cacheable playground session`);
      assert.deepEqual(session.dependsOn, [vectors.playgroundSessions.prerequisite]);
      assert.deepEqual(session.outputs, [`{projectRoot}/dist/sessions/${variant}`]);
      assert.ok(session.inputs.some((input: any) => input.dependentTasksOutputFiles === "**/*"));
    }
    console.log("[DEBUG] Component and activation contracts passed; checking editor and playground contracts");
    const registryRoot = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry");
    const registry = await import(join(registryRoot, "📜️script.ts"));
    const launch = await import(join(registryRoot, "🖥️launch.ts"));
    const configurations = Bun.JSONC.parse(launch.generateLaunchJson(root, registry.generatePlaygroundRegistry(root), componentLaunchers)).configurations;
    for (const project of contracts) for (const [name, target] of Object.entries(project.targets) as [string, any][]) if (target.options?.command?.includes("⚡️caching/🦀️cargo/📜️script.ts") && ["build", "check", "test"].includes(name)) assert.ok(configurations.some((row: any) => row.command === `bun nx run ${project.name}:${name}`), `Missing native editor command ${project.name}:${name}`);
    const preparationProject = contracts.find((project) => project.name === vectors.playgroundPreparation.project)!;
    for (const playground of registry.generatePlaygroundRegistry(root)) {
      for (const engine of playground.engines) assert.ok(contracts.some((project) => resolve(root, project.root) === resolve(root, engine) && project.targets.wasm), `Engine ${engine} must name an authored producer`);
      for (const profile of vectors.playgroundPreparation.profiles) {
        const targetName = `prepare-${playground.variant}-react-${profile}`, preparation = preparationProject.targets[targetName];
        assert.ok(preparation, `${targetName} needs declared prerequisites`);
        assert.equal(preparation.cache, false);
        assert.deepEqual(preparation.outputs, []);
        const activationName = `activate-${playground.variant}-react-${profile}`, activationTarget = preparationProject.targets[activationName];
        assert.ok(activationTarget, `${activationName} must follow completed preparation`);
        assert.equal(activationTarget.cache, false);
        assert.deepEqual(activationTarget.outputs, []);
        assert.deepEqual(activationTarget.dependsOn, [targetName]);
        assert.equal(configurations.filter((configuration: any) => configuration.command === `bun nx run ${preparationProject.name}:${activationName}`).length, 1);
        for (const command of ["serve", "dev"]) {
          const serverName = `${command}-${playground.variant}-react-${profile}`, server = preparationProject.targets[serverName];
          assert.ok(server, `${serverName} needs an Nx server owner`);
          assert.equal(server.continuous, true);
          assert.equal(server.cache, false);
          assert.deepEqual(server.outputs, []);
          assert.deepEqual(server.dependsOn, [activationName]);
          assert.equal(server.options.command, `bun ./📜️script.ts serve ${playground.variant} react ${profile}`);
          assert.equal(configurations.filter((configuration: any) => configuration.command === `bun nx run ${preparationProject.name}:${serverName}`).length, 1);
        }
        const components = registry.buildPlaygroundSession(playground.variant).plugins.map((row: any) => `${componentLaunchers.find((entry) => entry.pluginId === row.pluginId)!.project}:materialize-${profile}`);
        assert.deepEqual([...preparation.dependsOn].sort(), [...new Set([`@semio-tech/plugin-registry:session-${playground.variant}`, `@semio-tech/framework-plugin-web:support-${profile}`, vectors.playgroundPreparation.fonts, ...vectors.playgroundPreparation.engines, ...components])].sort());
        assert.equal(configurations.filter((configuration: any) => configuration.command === `bun nx run ${preparationProject.name}:${targetName}`).length, 1);
      }
      const target = `session-${playground.variant}`;
      assert.ok(sessionProject.targets[target]);
      assert.equal(configurations.filter((configuration: any) => configuration.command === `bun nx run ${sessionProject.name}:${target}`).length, 1);
    }
    for (const entry of componentLaunchers) for (const row of [...vectors.componentProfiles, ...vectors.materialization.profiles]) assert.equal(configurations.filter((configuration: any) => configuration.command === `bun nx run ${entry.project}:${row.target}`).length, 1, `${entry.project}:${row.target} must have one editor command`);
    assert.deepEqual(result.violations.filter((finding) => finding.rule === "ORCH-01"), [], "All public commands must use one script entrypoint");
    for (const entry of vectors.entrypoints) assert.ok(contracts.find((project) => project.name === entry.project)?.targets[entry.target].dependsOn?.includes(entry.prerequisite), `${entry.project}:${entry.target} needs ${entry.prerequisite} in Nx`);
    for (const entry of vectors.nativeBinaries) {
      const project = contracts.find((project) => project.name === entry.project)!;
      assert.equal(project.targets[entry.target].cache, true, `${entry.project} binary must be restorable`);
      assert.deepEqual(project.targets[entry.target].outputs, [entry.output]);
      assert.ok(project.targets[entry.consumer].dependsOn.includes(entry.target));
    }
    for (const entry of vectors.components) {
      const project = contracts.find((project) => project.name === entry.project)!;
      const target = project.targets[entry.target];
      assert.ok(project.namedInputs?.default?.includes(entry.schemaInput), `${entry.project} must hash its shared component schema`);
      assert.equal(target.cache, true, `${entry.project}:${entry.target} component must be restorable`);
      assert.deepEqual(target.outputs, [entry.output]);
      assert.ok(contracts.find((project) => project.name === entry.consumerProject)!.targets[entry.consumerTarget].dependsOn.includes(`${entry.project}:${entry.target}`));
    }
    console.log("[DEBUG] Editor and playground contracts passed; checking lifecycle and compiler contracts");
    const mcpRoot = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp";
    const mcp = await import(join(root, mcpRoot, "🟦️.ts"));
    const binaryVectors = JSON.parse(readFileSync(join(root, mcpRoot, "🧫️fixtures/🧱️binary-gate.json"), "utf8"));
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
      assert.deepEqual(target.outputs, authority.outputRoots.map((output: any) => `{workspaceRoot}/${output.path}`), id);
      for (const path of authority.inputPatterns) assert.ok(target.inputs.includes(`{workspaceRoot}/${path}`), `${id} missing ${path}`);
      if (authority.inputDiscovery) {
        const fingerprint = policy.generatorInputs[authority.inputDiscovery.kind], separator = fingerprint.target.lastIndexOf(":"), owner = contracts.find((project) => project.name === fingerprint.target.slice(0, separator));
        const guard = owner.targets[fingerprint.target.slice(separator + 1)];
        assert.ok(target.inputs.some((input: any) => input.dependentTasksOutputFiles === fingerprint.output), `${id} must hash the discovered input receipt`);
        assert.ok(!target.inputs.some((input: any) => input.runtime?.includes("generator-inputs")), `${id} must not repeat discovery inside the hasher`);
        assert.ok(target.dependsOn.includes(fingerprint.target)); assert.equal(guard.cache, false);
        assert.deepEqual(guard.outputs, [`{workspaceRoot}/${fingerprint.output}`]);
      }
      if (authority.checkTarget) assert.equal(project.targets[authority.checkTarget.slice(authority.checkTarget.lastIndexOf(":") + 1)].cache, false, `${id} freshness checks must inspect current bytes`);
    }
    const [validatedProject, validatedTarget] = vectors.validationPrerequisite.target.split(":");
    assert.ok(contracts.find((project) => project.name === validatedProject)?.targets[validatedTarget].dependsOn.includes(vectors.validationPrerequisite.prerequisite));
    assert.ok(!contracts.find((project) => project.name === validatedProject)?.targets[validatedTarget].inputs.includes("^default"), "Exact catalog discovery must not reintroduce generated outputs through broad dependency defaults");
    const workspace = contracts.find((project) => project.name === "workspace")!;
    assert.deepEqual(workspace.targets.setup.dependsOn, vectors.lifecycle.setupDependencies);
    assert.deepEqual(workspace.targets.prepare.dependsOn, vectors.lifecycle.prepareDependencies);
    for (const target of vectors.lifecycle.setupDependencies) assert.equal(workspace.targets[target]?.cache, false);
    for (const name of ["test", "lint", "build", "generate", "verify", "test-exhaustive"]) assert.equal(workspace.targets[name]?.cache, true, name);
    assert.equal(workspace.targets.dev.cache, false);
    assert.equal(workspace.targets.dev.continuous, true);
    assert.equal(cacheInternals.nativeTargetCommandInputs({ options: { command: "bun ./x.ts", cwd: "." } }, root).some((input) => input.env && input.env.startsWith("SEMIO_")), true, "native leaves must fingerprint SEMIO toolchain env");
    const hostProject = contracts.find((project) => project.name === vectors.platformEnvironment.project)!;
    for (const target of Object.values(hostProject.targets) as any[]) for (const key of vectors.platformEnvironment.keys) assert.equal(target.options?.env?.[key], undefined, `Shared target metadata cannot force ${key}`);
    const bootstrap = vectors.bootstrap, dotnet = contracts.find((project) => project.name === bootstrap.dotnetProject);
    assert.ok(dotnet, "The current .NET support library needs an Nx owner");
    assert.deepEqual(dotnet.targets.build.outputs, ["{projectRoot}/dist/build"]);
    assert.ok(dotnet.targets.build.dependsOn.includes("deps"));
    assert.equal(dotnet.targets.deps.cache, false);
    assert.ok(workspace.targets["deps-dotnet"].dependsOn.includes(`${bootstrap.dotnetProject}:deps`));
    assert.ok(readFileSync(join(root, "Monorepo.sln"), "utf8").includes(bootstrap.dotnetPath.replaceAll("/", "\\")));
    assert.ok(readFileSync(join(root, bootstrap.dotnetPath, "🧪️Semio.Repo.Test.csproj"), "utf8").includes(`Include="${bootstrap.compile}"`));
    const python = createRequire(testSource.url)("@iarna/toml").parse(readFileSync(join(root, "pyproject.toml"), "utf8"));
    assert.deepEqual(python.tool.uv.workspace.members, bootstrap.pythonMembers);
    assert.deepEqual(workspace.targets["deps-python"].dependsOn, bootstrap.pythonDependencies);
    const styling = contracts.find((project) => project.name === bootstrap.stylingProject)!;
    assert.ok(styling.targets.generate.dependsOn.includes(bootstrap.stylingGenerator));
    assert.deepEqual(styling.targets.generate.outputs, []);
    for (const target of Object.values(styling.targets) as any[]) if (target.options.command.includes(" test")) assert.ok(target.dependsOn.includes(bootstrap.stylingGenerator));
    assert.ok(!/compose|topologic|vcpkg/i.test(readFileSync(join(root, "CMakeLists.txt"), "utf8") + readFileSync(join(root, "CMakePresets.json"), "utf8")));
    const ts = createRequire(testSource.url)("typescript");
    for (const row of vectors.engineOutputs) {
      const project = contracts.find((project) => project.name === row.project)!;
      assert.deepEqual(project.targets.wasm.outputs, row.outputs);
      const engineSource = ts.createSourceFile("engine.ts", readFileSync(join(root, project.root, "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
      const outputs: string[] = [];
      const inspect = (node: any): void => {
        if (ts.isCallExpression(node) && node.expression.getText(engineSource) === "runWasmPackWebBuild") {
          const output = node.arguments[0].properties.find((property: any) => property.name?.getText(engineSource) === "outputDirectory");
          outputs.push(output?.initializer.text ?? "pkg");
        }
        ts.forEachChild(node, inspect);
      };
      inspect(engineSource);
      assert.deepEqual(outputs, [row.directory]);
    }
    const source = ts.createSourceFile("📜️script.ts", readFileSync(join(root, "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
    const nxSource = ts.createSourceFile("nx.ts", readFileSync(join(SCRIPT_ROOT, "🚀️bootstrap/📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
    const coordinator = nxSource.statements.find((node: any) => ts.isClassDeclaration(node) && node.name?.text === "NxScript");
    const coordinatorCode = ts.transpileModule(coordinator.getText(nxSource).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
    const coordinatorFixture = JSON.parse(readFileSync(join(SCRIPT_ROOT, "🧫️fixtures/🛑️cancellation.json"), "utf8"));
    assert.equal(validate(coordinatorFixture, schema.$defs.NxCoordinatorFixture).valid, true);
    for (const vector of coordinatorFixture.cases) {
      const killed: number[] = [], child = Object.assign(new EventEmitter(), { pid: 1234 });
      const runtime = Object.assign(new EventEmitter(), { env: vector.environment, platform: vector.platform, exitCode: 0, kill: (pid: number, signal: number | string) => { if (!signal) throw new Error("No process"); killed.push(pid); } });
      let launchEnvironment: Record<string, string | undefined> = {};
      const Coordinator = new Function("Script", "process", "createRequire", "join", "existsSync", "resolveNxInvocation", "devToolingEnv", "orchestratorBudgetOpts", "spawnNxProcess", "stopNxProcessTree", coordinatorCode + "; return NxScript;")(
        class { root = root; }, runtime, () => ({ resolve: (name: string) => name }), join, (path: string) => path.endsWith("node_modules/nx/package.json"), (args: string[]) => ({ args, env: {} }), (env: unknown) => env, () => ({}), (_command: string, _args: string[], options: { env: Record<string, string | undefined> }) => { launchEnvironment = options.env; return child; },
        (command: string) => { if (command === "taskkill") { killed.push(child.pid); return { status: 0 }; } if (vector.throws) throw new Error("Snapshot unavailable"); return { status: 0, stdout: vector.stdout }; });
      const done = new Coordinator().run(["run", "fixture:build"]);
      assert.doesNotThrow(() => runtime.emit("SIGTERM"), vector.name);
      child.emit("close", null);
      await done;
      assert.equal(launchEnvironment.NX_WORKSPACE_DATA_DIRECTORY, vector.environment.NX_WORKSPACE_DATA_DIRECTORY ?? pathOracle.join(root, ".nx", "workspace-data"), vector.name);
      assert.equal(runtime.exitCode, vector.expectedExit, vector.name);
      assert.ok(killed.length > 0, `${vector.name}: owned launch process must still be stopped`);
      assert.equal(runtime.listenerCount("SIGTERM"), 0);
    }
    console.log("[DEBUG] Nx coordinator preserves explicit workspace data paths and stops owned launch processes after malformed or unavailable snapshots PASS");
    const invocation = nxSource.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "resolveNxInvocation");
    const route = ts.transpileModule(invocation.getText(nxSource).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
    const resolveInvocation = new Function("process", `${route}; return resolveNxInvocation;`)({ env: {} });
    const resolvePrint = new Function("process", "readFileSync", "join", "WORKSPACE_ROOT", `${route}; return resolveNxInvocation;`)({ env: {} }, readFileSync, join, root);
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
      const selected = `@semio-tech/framework-os-dev:${command}-note-react-dev`;
      const invocation = resolveInvocation(["run", selected]);
      assert.equal(invocation.watch, command === "dev" ? "@semio-tech/framework-os-dev:activate-note-react-dev" : undefined);
      assert.equal(invocation.env.SEMIO_BUILD_MODE, "dev");
      assert.equal(resolveInvocation(["run", selected, "--graph=stdout"]).watch, undefined);
    }
    assert.deepEqual(resolveInvocation(["run", "workspace:dev", "--", "mcp", "stdio", "os", "--folder", "chosen"]).args, ["run", "@semio-tech/framework-os-mcp-rs:dev", "--", "stdio", "--folder", "chosen"]);
    assert.deepEqual(resolveInvocation(["run", "workspace:dev", "--", "mcp", "http", "os"]).args, ["run", "@semio-tech/framework-os-mcp-rs:dev", "--", "http", "--port", "6300"]);
    for (const renderer of vectors.benchmarkRenderers) {
      const target = `bench-plugins-${renderer}`;
      const args = resolveInvocation(["run", "workspace:bench", "--", "plugins", `--renderer=${renderer}`, "--count", "3"]).args;
      assert.deepEqual(args, ["run", `@semio-tech/framework-os-dev:${target}`, "--", "--count", "3"]);
      assert.equal(hostProject.targets[target].dependsOn.includes("@semio-tech/framework-os-scale-fixture:build-wasm"), renderer === "native");
      assert.equal(hostProject.targets[target].dependsOn.includes("@semio-tech/framework-renderer-wgpu:native-build"), renderer === "native");
    }
    assert.deepEqual(resolveInvocation(["run", "@semio-tech/framework-renderer-wgpu:native", "--", "s", "--release"]).args, ["run", "@semio-tech/framework-renderer-wgpu:native-release", "--", "s"]);
    const hostSource = ts.createSourceFile("host.ts", readFileSync(join(root, hostProject.root, "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
    const apple = hostSource.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "ensureAppleDeveloperDir");
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
      assert.ok(project?.targets.wasm.outputs?.includes(row.output), `${row.project}:wasm must own ${row.output}`);
      assert.notEqual(project?.targets.wasm.cache, false, `${row.project}:wasm must be cacheable`);
      assert.ok(project?.namedInputs?.default.some((input: any) => input.env === "RUSTFLAGS"), `${row.project} must preserve toolchain inputs`);
      if (row.output.startsWith("{projectRoot}/") && !row.output.includes("../")) assert.ok(project?.namedInputs?.default.includes(`!${row.output}/**/*`), `${row.project} must exclude its output in the project fileset`);
    }
    const ticket = process.env.SEMIO_TICKET_DIR;
    assert.ok(ticket, "SEMIO_TICKET_DIR must point to the active ticket for generated test files");
    console.log("[DEBUG] Lifecycle and compiler contracts passed; checking source discovery and cancellation");
    const fixture = join(resolve(root, ticket), "🗑️generated", "policy-contract");
    const rustFixture = join(fixture, "rust-inputs");
    for (const [path, contents] of Object.entries(vectors.rust.files)) { mkdirSync(dirname(join(rustFixture, path)), { recursive: true }); writeFileSync(join(rustFixture, path), contents as string); }
    const rustInputs = cacheInternals.rustSourceFiles([join(rustFixture, vectors.rust.entry)], rustFixture).map((path: string) => slash(relative(rustFixture, path))).sort();
    assert.deepEqual(rustInputs, vectors.rust.inputs);
    const rustOracle = Bun.spawnSync(["rustc", "--crate-type=lib", "--emit=dep-info", "-o", join(rustFixture, "oracle.d"), vectors.rust.entry], { cwd: rustFixture, stdout: "pipe", stderr: "pipe" });
    assert.equal(rustOracle.exitCode, 0, rustOracle.stderr.toString());
    const rustDependencies = readFileSync(join(rustFixture, "oracle.d"), "utf8").split("\n")[0]!.split(": ")[1]!.trim().split(/\s+/).map((path) => slash(relative(rustFixture, resolve(rustFixture, path)))).sort();
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
      const dependencies = readFileSync(join(revisionRoot, "oracle.d"), "utf8").split("\n")[0]!.split(": ")[1]!.trim().split(/\s+/).map((path) => slash(relative(revisionRoot, resolve(revisionRoot, path)))).sort();
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
    const materializerSource = ts.createSourceFile("materializer.ts", readFileSync(join(root, vectors.materialization.root, "🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);
    const spawnDeclaration = materializerSource.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "spawnAsync");
    const spawnCode = ts.transpileModule(spawnDeclaration.getText(materializerSource), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
    const materializerSpawn = new Function("spawn", `${spawnCode}; return spawnAsync;`)(spawn);
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
    writeFileSync(join(fixture, "📋️project.json"), JSON.stringify({ name: graph.workspaceRoot.name, targets: Object.fromEntries(graph.workspaceRoot.targets.map((name) => [name, { options: { command: `bun ./📜️script.ts ${name}` } }])) }));
    const goManifest = vectors.go.manifest;
    writeFileSync(join(fixture, "go.mod"), goManifest);
    const goOracle = Bun.spawnSync(["go", "mod", "edit", "-json", `-modfile=${join(fixture, "go.mod")}`], { cwd: fixture, env: { ...process.env, GOWORK: "off" }, stdout: "pipe", stderr: "pipe" });
    assert.equal(goOracle.exitCode, 0, goOracle.stderr.toString());
    const go = JSON.parse(goOracle.stdout.toString());
    const { manifest: _manifest, ...expectedGo } = vectors.go;
    assert.deepEqual(cacheInternals.goDependencies(goManifest), expectedGo);
    assert.deepEqual(cacheInternals.goDependencies(goManifest), { module: go.Module.Path, requires: go.Require.map((row: any) => row.Path), replacements: go.Replace.map((row: any) => ({ module: row.Old.Path, path: row.New.Path })) });
    assert.throws(() => plugin.createNodesV2[1]([`${graph.leaf.directory}/📋️project.json`, "other/📋️project.json"], {}, { workspaceRoot: fixture }), new RegExp(`duplicate.*${graph.leaf.name}`, "i"));
    const projects = plugin.createNodesV2[1]([`${graph.leaf.directory}/📋️project.json`], {}, { workspaceRoot: fixture });
    assert.equal(projects[0][1].projects[graph.leaf.name].targets[graph.leaf.derivedTarget].cache, true);
    const rooted = plugin.createNodesV2[1](["📋️project.json"], {}, { workspaceRoot: fixture });
    const resolvedRoot = rooted[0][1].projects[graph.workspaceRoot.name].targets;
    for (const name of graph.workspaceRoot.targets) {
      const expected = vectors.policies.find((row) => row.target === name);
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
    stageArtifacts(staged, "leaf/Cargo.toml", new Map([["consumer", artifact], ["obsolete", artifact]]));
    stageArtifacts(staged, "leaf/Cargo.toml", new Map([["consumer", artifact]]));
    assert.deepEqual(readFileSync(join(staged, "consumer")), readFileSync(artifact));
    assert.equal(existsSync(join(staged, "obsolete")), false);
    if (process.platform !== "win32") assert.equal(lstatSync(join(staged, "consumer")).mode & 0o111, 0o111);
    assert.throws(() => stageArtifacts(staged, "another/Cargo.toml", new Map()), /Unowned/);
    assert.throws(() => stageArtifacts(staged, "leaf/Cargo.toml", new Map([["../escape", artifact]])), /Invalid artifact/);
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
    assert.deepEqual(resolveNxInvocation(["run", "workspace:dev", "--", "storybook", "ui"]).args, ["run", "workspace:dev-storybook", "--", "ui"]);
    assert.deepEqual(resolveNxInvocation(["show", "projects"]).args, ["show", "projects"]);
    assert.deepEqual(resolveNxInvocation(["run", "workspace:cpp", "--", "build", "macos-release"]).args, ["run", "workspace:cpp-build", "--", "macos-release"]);
    const cpp = source.statements.find((node: any) => ts.isClassDeclaration(node) && node.name?.text === "CppScript");
    const configure = cpp.members.find((node: any) => node.name?.getText(source) === "runConfigure").getText(source);
    assert.ok(!configure.includes("runSetup"), "CMake configuration must not reinstall its prerequisites");
    assert.ok(!cpp.getText(source).includes("ensureVcpkg"), "No current project consumes a vcpkg installation");
    assert.equal(resolveNxInvocation(["run", "workspace:test", "--", "quick"]).args[1], "workspace:test-quick");
    assert.equal(resolveNxInvocation(["run", "workspace:test", "--", "fundamental"]).args[1], "workspace:test-fundamental");
    assert.equal(resolveNxInvocation(["run", "workspace:setup", "--", "deps", "wasm"]).args[1], "workspace:deps-wasm");
    const rootTest = source.statements.find((node: any) => ts.isClassDeclaration(node) && node.name?.text === "TestScript").getText(source);
    assert.ok(!rootTest.includes("run-many"), "The root test target cannot schedule a second Nx graph");
    for (const level of ["fundamental", "quick", "long", "exhaustive"]) assert.ok(workspace.targets[`test-${level}`].dependsOn.length > 0);
    assert.equal(resolveNxInvocation(["run", "workspace:dev", "--", "s"]).args[1], "@semio-tech/framework-os-dev:dev");
    assert.ok(invocation.getText(nxSource).includes("loadFrameworkOsPlaygroundSelections()"));
    assert.ok(!invocation.getText(nxSource).includes("loadFrameworkOsPlaygroundCatalog()"));
    assert.equal(root.length > 0, true);
    rmSync(fixture, { recursive: true });
    console.log("[cache-contract] schema, graph ownership, source-byte discovery, native dependency oracles and materializer cancellation passed");
  }
  return { testCacheContracts };
}
