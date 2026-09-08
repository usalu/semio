import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { createRequire } from "node:module";
import { runInNewContext } from "node:vm";

/** 🧵️ Exercises the installed Nx scheduler at controlled native discovery boundaries. */
export async function testGraphCoalescing(workspace: string, source?: string): Promise<void> {
  const require = createRequire(join(workspace, "package.json")), fixtureRoot = join(import.meta.dir, "../../🧫️fixtures/graph-coalescing");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
  assert.ok(new (require("ajv/dist/2020").default)().validate(JSON.parse(readFileSync(join(fixtureRoot, "🧬️schema.json"), "utf8")), fixture));
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
