import {testResourceLeaseConcurrencyV1} from "../⚔️concurrency/🟦️.ts";
import assert from "node:assert/strict";
import { createHash, randomBytes } from "node:crypto";
import { closeSync,ftruncateSync,openSync,copyFileSync, linkSync,mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, statSync, symlinkSync, writeFileSync } from "node:fs";

import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

/** 🔒️ One side of a lease race, exactly as `🧬️schema/🔣️.json` declares it. */
type ResourceLeaseParticipant = { readonly runtime: "bun" | "node" | "python"; readonly mode: "shared" | "exclusive"; readonly resource: string;readonly queuedOwner?:string };

/** 🔒️ The lease race corpus (`🧫️fixtures/🔣️.json`). */
type ResourceLeaseFixture = {
  readonly repetitions: number;
  readonly cases: readonly { readonly name: string; readonly first: ResourceLeaseParticipant; readonly second: ResourceLeaseParticipant; readonly action: "parallel" | "release-first" | "cancel-second" | "kill-first" }[];
};

/** 🔒️ One spawned participant plus the output the race is decided on. */
type ResourceLeaseChild = { process: Bun.Subprocess<"pipe", "pipe", "pipe">; output: string; errors: string; done: Promise<number> };

/** 🔒️ Compares resource ownership against independent Node, Bun and Python SQLite processes. */
export async function testResourceLeases(generated:string,control:import("../../🟦️.ts").ResourceLeaseControl,original:import("../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation):Promise<void> {
  await testResourceLeaseConcurrencyV1(generated,original,control);
  await (await import("../♻️retirement/🟦️.ts")).testResourceLeaseRetirementV1(generated,control);
  await testResourceLeaseReadCustodyV1(generated,control);
  const moduleRoot = join(dirname(fileURLToPath(import.meta.url)), "../.."), implementation = join(moduleRoot, "🟦️.ts");
  const fixture: ResourceLeaseFixture = JSON.parse(readFileSync(join(moduleRoot, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
  const api = await import(pathToFileURL(implementation).href);
  const queueLaw = JSON.parse(readFileSync(join(moduleRoot,"🧪️tests/🧫️fixtures/📜️queue.json"),"utf8"));
  const queueSchema = JSON.parse(readFileSync(join(moduleRoot,"🧬️schema/📜️ticket.json"),"utf8"));
  const {default:Ajv}=await import("ajv");const validateQueue=new Ajv({strict:true}).compile(queueSchema);
  const custody=JSON.parse(readFileSync(join(moduleRoot,"🧪️tests/🧫️fixtures/🎟️custody.json"),"utf8")),custodySchema=JSON.parse(readFileSync(join(moduleRoot,"🧬️schema/🎟️custody.json"),"utf8"));assert(new Ajv({strict:true}).compile(custodySchema)({version:custody.version,claimDirectory:custody.claimDirectory,resourcePrefix:custody.resourcePrefix,maximumDatabaseBytes:custody.maximumDatabaseBytes}));for(const row of custody.cases)assert.equal(!row.kernelHeld,row.retired);
  const {Database:CustodyOracle}=await import("bun:sqlite"),custodyOracle=new CustodyOracle(":memory:");try{for(const row of custody.cases){const oracle=custodyOracle.query("SELECT CASE WHEN ?1 THEN 0 ELSE 1 END AS retired").get(Number(row.kernelHeld)) as {retired:number};assert.equal(Boolean(oracle.retired),row.retired);}}finally{custodyOracle.close();}
  const compact=JSON.parse(readFileSync(join(moduleRoot,"🧪️tests/🧫️fixtures/📜️compact.json"),"utf8")),codecOracle=new CustodyOracle(":memory:");try{for(const row of compact.cases){const encode=(value:number,width:number)=>(codecOracle.query("WITH RECURSIVE encoded(n,s) AS(VALUES(?1,'') UNION ALL SELECT CAST(n/36 AS INTEGER),substr('0123456789abcdefghijklmnopqrstuvwxyz',n%36+1,1)||s FROM encoded WHERE n>0) SELECT substr('0000000000'||s,-?2) value FROM encoded WHERE n=0").get(value,width)as{value:string}).value;assert.equal(encode(row.arrivedAtMs,10)+"-"+encode(row.pid,7)+"-"+compact.owner,row.expected);assert.equal(api.resourceQueueTicketV1({...row,owner:compact.owner}),row.expected);}assert.deepEqual([...Buffer.from(compact.owner,"base64url")],compact.ownerBytes);for(const row of compact.rejected)assert.throws(()=>api.resourceQueueTicketV1({...row,owner:compact.owner}));for(const ticket of compact.rejectedTickets)assert.throws(()=>api.resourceQueueClaimV1("/owned","opaque-current-resource",ticket));}finally{codecOracle.close();}
  const owner=Buffer.from(queueLaw.ownerBytes).toString("base64url");
  const arrivals=queueLaw.arrivals.map((arrivedAtMs:number)=>{const request={arrivedAtMs,pid:queueLaw.pid,owner};assert(validateQueue(request));const actual=api.resourceQueueTicketV1(request);const oracle=[BigInt(arrivedAtMs).toString(36).padStart(10,"0"),BigInt(queueLaw.pid).toString(36).padStart(7,"0"),owner].join("-");assert.equal(actual,oracle);return actual;});assert.deepEqual([...arrivals].sort(),arrivals);assert.deepEqual(Buffer.from(owner,"base64url"),Buffer.from(queueLaw.ownerBytes));
  const root = mkdtempSync(join(generated, "resource-leases-")),leaseStore=join(process.cwd(),".cache/b/l/m/data"),leasedRoot=(mkdirSync(leaseStore,{recursive:true}),mkdtempSync(join(leaseStore,"proof-"))), directory = join(leasedRoot, "store");
  const children: ResourceLeaseChild[] = [];
  try {
  const metadataFixture:{version:number;filename:string;capacityBytes:number}=JSON.parse(readFileSync(join(moduleRoot,"📦️metadata/🧪️tests/🔣️.json"),"utf8")),metadataSchema=JSON.parse(readFileSync(join(moduleRoot,"📦️metadata/🧬️schema/🔣️.json"),"utf8"));assert(new Ajv({strict:true}).validate(metadataSchema,metadataFixture));const metadataOracle=new CustodyOracle(":memory:");try{assert.equal((metadataOracle.query("SELECT 4*1024*1024 AS bytes").get() as {bytes:number}).bytes,metadataFixture.capacityBytes);}finally{metadataOracle.close();}
  api.checkResourceLeaseControl(control);
  const retention=JSON.parse(readFileSync(join(moduleRoot,"🧪️tests/🧫️fixtures/🎟️retention.json"),"utf8")),retentionSchema=JSON.parse(readFileSync(join(moduleRoot,"🧬️schema/🎟️retention.json"),"utf8"));assert(new Ajv({strict:true}).validate(retentionSchema,{version:retention.version,resource:retention.resource,maximumDatabaseBytes:retention.maximumDatabaseBytes,maximumClaimsPerTurn:retention.maximumClaimsPerTurn}));const retirementOracle=new CustodyOracle(":memory:");try{for(const row of retention.cases)assert.equal(Boolean((retirementOracle.query("SELECT NOT ?1 AS retired").get(Number(row.kernelHeld)) as {retired:number}).retired),row.retired);}finally{retirementOracle.close();}
  const retentionDirectory=join(leasedRoot,"retention");mkdirSync(retentionDirectory);const held=await api.acquireQueuedResourceLease({directory:retentionDirectory,resource:"real-retained-claim",mode:"shared",owner:randomBytes(16).toString("base64url"),signal:control.signal,control});try{let first=await api.retireResourceQueueClaimsV1(retentionDirectory,control);assert.equal(first.retired,0);assert.equal(first.examined,1);held.release();const settled=await api.retireResourceQueueClaimsV1(retentionDirectory,control);assert.equal(settled.retired,1);assert(settled.allocatedBytes>0);assert.deepEqual(readdirSync(join(retentionDirectory,"claims")),[api.resourceLeasePathV1(join(retentionDirectory,"claims"),"resource-directory-admission:v1").split(/[\\/]/u).at(-1)]);console.log("[DEBUG] Real SQLite SH claim survives catalog EX retirement; released Original EX claim retires exact inode and allocation, no PID or age authority");}finally{held.release();}
  const sparseDirectory=process.platform==="win32"?join(leasedRoot,"sparse"):join(process.cwd(),".cache/b/c/sp-"+randomBytes(4).toString("hex")),metadataImplementation=await import("../../📦️metadata/🟦️.ts");mkdirSync(sparseDirectory);const sparseFile=join(sparseDirectory,metadataFixture.filename),sparse=openSync(sparseFile,"wx");try{ftruncateSync(sparse,metadataFixture.capacityBytes);}finally{closeSync(sparse);}const originalCatalog=await api.acquireResourceLease({directory:sparseDirectory,resource:retention.resource,mode:"exclusive",signal:control.signal,control,maximumDatabaseBytes:retention.maximumDatabaseBytes});try{const extent=statSync(sparseFile);if(process.platform!=="win32"&&extent.blocks*512<metadataFixture.capacityBytes){assert.throws(()=>metadataImplementation.retainResourceQueueMetadataHeadroomV1(sparseDirectory,originalCatalog,control),/Actual allocated metadata headroom required/);console.log("[DEBUG] Actual host sparse metadata fixture",{logicalBytes:extent.size,allocatedBytes:extent.blocks*512});}else{metadataImplementation.retainResourceQueueMetadataHeadroomV1(sparseDirectory,originalCatalog,control);console.log("[DEBUG] Host filesystem allocated the entire logical extent",{logicalBytes:extent.size,allocatedBytes:extent.blocks*512});}}finally{originalCatalog.release();rmSync(sparseDirectory,{recursive:true});}console.log("[DEBUG] Actual regular sparse metadata extent is refused despite matching logical bytes; physical reserve is never inferred from file length");
  const worker = join(root, "📜️script.ts");
  const readLaw = JSON.parse(readFileSync(join(moduleRoot, "🧪️tests/🧫️fixtures/🔁️interrupted-read.json"), "utf8"));
  const readRoot = join(root, "interrupted-read");
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
  writeFileSync(worker, `import { acquireResourceLease,acquireQueuedResourceLease } from ${JSON.stringify(implementation)};
const [directory, resource, mode,queuedOwner] = process.argv.slice(2), controller = new AbortController();
const timer = setTimeout(() => controller.abort(), 15000);
process.once("SIGTERM", () => controller.abort());
let stop; const stopped = new Promise(resolve => { stop = resolve; });
process.stdin.on("data", chunk => { if (String(chunk).trim() === "cancel") controller.abort(); stop(); });
try {
  const lease = await (queuedOwner?acquireQueuedResourceLease:acquireResourceLease)({ directory, resource, mode,owner:queuedOwner, signal: controller.signal, onWait: () => console.log("waiting") });
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
  const start = (row: ResourceLeaseParticipant): ResourceLeaseChild => {
    const database = join(directory, createHash("sha256").update(row.resource).digest("hex") + ".sqlite");
    const args = row.runtime === "python" ? [Bun.which("python3") ?? Bun.which("python") ?? "python3", "-u", "-c", python, database, row.mode] : [row.runtime === "bun" ? process.execPath : "node", worker, directory, row.resource, row.mode,...(row.queuedOwner?[row.queuedOwner]:[])];
    const child = { process: Bun.spawn(args, { stdout: "pipe", stderr: "pipe", stdin: "pipe" }), output: "", errors: "", done: undefined as unknown as Promise<number> };
    const drain = async (stream: ReadableStream<Uint8Array>, key: "output" | "errors") => { for await (const chunk of stream) child[key] += Buffer.from(chunk).toString(); };
    child.done = Promise.all([drain(child.process.stdout, "output"), drain(child.process.stderr, "errors"), child.process.exited]).then(rows => rows[2]);
    children.push(child); return child;
  };
  const until = async (child: ResourceLeaseChild, phase: string) => { const deadline = Date.now() + 10000; while (!child.output.split("\n").includes(phase)) { assert.equal(child.process.exitCode, null, child.errors || child.output); assert.ok(Date.now() < deadline, `${phase}: ${child.output} ${child.errors}`); await Bun.sleep(20); } };
  const release = async (child: ResourceLeaseChild) => { child.process.stdin.write("release\n"); child.process.stdin.end(); assert.equal(await child.done, 0, child.errors); };
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
    copyFileSync(databasePath(resources[0]!), databasePath("wrong-identity"));
    await expectRejected("wrong-identity", /identity mismatch/);
    const { Database } = await import("bun:sqlite");
    const foreign = new Database(databasePath("foreign")); foreign.exec("CREATE TABLE foreign_owner (value)"); foreign.close();
    await expectRejected("foreign", /unowned/i);
    (await api.acquireResourceLease({ directory, resource: "wrong-journal", mode: "exclusive", signal: new AbortController().signal })).release();
    const wal = new Database(databasePath("wrong-journal")); wal.exec("PRAGMA journal_mode=WAL"); wal.close();
    await expectRejected("wrong-journal", /rollback journal/);
    linkSync(databasePath(resources[0]!), databasePath("hardlink"));
    try { await expectRejected("hardlink", /invalid.*database/i); }
    finally { rmSync(databasePath("hardlink")); }
    symlinkSync(directory, join(leasedRoot, "linked-store"), process.platform === "win32" ? "junction" : "dir");
    await assert.rejects(() => api.acquireResourceLease({ directory: join(leasedRoot, "linked-store"), resource: "linked", mode: "exclusive", signal: new AbortController().signal }), /invalid.*directory/i);
    const bootstrapHolder=await api.acquireResourceLease({directory,resource:"bounded-bootstrap-contention",mode:"exclusive",signal:new AbortController().signal,maximumDatabaseBytes:16384});let bootstrapAcquired=false,bootstrapWaited=false;const bootstrapReader=api.acquireResourceLease({directory,resource:"bounded-bootstrap-contention",mode:"shared",signal:new AbortController().signal,maximumDatabaseBytes:16384,onWait:()=>{bootstrapWaited=true;}}).then((lease:{release():void})=>{bootstrapAcquired=true;return lease;});await Bun.sleep(100);assert.equal(bootstrapAcquired,false);assert.equal(bootstrapWaited,true);bootstrapHolder.release();(await bootstrapReader).release();console.log("[DEBUG] Actual capped SQLite bootstrap setup preserves queued BUSY retry before original holder release");
    const queueStore = join(leasedRoot, "q"), queueResource="fifo-"+randomBytes(16).toString("base64url"), order: string[] = [], quiet = () => {};
    const holder = await api.acquireQueuedResourceLease({ directory: queueStore, resource: queueResource, mode: "exclusive", owner: "holder", signal: new AbortController().signal, onWait: quiet });
    const queue = join(queueStore, createHash("sha256").update(queueResource).digest("hex") + ".queue");
    const releasedTicket=api.resourceQueueTicketV1({arrivedAtMs:1,pid:process.pid,owner:"released-original"}),claim=api.resourceQueueClaimV1(queueStore,queueResource,releasedTicket);(await api.acquireResourceLease({...claim,mode:"shared",signal:new AbortController().signal,maximumDatabaseBytes:16384})).release();writeFileSync(join(queue,releasedTicket),JSON.stringify({version:1,pid:process.pid,mode:"exclusive",sharedCohort:false}));
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
    const retainedTicket=api.resourceQueueTicketV1({arrivedAtMs:2,pid:999999999,owner:"retained-original"}),retainedClaim=await api.acquireResourceLease({...api.resourceQueueClaimV1(queueStore,queueResource,retainedTicket),mode:"shared",signal:new AbortController().signal,maximumDatabaseBytes:16384});writeFileSync(join(queue,retainedTicket),JSON.stringify({version:1,pid:999999999,mode:"exclusive",sharedCohort:false}));let received=false;const retainedWaiter=api.acquireQueuedResourceLease({directory:queueStore,resource:queueResource,mode:"exclusive",owner:"after-retained",signal:new AbortController().signal,onWait:quiet}).then((lease:{release():void})=>{received=true;return lease;});await Bun.sleep(300);assert.equal(received,false);assert(readdirSync(queue).includes(retainedTicket));retainedClaim.release();(await retainedWaiter).release();assert.deepEqual(readdirSync(queue),[]);
    const crashResource="queued-crash-"+randomBytes(8).toString("hex"),crashHolder=start({runtime:"bun",mode:"exclusive",resource:crashResource,queuedOwner:"crash-holder"});await until(crashHolder,"held");const crashedWaiter=start({runtime:"node",mode:"exclusive",resource:crashResource,queuedOwner:"crashed-waiter"});await until(crashedWaiter,"waiting");const survivingWaiter=start({runtime:"bun",mode:"exclusive",resource:crashResource,queuedOwner:"surviving-waiter"});await until(survivingWaiter,"waiting");crashedWaiter.process.kill("SIGKILL");await crashedWaiter.done;await release(crashHolder);await until(survivingWaiter,"held");await release(survivingWaiter);console.log("[DEBUG] Actual original SQLite queue custody retained despite dead PID label and Node waiter SIGKILL releases kernel claim before survivor admission");
    const boundaryResource="path-"+randomBytes(16).toString("base64url"),padding=Math.max(1,queueLaw.maximumAbsolutePathUtf16-queueStore.length-70-20-64),boundaryStore=join(queueStore,"x".repeat(padding)),boundaryQueue=join(boundaryStore,createHash("sha256").update(boundaryResource).digest("hex")+".queue"),ownerLength=queueLaw.maximumAbsolutePathUtf16-boundaryQueue.length-20;
    assert(ownerLength>0&&ownerLength<=64);
    const boundary=await api.acquireQueuedResourceLease({directory:boundaryStore,resource:boundaryResource,mode:"exclusive",owner:"x".repeat(ownerLength),signal:new AbortController().signal,onWait:quiet});
    try{const names=readdirSync(boundaryQueue);assert.equal(names.length,1);assert.equal(join(boundaryQueue,names[0]!).length,queueLaw.maximumAbsolutePathUtf16);assert.equal(Number.parseInt(names[0]!.split("-")[1]!,36),process.pid);}finally{boundary.release();}
    const overResource="over-"+randomBytes(16).toString("base64url"),overQueue=join(boundaryStore,createHash("sha256").update(overResource).digest("hex")+".queue");
    await assert.rejects(()=>api.acquireQueuedResourceLease({directory:boundaryStore,resource:overResource,mode:"exclusive",owner:"x".repeat(ownerLength+1),signal:new AbortController().signal,onWait:quiet}),/path/i);assert.equal(readdirSync(boundaryStore).includes(overQueue.split(/[\\/]/).at(-1)!),false);
    const cargo=await import("../../../📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts"),buildDirectory=join(root,"native"),identity=cargo.cargoBuildLeaseIdentityV1(buildDirectory,[]),nativeSchema=JSON.parse(readFileSync(join(moduleRoot,"../📦️artifacts/🏗️native-build/🔒️lease/🧬️schema/🔣️.json"),"utf8")),nativeResource=JSON.stringify([nativeSchema.definitions.Protocol.const.namespace,identity.buildDirectory,identity.profile]),nativeQueue=join(queueStore,createHash("sha256").update(nativeResource).digest("hex")+".queue");
    const nativeLease=await cargo.acquireCargoBuildLeaseV1({directory:queueStore,buildDirectory,targetDirectory:buildDirectory,args:[],signal:new AbortController().signal,onWait:quiet});
    let firstNativeOwner="";try{const names=readdirSync(nativeQueue);assert.equal(names.length,1);firstNativeOwner=names[0]!.split("-").slice(2).join("-");assert.equal(firstNativeOwner.length,22);assert.equal(Buffer.from(firstNativeOwner,"base64url").length,16);assert(join(nativeQueue,names[0]!).length<=queueLaw.maximumAbsolutePathUtf16);}finally{nativeLease.release();}
    const secondNativeLease=await cargo.acquireCargoBuildLeaseV1({directory:queueStore,buildDirectory,targetDirectory:buildDirectory,args:[],signal:new AbortController().signal,onWait:quiet});try{const secondNativeOwner=readdirSync(nativeQueue)[0]!.split("-").slice(2).join("-");assert.notEqual(secondNativeOwner,firstNativeOwner);assert.equal(Buffer.from(secondNativeOwner,"base64url").length,16);}finally{secondNativeLease.release();}
    const retainedResource="retained-"+randomBytes(16).toString("base64url"),retainedQueue=join(queueStore,createHash("sha256").update(retainedResource).digest("hex")+".queue");mkdirSync(retainedQueue,{recursive:true});
    const retainedName=api.resourceQueueTicketV1({arrivedAtMs:Date.now()-1000,pid:process.pid,owner:"original"}),retainedPath=join(retainedQueue,retainedName),retainedBytes=JSON.stringify({version:1,pid:process.pid,mode:"exclusive",sharedCohort:false}),retainedOriginal=await api.acquireResourceLease({...api.resourceQueueClaimV1(queueStore,retainedResource,retainedName),mode:"shared",signal:new AbortController().signal,maximumDatabaseBytes:16384});writeFileSync(retainedPath,retainedBytes,{flag:"wx"});
    const retainedCancel=new AbortController();let overtook=false;const behindOriginal=api.acquireQueuedResourceLease({directory:queueStore,resource:retainedResource,mode:"exclusive",owner:"new",signal:retainedCancel.signal,onWait:quiet}).then((lease:{release():void})=>{overtook=true;lease.release();});
    try{await Bun.sleep(300);assert.equal(overtook,false);assert.equal(readFileSync(retainedPath,"utf8"),retainedBytes);retainedCancel.abort();await assert.rejects(()=>behindOriginal,/abort/i);assert.deepEqual(readdirSync(retainedQueue),[retainedName]);}finally{retainedCancel.abort();await behindOriginal.catch(()=>{});rmSync(retainedPath);retainedOriginal.release();}
    console.log("[DEBUG] Actual queue: fullSHA256=true exactPath256=true overlongBeforeInput=true nativeNonce128bits=true independentBigIntBase64=true arrivalOrder=true cancellation=true");
    console.log("Resource lease queue: arrival order served, a crashed waiter's ticket swept, a cancelled waiter leaves no ticket PASS");
  } finally {
    for (const child of children) if (child.process.exitCode === null) child.process.kill("SIGKILL");
    await Promise.all(children.map(child => child.done));
    rmSync(root, { recursive: true, force: true });rmSync(leasedRoot,{recursive:true,force:true});
  }
}


/** 🧷️ Matches issued shared custody with an independent actual SQLite reader transaction. */
export async function testResourceLeaseReadCustodyV1(generated:string,control:import("../../🟦️.ts").ResourceLeaseControl):Promise<void>{
 const api=await import("../../🟦️.ts"),{requestedCompilerAdmissionControlV1}=await import("../../../🏗️build/🎟️capacity/🟦️.ts"),{Database}=await import("bun:sqlite"),directory=mkdtempSync(join(generated,"lr-")),resource="reader-custody:"+process.pid,fixture=JSON.parse(readFileSync(new URL("../🧫️fixtures/🎟️reader.json",import.meta.url),"utf8")),verify=Reflect.get(api,"requireResourceLeaseReadCustodyV1");assert.equal(typeof verify,"function","Genuine shared-reader custody API required");mkdirSync(directory,{recursive:true});const path=api.resourceLeasePathV1(directory,resource),reader=await api.acquireResourceLease({directory,resource,mode:"shared",signal:control.signal,control}),oracle=new Database(path);let released=false;
 try{oracle.exec("pragma busy_timeout=0");assert.throws(()=>oracle.exec("begin exclusive"));for(const row of fixture.cases){if(row.name==="shared"){verify(reader,directory,resource,control);assert.equal(row.accepted,true);}else if(row.name==="forged")assert.throws(()=>verify({...reader},directory,resource,control));else if(row.name==="wrong-directory")assert.throws(()=>verify(reader,join(directory,"other"),resource,control));else if(row.name==="wrong-resource")assert.throws(()=>verify(reader,directory,resource+"other",control));else if(row.name==="exclusive"||row.name==="mutated-exclusive"){const writer=await api.acquireResourceLease({directory,resource:resource+"writer",mode:"exclusive",signal:control.signal,control});try{if(row.name==="mutated-exclusive")(writer as any).mode="shared";assert.throws(()=>verify(writer,directory,resource+"writer",control));(writer as any).mode="exclusive";api.requireResourceLeaseCustodyV1(writer,directory,resource+"writer",control);}finally{writer.release();}}else if(row.name==="released"){reader.release();released=true;assert.throws(()=>verify(reader,directory,resource,control));oracle.exec("begin exclusive;rollback");}else if(row.name==="expired"){const held=await api.acquireResourceLease({directory,resource:resource+"expired",mode:"shared",signal:control.signal,control});try{const bounded=requestedCompilerAdmissionControlV1(control,1);await new Promise(resolve=>setTimeout(resolve,5));assert.throws(()=>verify(held,directory,resource+"expired",bounded));verify(held,directory,resource+"expired",control);}finally{held.release();}}else assert.fail("Unknown neutral custody vector");if(row.name!=="shared")assert.equal(row.accepted,false);}console.log("[DEBUG] Genuine shared custody, independent SQLite exclusion and eight neutral reader authority vectors agree");}finally{if(!released)reader.release();oracle.close();rmSync(directory,{recursive:true,force:true});}
}
