import {afterEach,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdtempSync,rmSync} from "node:fs";
import {spawn,spawnSync} from "node:child_process";
import {join} from "node:path";
import {testScratchRootV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import {encodeProtobufV1,REAPI_V1} from "../../📡️wire/📦️protobuf/🟦️.ts";
import type {RemoteExecutionContentV1} from "../../🗄️content/🟦️.ts";
import {remoteExecutionPeakEntryV1,requireRemoteExecutionShapeV1,remoteExecutionPeakOfMetadataV1,type RemoteExecutionDigestV1,type RemoteExecutionWorkerRegistrationV1} from "../../🟦️.ts";
import {readOffloadOutputsV1} from "../../🚀️offload/🔌️client/🟦️.ts";
import {offloadDeclaredThreadsV1,readFamilyPeakV1,readUnitPeakV1,writeUnitPeakV1} from "../../🚀️offload/🚦️admission/🟦️.ts";
import {placementChargeMegabytesV1} from "../../🗓️scheduler/⚖️placement/🟦️.ts";
import {RemoteExecutionSchedulerV1} from "../../🗓️scheduler/🟦️.ts";
import {createRemoteExecutionMeterV1,remoteExecutionGroupMegabytesV1,remoteExecutionProcessTableV1} from "../../👷️worker/📏️task-memory/🟦️.ts";

const here=import.meta.dir,scratch:string[]=[];
afterEach(()=>{for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});
const oracle=JSON.parse(spawnSync("python3",["-I",join(here,"🐍️.py")],{encoding:"utf8"}).stdout) as {lifetime:{declared:number;measured:number|null;charge:number}[];ramps:{declared:number;measured:number|null;startedAtMs:number;reportAtMs:number|null;rampMs:number;charge:number}[];tables:{table:string;groups:number[];megabytes:Record<string,number>}[];peaks:{megabytes:number;text:string}[]};

test("a running action is charged once over its life: its declared peak minus what is measured of it, equal to the Python oracle (not started, growing, at its peak, beyond it)",()=>{
 expect(oracle.lifetime.length).toBe(32);
 for(const row of oracle.lifetime)expect(placementChargeMegabytesV1({declaredMegabytes:row.declared,...row.measured===null?{}:{measuredMegabytes:row.measured},startedAtMs:1000},undefined,120000),JSON.stringify(row)).toBe(row.charge);
 const peak=4096,growth=[0,1024,3000,4096,5000].map(measured=>placementChargeMegabytesV1({declaredMegabytes:peak,measuredMegabytes:measured,startedAtMs:0},undefined,0));
 expect(growth).toEqual([4096,3072,1096,0,0]);
});

test("without a measurement the declared peak is reserved until a load report arrived the ramp after the start; equal to the Python oracle in 400 cases",()=>{
 for(const row of oracle.ramps)expect(placementChargeMegabytesV1({declaredMegabytes:row.declared,...row.measured===null?{}:{measuredMegabytes:row.measured},startedAtMs:row.startedAtMs},row.reportAtMs??undefined,row.rampMs),JSON.stringify(row)).toBe(row.charge);
});

test("the resident megabytes of a process group are the sum of its rows of a ps table, equal to the Python oracle on 60 random tables",()=>{
 for(const row of oracle.tables)expect(Object.fromEntries(remoteExecutionGroupMegabytesV1(row.table,new Set(row.groups))),JSON.stringify(row.groups)).toEqual(row.megabytes);
});

test("the meter keeps the current and the peak megabytes of an attached task, reports only sampled tasks and forgets a finished one",async()=>{
 let table="",calls=0;const meter=createRemoteExecutionMeterV1(async()=>{calls++;return table;},5);
 table="500 1024\n500 1024\n600 4096\n";meter.attach("a.1",500);meter.attach("b.1",700);
 await Bun.sleep(40);expect(meter.usage()).toEqual([{task:"a.1",megabytes:2}]);
 table="500 8192\n";await Bun.sleep(40);table="500 2048\n";await Bun.sleep(40);
 expect(meter.usage()).toEqual([{task:"a.1",megabytes:2}]);
 expect(meter.finish("a.1")).toBe(8);expect(meter.finish("b.1")).toBeUndefined();expect(meter.usage()).toEqual([]);
 const before=calls;await Bun.sleep(40);expect(calls,"no sampling while nothing is attached").toBe(before);meter.close();
});

test("the measured peak travels in the auxiliary metadata of the action result and reads back equal to the oracle; an entry of another type or a malformed value is ignored",()=>{
 for(const row of oracle.peaks){const entry=remoteExecutionPeakEntryV1(row.megabytes);expect(new TextDecoder().decode(entry.value)).toBe(row.text);expect(remoteExecutionPeakOfMetadataV1([entry])).toBe(Number(row.text));}
 expect(remoteExecutionPeakOfMetadataV1([{type_url:"x",value:new TextEncoder().encode("9")}])).toBeUndefined();
 expect(remoteExecutionPeakOfMetadataV1([{...remoteExecutionPeakEntryV1(5),value:new TextEncoder().encode("-3")}])).toBeUndefined();
 expect(remoteExecutionPeakOfMetadataV1(undefined)).toBeUndefined();
});

test("a peak a worker measured reaches the learned-peak ledger of the unit's crate and its family exactly like a local compile",async()=>{
 const root=mkdtempSync(join(testScratchRootV1(),"o12-"));scratch.push(root);
 const content={read:async()=>undefined} as unknown as RemoteExecutionContentV1;
 const result=encodeProtobufV1(REAPI_V1.actionResult,{exit_code:0,execution_metadata:{worker:"w",auxiliary_metadata:[remoteExecutionPeakEntryV1(6549.2)]}});
 const outputs=await readOffloadOutputsV1(content,"main",(await import("../../📡️wire/📦️protobuf/🟦️.ts")).decodeProtobufV1(REAPI_V1.actionResult,result),"out",[]);
 expect(outputs.peakMegabytes).toBe(6550);
 for(const crate of ["a_component","b_component","c_component"])writeUnitPeakV1(root,crate,outputs.peakMegabytes!+(crate==="a_component"?0:crate==="b_component"?-1000:-2000),offloadDeclaredThreadsV1(true));
 expect(readUnitPeakV1(root,"a_component",offloadDeclaredThreadsV1(true))).toBe(6550);
 expect(readUnitPeakV1(root,"b_component",offloadDeclaredThreadsV1(true))).toBe(5550);
 expect(readFamilyPeakV1(root)).toBeDefined();
 writeUnitPeakV1(root,"a_component",2000,offloadDeclaredThreadsV1(true));expect(readUnitPeakV1(root,"a_component",offloadDeclaredThreadsV1(true))!).toBeLessThan(6550);
});

const sha=(data:Uint8Array)=>createHash("sha256").update(data).digest("hex");
class Memory implements RemoteExecutionContentV1{
 blobs=new Map<string,Uint8Array>();results=new Map<string,Uint8Array>();
 put(bytes:Uint8Array):RemoteExecutionDigestV1{const digest={hash:sha(bytes),size:bytes.length};this.blobs.set(digest.hash,bytes);return digest;}
 async has(_:string,hash:string){return this.blobs.has(hash);}
 async missing(_:string,digests:readonly RemoteExecutionDigestV1[]){return digests.filter(digest=>!this.blobs.has(digest.hash));}
 async read(_:string,digest:RemoteExecutionDigestV1){return this.blobs.get(digest.hash);}
 async stream(_:string,digest:RemoteExecutionDigestV1){const bytes=this.blobs.get(digest.hash);return bytes?(async function*(){yield bytes;})():undefined;}
 async write(_:string,digest:RemoteExecutionDigestV1,source:Uint8Array|AsyncIterable<Uint8Array>){this.blobs.set(digest.hash,source instanceof Uint8Array?source:new Uint8Array());}
 async readActionResult(_:string,hash:string){return this.results.get(hash);}
 async writeActionResult(_:string,hash:string,bytes:Uint8Array){this.results.set(hash,bytes);}
 action(properties:{name:string;value:string}[]):RemoteExecutionDigestV1{
  const command=this.put(encodeProtobufV1(REAPI_V1.command,{arguments:["/bin/true"],platform:{properties}}));
  return this.put(encodeProtobufV1(REAPI_V1.action,{command_digest:{hash:command.hash,size_bytes:command.size},salt:new Uint8Array(0)}));
 }
}

test("the scheduler charges a running action once: the slots a machine offers grow as the action grows into its declared peak, and with an old worker (no usage) the whole peak is reserved only until a load report arrived the ramp after the start",async()=>{
 const T="aaaaaaaaaaaaaaaa",properties=[{name:"Arch",value:"arm64"},{name:"OSFamily",value:"macos"},{name:"semio-toolchain",value:T},{name:"semio-target",value:"aarch64-apple-darwin"},{name:"semio-offload-format",value:"argv-1"}];
 const report=(origin:string,free:number,at:number)=>({version:1 as const,origin,platform:{key:`macos-arm64-${T}`,os:"macos" as const,arch:"arm64",toolchain:T,target:"aarch64-apple-darwin"},capacity:{cores:8,memoryMegabytes:16384},running:0,queued:0,pressure:"normal" as const,freeMegabytes:free,load:1,headroom:true,wallMs:at,worker:{state:"idle" as const,concurrency:4,running:0,sandbox:null,confinement:null,network:null},counters:{unit:{localHit:0,remoteHit:0,miss:0,stored:0},offload:{attempted:0,hit:0,declined:0,failed:0,fellBack:0}}} as never);
 const control=[{name:"semio-origin",value:"requester"},{name:"semio-offload-format",value:"argv-1"},{name:"semio-peak-mb",value:"4096"}];
 const scenario=async(measured:readonly (number|undefined)[])=>{
  let now=1_000_000;const memory=new Memory(),scheduler=new RemoteExecutionSchedulerV1({content:memory,unmatchedSeconds:30,lingerSeconds:0.05,leaseSeconds:10,maximumAttempts:3,defaultActionSeconds:60,maximumActionSeconds:600,maximumQueued:100,heartbeatSeconds:2,pollSeconds:5,loadExpireSeconds:600,refusalCooldownSeconds:2,cache:{url:"http://c",relative:false},language:"none",sweepMilliseconds:0,chargeRampSeconds:120,now:()=>now});
  const slots=():number=>scheduler.idleFor("requester",2048).slots;
  scheduler.acceptLoad(report("requester",8000,now));scheduler.acceptLoad(report("worker-a",6000,now));
  const worker=scheduler.register({...profile(properties),origin:"worker-a"} as RemoteExecutionWorkerRegistrationV1),seen:number[]=[];
  seen.push(slots());
  const run=(async()=>{for await(const _ of scheduler.execute({instance:"main",action:memory.action(control),skipCacheLookup:false},new AbortController().signal));})();
  await Bun.sleep(20);const task=(await scheduler.poll(worker.id,1000,new AbortController().signal)).task!;
  seen.push(slots());
  for(const mb of measured){now+=10_000;scheduler.acceptLoad(report("worker-a",6000,now));scheduler.heartbeat(worker.id,[task.id],mb===undefined?undefined:[{task:task.id,megabytes:mb}]);seen.push(slots());}
  scheduler.report(worker.id,task.id,{version:1,outcome:"completed",status:{code:0,message:""},actionResult:Buffer.from(encodeProtobufV1(REAPI_V1.actionResult,{exit_code:0})).toString("base64")});await run;
  seen.push(slots());return {seen,scheduler,worker,advance:(seconds:number)=>{now+=seconds*1000;scheduler.acceptLoad(report("worker-a",6000,now));}};
 };
 const growing=await scenario([0,1024,3000,4096]);
 expect(growing.seen,"before / just placed (declared) / not grown / growing / grown / at peak / finished").toEqual([2,0,0,1,2,2,2]);
 const old=await scenario([undefined,undefined]);
 expect(old.seen,"an old worker: the whole peak is still reserved 20 s after the start, and again after it ended").toEqual([2,0,0,0,2]);
 const settled=await scenario(Array.from({length:14},()=>undefined));
 expect(settled.seen.slice(11,15),"an old worker: 110 s after the start still reserved, from 120 s on the machine's free memory already shows the action").toEqual([0,0,2,2]);
});

function profile(properties:{name:string;value:string}[]):RemoteExecutionWorkerRegistrationV1{return {version:1,name:"a",host:"h",platform:"darwin-arm64",properties,concurrency:4,sandbox:"isolated-root",runtime:"test",origin:"a",capacity:{cores:8,memoryMegabytes:16384},confinement:"none",network:"unenforced"};}

test("the real process table measures the group of a spawned child, and the closed heartbeat shape admits the usage rows and nothing else",async()=>{
 if(process.platform!=="win32"){
  const child=spawn("sleep",["3"],{detached:true,stdio:"ignore"}),meter=createRemoteExecutionMeterV1(remoteExecutionProcessTableV1,50);
  try{meter.attach("t.1",child.pid!);await Bun.sleep(400);expect(meter.usage()[0]?.megabytes??0).toBeGreaterThanOrEqual(1);expect(meter.finish("t.1")).toBeGreaterThanOrEqual(1);}
  finally{try{process.kill(-child.pid!,"SIGKILL");}catch{}meter.close();}
 }
 const heartbeat={version:1,running:["a.1"]};
 expect(()=>requireRemoteExecutionShapeV1("Heartbeat",heartbeat,"x")).not.toThrow();
 expect(()=>requireRemoteExecutionShapeV1("Heartbeat",{...heartbeat,usage:[{task:"a.1",megabytes:12}]},"x")).not.toThrow();
 expect(()=>requireRemoteExecutionShapeV1("Heartbeat",{...heartbeat,usage:[{task:"a.1",megabytes:-1}]},"x")).toThrow();
 expect(()=>requireRemoteExecutionShapeV1("Heartbeat",{...heartbeat,usage:[{task:"a.1",megabytes:1,extra:1}]},"x")).toThrow();
});
