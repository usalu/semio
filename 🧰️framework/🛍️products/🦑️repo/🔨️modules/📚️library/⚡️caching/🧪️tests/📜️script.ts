import assert from "node:assert/strict";
import { readFileSync, mkdirSync, writeFileSync, mkdtempSync, rmSync } from "node:fs";
import { dirname, join, resolve, relative } from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath, pathToFileURL } from "node:url";
import { testGraphCoalescing } from "./🕸️daemon/📜️script.ts";
import { testBrowserDistribution } from "./🌐️browser/📜️script.ts";
import { testContinuousServices } from "./🖥️services/📜️script.ts";
import { testServiceReadiness } from "./🖥️services/🌐️readiness/📜️script.ts";

/** 🧪️ Verifies native source and command ownership against compiler and bundler input oracles. */
export async function testCommandInputs(workspace: string, output: string): Promise<void> {
  testNxDaemonTaskEnvironment(workspace);
  testNxDaemonDiagnostics(workspace, output);
  testNxDaemonRetention(workspace, output);
  await testGraphCoalescing(workspace);
  await testContinuousServices(workspace, output);
  testWorkspaceRoots(workspace, output);
  await testRuntimeComponents(workspace);
  await testDemonstratorRuntime(workspace);
  await testBrowserModuleRelocation(workspace);
  await testBrowserDistribution(workspace, output);
  await testServiceReadiness(workspace, output);
  await testBunDependencies(workspace, output);
  await testNativePreparation(workspace, output);
  const require = createRequire(import.meta.url), fixtures = join(dirname(fileURLToPath(import.meta.url)), "../🧫️fixtures");
  const root = mkdtempSync(join(output, "native-inputs-"));
  try {
    const cases = JSON.parse(readFileSync(join(fixtures, "native-inputs/🧫️cases.json"), "utf8"));
    assert.equal(require("jsonschema").validate(cases, JSON.parse(readFileSync(join(fixtures, "native-inputs/🧬️schema.json"), "utf8"))).valid, true);
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
    assert.deepEqual(actual, Object.keys(built.metafile.inputs).map((path) => "{workspaceRoot}/" + relative(workspace, resolve(workspace, path))).sort());
    console.log("[DEBUG] Native input closure preserves rustc-consumed assets, excludes frontend and separate tests, and matches the esbuild command import oracle PASS");
    
    const boundaries = JSON.parse(readFileSync(join(fixtures, "command-boundaries/🧫️cases.json"), "utf8"));
    assert.equal(require("jsonschema").validate(boundaries, JSON.parse(readFileSync(join(fixtures, "command-boundaries/🧬️schema.json"), "utf8"))).valid, true);
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
  const fixture = JSON.parse(readFileSync(join(directory, "🧫️cases.json"), "utf8"));
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
  const require = createRequire(import.meta.url), fixtures = join(dirname(fileURLToPath(import.meta.url)), "../🧫️fixtures/runtime-components");
  const fixture = JSON.parse(readFileSync(join(fixtures, "🔣️.json"), "utf8"));
  assert.ok(new (require("ajv/dist/2020").default)().validate(JSON.parse(readFileSync(join(fixtures, "🧬️schema.json"), "utf8")), fixture));
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
    checked++;
  }
  console.log("[DEBUG] Runtime component closure includes transitive contributions, unions and hosts and matches graphlib PASS");
  console.log(`[DEBUG] ${checked} real playground preparation targets match the independent Cargo metadata runtime closure PASS`);
}

/** 🗂️ Keeps producers inside the workspace supplied by Nx's own task environment. */
export function testWorkspaceRoots(workspace: string, output: string): void {
  const require = createRequire(import.meta.url), root = mkdtempSync(join(output, "workspace-roots-"));
  const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../🧫️fixtures/🗂️workspace-roots.json"), "utf8"));
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
  const require = createRequire(import.meta.url), fixtures = join(dirname(fileURLToPath(import.meta.url)), "../🧫️fixtures/bun-dependencies");
  const cases = JSON.parse(readFileSync(join(fixtures, "🔣️.json"), "utf8"));
  assert.ok(new (require("ajv/dist/2020").default)().validate(JSON.parse(readFileSync(join(fixtures, "🧬️schema.json"), "utf8")), cases));
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
  const require = createRequire(import.meta.url), fixtures = join(dirname(fileURLToPath(import.meta.url)), "../🧫️fixtures/native-preparation");
  const cases = JSON.parse(readFileSync(join(fixtures, "🧫️cases.json"), "utf8"));
  assert.equal(require("jsonschema").validate(cases, JSON.parse(readFileSync(join(fixtures, "🧬️schema.json"), "utf8"))).valid, true);
  const native = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts")).href);
  assert.equal(typeof native.validateNativeCargoArguments, "function");
  for (const row of cases.arguments) if (row.valid) assert.doesNotThrow(() => native.validateNativeCargoArguments(row.operation, row.args)); else assert.throws(() => native.validateNativeCargoArguments(row.operation, row.args), /input contract/);
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
    console.log("[DEBUG] Installed Nx daemon diagnostics preserve the tail using bounded reads PASS");
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
  console.log("[DEBUG] Nx daemon graph identity ignores task-scoped environment and preserves real graph controls PASS");
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
  console.log("[DEBUG] Nx daemon retention: repeated native writes are bounded, tail preserved and existing append descriptor stays live PASS");
`;
  try {
    const child = Bun.spawnSync(["node", "--eval", code], { cwd: workspace, env: { ...process.env, NX_WORKSPACE_DATA_DIRECTORY: join(directory, "workspace-data") }, stdout: "pipe", stderr: "pipe", timeout: 10000 });
    assert.equal(child.exitCode, 0, child.stdout.toString() + child.stderr.toString());
    process.stdout.write(child.stdout);
  } finally { rmSync(directory, { recursive: true, force: true }); }
}
