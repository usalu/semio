import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { createRequire } from "node:module";
import { runInNewContext } from "node:vm";
import { EventEmitter } from "node:events";
import { spawnSync } from "node:child_process";

/** 🧹️ Confirms the native Nx source watcher excludes compiler and map cache writes. */
export function testWorkspaceWatchIgnores(workspace: string, output: string): void {
  const require = createRequire(join(workspace, "package.json"));
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/watcher-readiness/🔣️.json"), "utf8"));
  const rules = readFileSync(join(workspace, ".nxignore"), "utf8"), ignore = require("ignore")().add(rules);
  for (const path of fixture.ignored) assert.ok(ignore.ignores(path), `Nx must ignore derived cache ${path}`);
  assert.equal(ignore.ignores(fixture.source), false);
  const source = `
    const fs = require("node:fs"), path = require("node:path"), assert = require("node:assert/strict");
    const { Watcher } = require(${JSON.stringify(require.resolve("nx/src/native"))});
    const fixture = ${JSON.stringify(fixture)}, root = fs.mkdtempSync(path.join(${JSON.stringify(output)}, "nx-watch-"));
    fs.writeFileSync(path.join(root, ".nxignore"), ${JSON.stringify(rules)});
    for (const name of [fixture.source, ...fixture.ignored]) { fs.mkdirSync(path.dirname(path.join(root, name)), { recursive: true }); fs.writeFileSync(path.join(root, name), "before"); }
    const watcher = new Watcher(root, ${JSON.stringify(rules.split("\n").filter(line => line.trim() && !line.startsWith("#")).map(line => "!" + line))}, false), seen = [];
    watcher.watch((error, events) => { assert.ifError(error); seen.push(...events); });
    (async () => {
      try {
        await new Promise(resolve => setTimeout(resolve, 300));
        for (const name of [fixture.source, ...fixture.ignored]) fs.writeFileSync(path.join(root, name), "after");
        const deadline = Date.now() + 8000;
        while (!seen.some(event => event.path === fixture.source) && Date.now() < deadline) await new Promise(resolve => setTimeout(resolve, 100));
        seen.push(...watcher.forceFlushPending());
        assert.ok(seen.some(event => event.path === fixture.source), "source edit was not observed: " + JSON.stringify(seen));
        for (const name of fixture.ignored) assert.ok(!seen.some(event => event.path === name), name);
      } finally { await watcher.stop(); fs.rmSync(root, { recursive: true, force: true }); }
    })().catch(error => { console.error(error); process.exitCode = 1; });
  `;
  const result = spawnSync("node", ["--eval", source], { cwd: workspace, encoding: "utf8", timeout: 15000 });
  assert.equal(result.status, 0, result.stdout + result.stderr);
  console.log("[DEBUG] Native Nx watcher observes source edits and excludes compiler/map cache writes PASS");
}

/** ⏳️ Uses Nx's own readiness output to verify progress, long startup and process cancellation. */
export async function testWatcherReadiness(workspace: string): Promise<void> {
  const require = createRequire(join(workspace, "package.json")), ts = require("typescript");
  const root = join(import.meta.dir, "../.."), fixture = JSON.parse(readFileSync(join(root, "🧫️fixtures/watcher-readiness/🔣️.json"), "utf8"));
  const source = ts.createSourceFile("bootstrap.ts", readFileSync(join(root, "🚀️bootstrap/📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
  const clientSource = ts.createSourceFile("client.js", readFileSync(require.resolve("nx/src/daemon/client/client"), "utf8"), ts.ScriptTarget.Latest, true);
  const clientClass = clientSource.statements.find((node: any) => ts.isClassDeclaration(node) && node.name?.text === "DaemonClient");
  const subscribe = clientClass.members.find((node: any) => node.name?.text === "registerFileWatcher");
  const Client = runInNewContext(`(class { ${subscribe.getText(clientSource)} })`);
  for (const row of fixture.subscriptions) {
    let graphReads = 0;
    const messages: unknown[] = [], client = Object.assign(new Client(), { getProjectGraphAndSourceMaps: async () => { graphReads++; }, fileWatcherCallbacks: new Map(), fileWatcherConfigs: new Map(), fileWatcherMessenger: { sendMessage: (message: unknown) => messages.push(message) }, queue: { sendToQueue: async (callback: () => Promise<void>) => callback() } });
    await client.registerFileWatcher(row.config, () => {});
    assert.equal(graphReads, row.graphReads, "A subscription to every workspace file needs no project classification");
    assert.equal(messages.length, 1);
    assert.equal(client.fileWatcherConfigs.size, 1);
  }
  const environmentDefinition = source.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "nxChildEnvironment");
  assert.ok(environmentDefinition, "Watched builds must own their graph independently of other daemon clients");
  const environmentCode = ts.transpileModule(environmentDefinition.getText(source).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  const environment = runInNewContext(`${environmentCode}; nxChildEnvironment;`);
  for (const row of fixture.processes) {
    const before = { ...row.env }, selected = environment(row.env, row.args, row.watching);
    assert.deepEqual(row.env, before, "Launcher must retain caller environment");
    const native = spawnSync("node", ["--eval", `process.stdout.write(String(require(${JSON.stringify(require.resolve("nx/src/daemon/client/client"))}).daemonClient.enabled()))`], { cwd: workspace, env: { ...process.env, ...selected }, encoding: "utf8", timeout: 10000 });
    assert.equal(native.status, 0, native.stderr);
    assert.equal(native.stdout, String(row.daemon), row.name);
  }
  const definition = source.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "waitForNxWatcher");
  assert.ok(definition, "Watcher startup must support progress and cancellation without a fixed cold-graph deadline");
  const code = ts.transpileModule(definition.getText(source).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  for (const row of fixture.cases) {
    const timers = new Map<number, () => void>(), progress: string[] = [], stdout = new EventEmitter(), watcher = Object.assign(new EventEmitter(), { stdout });
    let now = 0, interval = 0, next = 0;
    const wait = runInNewContext(`${code}; waitForNxWatcher;`, { Date: { now: () => now }, console: { log: (message: string) => progress.push(message) }, setInterval: (callback: () => void, ms: number) => { interval = ms; timers.set(++next, callback); return next; }, clearInterval: (id: number) => timers.delete(id) });
    const done = wait(watcher).then(() => "ready", () => "error");
    assert.equal(interval, fixture.progressIntervalMs);
    for (; now < row.elapsedMs; now += interval) for (const callback of timers.values()) callback();
    if (row.chunks) for (const chunk of row.chunks) stdout.emit("data", Buffer.from(chunk));
    else if (row.error) watcher.emit("error", new Error(row.error));
    else watcher.emit("close", row.exit, row.signal);
    assert.equal(await done, row.result, row.name);
    assert.equal(timers.size, 0, row.name);
    assert.equal(watcher.listenerCount("close") + watcher.listenerCount("error") + stdout.listenerCount("data"), 0, row.name);
    assert.equal(progress.length, row.elapsedMs / fixture.progressIntervalMs, row.name);
  }
  const output: string[] = [], exports: any = {};
  const modules: Record<string, any> = {
    child_process: {}, "../../daemon/client/client": { daemonClient: { enabled: () => true, registerFileWatcher: async () => {} } },
    "../../daemon/client/daemon-socket-messenger": {}, "../../utils/output": { output: { logSingleLine: (text: string) => output.push(text) } },
  };
  runInNewContext(readFileSync(require.resolve("nx/src/command-line/watch/watch"), "utf8"), { exports, require: (name: string) => modules[name], process: { env: {} } });
  void exports.watch({ all: true, includeGlobalWorkspaceFiles: true, verbose: true, command: "fixture" });
  await Promise.resolve();
  assert.ok(output.includes(fixture.cases[0].chunks.join("").trim().replace(/^NX /, "")), "Fixture must match the installed Nx watcher readiness output");
  console.log(`[DEBUG] Nx watcher readiness: ${fixture.cases.length} lifecycle cases, including 150-second startup, pass against installed Nx output`);
}

/** 🧵️ Exercises the installed Nx scheduler at controlled native discovery boundaries. */
export async function testGraphCoalescing(workspace: string, source?: string): Promise<void> {
  const require = createRequire(join(workspace, "package.json")), fixtureRoot = join(import.meta.dir, "../../🧫️fixtures/graph-coalescing");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
  assert.ok(new (require("ajv/dist/2020").default)().validate(JSON.parse(readFileSync(join(fixtureRoot, "🛂️schema/🔣️.json"), "utf8")), fixture));
  source ??= readFileSync(require.resolve("nx/src/daemon/server/project-graph-incremental-recomputation"), "utf8");
  const tick = () => new Promise<void>(accept => setImmediate(accept));
  for (const vector of [...fixture.cases, ...fixture.failures.map((failure: string) => ({ name: `${failure}-error-recovery`, failure, phase: "nodes", changes: 0, requests: 1, computations: 1 + Number(["nodes", "dependencies"].includes(failure)) }))]) {
    let revision = 0, pluginRevision = 0, dotenv = false, calls = 0, active = 0, maximum = 0, released = false;
    let failed = false;
    const fault = (phase: string) => { if (vector.failure === phase && !failed) { failed = true; throw new Error(`Fixture ${phase} failure`); } };
    let release!: () => void, entered!: () => void;
    const gate = new Promise<void>(accept => release = () => { released = true; accept(); }), ready = new Promise<void>(accept => entered = accept);
    const exported: any = {}, persisted: any[] = [], notifications: any[] = [];
    const bounded = async <T>(promise: Promise<T>): Promise<T> => {
      let timer: ReturnType<typeof setTimeout> | undefined;
      try { return await Promise.race([promise, new Promise<never>((_, reject) => timer = setTimeout(() => reject(new Error(`${vector.name}: graph requests did not settle`)), 5000))]); }
      finally { clearTimeout(timer); }
    };
    const nodes = () => ({ fixture: { name: "fixture", root: "fixture", metadata: { revision, pluginRevision } } });
    const fileMap = () => ({ fileMap: { projectFileMap: { fixture: [{ file: "fixture/source.json", hash: String(revision) }] }, nonProjectFiles: [] }, rustReferences: {} });
    const pause = async (phase: string) => {
      active++; maximum = Math.max(maximum, active);
      try { if (phase === vector.phase && !released) { entered(); await gate; } }
      finally { active--; }
    };
    const modules: Record<string, unknown> = {
      "perf_hooks": { performance: { mark() {}, measure() {} } },
      "../../config/nx-json": { readNxJson: () => { fault("configuration"); return { plugins: [String(pluginRevision)] }; } },
      "../../hasher/file-hasher": { hashArray: JSON.stringify, hashObject: JSON.stringify },
      "../../project-graph/build-project-graph": { buildProjectGraphUsingProjectFileMap: async (projects: any) => { fault("dependencies"); await pause("dependencies"); return { projectGraph: { nodes: projects, dependencies: {} }, projectFileMapCache: {} }; } },
      "../../project-graph/error-types": require("nx/src/project-graph/error-types"),
      "../../project-graph/file-map-utils": { updateFileMap: fileMap },
      "../../project-graph/nx-deps-cache": { nxProjectGraph: "project-graph.json", readFileMapCache: () => undefined, writeCache: (_cache: unknown, graph: unknown) => persisted.push(graph), writeCacheIfStale() {} },
      "../../project-graph/plugins/get-plugins": { getPlugins: async () => [], getPluginsSeparated: async () => { fault("plugins"); return { specifiedPlugins: [], defaultPlugins: [] }; } },
      "../../project-graph/utils/retrieve-workspace-files": { retrieveProjectConfigurations: async () => { calls++; fault("nodes"); const projects = nodes(); await pause("nodes"); return { projects, sourceMaps: {}, externalNodes: {}, projectRootMap: {} }; }, retrieveWorkspaceFiles: async () => fileMap() },
      "../../utils/fileutils": { fileExists: () => true },
      "../../utils/workspace-context": { resetWorkspaceContext() {}, updateFilesInContext: (_root: string, files: string[]) => Object.fromEntries(files.map(file => [file, String(revision)])) },
      "../../utils/workspace-root": { workspaceRoot: workspace },
      "../../utils/progress-topics": { ProgressTopics: { GraphConstruction: "graph" } },
      "./client-socket-context": { subscribeClientToTopic() {}, unsubscribeClientFromTopic() {} },
      "./dotenv-graph-changes": { clearDotEnvFileHashes() {}, hasPendingDotEnvEvidence: () => dotenv, hasRelevantPendingDotEnvEvidence: () => dotenv, drainPendingDotEnvEvents: () => { const invalidating = dotenv ? [".env"] : []; dotenv = false; return { overflowed: false, invalidating }; } },
      "./file-watching/file-change-events": { notifyFileChangeListeners() {} },
      "./file-watching/file-watcher-sockets": { notifyFileWatcherSockets() {} },
      "./project-graph-listener-sockets": { notifyProjectGraphListenerSockets: (graph: unknown) => notifications.push(graph) },
      "./watcher": { flushPendingWorkspaceChanges: async () => {} },
      "../logger": { serverLogger: { log() {}, requestLog() {} } },
    };
    runInNewContext(source, { exports: exported, require: (name: string) => { assert.ok(name in modules, name); return modules[name]; }, global: {}, setImmediate }, { filename: "nx-graph-recomputation.js" });
    if (vector.failure) {
      release();
      const rejected: any = await bounded(exported.getCachedSerializedProjectGraphPromise());
      assert.match(String(rejected.error), /Fixture/, vector.name);
      assert.equal(persisted.length, 0);
      assert.equal(notifications.length, 0);
    }
    const original = exported.getCachedSerializedProjectGraphPromise();
    if (vector.failure) entered();
    await bounded(ready);
    for (let index = 0; index < vector.changes; index++) { revision++; exported.scheduleProjectGraphRecomputation([], ["fixture/source.json"], []); }
    if (vector.invalidate) { revision++; exported.invalidateGraphCache(); }
    if (vector.plugins) pluginRevision++;
    if (vector.dotenv) { revision++; dotenv = true; }
    const pending = Array.from({ length: vector.requests }, () => exported.getCachedSerializedProjectGraphPromise());
    await tick(); await tick();
    release();
    const results: any[] = await bounded(Promise.all([original, ...pending]));
    for (const result of results) {
      assert.equal(result.error, null, vector.name);
      assert.deepEqual(JSON.parse(JSON.stringify(result.projectGraph.nodes.fixture.metadata)), { revision, pluginRevision }, vector.name + ": stale graph");
    }
    assert.equal(maximum, fixture.maximumConcurrentComputations, `${vector.name}: overlapping discovery work`);
    assert.equal(calls, vector.computations, `${vector.name}: redundant discovery work`);
    assert.equal(persisted.length, 1, `${vector.name}: stale graph persisted`);
    assert.equal(notifications.length, 1, `${vector.name}: stale graph announced`);
    const warm: any = await bounded(exported.getCachedSerializedProjectGraphPromise());
    assert.equal(warm.projectGraph, results[0].projectGraph);
    assert.equal(calls, vector.computations, vector.name + ": warm graph recomputed");
    console.log(`[DEBUG] Nx graph ${vector.name}: ${calls} computations, concurrency ${maximum}, fresh result for ${results.length} callers PASS`);
  }
}
