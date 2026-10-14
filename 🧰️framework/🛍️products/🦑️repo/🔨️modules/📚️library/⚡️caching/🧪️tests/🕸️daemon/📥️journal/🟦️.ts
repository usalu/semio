import assert from "node:assert/strict";
import { acquireResourceLease } from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import { readFilesystemJournalWorkerOptionsV1 } from "../../../🕸️graph/🔏️seal/📥️membership/📥️journal/👷️worker/📜️script.ts";
import { mkdir, readFile, rm, writeFile, symlink, unlink } from "node:fs/promises";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { runOwnedCommand } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { scriptInvocationEnvironment } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { createServer, type Socket } from "node:net";
import { createHash } from "node:crypto";
import type { ScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { createRepositoryFilesystemJournalV1 } from "../../../🕸️graph/🔏️seal/📥️membership/📥️journal/🟦️.ts";
import { createLiveFilesystemJournalV1, type FilesystemJournalChangeV1 } from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📥️journal/🟦️.ts";

/** 🧪️ Checks the pinned daemon journal protocol and real native observation boundaries. */
export async function proveDaemonJournal(workspace: string, output: string, invocation: ScriptInvocation, transport = false, fullWorkspace = false): Promise<void> {
  if (transport) return proveDaemonTransport(workspace, output, invocation);
  const control = scriptCompilerAdmissionControlV1(invocation, "nx:daemon-journal", 60_000);
  const require = createRequire(join(process.env.NODE_PATH ?? join(workspace, ".nx/installation/node_modules"), "../package.json")), fixture = JSON.parse(await readFile(join(import.meta.dir, "🧫️fixtures/🔣️.json"), "utf8"));
  const { DaemonClient } = require("nx/src/daemon/client/client"), { Watcher, WorkspaceContext } = require("nx/src/native");
  assert.equal(typeof DaemonClient.prototype.getSemioFilesystemJournalV1, "function", "Pinned Nx must expose its bounded filesystem journal request");
  const schema = JSON.parse(await readFile(join(import.meta.dir, "../../../🕸️graph/🔏️seal/📥️membership/📥️journal/🌐️rpc/🧬️schema/🔣️.json"), "utf8")), neutral = JSON.parse(await readFile(join(workspace, "🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📥️journal/🧬️schema/🔣️.json"), "utf8"));
  const ajv = new (require("ajv").default)().addSchema(neutral), oracle = ajv.compile(schema), witnessOracle = ajv.compile({ $ref: neutral.$id + "#/$defs/Witness" });
  assert.equal(oracle(fixture.request), true);
  for (const request of fixture.invalid) assert.equal(oracle(request), false);
  const workerSchema = JSON.parse(await readFile(join(import.meta.dir, "../../../🕸️graph/🔏️seal/📥️membership/📥️journal/👷️worker/🧬️schema/🔣️.json"), "utf8")), workerOracle = ajv.compile(workerSchema);
  const workerOptions = { ...fixture.worker, root: workspace, directory: output };
  assert.equal(workerOracle(workerOptions), true); assert.deepEqual(readFilesystemJournalWorkerOptionsV1(workerOptions), workerOptions);
  for (const invalid of [{ ...workerOptions, extra: true }, { ...workerOptions, generation: "stale" }, { ...workerOptions, toolingIdentity: "wrong" }]) { assert.equal(workerOracle(invalid), false); assert.throws(() => readFilesystemJournalWorkerOptionsV1(invalid), /deployment/); }
  await mkdir(output, { recursive: true });
  const root = join(output, "native"), data = join(output, "native-data");
  await rm(root, { recursive: true, force: true });
  for (const path of [fixture.source, fixture.unselected, fixture.manifest]) { await mkdir(dirname(join(root, path)), { recursive: true }); await writeFile(join(root, path), path.endsWith(".json") ? "{}" : "export const value=1;\n"); }
  await writeFile(join(root, ".gitignore"), "ignored/\n");
  const native = new WorkspaceContext(root, data), receiver = { current: undefined as undefined | ((error: unknown, events: readonly FilesystemJournalChangeV1[]) => void) }, pending: FilesystemJournalChangeV1[] = [];
  const watcher = new Watcher(root, ["!**/dist/**"], false);
  watcher.watch((error: unknown, events: FilesystemJournalChangeV1[]) => { if (receiver.current) receiver.current(error, events ?? []); else pending.push(...events ?? []); });
  await new Promise(resolve => setTimeout(resolve, 150)); watcher.forceFlushPending(); pending.splice(0);
  const observation = {
    watch(receive: (error: unknown, events: readonly FilesystemJournalChangeV1[]) => void) { receiver.current = receive; receive(null, pending.splice(0)); return () => { receiver.current = undefined; }; },
    flush: () => watcher.forceFlushPending(),
    async census() { await writeFile(join(root, "sample/race.ts"), "export const raced=true;\n"); return native.allFileData().map((row: { file: string }) => row.file); },
    async admittedPaths(paths: readonly string[]) { native.incrementalUpdate([...paths], []); return native.glob([...paths]); },
    isGlobalIdentity: (path: string) => path === fixture.manifest
  };
  let journal: Awaited<ReturnType<typeof createRepositoryFilesystemJournalV1>> | undefined;
  try {
    journal = await createRepositoryFilesystemJournalV1(root, observation, control);
    const scope = fixture.request.scope;
    const fresh = async () => { const value = await journal!.capture(scope, control); assert.equal(witnessOracle(value), true, JSON.stringify(witnessOracle.errors)); assert.equal(value.selectedMembership.sha256, createHash("sha256").update(value.selectedMembership.paths.map(path => path + "\0").join("")).digest("hex")); return value; };
    const until = async (predicate: (value: Awaited<ReturnType<typeof fresh>>) => boolean) => { const started = performance.now(); do { const value = await fresh(); if (predicate(value)) return value; assert.ok(performance.now() - started < 5000, "Native watcher must observe change within original finite budget"); await control.yieldContinuation(); await new Promise(resolve => setTimeout(resolve, 10)); } while (true); };
    const initial = await until(value => value.selectedMembership.paths.includes("sample/race.ts"));
    assert.ok(initial.selectedMembership.paths.includes(fixture.source));
    await writeFile(join(root, fixture.unselected), "export const value=2;\n");
    await new Promise(resolve => setTimeout(resolve, 100));
    assert.equal((await fresh()).sequence, initial.sequence, "Unselected source must not invalidate scoped witness");
    await writeFile(join(root, fixture.source), "export const value=3;\n");
    const updated = await until(value => value.sequence > initial.sequence);
    assert.equal(updated.selectedMembership.sha256, initial.selectedMembership.sha256);
    await writeFile(join(root, "sample/new.ts"), "export const created=true;\n");
    const created = await until(value => value.selectedMembership.paths.includes("sample/new.ts"));
    await rm(join(root, "sample/new.ts"));
    const deleted = await until(value => !value.selectedMembership.paths.includes("sample/new.ts") && value.sequence > created.sequence);
    await mkdir(join(root, "sample/dist")); await writeFile(join(root, "sample/dist/generated.ts"), "excluded");
    await new Promise(resolve => setTimeout(resolve, 100));
    assert.equal((await fresh()).sequence, deleted.sequence);
    await writeFile(join(root, fixture.manifest), '{"changed":true}');
    const identity = await until(value => value.sequence > deleted.sequence);
    assert.ok(identity.changes.some(row => row.path === fixture.manifest));
    await mkdir(join(root, "ignored")); await writeFile(join(root, "ignored/source.ts"), "ignored");
    await new Promise(resolve => setTimeout(resolve, 100));
    assert.ok(!native.glob(["ignored/source.ts"]).length, "Native effective census excludes ignored ordinary sources");
    const cancelled = new AbortController(); cancelled.abort(Error("Test query cancellation"));
    await assert.rejects(journal.capture(scope, scriptCompilerAdmissionControlV1(invocation, "nx:cancel-query", 1000, cancelled.signal)), /Test query cancellation/);
    assert.ok(await journal.verify(await fresh(), scope, control));
    await assert.rejects(journal.capture({ roots: ["../escape"], files: [] }, control), /path/);
    await journal.close(); await assert.rejects(journal.capture(scope, control), /closed/);
    journal = await createRepositoryFilesystemJournalV1(root, { ...observation, census: async () => native.allFileData().map((row: { file: string }) => row.file) }, control);
    if (process.platform !== "win32") { await symlink(join(root, "peer"), join(root, "sample/linked")); await new Promise(resolve => setTimeout(resolve, 100)); await assert.rejects(journal.capture({ roots: [], files: ["sample/linked"] }, control), /no-follow/); }
    console.log("[DEBUG] Native Nx Watcher + WorkspaceContext verified census race, flush, selected/global updates, create/delete, output/ignore exclusion, cancellation and no-follow refusal");
  } finally { await journal?.close(); await watcher.stop(); }
  let receive!: (error: unknown, events: readonly FilesystemJournalChangeV1[]) => void;
  const overflow = await createLiveFilesystemJournalV1({ source: { watch(callback) { receive = callback; return () => {}; }, async census() { return []; }, flush: () => [], async reconcile(event) { return [event.path]; }, async admitScope() {} }, excluded: () => false, isGlobalIdentity: () => false, maximumPaths: 10, maximumEvents: fixture.overflowEvents - 1 }, control);
  receive(null, Array.from({ length: fixture.overflowEvents }, (_, index) => ({ path: `sample/${index}.ts`, type: "create" as const })));
  await assert.rejects(overflow.capture(fixture.request.scope, control), /overflow/); await overflow.close();
  console.log("[DEBUG] Pinned Nx journal protocol, bounded overflow refusal and independent Ajv witness oracle verified");
  const transportRoot = fullWorkspace ? workspace : join(output, "transport"); if (!fullWorkspace) { await mkdir(transportRoot, { recursive: true }); await writeFile(join(transportRoot, "package.json"), JSON.stringify({ name: "journal-native-fixture", version: "1.0.0", private: true })); await writeFile(join(transportRoot, "nx.json"), JSON.stringify({ plugins: [] })); }
  await rm(join(output, "prior-observer.json"), { force: true });
  for (let generation = 0; generation < (fullWorkspace ? 1 : 2); generation++) await runOwnedCommand(process.execPath, [join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts"), "test", "daemon-journal", "transport"], transportRoot, "nx:journal-transport", 90_000, { signal: control.signal, env: scriptInvocationEnvironment(invocation, { ...process.env, SEMIO_DAEMON_FIXTURE_ROOT: transportRoot, SEMIO_DAEMON_FULL_WORKSPACE: fullWorkspace ? "true" : undefined, NX_DAEMON: "true", NX_SOCKET_DIR: undefined, NX_VERBOSE_LOGGING: "true", NX_WORKSPACE_DATA_DIRECTORY: join(output, "rpc-data"), SEMIO_TEST_ARTIFACT_DIR: output }) });
}


/** 🌐️ Uses a private daemon cache and owned child lifetime while retaining the real repository root. */
async function proveDaemonTransport(workspace: string, output: string, invocation: ScriptInvocation): Promise<void> {
  const fixture = JSON.parse(await readFile(join(import.meta.dir, "🧫️fixtures/🔣️.json"), "utf8"));
  if (process.platform !== "win32") {
    const available = fixture.configuredSocketLength - tmpdir().length - "/d.sock".length - 1, prefix = "semio-journal-" + process.pid + "-";
    assert.ok(available >= prefix.length); process.env.NX_SOCKET_DIR = join(tmpdir(), prefix.padEnd(available, "x")); await mkdir(process.env.NX_SOCKET_DIR, { recursive: true });
  }
  const daemonRoot = process.env.SEMIO_DAEMON_FIXTURE_ROOT ?? workspace;
  process.env.NX_WORKSPACE_ROOT_PATH = daemonRoot;
  const require = createRequire(join(process.env.NODE_PATH!, "../package.json")), { DaemonClient } = require("nx/src/daemon/client/client"), cache = require("nx/src/daemon/cache");
  const control = scriptCompilerAdmissionControlV1(invocation, "nx:journal-transport", 90_000), stop = new AbortController(), lines: string[] = [];
  const client = new DaemonClient(), scope = { roots: [], files: ["package.json"] };
  assert.equal(cache.serverProcessJsonPath, join(process.env.NX_WORKSPACE_DATA_DIRECTORY!, "d/server-process.json"), "Transport SDK client must use private service metadata");
  await rm(process.env.NX_WORKSPACE_DATA_DIRECTORY!, { recursive: true, force: true });
  assert.equal(await client.getSemioFilesystemJournalV1(scope, control), null, "Optional journal probe must not start a daemon");
  const cpuDirectory = join(output, "cpu"); await mkdir(cpuDirectory, { recursive: true });
  for (const file of ["cpu-request.json", "cpu-started.json", "cpu-finished.json"]) await rm(join(cpuDirectory, file), { force: true });
  const server = runOwnedCommand("node", ["--experimental-strip-types", "--import", join(import.meta.dir, "👷️cpu/📜️script.ts"), require.resolve("nx/src/daemon/server/start")], daemonRoot, "nx:private-daemon", 90_000, { signal: AbortSignal.any([control.signal, stop.signal]), env: { ...process.env, NX_DAEMON: "true", SEMIO_JOURNAL_CPU_PROBE_DIRECTORY: cpuDirectory }, onLine: line => lines.push(line) });
  let oracleResponse: unknown, ownedPid: number | undefined, failure: unknown; const tracked = server.catch(error => { failure = error; });
  try {
    const started = performance.now();
    while (!cache.readDaemonProcessJsonCache()) { control.signal.throwIfAborted(); if (failure) throw failure; assert.ok(performance.now() - started < 60_000, lines.slice(-20).join("\n")); await control.yieldContinuation(); await new Promise(resolve => setTimeout(resolve, 50)); }
    const metadata = cache.readDaemonProcessJsonCache(); ownedPid = metadata.processId;
    if (process.platform !== "win32") assert.equal(metadata.socketPath.length, fixture.configuredSocketLength, "Actual bootstrap-sized socket must fit both daemon and journal endpoint");
    assert.match(metadata.semioToolingIdentity, /^[a-f0-9]{64}$/u);
    const loaded = await require("nx/src/utils/workspace-context").loadSemioGraphAuthorizersV1();
    assert.ok(loaded, "Actual Bun client must load verified immutable authorizer module");
    assert.equal(loaded.toolingIdentity, metadata.semioToolingIdentity);
    const actualIdentity = (await import("../../../🕸️graph/🔏️seal/📥️membership/📥️journal/🌐️rpc/🟦️.ts")).loadedFilesystemJournalToolingIdentityV1(dirname(require.resolve("nx/src/daemon/client/client")));
    assert.equal(process.env.NX_DAEMON, "true", "Actual isolated client retains explicit enabled environment");
    assert.equal(client.enabled(), true, "Actual isolated client must be enabled");
    assert.equal(actualIdentity, metadata.semioToolingIdentity, "Actual isolated client and service must share immutable SDK receipt");
    let response = await client.getSemioFilesystemJournalV1(scope, control);
    const readinessStarted = performance.now();
    while (!response && performance.now() - readinessStarted < 60_000) { control.signal.throwIfAborted(); await control.yieldContinuation(); await new Promise(resolve => setTimeout(resolve, 50)); response = await client.getSemioFilesystemJournalV1(scope, control); }
    assert.ok(response, "Actual daemon journal unavailable: " + lines.slice(-20).join("\n")); oracleResponse = response;
    const serviceSchema = JSON.parse(await readFile(join(import.meta.dir, "../../../🕸️graph/🔏️seal/📥️membership/📥️journal/👷️worker/🧬️schema/🔣️.json"), "utf8")), serviceOracle = new (require("ajv").default)().compile({ $schema: serviceSchema.$schema, $defs: serviceSchema.$defs, $ref: "#/$defs/Service" });
    assert.equal(serviceOracle(JSON.parse(await readFile(join(process.env.NX_WORKSPACE_DATA_DIRECTORY!, "semio-journal/service.json"), "utf8"))), true, "Independent Ajv must accept actual worker service deployment");
    assert.deepEqual(response.witness.selectedMembership.paths, ["package.json"]);
    assert.equal(response.toolingIdentity, metadata.semioToolingIdentity);
    assert.equal(fixture.toolingLifetime.runningExclusive, "refused");
    await assert.rejects(acquireResourceLease({ directory: join(process.env.NODE_PATH!, "../../../../agents/resource-leases"), resource: fixture.toolingLifetime.resourcePrefix + metadata.semioToolingIdentity, mode: "exclusive", signal: control.signal, control, timeoutMs: 0 }), /timed out/, "Original live daemon must retain its immutable toolkit against pruning");
    console.log("[DEBUG] Actual Node daemon owner shared toolkit lease refused Bun exclusive retirement");
    const priorPath = join(output, "prior-observer.json");
    let prior: { epoch: string; processId: number } | undefined; try { prior = JSON.parse(await readFile(priorPath, "utf8")); } catch (error) { if ((error as { code?: string }).code !== "ENOENT") throw error; }
    if (prior) { assert.notEqual(response.witness.epoch, prior.epoch, "Fresh actual daemon observation must retire the previous epoch"); assert.notEqual(metadata.processId, prior.processId); console.log("[DEBUG] Actual daemon/worker retirement and restart produced a fresh observer epoch"); }
    await writeFile(priorPath, JSON.stringify({ epoch: response.witness.epoch, processId: metadata.processId }));
    if (process.env.SEMIO_DAEMON_FULL_WORKSPACE === "true") {
      assert.ok(response.witness.globalIdentity.paths.length > 1000, "Actual monorepo native observer must include structural graph identity");
      const timings: number[] = []; for (let index = 0; index < 5; index++) { const started = performance.now(), observed = await client.getSemioFilesystemJournalV1(scope, control); assert.ok(observed); assert.equal(observed.witness.epoch, response.witness.epoch); assert.equal(observed.witness.sequence, response.witness.sequence); timings.push(Math.round(performance.now() - started)); }
      console.log("[DEBUG] Actual monorepo journal coldReadyMs=" + Math.round(performance.now() - started) + " globalPaths=" + response.witness.globalIdentity.paths.length + " warmProbeMs=" + JSON.stringify(timings));
    } else {
    await writeFile(join(cpuDirectory, "cpu-request.json"), JSON.stringify({ version: 1, milliseconds: 2500 }));
    const blockStarted = performance.now();
    while (true) { try { const marker = JSON.parse(await readFile(join(cpuDirectory, "cpu-started.json"), "utf8")); assert.equal(marker.processId, metadata.processId); assert.equal(marker.threadId, 0, "CPU oracle must occupy the actual SDK main thread"); break; } catch { assert.ok(performance.now() - blockStarted < 5000, "Owned daemon main thread must start finite CPU oracle"); await new Promise(resolve => setTimeout(resolve, 10)); } }
    await writeFile(join(daemonRoot, "package.json"), JSON.stringify({ name: "journal-native-fixture", version: "1.0.1", private: true }));
    const busyStarted = performance.now(); let busy = await client.getSemioFilesystemJournalV1(scope, control);
    while ((!busy || busy.witness.sequence === response.witness.sequence) && performance.now() - busyStarted < 1200) { await new Promise(resolve => setTimeout(resolve, 10)); busy = await client.getSemioFilesystemJournalV1(scope, control); }
    assert.ok(busy && busy.witness.sequence > response.witness.sequence, "Independent native journal must observe mutation while SDK main thread is occupied");
    assert.ok(performance.now() - busyStarted < 1500, "Journal response must avoid the 2500ms occupied SDK main loop");
    await assert.rejects(readFile(join(cpuDirectory, "cpu-finished.json")), /ENOENT/, "Response must precede main-thread CPU completion");
    console.log("[DEBUG] Immutable journal worker observed source mutation and answered in " + Math.round(performance.now() - busyStarted) + "ms while actual SDK main thread remained occupied");
    }
    const disabled = process.env.NX_DAEMON; process.env.NX_DAEMON = "false";
    assert.equal(await client.getSemioFilesystemJournalV1(scope, control), null); process.env.NX_DAEMON = disabled;
    await writeFile(cache.serverProcessJsonPath, JSON.stringify({ ...metadata, semioToolingIdentity: "0".repeat(64) }));
    assert.equal(await client.getSemioFilesystemJournalV1(scope, control), null, "Old tooling daemon must not authorize reuse");
    await writeFile(cache.serverProcessJsonPath, JSON.stringify(metadata));
    const cancelled = new AbortController(); cancelled.abort(Error("RPC caller cancelled"));
    await assert.rejects(client.getSemioFilesystemJournalV1(scope, scriptCompilerAdmissionControlV1(invocation, "nx:rpc-cancel", 1000, cancelled.signal)), /RPC caller cancelled/);
    assert.ok(await client.getSemioFilesystemJournalV1(scope, control), "Caller cancellation must not stop shared daemon");
    console.log("[DEBUG] Configured bootstrap socket length=" + metadata.socketPath.length + " validated against deployed worker endpoint");
    console.log("[DEBUG] Actual immutable Nx daemon RPC verified live epoch, effective global census, SDK custody, explicit disable, old-tooling refusal, cancellation and continued service");
  } finally { stop.abort(Error("Private daemon runtime proof finished")); await tracked; await writeFile(join(output, "daemon-runtime.log"), lines.join("\n") + "\n"); }
  if (ownedPid) { const started = performance.now(); while (true) { try { process.kill(ownedPid, 0); } catch (error) { if ((error as { code?: string }).code === "ESRCH") break; throw error; } control.signal.throwIfAborted(); assert.ok(performance.now() - started < 5000, "Owned private daemon must exit before its socket is reused"); await new Promise(resolve => setTimeout(resolve, 10)); } }
  if (oracleResponse) {
    assert.equal(fixture.toolingLifetime.retiredExclusive, "admitted");
    const lease = await acquireResourceLease({ directory: join(process.env.NODE_PATH!, "../../../../agents/resource-leases"), resource: fixture.toolingLifetime.resourcePrefix + (oracleResponse as { toolingIdentity: string }).toolingIdentity, mode: "exclusive", signal: control.signal, control, timeoutMs: 0 });
    lease.release(); console.log("[DEBUG] Actual daemon retirement released its immutable toolkit lease to Bun exclusive retirement");
  }
  const socketPath = require("nx/src/daemon/socket-utils").getFullOsSocketPath();
  if (process.platform !== "win32") await unlink(socketPath).catch(error => { if (error.code !== "ENOENT") throw error; });
  let replaceGeneration = false;
  const sockets = new Set<Socket>(), slow = createServer(socket => {
    sockets.add(socket); socket.once("close", () => sockets.delete(socket));
    socket.on("data", () => { if (!replaceGeneration) return; replaceGeneration = false; setTimeout(() => { void (async () => { const file = join(process.env.NX_WORKSPACE_DATA_DIRECTORY!, "semio-journal/service.json"), metadata = JSON.parse(await readFile(file, "utf8")); await writeFile(file, JSON.stringify({ ...metadata, generation: "b".repeat(32) })); require("nx/src/daemon/socket-utils").sendMessage(socket, oracleResponse, "json"); })(); }, 50); });
  });
  await new Promise<void>((accept, reject) => { slow.once("error", reject); slow.listen(socketPath, accept); });
  const identity = (await import("../../../🕸️graph/🔏️seal/📥️membership/📥️journal/🌐️rpc/🟦️.ts")).loadedFilesystemJournalToolingIdentityV1(dirname(require.resolve("nx/src/daemon/client/client")));
  await mkdir(dirname(cache.serverProcessJsonPath), { recursive: true });
  await writeFile(cache.serverProcessJsonPath, JSON.stringify({ processId: process.pid, socketPath, nxVersion: require("nx/package.json").version, semioToolingIdentity: identity }));
  const serviceDirectory = join(process.env.NX_WORKSPACE_DATA_DIRECTORY!, "semio-journal"); await mkdir(serviceDirectory, { recursive: true });
  await writeFile(join(serviceDirectory, "service.json"), JSON.stringify({ version: 1, root: daemonRoot, processId: process.pid, generation: "a".repeat(32), socketPath, toolingIdentity: identity }));
  try {
    replaceGeneration = true;
    assert.equal(await client.getSemioFilesystemJournalV1(scope, control), null, "An old in-flight observer response cannot authorize a replacement generation");
    console.log("[DEBUG] Actual SDK JSON response refused after its service generation changed while in flight");
    const started = performance.now();
    assert.equal(await client.getSemioFilesystemJournalV1(scope, control), null);
    assert.ok(performance.now() - started >= 900 && performance.now() - started < 2000, "Optional daemon probe must be bounded to 1 second");
    await assert.rejects(client.getSemioFilesystemJournalV1(scope, scriptCompilerAdmissionControlV1(invocation, "nx:rpc-deadline", 100)), /deadline/);
    const cancelled = new AbortController(), probing = client.getSemioFilesystemJournalV1(scope, scriptCompilerAdmissionControlV1(invocation, "nx:rpc-mid-cancel", 5000, cancelled.signal));
    setTimeout(() => cancelled.abort(Error("RPC active caller cancelled")), 100);
    await assert.rejects(probing, /RPC active caller cancelled/);
    await new Promise(resolve => setTimeout(resolve, 50));
    assert.equal(sockets.size, 0, "Only dedicated request sockets must close after timeout/cancellation");
    assert.equal(slow.listening, true, "Request cancellation preserves service lifetime");
    invocation.control.signal.throwIfAborted();
    console.log("[DEBUG] Installed Nx client actual stalled socket verified 1s fallback, original deadline, mid-request cancellation and socket release");
  } finally { for (const socket of sockets) socket.destroy(); await new Promise<void>(accept => slow.close(() => accept())); }
}
