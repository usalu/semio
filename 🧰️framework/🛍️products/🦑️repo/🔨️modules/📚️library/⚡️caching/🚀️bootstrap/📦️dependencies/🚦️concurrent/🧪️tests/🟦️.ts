import {test,expect} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv from "ajv";
import {spawn,spawnSync} from "node:child_process";
import {existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,renameSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {fileURLToPath} from "node:url";
import {join} from "node:path";
import {acquireResourceLease,type ResourceLease} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import * as core from "../🟦️.ts";
import * as glue from "../💾️storage/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import {javascriptInstallSchemaV1 as installSchema} from "../../💾️storage/🧱️schemas/🟦️.ts";
import policy from "../../../../💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import {readRepositoryCacheBudgetV1} from "../../../../💾️storage/🔒️enforcement/📦️budget/🟦️.ts";

const protocol=core.concurrentProtocolV1,here=(name:string)=>fileURLToPath(new URL(name,import.meta.url));
const oracle=(command:string,cases:unknown):any=>{const run=spawnSync(testPythonV1()??"python3",["-I",here("./🐍️.py"),command],{encoding:"utf8",input:JSON.stringify(cases),maxBuffer:1<<26});if(run.status!==0)throw Error("oracle failed: "+run.stderr);return JSON.parse(run.stdout);};
const scratch=()=>realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"conc-")));
const sleep=(ms:number)=>new Promise<void>(accept=>setTimeout(accept,ms));
const signal=new AbortController().signal,generation=(letter:string)=>letter.repeat(64);
const source=(relative:string)=>readFileSync(new URL(relative,import.meta.url),"utf8");
const storage=()=>source("../../💾️storage/🟦️.ts"),bootstrap=()=>source("../../../../../🏗️build/🚀️bootstrap/🟦️.ts");
const slice=(text:string,start:string,end:string)=>{const first=text.indexOf(start);expect(first,start).toBeGreaterThanOrEqual(0);const last=text.indexOf(end,first+start.length);expect(last,end).toBeGreaterThan(first);return text.slice(first,last);};

/** 🧬️ Pins the neutral contract: a third-party validator accepts every sealed pointer and event, both languages say the same things and the lease names stay the ones readers already hold. */
test("concurrent contract: Ajv accepts sealed pointers and events, constants equal the fixture and both languages carry the same messages",()=>{
 const ajv=new Ajv({strict:false}).addSchema(schema),validate=(name:string,value:unknown)=>{const check=ajv.getSchema(schema.$id+"#/$defs/"+name)!;expect(check(JSON.parse(JSON.stringify(value))),name+JSON.stringify(check.errors)).toBe(true);};
 const first=core.sealConcurrentPointerV1(null,"🏢️semio-tech",generation("1"),1000,1500000000),second=core.sealConcurrentPointerV1(first,"🏢️semio-tech",generation("2"),2000,0);
 validate("Pointer",first);validate("Pointer",second);
 for(const event of protocol.events)validate("Event",core.concurrentEventV1({event,resource:"build",count:1,epoch:1,neededBytes:2,freeBytes:1}));
 expect(()=>core.concurrentEventV1({event:"waiting",resource:"pin" as never})).toThrow();
 expect(protocol.resources).toEqual(fixture.resources);
 expect(source("../../🧾️generation/💾️storage/🟦️.ts")).toContain('"'+protocol.resources.writer+'"');
 expect(protocol.languages).toEqual(["en","de"]);
 const texts=schema.$defs.Texts.const as Record<string,Record<string,string>>,placeholders=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(row=>row[1]).sort();
 expect(Object.keys(texts).sort()).toEqual([...protocol.languages].sort());
 for(const name of Object.keys(texts.en!)){expect(Object.keys(texts.de!)).toContain(name);expect(placeholders(texts.de![name]!),name).toEqual(placeholders(texts.en![name]!));expect(texts.de![name]).not.toBe(texts.en![name]);}
 expect(Object.keys(texts.de!).length).toBe(Object.keys(texts.en!).length);
 expect(core.concurrentTextV1("de","published",{scope:"X",epoch:3})).toBe("X wurde auf die neue Abhängigkeits-Generation umgeschaltet (Epoche 3).");
 expect(core.concurrentTextV1("en","published",{scope:"X",epoch:3})).toBe("Switched X to its new dependency generation (epoch 3).");
 expect(()=>core.concurrentTextV1("fr","published")).toThrow("Declared language");
 expect(protocol.wait.maximumPublishMilliseconds).toBeLessThan(installSchema.$defs.SourceOwnerStoreSyncProtocol.const.maximumQueueWaitMilliseconds);
 expect(protocol.retention.rollbackMilliseconds).toBeGreaterThan(protocol.retention.graceMilliseconds);
});

/** 🐍️ Compares every pure decision with the independent Python oracle: pointer evolution, retention, wait scheduling and language choice. */
test("concurrent oracle: pointer chains, retention decisions, wait schedule and language agree with the Python oracle",()=>{
 const expectedPointers=oracle("pointer",fixture.pointer);
 for(const [index,row] of fixture.pointer.entries()){
  let state:core.ConcurrentPointerV1|null=null;const actual:core.ConcurrentPointerV1[]=[];
  for(const step of row.steps){state=core.sealConcurrentPointerV1(state,row.scope,step.generation,step.nowMs,step.bytes);actual.push(state);}
  expect(JSON.parse(JSON.stringify(actual)),row.name).toEqual(expectedPointers[index].pointers);
  expect(core.concurrentPointerNameV1(row.scope),row.name).toBe(expectedPointers[index].file);
  expect(core.parseConcurrentPointerV1(core.encodeConcurrentPointerV1(state!)),row.name).toEqual(state!);
 }
 const overflow=oracle("pointer",fixture.pointer).at(-1).pointers.at(-1);
 expect(overflow.retired.length).toBe(protocol.pointer.maximumRetired);
 expect(overflow.epoch).toBe(12);
 const expectedCollect=oracle("collect",fixture.collect);
 for(const [index,row] of fixture.collect.entries()){
  const pointers=row.pointers.map((pointer,at)=>({...core.sealConcurrentPointerV1(null,"scope-"+at,generation("f"),0,0),retired:pointer.retired})) as unknown as core.ConcurrentPointerV1[];
  const actual=core.concurrentCollectionV1({rows:row.rows,pointers,nowMs:row.nowMs,freeFraction:row.freeFraction,freeBytes:(row as {freeBytes?:number}).freeBytes,neededBytes:(row as {neededBytes?:number}).neededBytes});
  expect(JSON.parse(JSON.stringify(actual)),row.name).toEqual({collect:expectedCollect[index].collect,keep:expectedCollect[index].keep});
 }
 const expectedAdmission=oracle("admission",fixture.admission);
 for(const [index,row] of fixture.admission.entries())expect(JSON.parse(JSON.stringify(core.concurrentAdmissionV1({scope:row.scope,freeBytes:row.freeBytes,pointers:row.pointers.map(pointer=>({...core.sealConcurrentPointerV1(null,pointer.scope,generation("e"),0,pointer.bytes)}))}))),row.name).toEqual({neededBytes:expectedAdmission[index].neededBytes,requiredBytes:expectedAdmission[index].requiredBytes,freeBytes:expectedAdmission[index].freeBytes,admitted:expectedAdmission[index].admitted});
 const expectedBudget=oracle("budget",fixture.budget);
 for(const [index,row] of fixture.budget.entries())expect(core.concurrentRequiredImageBytesV1(row.sharing),row.name).toBe(expectedBudget[index].requiredBytes);
 const expectedStale=oracle("stale",fixture.stale);
 for(const [index,row] of fixture.stale.entries())expect(core.concurrentStaleTemporariesV1(row.rows,row.nowMs,row.pressure),row.name).toEqual(expectedStale[index].stale);
 const expectedWait=oracle("wait",fixture.wait);
 for(const [index,row] of fixture.wait.entries())expect(core.concurrentWaitStepV1(row),row.name).toEqual({expired:expectedWait[index].expired,recheck:expectedWait[index].recheck,progress:expectedWait[index].progress});
});

/** 🛡️ Refuses a tampered, oversized or foreign pointer and never trusts a torn file. */
test("concurrent pointer: tampering, size and scope mismatch are refused and invalid files are skipped",()=>{
 const pointer=core.sealConcurrentPointerV1(core.sealConcurrentPointerV1(null,".",generation("1"),1000,10),".",generation("2"),2000,20),bytes=core.encodeConcurrentPointerV1(pointer);
 expect(core.parseConcurrentPointerV1(bytes)).toEqual(pointer);
 expect(()=>core.parseConcurrentPointerV1(Buffer.from(bytes.toString().replace('"epoch":2','"epoch":3')))).toThrow("Sealed dependency pointer");
 expect(()=>core.parseConcurrentPointerV1(Buffer.from(bytes.toString().replace('"version":1','"version":2')))).toThrow("Closed dependency pointer");
 expect(()=>core.parseConcurrentPointerV1(Buffer.alloc(protocol.pointer.maximumBytes+1))).toThrow("Bounded dependency pointer");
 expect(()=>core.sealConcurrentPointerV1(pointer,"other",generation("3"),3000,0)).toThrow("same scope");
 expect(()=>core.sealConcurrentPointerV1(null,".","short",1,0)).toThrow("Complete generation");
 const root=scratch();
 glue.writeConcurrentPointerV1(root,pointer);
 writeFileSync(join(root,protocol.pointer.directory,"0000000000000000"+protocol.pointer.suffix),bytes);
 writeFileSync(join(root,protocol.pointer.directory,"1111111111111111"+protocol.pointer.suffix),"{not json");
 expect(glue.readConcurrentPointersV1(root)).toEqual([pointer]);
 expect(glue.readConcurrentPointerV1(root,".")).toEqual(pointer);
 expect(glue.readConcurrentPointerV1(root,"missing")).toBeNull();
 expect(readdirSync(join(root,protocol.pointer.directory)).filter(name=>name.endsWith(".tmp"))).toEqual([]);
 const linked=scratch();mkdirSync(join(linked,"elsewhere"));symlinkSync(join(linked,"elsewhere"),join(linked,protocol.pointer.directory));
 expect(()=>glue.writeConcurrentPointerV1(linked,pointer)).toThrow("Physical pointer directory");
 rmSync(root,{recursive:true,force:true});rmSync(linked,{recursive:true,force:true});
});

/** ⏱️ Drives the polite acquisition on virtual time: it never sleeps, rechecks readiness every five seconds, reports every ten and honours its maximum and cancellation. */
test("concurrent acquisition: a busy lease is probed, readiness coalesces the waiter and the wait is bounded and cancellable",async()=>{
 const make=(busyRounds:number,readyAt:number|null,maximum:number,abort=new AbortController())=>{
  let clock=0,rounds=0,rechecks:number[]=[],events:core.ConcurrentEventV1[]=[];const lease={resource:"x",mode:"exclusive",release(){}} as ResourceLease;
  const ports:glue.ConcurrentLeasePortsV1={acquire:async()=>++rounds>busyRounds?lease:null,ready:readyAt===null?undefined:async()=>{rechecks.push(clock);return clock>=readyAt;},sleep:async ms=>{clock+=ms;},now:()=>clock,report:event=>{events.push(event);},signal:abort.signal};
  return{ports,rechecks,events,abort,get clock(){return clock;}};
 };
 const free=make(0,null,60000);
 expect(await glue.acquireConcurrentLeaseV1("build","/w","exclusive",free.ports,60000)).not.toBeNull();
 expect(free.clock).toBe(0);expect(free.events).toEqual([]);
 const busy=make(45,null,120000),lease=await glue.acquireConcurrentLeaseV1("build","/w","exclusive",busy.ports,120000);
 expect(lease).not.toBeNull();expect(busy.clock).toBe(45*protocol.wait.probeMilliseconds);
 expect(busy.events.map(event=>event.event)).toEqual(["waiting","waiting"]);
 expect(busy.events.map(event=>event.elapsedMs)).toEqual([10000,20000]);
 const coalesced=make(10_000,12000,600000);
 expect(await glue.acquireConcurrentLeaseV1("build","/w","exclusive",coalesced.ports,600000)).toBeNull();
 expect(coalesced.rechecks).toEqual([5000,10000,15000]);
 expect(coalesced.events.at(-1)).toMatchObject({event:"coalesced",resource:"build"});
 const bounded=make(10_000,null,3000);
 await expect(glue.acquireConcurrentLeaseV1("build","/w","exclusive",bounded.ports,3000)).rejects.toThrow("declared maximum");
 const abort=new AbortController(),cancelled=make(10_000,null,600000,abort);abort.abort(new Error("cancelled by the caller"));
 await expect(glue.acquireConcurrentLeaseV1("build","/w","exclusive",cancelled.ports,600000)).rejects.toThrow("cancelled by the caller");
 await expect(glue.acquireConcurrentLeaseV1("pin" as never,"/w","exclusive",free.ports,1)).rejects.toThrow("Closed concurrent lease kind");
});

/** 🔀️ Proves the pointer switch is atomic for a reader in another process: every read is complete, sealed and monotone while the publisher replaces the file hundreds of times. */
test("concurrent pointer switch: a second-process reader sees only complete, sealed, monotone pointers",async()=>{
 const root=scratch(),file=join(root,protocol.pointer.directory,core.concurrentPointerNameV1(".")),stop=join(root,"stop");
 let pointer=core.sealConcurrentPointerV1(null,".",generation("0"),1,1);
 glue.writeConcurrentPointerV1(root,pointer);
 const reader=spawn(testPythonV1()??"python3",["-I",here("./🐍️switch/🐍️.py"),file,stop,"20"],{stdio:["ignore","pipe","inherit"]});let out="";reader.stdout.on("data",chunk=>{out+=chunk;});
 const done=new Promise<void>(accept=>reader.on("close",()=>accept()));
 await sleep(150);
 for(let index=1;index<=400;index++){pointer=core.sealConcurrentPointerV1(pointer,".",generation("0123456789abcdef"[index%16]!),index*10,index);glue.writeConcurrentPointerV1(root,pointer);if(index%20===0)await sleep(1);}
 await sleep(300);writeFileSync(stop,"stop");await done;
 const result=JSON.parse(out.trim());
 expect(result.invalid).toBe(0);expect(result.missing).toBe(0);expect(result.backwards).toBe(0);
 expect(result.reads).toBeGreaterThan(100);expect(result.distinct).toBeGreaterThan(20);expect(result.last).toBeGreaterThanOrEqual(pointer.epoch-50);expect(result.last).toBeLessThanOrEqual(pointer.epoch);
 rmSync(root,{recursive:true,force:true});
});

/** 🧲️ Negative control of the switch oracle: a deliberately torn in-place write is detected by the same independent reader, so a clean result for the production writer is evidence and not blindness. */
test("concurrent pointer switch: the independent reader detects a torn in-place write",async()=>{
 const root=scratch(),file=join(root,"pointer.json"),stop=join(root,"stop"),first=core.sealConcurrentPointerV1(null,".",generation("1"),1,1),second=core.sealConcurrentPointerV1(first,".",generation("2"),2,2),bytes=core.encodeConcurrentPointerV1(second);
 writeFileSync(file,core.encodeConcurrentPointerV1(first));
 const reader=spawn(testPythonV1()??"python3",["-I",here("./🐍️switch/🐍️.py"),file,stop,"20"],{stdio:["ignore","pipe","inherit"]});let out="";reader.stdout.on("data",chunk=>{out+=chunk;});
 const done=new Promise<void>(accept=>reader.on("close",()=>accept()));
 await sleep(150);
 for(let index=0;index<5;index++){writeFileSync(file,bytes.subarray(0,Math.floor(bytes.length*0.4)));await sleep(30);writeFileSync(file,bytes);await sleep(10);}
 writeFileSync(stop,"stop");await done;
 expect(JSON.parse(out.trim()).invalid).toBeGreaterThan(0);
 rmSync(root,{recursive:true,force:true});
});

type Hooks={build:()=>Promise<void>;publish:()=>Promise<void>;ready?:()=>Promise<boolean>};
/** 🧪️ Runs the writer protocol of the storage module (build lease, build, bindings gate, publish) over real leases. */
async function writer(directory:string,workspace:string,hooks:Hooks,maximum=60000):Promise<"built"|"coalesced">{
 const ports=glue.concurrentLeasePortsV1(directory,undefined,signal,hooks.ready,()=>{}),build=await glue.acquireConcurrentLeaseV1("build",workspace,"exclusive",ports,maximum);
 if(!build)return "coalesced";
 try{
  if(hooks.ready&&await hooks.ready())return "coalesced";
  await hooks.build();
  const gate=await glue.acquireConcurrentLeaseV1("bindings",workspace,"exclusive",glue.concurrentLeasePortsV1(directory,undefined,signal,undefined,()=>{}),maximum);
  try{await hooks.publish();}finally{gate!.release();}
  return "built";
 }finally{build.release();}
}
const probe=async(directory:string,workspace:string,pin:string,timeoutMs=0):Promise<{served:boolean;ms:number}>=>{
 const started=performance.now();
 try{const gate=await acquireResourceLease({directory,resource:core.concurrentLeaseResourceV1("bindings",workspace),mode:"shared",signal,timeoutMs});try{await glue.withConcurrentPinsV1(directory,workspace,[pin],undefined,signal,async()=>{});}finally{gate.release();}return{served:true,ms:performance.now()-started};}
 catch(error){if(error instanceof Error&&error.message.startsWith("Resource lease timed out:"))return{served:false,ms:performance.now()-started};throw error;}
};

/** 🧵️ The wave's central law: while one process builds for a second and a half, readers in this process and in a second process keep being served; they are refused only during the short exclusive publication. */
test("concurrent build: readers are never blocked during the build, only during the short publication",async()=>{
 const directory=scratch(),workspace="/w/readers",pin=generation("a"),stamps:Record<string,number>={};
 const building=writer(directory,workspace,{build:async()=>{stamps.buildStart=Date.now();await sleep(1500);stamps.buildEnd=Date.now();},publish:async()=>{stamps.publishStart=Date.now();await sleep(400);stamps.publishEnd=Date.now();}});
 await sleep(150);
 const stop=join(directory,"stop"),child=spawn(process.execPath,[here("./📖️reader/🟦️.ts"),directory,workspace,pin,stop,"2600"],{stdio:["ignore","pipe","inherit"]});let out="";child.stdout.on("data",chunk=>{out+=chunk;});
 const finished=new Promise<void>(accept=>child.on("close",()=>accept()));
 const local:{served:boolean;ms:number}[]=[];
 while(stamps.buildEnd===undefined){local.push(await probe(directory,workspace,pin));await sleep(25);}
 expect(local.length).toBeGreaterThan(20);
 expect(local.every(row=>row.served),"every in-process probe during the build is served").toBe(true);
 expect(Math.max(...local.map(row=>row.ms))).toBeLessThan(250);
 expect(await building).toBe("built");
 await sleep(200);writeFileSync(stop,"stop");await finished;
 const remote=JSON.parse(out.trim()) as {served:number;refused:number;timeline:[number,string][]};
 expect(remote.served).toBeGreaterThan(30);
 const refusedWindows=remote.timeline.filter(row=>row[1]==="refused"),servedAfter=remote.timeline.filter(row=>row[1]==="served");
 expect(refusedWindows.length).toBeGreaterThan(0);
 expect(refusedWindows.every(row=>row[0]>=stamps.publishStart!-80&&row[0]<=stamps.publishEnd!+80),"refusals lie inside the publication window").toBe(true);
 expect(servedAfter.some(row=>row[0]>=stamps.publishEnd!-80),"service resumes after the publication").toBe(true);
 expect(remote.timeline[0]![1]).toBe("served");
 expect(stamps.publishEnd!-stamps.publishStart!).toBeLessThan(700);
 rmSync(directory,{recursive:true,force:true});
});

/** 🔬️ Negative control of the reader measurement: a writer that keeps the bindings gate for its whole build is seen blocking readers by the same probe. */
test("concurrent build: the probe detects a writer that holds the bindings gate during its build",async()=>{
 const directory=scratch(),workspace="/w/blocking",pin=generation("a");
 const gate=await glue.acquireConcurrentLeaseV1("bindings",workspace,"exclusive",glue.concurrentLeasePortsV1(directory,undefined,signal,undefined,()=>{}),5000);
 const rows=[await probe(directory,workspace,pin),await probe(directory,workspace,pin)];
 expect(rows.every(row=>!row.served)).toBe(true);
 gate!.release();
 expect((await probe(directory,workspace,pin)).served).toBe(true);
 rmSync(directory,{recursive:true,force:true});
});

/** 🤝️ Four concurrent ensure calls on a cold store start exactly one build and the others coalesce. */
test("concurrent ensure: callers coalesce on one build",async()=>{
 const directory=scratch(),workspace="/w/ensure";let ready=false,builds=0;
 const hooks:Hooks={ready:async()=>ready,build:async()=>{builds++;await sleep(900);ready=true;},publish:async()=>{await sleep(50);}};
 const ensure=async():Promise<string>=>{if(await hooks.ready!())return "current";return writer(directory,workspace,hooks);};
 const results=await Promise.all([ensure(),ensure(),ensure(),ensure()]);
 expect(builds).toBe(1);expect(results.filter(row=>row==="built").length).toBe(1);expect(results.filter(row=>row==="coalesced").length).toBe(3);
 rmSync(directory,{recursive:true,force:true});
},30000);

/** 🚪️ A waiting publisher never queues: new readers keep being served while it waits for a long reader, and it gets the gate as soon as the readers drain. */
test("concurrent publish: a waiting publisher does not block new readers and takes the gate once readers drain",async()=>{
 const directory=scratch(),workspace="/w/publish",pin=generation("b");
 const longReader=await acquireResourceLease({directory,resource:core.concurrentLeaseResourceV1("bindings",workspace),mode:"shared",signal,timeoutMs:0});
 const started=Date.now();let gotGate=0;
 const publishing=(async()=>{const gate=await glue.acquireConcurrentLeaseV1("bindings",workspace,"exclusive",glue.concurrentLeasePortsV1(directory,undefined,signal,undefined,()=>{}),20000);gotGate=Date.now()-started;gate!.release();})();
 await sleep(300);
 for(let index=0;index<5;index++){const row=await probe(directory,workspace,pin);expect(row.served,"new reader while the publisher waits").toBe(true);await sleep(40);}
 expect(gotGate).toBe(0);
 longReader.release();await publishing;
 expect(gotGate).toBeGreaterThan(299);expect(gotGate).toBeLessThan(1600);
 rmSync(directory,{recursive:true,force:true});
});

/** 📌️ A reader's pin keeps its generation from the collector and a collector holds the exclusive pin only while it removes. */
test("concurrent pins: a pinned generation is never collected and an unpinned one is",async()=>{
 const directory=scratch(),workspace="/w/pins",name=generation("c").slice(0,20);
 let tried:ResourceLease|null|undefined;
 await glue.withConcurrentPinsV1(directory,workspace,[generation("c"),generation("c"),generation("d")],undefined,signal,async()=>{tried=await glue.tryConcurrentPinExclusiveV1(directory,workspace,name,undefined,signal);});
 expect(tried).toBeNull();
 const lease=await glue.tryConcurrentPinExclusiveV1(directory,workspace,name,undefined,signal);
 expect(lease).not.toBeNull();
 const blocked=await glue.withConcurrentPinsV1(directory,workspace,[generation("d")],undefined,signal,async()=>"other generation unaffected");
 expect(blocked).toBe("other generation unaffected");
 lease!.release();
 expect(core.concurrentPinNamesV1([generation("d"),generation("c"),generation("c")])).toEqual([name,generation("d").slice(0,20)]);
 expect(()=>core.concurrentPinNamesV1(["short"])).toThrow("Complete generation");
 rmSync(directory,{recursive:true,force:true});
});

/** 🔒️ Pins the wiring in the patched sources: lock order, shared image everywhere, no whole-image maintenance, retirement through a tombstone and a cheap ready path. */
test("concurrent wiring laws: lock order build then bindings, shared image only, tombstone retirement, receipt writes without a build",()=>{
 const text=storage();
 expect(text).not.toContain("QueuedMaintenance");expect(text).not.toContain("requireRepositoryCacheStoreMaintenanceV1");
 const entry=slice(text,"export function withJavascriptDependencyStorageV1<T>(","export function withJavascriptDependencyReceiptStorageV1<T>(");
 expect(entry).toContain('withRepositoryCacheStoreV1(workspace,"dependencies"');expect(entry).toContain('acquireConcurrentLeaseV1("build"');expect(entry).not.toContain('"bindings"');expect(entry).toContain("installSchema.$defs.SourceOwnerStoreSyncProtocol.const.maximumQueueWaitMilliseconds");
 const receipt=slice(text,"export function withJavascriptDependencyReceiptStorageV1<T>(","/** 📜️");
 expect(receipt).toContain('acquireConcurrentLeaseV1("writer"');expect(receipt).not.toContain('"build"');expect(receipt).not.toContain('"bindings"');
 const orchestrator=slice(text,"async function runJavascriptDependencyStoreGenerationsV1(","/** 📌️ Pins the generations a producer is about to create");
 const order=['"build")','acquireConcurrentLeaseV1("bindings"','"publish")','gate.release()'].map(part=>orchestrator.indexOf(part));
 expect(order.every(index=>index>=0)).toBe(true);
 expect(order[0]).toBeLessThan(order[1]!);expect(order[1]).toBeLessThan(order[2]!);expect(order[2]).toBeLessThan(order[3]!);
 expect(text.match(/acquireConcurrentLeaseV1\("build"/gu)?.length).toBe(1);
 expect(text.match(/acquireConcurrentLeaseV1\("bindings"/gu)?.length).toBe(1);
 expect(text.match(/acquireConcurrentLeaseV1\("writer"/gu)?.length).toBe(2);
 const producer=slice(text,"async function runJavascriptDependencyStoreProducerV1(","/** 📗️ Matches an inactive candidate");
 expect(producer).not.toContain("retire-obsolete");expect(producer).not.toContain("removeJavascriptDependencyTreeV1(join(container");
 expect(producer.indexOf('if(phase==="build")return;')).toBeGreaterThan(producer.indexOf("await verifyReady();"));
 expect(producer.indexOf('if(phase==="build")return;')).toBeLessThan(producer.indexOf("publishJavascriptDependencyPreservingSourceDescriptorV1(binding"));
  expect(producer).toContain('if(phase==="publish"&&fresh.length)throw Error("Built generations required before publication");');
 expect(producer.indexOf("sealConcurrentPointerV1(")).toBeGreaterThan(producer.indexOf("publishJavascriptDependencyGenerationReceiptsElseV1("));
 const collect=slice(text,"async function collectJavascriptDependencyGenerationsV1(","/** 🧰 Creates only the six current installer containers");
 expect(collect).not.toContain("requireJavascriptDependencyStoreFrameV1");expect(collect).not.toContain('"build"');expect(collect).not.toContain('"bindings"');expect(collect).not.toContain("QueuedMaintenance");
 expect(collect).toContain('requireRepositoryCacheStoreV1(store,selected,"dependencies")');
 expect(collect.indexOf("tryConcurrentPinExclusiveV1(")).toBeLessThan(collect.indexOf("renameSync(join(container,name),tombstone)"));
 expect(collect.indexOf("renameSync(join(container,name),tombstone)")).toBeLessThan(collect.indexOf("removeJavascriptDependencyTreeV1(tombstone"));
 expect(collect).toContain("selectObsoleteJavascriptDependencyGenerationsV1(names,[],[])");expect(collect).toContain("concurrentGenerationReferencedV1(container,name)");
 expect(collect.match(/concurrentGenerationReferencedV1\(/gu)?.length).toBe(2);
 const maintain=slice(text,"export function maintainJavascriptDependencyGenerationsV1(","/** 🧰 Creates only the six current installer containers");
 expect(maintain).toContain('withRepositoryCacheStoreV1(workspace,"dependencies"');expect(maintain).not.toContain("acquireConcurrentLeaseV1");expect(maintain).not.toContain("Maintenance(");
 expect(producer).not.toContain("collectJavascriptDependencyStoreGenerationsV1");
 expect(producer.indexOf("pinJavascriptDependencyGenerationsV1(")).toBeGreaterThan(0);
 expect(producer.indexOf("pinJavascriptDependencyGenerationsV1(")).toBeLessThan(producer.indexOf("captureOwnedCommandResultV1("));
 expect(producer.indexOf("admitJavascriptDependencyGenerationV1(")).toBeLessThan(producer.indexOf("captureOwnedCommandResultV1("));
 expect(producer.indexOf("admitJavascriptDependencyGenerationV1(")).toBeGreaterThan(producer.indexOf("for(const row of fresh){"));
 expect(producer).toContain("sealConcurrentPointerV1(previous,scope,generation,switched,frame.built.get(scope)??previous?.bytes??0)");
 expect(orchestrator.indexOf("await collectJavascriptDependencyGenerationsV1(workspace")).toBeGreaterThan(orchestrator.indexOf("gate.release()"));
 const admit=slice(text,"async function admitJavascriptDependencyGenerationV1(","/** 🏗️ Retires superseded gene");
 expect(admit.indexOf("concurrentAdmissionV1(")).toBeLessThan(admit.indexOf("collectJavascriptDependencyGenerationsV1("));
 expect(admit.lastIndexOf("concurrentAdmissionV1(")).toBeGreaterThan(admit.indexOf("collectJavascriptDependencyGenerationsV1("));
 expect(admit).toContain('concurrentTextV1(resolveLanguageV1(concurrentProtocolV1.languages,[],process.env).language,"admissionRefused"');expect(admit).toContain('event:"refused"');expect(admit).toContain("throw Error(");
 expect(slice(text,"async function readJavascriptDependencyGenerationElseV1<T>(","/** 📮️")).toContain("withConcurrentPinsV1(");
 expect(slice(text,"async function requireJavascriptDependencyBudgetV1(","/** 🛂️")).toContain("repositoryCacheBudgetV1(workspace).partitions.dependencies.usableBytes");
 expect(slice(text,"async function admitJavascriptDependencyGenerationV1(","/** 🏗️ Retires superseded gene").indexOf("requireJavascriptDependencyBudgetV1(workspace,control)")).toBeGreaterThan(0);
 expect(collect.indexOf("sweepConcurrentTemporariesV1(")).toBeGreaterThan(0);expect(collect.indexOf("sweepConcurrentTemporariesV1(")).toBeLessThan(collect.indexOf("concurrentCollectionV1("));
 const boot=bootstrap();
 expect(boot).toContain("publishReceipt?withJavascriptDependencyReceiptStorageV1(workspace,invocation,phaseControl,body):withRepositoryCacheStoreV1(");
 expect(boot).toContain(",{ready:recheck});");
 expect(boot.indexOf('if(inspection==="ready"){check();return;}')).toBeLessThan(boot.indexOf("await install(root,invocation,phaseControl,recheck)"));
 expect(boot).toContain("const inspect=async(publishReceipt:boolean)=>storage(publishReceipt,inspectIn(publishReceipt,false))");
});

/** 🛂️ Renders the refusal diagnostic in both languages with the measured numbers and never silently picks a language. */
test("concurrent admission: the refusal diagnostic is complete in English and German",()=>{
 const values={needed:1177,free:812};
 const en=core.concurrentTextV1("en","admissionRefused",values),de=core.concurrentTextV1("de","admissionRefused",values),reclaim=[core.concurrentTextV1("en","reclaiming",values),core.concurrentTextV1("de","reclaiming",values)];
 for(const text of [en,de,...reclaim]){expect(text).toContain("1177");expect(text).toContain("812");expect(text).not.toMatch(/\{[a-z]+\}/u);}
 expect(en).not.toBe(de);expect(de).toContain("Abhängigkeits-Image");expect(en).toContain("published generations are untouched");expect(de).toContain("bleiben unberührt");
 expect(core.concurrentAdmissionV1({scope:".",pointers:[],freeBytes:protocol.admission.defaultGenerationBytes}).admitted).toBe(false);
 expect(core.concurrentAdmissionV1({scope:".",pointers:[],freeBytes:Math.ceil(protocol.admission.defaultGenerationBytes*protocol.admission.marginPercent/100)}).admitted).toBe(true);
 expect(()=>core.concurrentAdmissionV1({scope:".",pointers:[],freeBytes:-1})).toThrow("Measured free bytes");
});

/** 🧹️ Model of the generation-scoped collector over real leases and a real tree: it needs no build lease, so a running build never blocks it, and it spares referenced, pinned, builder-pinned, current and young generations. */
test("concurrent maintenance: collection is generation-scoped and never waits for a build",async()=>{
 const directory=scratch(),workspace="/w/maintenance",root=scratch(),container=join(root,"j"),now=Date.now(),hour=3600000;
 const make=(letter:string,ready:boolean,link:boolean,ageHours:number)=>{
  const name=letter.repeat(20),path=join(container,name);mkdirSync(join(path,"node_modules"),{recursive:true});
  const source=join(root,"workspace-"+letter);mkdirSync(source,{recursive:true});
  if(ready)writeFileSync(join(path,"bindings.json"),JSON.stringify({bindings:[{sourceDirectory:join(source,"node_modules"),targetDirectory:join(path,"node_modules")}]}));
  if(link)symlinkSync(join(path,"node_modules"),join(source,"node_modules"));
  const at=new Date(now-ageHours*hour);utimesSync(path,at,at);return name;
 };
 const referenced=make("1",true,true,100),old=make("2",true,false,100),young=make("3",true,false,1),readerPinned=make("4",true,false,100),builderPinned=make("5",false,false,100),current=make("6",true,false,100),unreadable=(()=>{const name="7".repeat(20);mkdirSync(join(container,name),{recursive:true});writeFileSync(join(container,name,"bindings.json"),"{torn");const at=new Date(now-100*hour);utimesSync(join(container,name),at,at);return name;})();
 const pointer=core.sealConcurrentPointerV1(null,".",current.repeat(3).slice(0,64).padEnd(64,"6"),now,1);
 const collect=async():Promise<string[]>=>{
  const removed:string[]=[],names=readdirSync(container).filter(name=>/^[a-f0-9]{20}$/u.test(name)),currents=new Set([pointer.generation.slice(0,20)]);
  const rows=names.filter(name=>!currents.has(name)&&!glue.concurrentGenerationReferencedV1(container,name)).map(name=>({name,modifiedAtMs:lstatSync(join(container,name)).mtimeMs}));
  for(const name of core.concurrentCollectionV1({rows,pointers:[pointer],nowMs:Date.now(),freeFraction:0.9}).collect){
   const pin=await glue.tryConcurrentPinExclusiveV1(directory,workspace,name,undefined,signal);if(!pin)continue;
   try{if(glue.concurrentGenerationReferencedV1(container,name))continue;const tombstone=join(container,".x"+name);renameSync(join(container,name),tombstone);rmSync(tombstone,{recursive:true});removed.push(name);}finally{pin.release();}
  }
  return removed.sort();
 };
 const build=await glue.acquireConcurrentLeaseV1("build",workspace,"exclusive",glue.concurrentLeasePortsV1(directory,undefined,signal,undefined,()=>{}),5000);
 const builderPins=await glue.acquireConcurrentPinsV1(directory,workspace,[builderPinned],undefined,signal);
 const started=performance.now();
 const removed=await glue.withConcurrentPinsV1(directory,workspace,[readerPinned],undefined,signal,collect);
 expect(performance.now()-started,"no wait for the running build").toBeLessThan(500);
 expect(removed).toEqual([old].sort());
 for(const name of [referenced,young,readerPinned,builderPinned,current,unreadable])expect(existsSync(join(container,name)),name).toBe(true);
 expect(existsSync(join(container,old))).toBe(false);expect(readdirSync(container).filter(name=>name.startsWith(".x"))).toEqual([]);
 for(const lease of builderPins)lease.release();build!.release();
 expect(await collect()).toEqual([builderPinned,readerPinned].sort());
 rmSync(directory,{recursive:true,force:true});rmSync(root,{recursive:true,force:true});
});

/** 📐️ The declared dependencies partition must hold the current generation set plus one building set: derived from the measured sizes in the schema, checked against every declared platform backend, including Linux copying, with an independent Python arithmetic oracle. */
test("concurrent budget: the declared partition funds every platform backend and both generation sets",()=>{
 const budget=protocol.budget,declared=readRepositoryCacheBudgetV1(policy).partitions.dependencies.usableBytes;
 expect(core.concurrentRequiredImageBytesV1(true)).toBeLessThanOrEqual(declared);
 const independent=oracle("budget",fixture.budget);
 for(const[index,row]of fixture.budget.entries()){expect(core.concurrentRequiredImageBytesV1(row.sharing)).toBe(row.requiredBytes);expect(independent[index].requiredBytes).toBe(row.requiredBytes);expect(row.requiredBytes).toBeLessThanOrEqual(declared);}
 expect(core.concurrentRequiredImageBytesV1(false)).toBeGreaterThan(core.concurrentRequiredImageBytesV1(true));
 expect(protocol.admission.defaultGenerationBytes).toBe(4*budget.physicalGenerationBytes);
 const measured=budget.measured,unique=(measured.usedKiB-measured.packageCacheKiB-measured.temporariesKiB-measured.receiptsKiB-measured.capsulesKiB)*1024/measured.generationsCounted;
 expect(budget.physicalGenerationBytes).toBeGreaterThanOrEqual(unique);expect(budget.physicalGenerationBytes).toBeLessThan(2*unique);
 expect(budget.copiedGenerationBytes).toBeGreaterThanOrEqual(measured.logicalGenerationSetKiB*1024/budget.scopes);
 expect(budget.packageCacheBytes).toBeGreaterThan(measured.packageCacheKiB*1024);
 const backends=(installSchema.$defs.ProducerRecipeProtocol.const as unknown as {backends:Record<string,string>}).backends;
 expect(backends.darwin).toBe("clonefile");
 for(const backend of Object.values(backends))expect(core.concurrentRequiredImageBytesV1(backend==="clonefile")).toBeLessThanOrEqual(declared);
 for(const language of protocol.languages){const text=core.concurrentTextV1(language,"budgetTooSmall",{needed:6400,free:512});expect(text).toContain("6400");expect(text).toContain("512");}
});

/** 🧽️ Sweeps only stale scratch directories by name and age, without following links, and reports the count. */
test("concurrent temporaries: only stale scratch directories of the declared shape are removed",()=>{
 const root=scratch(),temporary=join(root,protocol.temporaries.directory),now=Date.now(),hour=3600000,make=(name:string,ageHours:number)=>{mkdirSync(join(temporary,name,"inner"),{recursive:true});writeFileSync(join(temporary,name,"inner","file"),"x");const at=new Date(now-ageHours*hour);utimesSync(join(temporary,name),at,at);};
 make(".18f4dcdd67fbfed8-00000001.nx",30);make(".19efff7ab77ffdbf-00000002.tmp",3);make("cc-abcdef012345",100);
 mkdirSync(join(root,"elsewhere"));symlinkSync(join(root,"elsewhere"),join(temporary,".1a-1.tmp"));const old=new Date(now-100*hour);
 expect(glue.sweepConcurrentTemporariesV1(root,now,false,()=>{})).toBe(1);
 expect(existsSync(join(temporary,".18f4dcdd67fbfed8-00000001.nx"))).toBe(false);expect(existsSync(join(temporary,".19efff7ab77ffdbf-00000002.tmp"))).toBe(true);
 expect(glue.sweepConcurrentTemporariesV1(root,now,true,()=>{})).toBe(1);
 expect(existsSync(join(temporary,".19efff7ab77ffdbf-00000002.tmp"))).toBe(false);expect(existsSync(join(temporary,"cc-abcdef012345"))).toBe(true);expect(existsSync(join(root,"elsewhere"))).toBe(true);void old;
 expect(glue.sweepConcurrentTemporariesV1(join(root,"missing"),now,true,()=>{})).toBe(0);
 let checks=0;make(".1b-1.nx",30);make(".1c-1.nx",30);expect(glue.sweepConcurrentTemporariesV1(root,now,false,()=>{checks++;})).toBe(2);expect(checks).toBe(2);
 rmSync(root,{recursive:true,force:true});
});
