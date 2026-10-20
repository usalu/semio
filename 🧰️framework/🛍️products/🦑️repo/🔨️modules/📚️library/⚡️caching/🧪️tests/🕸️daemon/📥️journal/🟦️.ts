import assert from "node:assert/strict";
import { acquireResourceLease, resourceLeasePathV1, checkResourceLeaseControl } from "@semio-tech/process/leases";
import { readFilesystemJournalWorkerOptionsV1 } from "../../../🕸️graph/🔏️seal/📥️membership/📥️journal/👷️worker/🟦️.ts";
import { mkdir, readFile, readdir, lstat, realpath, rm, writeFile, symlink, unlink } from "node:fs/promises";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { runOwnedCommand } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { scriptInvocationEnvironment } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { createConnection, createServer, type Socket } from "node:net";
import { createHash } from "node:crypto";
import type { ScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { createRepositoryFilesystemJournalV1 } from "../../../🕸️graph/🔏️seal/📥️membership/📥️journal/🟦️.ts";
import { createLiveFilesystemJournalV1, type FilesystemJournalChangeV1 } from "@semio-tech/framework-filesystem/observation/journal";

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
  const workerSchema = JSON.parse(await readFile(join(import.meta.dir, "../../../🕸️graph/🔏️seal/📥️membership/📥️journal/👷️worker/🧬️schema/🔣️.json"), "utf8")), toolingWitnessSchema=JSON.parse(await readFile(join(import.meta.dir,"../../../🚀️bootstrap/🛠️tools/📥️use/🧬️schema/📥️witness.json"),"utf8")), workerOracle = ajv.addSchema(toolingWitnessSchema).compile(workerSchema);
  const workerOptions = { ...fixture.worker, root: workspace, directory: output,repositoryRoot:workspace,readerToken:"d".repeat(32),serviceWitness:{version:1,identity:fixture.worker.toolingIdentity,toolkitRoot:join(workspace,".cache/b/t/m/nx",fixture.worker.toolingIdentity),processId:process.pid,nonce:"12345678-1234-4234-8234-123456789abc",startedAt:1,endpoint:{host:"127.0.0.1",port:1},original:{version:1,policy:{version:1,owner:"repository:nx-daemon-service",maximumElapsedMilliseconds:0},deadlineEpochMilliseconds:null,capabilities:{version:1,kind:"nx-daemon-service",repositoryRoot:workspace,workspaceRoot:workspace,toolingIdentity:fixture.worker.toolingIdentity,generation:"e".repeat(32),directory:output,watcherIdentity:null,graphIdentity:null,implementationIdentity:"d".repeat(64)}}} };
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
  } finally { await journal?.close(); await watcher.stop(); }
  let receive!: (error: unknown, events: readonly FilesystemJournalChangeV1[]) => void;
  const overflow = await createLiveFilesystemJournalV1({ source: { watch(callback) { receive = callback; return () => {}; }, async census() { return []; }, flush: () => [], async reconcile(event) { return [event.path]; }, async admitScope() {} }, excluded: () => false, isGlobalIdentity: () => false, maximumPaths: 10, maximumEvents: fixture.overflowEvents - 1 }, control);
  receive(null, Array.from({ length: fixture.overflowEvents }, (_, index) => ({ path: `sample/${index}.ts`, type: "create" as const })));
  await assert.rejects(overflow.capture(fixture.request.scope, control), /overflow/); await overflow.close();
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
  process.env.SEMIO_WORKSPACE_ROOT_PATH = daemonRoot;
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
    const priorPath = join(output, "prior-observer.json");
    let prior: { epoch: string; processId: number } | undefined; try { prior = JSON.parse(await readFile(priorPath, "utf8")); } catch (error) { if ((error as { code?: string }).code !== "ENOENT") throw error; }
    if (prior) { assert.notEqual(response.witness.epoch, prior.epoch, "Fresh actual daemon observation must retire the previous epoch"); assert.notEqual(metadata.processId, prior.processId);  }
    await writeFile(priorPath, JSON.stringify({ epoch: response.witness.epoch, processId: metadata.processId }));
    if (process.env.SEMIO_DAEMON_FULL_WORKSPACE === "true") {
      assert.ok(response.witness.globalIdentity.paths.length > 1000, "Actual monorepo native observer must include structural graph identity");
      const timings: number[] = []; for (let index = 0; index < 5; index++) { const started = performance.now(), observed = await client.getSemioFilesystemJournalV1(scope, control); assert.ok(observed); assert.equal(observed.witness.epoch, response.witness.epoch); assert.equal(observed.witness.sequence, response.witness.sequence); timings.push(Math.round(performance.now() - started)); }
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
    }
    const disabled = process.env.NX_DAEMON; process.env.NX_DAEMON = "false";
    assert.equal(await client.getSemioFilesystemJournalV1(scope, control), null); process.env.NX_DAEMON = disabled;
    await writeFile(cache.serverProcessJsonPath, JSON.stringify({ ...metadata, semioToolingIdentity: "0".repeat(64) }));
    assert.equal(await client.getSemioFilesystemJournalV1(scope, control), null, "Old tooling daemon must not authorize reuse");
    await writeFile(cache.serverProcessJsonPath, JSON.stringify(metadata));
    const cancelled = new AbortController(); cancelled.abort(Error("RPC caller cancelled"));
    await assert.rejects(client.getSemioFilesystemJournalV1(scope, scriptCompilerAdmissionControlV1(invocation, "nx:rpc-cancel", 1000, cancelled.signal)), /RPC caller cancelled/);
    assert.ok(await client.getSemioFilesystemJournalV1(scope, control), "Caller cancellation must not stop shared daemon");
  } finally { stop.abort(Error("Private daemon runtime proof finished")); await tracked; await writeFile(join(output, "daemon-runtime.log"), lines.join("\n") + "\n"); }
  if (ownedPid) { const started = performance.now(); while (true) { try { process.kill(ownedPid, 0); } catch (error) { if ((error as { code?: string }).code === "ESRCH") break; throw error; } control.signal.throwIfAborted(); assert.ok(performance.now() - started < 5000, "Owned private daemon must exit before its socket is reused"); await new Promise(resolve => setTimeout(resolve, 10)); } }
  if (oracleResponse) {
    assert.equal(fixture.toolingLifetime.retiredExclusive, "admitted");
    const toolingIdentity = (oracleResponse as { toolingIdentity: string }).toolingIdentity;
    try {
      const lease = await acquireResourceLease({ directory: join(process.env.NODE_PATH!, "../../../../agents/resource-leases"), resource: fixture.toolingLifetime.resourcePrefix + toolingIdentity, mode: "exclusive", signal: control.signal, control, timeoutMs: 0 });
      lease.release(); 
    } catch (error) {
      if (!String(error).includes("Resource lease timed out")) throw error;
      const schema = JSON.parse(await readFile(join(import.meta.dir, "../../../🕸️graph/🔏️seal/📥️membership/📥️journal/👷️worker/🧬️schema/🔣️.json"), "utf8")), validate = new (require("ajv").default)().compile({ $schema: schema.$schema, $defs: schema.$defs, $ref: "#/$defs/Service" }), protectedOwners: number[] = [];
      for (const namespace of ["tooling", "watchers"]) {
        const directory = join(workspace, ".nx/workspace-data", namespace);
        let names: string[]; try { names = await readdir(directory); } catch (error) { if ((error as { code?: string }).code === "ENOENT") continue; throw error; }
        for (const name of names) {
          control.signal.throwIfAborted(); await control.yieldContinuation();
          if (!/^[a-f0-9]{24}$/u.test(name)) continue;
          try {
            const file = join(directory, name, "semio-journal/service.json"), status = await lstat(file);
            assert.ok(status.isFile() && !status.isSymbolicLink());
            const service = JSON.parse(await readFile(file, "utf8"));
            if (!validate(service) || service.processId === ownedPid || service.toolingIdentity !== toolingIdentity || service.root !== workspace) continue;
            process.kill(service.processId, 0); protectedOwners.push(service.processId);
          } catch (error) { if (!["ENOENT", "ESRCH"].includes((error as { code?: string }).code ?? "")) throw error; }
        }
      }
      const finiteSchema=JSON.parse(await readFile(join(import.meta.dir,"../../../🚀️bootstrap/🛠️tools/📥️use/🧬️schema/📥️witness.json"),"utf8")),validateFinite=new(require("ajv").default)().compile(finiteSchema),finiteDirectory=join(workspace,".🧬semio/🦑️repo/⚡️cache/agents/nx-tooling-use"),toolkitRoot=await realpath(join(process.env.NODE_PATH!,".."));
      let finiteNames:string[];try{const status=await lstat(finiteDirectory);assert.ok(status.isDirectory()&&!status.isSymbolicLink());assert.equal(await realpath(finiteDirectory),finiteDirectory);finiteNames=await readdir(finiteDirectory)}catch(error){if((error as{code?:string}).code!=="ENOENT")throw error;finiteNames=[]}
      for(const name of finiteNames){
        checkResourceLeaseControl(control);await control.yieldContinuation();if(!/^[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}\.json$/u.test(name))continue;
        try{
          const file=join(finiteDirectory,name),status=await lstat(file);assert.ok(status.isFile()&&!status.isSymbolicLink()&&status.size<=65536);const bytes=await readFile(file,"utf8"),witness=JSON.parse(bytes);
          if(!validateFinite(witness)||name!==witness.nonce+".json"||witness.processId===ownedPid||witness.identity!==toolingIdentity||witness.toolkitRoot!==toolkitRoot)continue;
          const marker=join(toolkitRoot,".semio-nx-tooling.json"),markerStatus=await lstat(marker);assert.ok(markerStatus.isFile()&&!markerStatus.isSymbolicLink());assert.equal(JSON.parse(await readFile(marker,"utf8")).digest,toolingIdentity);process.kill(witness.processId,0);
          const remaining=checkResourceLeaseControl(control),timeout=Math.min(1000,remaining??1000);
          const reply=await new Promise<string>((accept,reject)=>{const socket=createConnection({...witness.endpoint,signal:control.signal});let response="",received=0,settled=false;const decoder=new TextDecoder("utf-8",{fatal:true});const finish=(error?:Error)=>{if(settled)return;settled=true;socket.destroy();if(error)reject(error);else accept(response)};socket.setTimeout(timeout,()=>finish(Error("finite toolkit challenge timed out")));socket.once("error",finish);socket.once("connect",()=>socket.write(witness.nonce+"\n"));socket.on("data",part=>{received+=part.length;if(received>65536){finish(Error("finite toolkit challenge response exceeded original bound"));return}try{response+=decoder.decode(part,{stream:true})}catch(error){finish(error as Error)}});socket.once("end",()=>{try{response+=decoder.decode();finish()}catch(error){finish(error as Error)}});socket.once("close",()=>{if(!settled)finish(Error("finite toolkit challenge ended without its owned reply"))})});
          checkResourceLeaseControl(control);if(!reply)continue;const actual=JSON.parse(reply);assert.equal(validateFinite(actual),true);assert.deepEqual(actual,witness);assert.equal(await readFile(file,"utf8"),bytes);process.kill(witness.processId,0);
          const {Database}=await import("bun:sqlite"),database=new Database(resourceLeasePathV1(join(process.env.NODE_PATH!,"../../../../agents/resource-leases"),fixture.toolingLifetime.resourcePrefix+toolingIdentity),{readwrite:true,create:false});try{database.exec("PRAGMA busy_timeout=0");assert.throws(()=>database.exec("BEGIN EXCLUSIVE"),/locked|busy/iu)}finally{database.close()}
          protectedOwners.push(witness.processId);
        }catch(error){if(["ENOENT","ESRCH","ECONNREFUSED","ECONNRESET"].includes((error as{code?:string}).code??""))continue;throw error}
      }
      assert.equal(fixture.toolingLifetime.otherLiveOwnerExclusive, "refused");
      assert.ok(protectedOwners.length, "Only an actual live immutable journal owner or challenged original finite SDK owner may preserve toolkit protection after the exact fixture PID exited");
    }
  }
  const socketPath = require("nx/src/daemon/socket-utils").getFullOsSocketPath();
  if (process.platform !== "win32") await unlink(socketPath).catch(error => { if (error.code !== "ENOENT") throw error; });
  let replaceGeneration = false, delayedReply = false;
  const sockets = new Set<Socket>(), slow = createServer(socket => {
    sockets.add(socket); socket.once("close", () => sockets.delete(socket));
    socket.on("data", () => { if(delayedReply){delayedReply=false;setTimeout(()=>require("nx/src/daemon/socket-utils").sendMessage(socket,oracleResponse,"json"),fixture.deadline.workMilliseconds);return;} if (!replaceGeneration) return; replaceGeneration = false; setTimeout(() => { void (async () => { const file = join(process.env.NX_WORKSPACE_DATA_DIRECTORY!, "semio-journal/service.json"), metadata = JSON.parse(await readFile(file, "utf8")); await writeFile(file, JSON.stringify({ ...metadata, generation: "b".repeat(32) })); require("nx/src/daemon/socket-utils").sendMessage(socket, oracleResponse, "json"); })(); }, 50); });
  });
  await new Promise<void>((accept, reject) => { slow.once("error", reject); slow.listen(socketPath, accept); });
  const identity = (await import("../../../🕸️graph/🔏️seal/📥️membership/📥️journal/🌐️rpc/🟦️.ts")).loadedFilesystemJournalToolingIdentityV1(dirname(require.resolve("nx/src/daemon/client/client")));
  await mkdir(dirname(cache.serverProcessJsonPath), { recursive: true });
  await writeFile(cache.serverProcessJsonPath, JSON.stringify({ processId: process.pid, socketPath, nxVersion: require("nx/package.json").version, semioToolingIdentity: identity }));
  const serviceDirectory = join(process.env.NX_WORKSPACE_DATA_DIRECTORY!, "semio-journal"); await mkdir(serviceDirectory, { recursive: true });
  await writeFile(join(serviceDirectory, "service.json"), JSON.stringify({ version: 1, root: daemonRoot, processId: process.pid, generation: "a".repeat(32), socketPath, toolingIdentity: identity, graphIdentity:null,implementationIdentity:"d".repeat(64) }));
  try {
    replaceGeneration = true;
    assert.equal(await client.getSemioFilesystemJournalV1(scope, control), null, "An old in-flight observer response cannot authorize a replacement generation");
    delayedReply=true;const started=performance.now();
    const delayed=await client.getSemioFilesystemJournalV1(scope,scriptCompilerAdmissionControlV1(invocation,"nx:rpc-original-delayed",fixture.deadline.deadlineUnixMilliseconds-fixture.deadline.now));
    assert.deepEqual(delayed,oracleResponse,"An authentic response after 1 second remains admitted by its original longer deadline");
    assert.ok(performance.now()-started>=fixture.deadline.workMilliseconds-50);
    await assert.rejects(client.getSemioFilesystemJournalV1(scope, scriptCompilerAdmissionControlV1(invocation, "nx:rpc-deadline", 100)), /deadline/);
    const cancelled = new AbortController(), probing = client.getSemioFilesystemJournalV1(scope, scriptCompilerAdmissionControlV1(invocation, "nx:rpc-mid-cancel", 5000, cancelled.signal));
    setTimeout(() => cancelled.abort(Error("RPC active caller cancelled")), 100);
    await assert.rejects(probing, /RPC active caller cancelled/);
    await new Promise(resolve => setTimeout(resolve, 50));
    assert.equal(sockets.size, 0, "Only dedicated request sockets must close after timeout/cancellation");
    assert.equal(slow.listening, true, "Request cancellation preserves service lifetime");
    invocation.control.signal.throwIfAborted();
  } finally { for (const socket of sockets) socket.destroy(); await new Promise<void>(accept => slow.close(() => accept())); }
}
