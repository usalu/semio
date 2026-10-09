import assert from "node:assert/strict";
import { createHash, randomBytes } from "node:crypto";
import { copyFileSync, linkSync, mkdtempSync, readFileSync, readdirSync, rmSync, statSync, symlinkSync, writeFileSync } from "node:fs";

import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

/** 🔒️ One side of a lease race, exactly as `🧬️schema/🔣️.json` declares it. */
type ResourceLeaseParticipant = { readonly runtime: "bun" | "node" | "python"; readonly mode: "shared" | "exclusive"; readonly resource: string };

/** 🔒️ The lease race corpus (`🧫️fixtures/🔣️.json`). */
type ResourceLeaseFixture = {
  readonly repetitions: number;
  readonly cases: readonly { readonly name: string; readonly first: ResourceLeaseParticipant; readonly second: ResourceLeaseParticipant; readonly action: "parallel" | "release-first" | "cancel-second" | "kill-first" }[];
};

/** 🔒️ One spawned participant plus the output the race is decided on. */
type ResourceLeaseChild = { process: Bun.Subprocess<"pipe", "pipe", "pipe">; output: string; errors: string; done: Promise<number> };

/** 🔒️ Compares resource ownership against independent Node, Bun and Python SQLite processes. */
export async function testResourceLeases(generated: string): Promise<void> {
  const moduleRoot = join(dirname(fileURLToPath(import.meta.url)), "../.."), implementation = join(moduleRoot, "🟦️.ts");
  const fixture: ResourceLeaseFixture = JSON.parse(readFileSync(join(moduleRoot, "🧫️fixtures/🔣️.json"), "utf8"));
  const api = await import(pathToFileURL(implementation).href);
  const queueLaw = JSON.parse(readFileSync(join(moduleRoot,"🧫️fixtures/📜️queue.json"),"utf8"));
  const queueSchema = JSON.parse(readFileSync(join(moduleRoot,"🧬️schema/📜️ticket.json"),"utf8"));
  const {default:Ajv}=await import("ajv");const validateQueue=new Ajv({strict:true}).compile(queueSchema);
  const owner=Buffer.from(queueLaw.ownerBytes).toString("base64url");
  const arrivals=queueLaw.arrivals.map((arrivedAtMs:number)=>{const request={arrivedAtMs,pid:queueLaw.pid,owner};assert(validateQueue(request));const actual=api.resourceQueueTicketV1(request);const oracle=[BigInt(arrivedAtMs).toString(16).padStart(12,"0"),String(queueLaw.pid).padStart(10,"0"),owner].join("-");assert.equal(actual,oracle);return actual;});assert.deepEqual([...arrivals].sort(),arrivals);assert.deepEqual(Buffer.from(owner,"base64url"),Buffer.from(queueLaw.ownerBytes));
  const root = mkdtempSync(join(generated, "resource-leases-")), directory = join(root, "store");
  const worker = join(root, "📜️script.ts");
  const readLaw = JSON.parse(readFileSync(join(moduleRoot, "🧫️fixtures/🔁️interrupted-read.json"), "utf8"));
  const readRoot = join(root, "interrupted-read");
  const { mkdirSync } = await import("node:fs");
  mkdirSync(readRoot);
  for (const file of readLaw.files) writeFileSync(join(readRoot, file), "");
  const { default: glob } = await import("fast-glob");
  const oracle = glob.sync("*", { cwd: readRoot, onlyFiles: true }).sort();
  let readCalls = 0;
  const read = await api.retryInterruptedFilesystemRead(new AbortController().signal, () => {
    if (readCalls++ < readLaw.interruptions) throw Object.assign(new Error("interrupted read"), { code: "EINTR" });
    return readdirSync(readRoot).sort();
  });
  assert.deepEqual(read, readLaw.files);
  assert.deepEqual(read, oracle);
  assert.equal(readCalls, readLaw.interruptions + 1);
  const readAbort = new AbortController();
  let cancelCalls = 0;
  await assert.rejects(() => api.retryInterruptedFilesystemRead(readAbort.signal, () => {
    if (++cancelCalls === readLaw.cancellationAfterInterruptions) readAbort.abort();
    throw Object.assign(new Error("interrupted read"), { code: "EINTR" });
  }), /abort/i);
  assert.equal(cancelCalls, readLaw.cancellationAfterInterruptions);
  const refused = Object.assign(new Error("denied read"), { code: readLaw.unrecoverableCode });
  await assert.rejects(() => api.retryInterruptedFilesystemRead(new AbortController().signal, () => { throw refused; }), error => error === refused);
  console.log("[DEBUG] interrupted filesystem read preserves exact fast-glob oracle output and cancellation");
  writeFileSync(worker, `import { acquireResourceLease } from ${JSON.stringify(implementation)};
const [directory, resource, mode] = process.argv.slice(2), controller = new AbortController();
const timer = setTimeout(() => controller.abort(), 15000);
process.once("SIGTERM", () => controller.abort());
let stop; const stopped = new Promise(resolve => { stop = resolve; });
process.stdin.on("data", chunk => { if (String(chunk).trim() === "cancel") controller.abort(); stop(); });
try {
  const lease = await acquireResourceLease({ directory, resource, mode, signal: controller.signal, onWait: () => console.log("waiting") });
  try { console.log("held"); await stopped; }
  finally { lease.release(); console.log("released"); }
} catch (error) { if (controller.signal.aborted) console.log("cancelled"); else throw error; }
finally { clearTimeout(timer); process.stdin.destroy(); }
`);
  const resources = [...new Set(fixture.cases.flatMap(row => [row.first.resource, row.second.resource]))];
  for (const resource of new Set(fixture.cases.filter(row => row.first.runtime === "python").map(row => row.first.resource))) (await api.acquireResourceLease({ directory, resource, mode: "exclusive", signal: new AbortController().signal })).release();
  const python = `import sqlite3,sys
db=sqlite3.connect(sys.argv[1], timeout=0, isolation_level=None)
db.execute("BEGIN EXCLUSIVE" if sys.argv[2] == "exclusive" else "BEGIN")
db.execute("SELECT resource FROM semio_resource_lease").fetchone()
print("held", flush=True)
sys.stdin.readline()
db.rollback()
db.close()
print("released", flush=True)
`;
  const children: ResourceLeaseChild[] = [];
  const start = (row: ResourceLeaseParticipant): ResourceLeaseChild => {
    const database = join(directory, createHash("sha256").update(row.resource).digest("hex") + ".sqlite");
    const args = row.runtime === "python" ? [Bun.which("python3") ?? Bun.which("python") ?? "python3", "-u", "-c", python, database, row.mode] : [row.runtime === "bun" ? process.execPath : "node", worker, directory, row.resource, row.mode];
    const child = { process: Bun.spawn(args, { stdout: "pipe", stderr: "pipe", stdin: "pipe" }), output: "", errors: "", done: undefined as unknown as Promise<number> };
    const drain = async (stream: ReadableStream<Uint8Array>, key: "output" | "errors") => { for await (const chunk of stream) child[key] += Buffer.from(chunk).toString(); };
    child.done = Promise.all([drain(child.process.stdout, "output"), drain(child.process.stderr, "errors"), child.process.exited]).then(rows => rows[2]);
    children.push(child); return child;
  };
  const until = async (child: ResourceLeaseChild, phase: string) => { const deadline = Date.now() + 10000; while (!child.output.split("\n").includes(phase)) { assert.equal(child.process.exitCode, null, child.errors || child.output); assert.ok(Date.now() < deadline, `${phase}: ${child.output} ${child.errors}`); await Bun.sleep(20); } };
  const release = async (child: ResourceLeaseChild) => { child.process.stdin.write("release\n"); child.process.stdin.end(); assert.equal(await child.done, 0, child.errors); };
  try {
    for (const row of fixture.cases) {
      const first = start(row.first); await until(first, "held");
      const second = start(row.second);
      if (row.action === "parallel") { await until(second, "held"); await release(second); await release(first); }
      else {
        await until(second, "waiting"); assert.equal(second.output.includes("held"), false);
        if (row.action === "cancel-second") {
          second.process.stdin.write("cancel\n"); second.process.stdin.end(); assert.equal(await second.done, 0, second.errors); assert.ok(second.output.includes("cancelled"));
          assert.equal(first.process.exitCode, null); await release(first); assert.equal(second.output.includes("held"), false);
        } else {
          if (row.action === "kill-first") { first.process.kill("SIGKILL"); await first.done; }
          else await release(first);
          await until(second, "held"); await release(second);
        }
      }
    }
    const files = readdirSync(directory).sort(), before = files.map(file => statSync(join(directory, file)).size);
    for (let index = 0; index < fixture.repetitions; index++) (await api.acquireResourceLease({ directory, resource: resources[0], mode: "exclusive", signal: new AbortController().signal })).release();
    assert.deepEqual(readdirSync(directory).sort(), files);
    assert.deepEqual(files.map(file => statSync(join(directory, file)).size), before);
    const abort = new AbortController(); abort.abort();
    await assert.rejects(() => api.acquireResourceLease({ directory, resource: "cancelled-before-open", mode: "shared", signal: abort.signal }), /abort/i);
    assert.deepEqual(readdirSync(directory).sort(), files);
    const heldController = new AbortController(), owner = await api.acquireResourceLease({ directory, resource: "deadline", mode: "exclusive", signal: heldController.signal });
    try {
      heldController.abort();
      const progress: unknown[] = [];
      await assert.rejects(() => api.acquireResourceLease({ directory, resource: "deadline", mode: "shared", signal: new AbortController().signal, timeoutMs: 60, onWait: (row: unknown) => progress.push(row) }), /timed out/i);
      assert.ok(progress.length > 0, "Contended acquisition must report waiting");
      await assert.rejects(() => api.acquireResourceLease({ directory, resource: "deadline", mode: "shared", signal: new AbortController().signal, onWait: () => { throw new Error("progress consumer failed"); } }), /progress consumer failed/);
    } finally { owner.release(); owner.release(); }
    (await api.acquireResourceLease({ directory, resource: "deadline", mode: "exclusive", signal: new AbortController().signal })).release();
    let active = 0, maximum = 0, completed = 0;
    const operation = async () => { active++; maximum = Math.max(maximum, active); await Bun.sleep(40); completed++; active--; };
    await Promise.all([false, true].map(reverse => api.withResourceLeases({ directory, signal: new AbortController().signal, onWait: () => {}, resources: (reverse ? ["b", "a"] : ["a", "b"]).map(resource => ({ resource, mode: "exclusive" })) }, operation)));
    assert.equal(maximum, 1); assert.equal(completed, 2);
    await assert.rejects(() => api.withResourceLeases({ directory, signal: new AbortController().signal, resources: [{ resource: "a", mode: "exclusive" }, { resource: "a", mode: "shared" }] }, async () => { throw new Error("consumer failed"); }), /consumer failed/);
    (await api.acquireResourceLease({ directory, resource: "a", mode: "exclusive", signal: new AbortController().signal })).release();
    const databasePath = (resource: string) => join(directory, createHash("sha256").update(resource).digest("hex") + ".sqlite");
    const expectRejected = (resource: string, pattern: RegExp) => assert.rejects(() => api.acquireResourceLease({ directory, resource, mode: "exclusive", signal: new AbortController().signal }), pattern);
    copyFileSync(databasePath(resources[0]), databasePath("wrong-identity"));
    await expectRejected("wrong-identity", /identity mismatch/);
    const { Database } = await import("bun:sqlite");
    const foreign = new Database(databasePath("foreign")); foreign.exec("CREATE TABLE foreign_owner (value)"); foreign.close();
    await expectRejected("foreign", /unowned/i);
    (await api.acquireResourceLease({ directory, resource: "wrong-journal", mode: "exclusive", signal: new AbortController().signal })).release();
    const wal = new Database(databasePath("wrong-journal")); wal.exec("PRAGMA journal_mode=WAL"); wal.close();
    await expectRejected("wrong-journal", /rollback journal/);
    linkSync(databasePath(resources[0]), databasePath("hardlink"));
    try { await expectRejected("hardlink", /invalid.*database/i); }
    finally { rmSync(databasePath("hardlink")); }
    symlinkSync(directory, join(root, "linked-store"), process.platform === "win32" ? "junction" : "dir");
    await assert.rejects(() => api.acquireResourceLease({ directory: join(root, "linked-store"), resource: "linked", mode: "exclusive", signal: new AbortController().signal }), /invalid.*directory/i);
    const queueStore = join(generated, "q"), queueResource="fifo-"+randomBytes(16).toString("base64url"), order: string[] = [], quiet = () => {};
    const holder = await api.acquireQueuedResourceLease({ directory: queueStore, resource: queueResource, mode: "exclusive", owner: "holder", signal: new AbortController().signal, onWait: quiet });
    const queue = join(queueStore, createHash("sha256").update(queueResource).digest("hex") + ".queue");
    writeFileSync(join(queue, "000000000000001-0000999999-crashed"), "999999\n");
    const first = api.acquireQueuedResourceLease({ directory: queueStore, resource: queueResource, mode: "exclusive", owner: "first", signal: new AbortController().signal, onWait: quiet }).then((lease: { release(): void }) => (order.push("first"), lease));
    await Bun.sleep(50);
    const second = api.acquireQueuedResourceLease({ directory: queueStore, resource: queueResource, mode: "exclusive", owner: "second", signal: new AbortController().signal, onWait: quiet }).then((lease: { release(): void }) => (order.push("second"), lease));
    await Bun.sleep(600);
    assert.deepEqual(order, []);
    holder.release();
    const firstLease = await first;
    await Bun.sleep(600);
    assert.deepEqual(order, ["first"]);
    firstLease.release();
    (await second).release();
    assert.deepEqual(order, ["first", "second"]);
    assert.deepEqual(readdirSync(queue), []);
    const cancelled = new AbortController(), blocker = await api.acquireQueuedResourceLease({ directory: queueStore, resource: queueResource, mode: "exclusive", owner: "blocker", signal: new AbortController().signal, onWait: quiet });
    const abandoned = api.acquireQueuedResourceLease({ directory: queueStore, resource: queueResource, mode: "exclusive", owner: "abandoned", signal: cancelled.signal, onWait: quiet });
    await Bun.sleep(50);
    cancelled.abort();
    await assert.rejects(() => abandoned, /abort/i);
    blocker.release();
    assert.deepEqual(readdirSync(queue), []);
    const boundaryResource="path-"+randomBytes(16).toString("base64url"),boundaryQueue=join(queueStore,createHash("sha256").update(boundaryResource).digest("hex")+".queue"),ownerLength=queueLaw.maximumAbsolutePathUtf16-boundaryQueue.length-25;
    assert(ownerLength>0&&ownerLength<=64);
    const boundary=await api.acquireQueuedResourceLease({directory:queueStore,resource:boundaryResource,mode:"exclusive",owner:"x".repeat(ownerLength),signal:new AbortController().signal,onWait:quiet});
    try{const names=readdirSync(boundaryQueue);assert.equal(names.length,1);assert.equal(join(boundaryQueue,names[0]).length,queueLaw.maximumAbsolutePathUtf16);assert.equal(Number(names[0].split("-")[1]),process.pid);}finally{boundary.release();}
    const overResource="over-"+randomBytes(16).toString("base64url"),overQueue=join(queueStore,createHash("sha256").update(overResource).digest("hex")+".queue");
    await assert.rejects(()=>api.acquireQueuedResourceLease({directory:queueStore,resource:overResource,mode:"exclusive",owner:"x".repeat(ownerLength+1),signal:new AbortController().signal,onWait:quiet}),/path/i);assert.equal(readdirSync(queueStore).includes(overQueue.split(/[\\/]/).at(-1)!),false);
    const cargo=await import("../../../📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts"),buildDirectory=join(root,"native"),identity=cargo.cargoBuildLeaseIdentityV1(buildDirectory,[]),nativeSchema=JSON.parse(readFileSync(join(moduleRoot,"../📦️artifacts/🏗️native-build/🔒️lease/🧬️schema/🔣️.json"),"utf8")),nativeResource=JSON.stringify([nativeSchema.definitions.Protocol.const.namespace,identity.buildDirectory,identity.profile]),nativeQueue=join(queueStore,createHash("sha256").update(nativeResource).digest("hex")+".queue");
    const nativeLease=await cargo.acquireCargoBuildLeaseV1({directory:queueStore,buildDirectory,args:[],signal:new AbortController().signal,onWait:quiet});
    let firstNativeOwner="";try{const names=readdirSync(nativeQueue);assert.equal(names.length,1);firstNativeOwner=names[0].split("-").slice(2).join("-");assert.equal(firstNativeOwner.length,22);assert.equal(Buffer.from(firstNativeOwner,"base64url").length,16);assert(join(nativeQueue,names[0]).length<=queueLaw.maximumAbsolutePathUtf16);}finally{nativeLease.release();}
    const secondNativeLease=await cargo.acquireCargoBuildLeaseV1({directory:queueStore,buildDirectory,args:[],signal:new AbortController().signal,onWait:quiet});try{const secondNativeOwner=readdirSync(nativeQueue)[0].split("-").slice(2).join("-");assert.notEqual(secondNativeOwner,firstNativeOwner);assert.equal(Buffer.from(secondNativeOwner,"base64url").length,16);}finally{secondNativeLease.release();}
    const retainedResource="retained-"+randomBytes(16).toString("base64url"),retainedQueue=join(queueStore,createHash("sha256").update(retainedResource).digest("hex")+".queue");mkdirSync(retainedQueue,{recursive:true});
    const retainedName=String(Date.now()-1000).padStart(15,"0")+"-"+String(process.pid).padStart(10,"0")+"-original",retainedPath=join(retainedQueue,retainedName),retainedBytes=String(process.pid)+"\n";writeFileSync(retainedPath,retainedBytes,{flag:"wx"});
    const retainedCancel=new AbortController();let overtook=false;const behindOriginal=api.acquireQueuedResourceLease({directory:queueStore,resource:retainedResource,mode:"exclusive",owner:"new",signal:retainedCancel.signal,onWait:quiet}).then((lease:{release():void})=>{overtook=true;lease.release();});
    try{await Bun.sleep(300);assert.equal(overtook,false);assert.equal(readFileSync(retainedPath,"utf8"),retainedBytes);retainedCancel.abort();await assert.rejects(()=>behindOriginal,/abort/i);assert.deepEqual(readdirSync(retainedQueue),[retainedName]);}finally{retainedCancel.abort();await behindOriginal.catch(()=>{});rmSync(retainedPath);}
    console.log("[DEBUG] Actual queue: fullSHA256=true exactPath256=true overlongBeforeInput=true nativeNonce128bits=true independentBigIntBase64=true arrivalOrder=true cancellation=true");
    console.log("Resource lease queue: arrival order served, a crashed waiter's ticket swept, a cancelled waiter leaves no ticket PASS");
  } finally {
    for (const child of children) if (child.process.exitCode === null) child.process.kill("SIGKILL");
    await Promise.all(children.map(child => child.done));
    rmSync(root, { recursive: true, force: true });
  }
}
