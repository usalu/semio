import assert from "node:assert/strict";
import { readFileSync, mkdirSync, writeFileSync, mkdtempSync, rmSync } from "node:fs";
import { dirname, join, resolve, relative } from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath, pathToFileURL } from "node:url";

/** 🧪️ Verifies native source and command ownership against compiler and bundler input oracles. */
export async function testCommandInputs(workspace: string, output: string): Promise<void> {
  testNxDaemonTaskEnvironment(workspace);
  testNxDaemonDiagnostics(workspace, output);
  testNxDaemonRetention(workspace, output);
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
