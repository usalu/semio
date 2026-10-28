/** 🚏️ Nx daemon admission laws over `./🧫️fixtures/🔣️.json`: limits, admission decisions and the idle rule equal an independent Python model (`./🐍️.py`), the Decision schema is validated by Ajv, and with the real lease library daemon slots enforce the cap (a free slot is taken, a full family refuses, a released slot is reusable, a live namespace daemon is detected). */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import Ajv from "ajv";
import {NX_DAEMON_SLOTS_EXHAUSTED_KEY_V1,acquireNxDaemonSlotV1,countNxDaemonSlotsV1,decideNxDaemonV1,nxDaemonIdleV1,nxDaemonLimitsV1,nxDaemonResidentDirectoryV1,nxDaemonResourceHeldV1,nxDaemonRetireV1,nxDaemonSlotResourceV1,nxDaemonYieldRequestedV1,nxDaemonYieldV1,readNxDaemonResidentV1,removeNxDaemonResidentV1,writeNxDaemonResidentV1} from "../🟦️.ts";
import {nxGraphYieldResourceV1} from "../../../../💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts";
import {acquireResourceLease} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

const fixturePath=resolve(import.meta.dir,"./🧫️fixtures/🔣️.json"),fixture=JSON.parse(readFileSync(fixturePath,"utf8")) as any;
const schema=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8")),validate=new Ajv({strict:false}).compile({...schema.definitions.Decision});
const artifacts=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR||tmpdir());
const model=(()=>{const run=spawnSync(testPythonV1()??"python3",["-I",resolve(import.meta.dir,"./🐍️.py"),fixturePath],{encoding:"utf8"});expect(run.status).toBe(0);return JSON.parse(run.stdout);})();
const signal=new AbortController().signal;

fixture.limits.forEach((row:any,index:number)=>test("limits: "+row.name,()=>{
 const found=nxDaemonLimitsV1(row.environment,row.memoryGiB*fixture.GiB);
 const seen={finite:found.finite,watch:found.watch,idleMilliseconds:found.idleMilliseconds,drainMilliseconds:found.drainMilliseconds,residentMegabytes:found.residentBytes/1048576};expect(seen).toEqual(row.expected);expect(seen).toEqual(model.limits[index]);
}));
fixture.decisions.forEach((row:any,index:number)=>test("decision: "+row.name,()=>{
 const found=decideNxDaemonV1(row.input);expect(found).toEqual(row.expected);expect(found).toEqual(model.decisions[index]);expect(validate(found)).toBe(true);
}));
fixture.idle.forEach((row:any,index:number)=>test("idle: "+row.name,()=>{expect(nxDaemonIdleV1(row.state)).toBe(row.expected);expect(row.expected).toBe(model.idle[index]);}));

fixture.retire.forEach((row:any,index:number)=>test("retire: "+row.name,()=>{expect(nxDaemonRetireV1(row.state)).toBe(row.expected);expect(row.expected).toBe(model.retire[index]);}));

test("the resident-memory census of a checkout equals the independent model: dead, stale, misnamed and schema-invalid records are not counted",async()=>{
 const root=mkdtempSync(join(artifacts,"nx-daemon-resident-")),leases=join(root,"lease"),directory=nxDaemonResidentDirectoryV1(leases),section=fixture.resident;
 try{
  mkdirSync(directory,{recursive:true});
  for(const row of section.records)writeFileSync(join(directory,row.file),JSON.stringify({version:1,pid:row.pid,kind:row.kind,rssBytes:row.rssBytes,atMs:row.atMs}));
  writeFileSync(join(directory,"700.json"),"{broken");writeFileSync(join(directory,"note.txt"),"x");
  const alive=new Map<number,boolean>(section.records.map((row:any)=>[row.pid,row.alive])),found=await readNxDaemonResidentV1(leases,section.now,pid=>alive.get(pid)??false);
  expect(found.daemons.map(row=>row.pid)).toEqual(section.expectedPids);expect(found.totalBytes).toBe(section.expectedTotalBytes);
  expect({pids:found.daemons.map(row=>row.pid),totalBytes:found.totalBytes}).toEqual({pids:model.resident.pids,totalBytes:model.resident.totalBytes});
  expect((await readNxDaemonResidentV1(join(root,"nothing"),section.now)).totalBytes).toBe(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a daemon publishes its resident memory atomically, the writer clears records of dead daemons, and the record is withdrawn at the end",async()=>{
 const root=mkdtempSync(join(artifacts,"nx-daemon-resident-write-")),leases=join(root,"lease"),directory=nxDaemonResidentDirectoryV1(leases);
 try{
  await writeNxDaemonResidentV1(leases,{version:1,pid:4242,kind:"finite",rssBytes:111,atMs:1000},()=>true);
  await writeNxDaemonResidentV1(leases,{version:1,pid:4243,kind:"watch",rssBytes:222,atMs:1000},pid=>pid===4243);
  expect(readdirSync(directory).sort()).toEqual(["4243.json"]);
  await writeNxDaemonResidentV1(leases,{version:1,pid:4243,kind:"watch",rssBytes:333,atMs:2000},pid=>pid===4243);
  expect((await readNxDaemonResidentV1(leases,2000,()=>true)).totalBytes).toBe(333);expect(readdirSync(directory).some(name=>name.endsWith(".tmp"))).toBe(false);
  await removeNxDaemonResidentV1(leases,4243);expect(existsSync(join(directory,"4243.json"))).toBe(false);expect((await readNxDaemonResidentV1(leases,2000,()=>true)).totalBytes).toBe(0);
  await removeNxDaemonResidentV1(leases,4243);
 }finally{rmSync(root,{recursive:true,force:true});}
});

fixture.yield.forEach((row:any,index:number)=>test("yield: "+row.name,()=>{expect(nxDaemonYieldV1(row.state)).toBe(row.expected);expect(row.expected).toBe(model.yield[index]);}));

test("an eviction's yield request is seen by a daemon probe of that namespace only, and probes never exclude each other",async()=>{
 const root=mkdtempSync(join(artifacts,"nx-daemon-yield-")),leases=join(root,"lease"),namespace=join(root,"graphs","a","b"),other=join(root,"graphs","a","c");
 try{
  expect(await nxDaemonYieldRequestedV1(leases,namespace,signal)).toBe(false);
  const request=await acquireResourceLease({directory:leases,resource:nxGraphYieldResourceV1(namespace),mode:"exclusive",signal});
  expect(await nxDaemonYieldRequestedV1(leases,namespace,signal)).toBe(true);expect(await nxDaemonYieldRequestedV1(leases,other,signal)).toBe(false);
  request.release();expect(await nxDaemonYieldRequestedV1(leases,namespace,signal)).toBe(false);
  const probes=await Promise.all([nxDaemonYieldRequestedV1(leases,namespace,signal),nxDaemonYieldRequestedV1(leases,namespace,signal),nxDaemonYieldRequestedV1(leases,namespace,signal)]);expect(probes).toEqual([false,false,false]);
  expect(nxGraphYieldResourceV1(namespace)).not.toBe(nxGraphYieldResourceV1(other));expect(nxGraphYieldResourceV1(namespace)).toMatch(/^nx:graph-yield:[a-f0-9]{64}$/u);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("slots enforce the cap with the real lease library",async()=>{
 const root=mkdtempSync(join(artifacts,"nx-daemon-slots-")),leases=join(root,"lease"),image=join(root,"checkout");
 try{
  expect(await countNxDaemonSlotsV1(leases,image,"finite",signal)).toBe(0);
  const first=await acquireNxDaemonSlotV1(leases,image,"finite",2,signal),second=await acquireNxDaemonSlotV1(leases,image,"finite",2,signal);
  expect(await countNxDaemonSlotsV1(leases,image,"finite",signal)).toBe(2);expect(await countNxDaemonSlotsV1(leases,image,"watch",signal)).toBe(0);
  await expect(acquireNxDaemonSlotV1(leases,image,"finite",2,signal)).rejects.toThrow(NX_DAEMON_SLOTS_EXHAUSTED_KEY_V1);
  first.release();expect(await countNxDaemonSlotsV1(leases,image,"finite",signal)).toBe(1);
  const again=await acquireNxDaemonSlotV1(leases,image,"finite",2,signal);again.release();second.release();
  expect(await countNxDaemonSlotsV1(leases,image,"finite",signal)).toBe(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a held namespace resource is seen as a live daemon and a free one is not",async()=>{
 const root=mkdtempSync(join(artifacts,"nx-daemon-held-")),leases=join(root,"lease"),resource=nxDaemonSlotResourceV1(root,"finite",0);
 try{
  expect(await nxDaemonResourceHeldV1(leases,resource,signal)).toBe(false);
  const lease=await acquireResourceLease({directory:leases,resource,mode:"exclusive",signal});
  expect(await nxDaemonResourceHeldV1(leases,resource,signal)).toBe(true);lease.release();
  expect(await nxDaemonResourceHeldV1(leases,resource,signal)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

const service=readFileSync(resolve(import.meta.dir,"../../🚦️service/🟦️.ts"),"utf8"),script=readFileSync(resolve(import.meta.dir,"../../../../🚀️bootstrap/📜️script.ts"),"utf8");
test("source: the daemon takes its slot before its service lifetime and gives it back last",()=>{
 const start=service.indexOf("acquireNxDaemonSlotV1(repositoryLeaseDirectoryV1(birth.repositoryRoot)");
 expect(start).toBeGreaterThan(-1);expect(start).toBeLessThan(service.indexOf("let release!:()=>void;"));
 expect(service).toContain("finally{release();slot.release();}");
});
test("source: finite daemons idle out through the pure rule and the timer is cleared with the service",()=>{
 expect(service).toContain("nxDaemonIdleV1({kind,openRequests:activity.open+attachedClients(),lastActivity:activity.last");expect(service).toContain("otherLeaseHoldersV1(repositoryLeaseDirectoryV1(birth.repositoryRoot),nxGraphNamespaceResourceV1(birth.directory))");
 expect(service).toContain("idleTimer.unref()");expect(service).toContain("finally{clearInterval(idleTimer);");
 expect(service).toContain("activity.open++;activity.last=Date.now();");expect(service).toContain("activity.open--;activity.last=Date.now();");
});
test("source: the daemon publishes its resident memory on every tick, retires once idle over its budget and withdraws the record with the service",()=>{
 expect(service).toContain("writeNxDaemonResidentV1(leaseDirectory,{version:1,pid:process.pid,kind,rssBytes,atMs:Date.now()})");
 expect(service).toContain("nxDaemonRetireV1({kind,openRequests:activity.open+attachedClients(),residentBytes,limitBytes:limits.residentBytes})");
 expect(service).toContain("removeNxDaemonResidentV1(leaseDirectory,process.pid)");expect(service.indexOf("removeNxDaemonResidentV1(leaseDirectory")).toBeGreaterThan(service.indexOf("finally{clearInterval(idleTimer);"));
});
test("source: a finite daemon yields to an eviction only while idle and ends within its drain deadline",()=>{
 expect(service).toContain("nxDaemonYieldRequestedV1(repositoryLeaseDirectoryV1(birth.repositoryRoot),birth.directory,original.control.signal)");
 expect(service).toContain("nxDaemonYieldV1({kind,openRequests:activity.open,requested})");expect(service).toContain("yieldTimer.unref()");
 expect(service).toContain("watchdog??=setTimeout(");expect(service).toContain("limits.drainMilliseconds");expect(service).toContain("finally{clearInterval(idleTimer);clearInterval(yieldTimer);clearTimeout(watchdog);");
});
test("source: a client whose daemon found every slot taken runs once more without a daemon (a daemon start precedes every task, so no executing check)",()=>{
 expect(script).toContain("state.slotsTaken ||= state.tail.includes(NX_DAEMON_SLOTS_EXHAUSTED_KEY_V1)");
 expect(script).toContain('env.NX_DAEMON === "true" && graphChanges.get(completed)?.slotsTaken)');expect(script).toContain('env.NX_DAEMON = "false"');
});
test("source: a run asks for admission once, after its namespace is bound and never for watch or explicit settings",()=>{
 const bind=script.indexOf("await bind(toolingIdentity,env.SEMIO_BUILD_GRAPH_DIGEST??null,implementationIdentity);"),admission=script.indexOf("nxDaemonAdmissionV1(this.root,env,actualPhase.signal)");
 expect(bind).toBeGreaterThan(-1);expect(admission).toBeGreaterThan(bind);
 expect(script).toContain('if(env.NX_DAEMON===undefined&&!invocation.watch&&invocation.args[0]!=="watch")');
});
