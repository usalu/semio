import{expect,test}from"bun:test";
import Ajv from"ajv";
import{execFileSync}from"node:child_process";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{readFileSync}from"node:fs";
import ts from"typescript";
import{fileURLToPath}from"node:url";
import{declaredTypeScriptProjectsV1}from"../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import{spawnSync}from"node:child_process";
import{implementationProcessEnvironmentV1}from"../../../🏛️sealed/📥️execution/🟦️.ts";
import{graphChildEnvironmentV1}from"../../🐏️child-environment/🟨️.mjs";
import{mkdirSync,writeFileSync}from"node:fs";
import{releaseOwnerStateV1,createScriptInputCache}from"../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import{INFERENCE_LIMITS_V1}from"../../../🕸️graph/📥️inference/🧬️schema/🟨️.mjs";
import{createSharedSourceFilesV1}from"../../../🕸️graph/📥️inference/🧭️script-inputs/♨️reuse/🟨️.mjs";
import{partInferenceEngagementV1}from"../../../🕸️graph/🧭️discovery/🟦️.ts";
import{openProjectFarmV1,closeProjectFarmsV1}from"../../../🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs";
import{noteSlowV1,releaseIfGrownV1,footprintPulseV1,bindProcessGateV1,createHeapSamplerV1,footprintStepV1,noteFootprintStepV1,useEnvironmentGateV1}from"../🟨️.mjs";
import{createDarwinFootprintProbeV1,createFootprintTraceV1,heapCensusV1,FOOTPRINT_MESSAGES_V1,FOOTPRINT_POLICY_V1,createFootprintGateV1,createFootprintMeterV1,decideFootprintV1,footprintGateV1,footprintHeadroomBytesV1,grantBytesOfV1,readProcessFootprintBytesV1,releaseRuntimeMemoryV1,renderFootprintMessageV1,settleWaveMemoryV1,useFootprintGateV1}from"../🟨️.mjs";
import{createGraphParentMemoryGovernorV1,createMemoryGovernorV1,serviceMemoryHeadroomBytesV1}from"../../🟨️.mjs";
import{createClosureSettlerV1,useCapacityGrantV1}from"../../../🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs";
import{createFactIndexV1,type GenerationRowV1}from"../../../🕸️graph/🕰️generations/🟨️.mjs";
import{derivedOwnerNameV1,suspectRowsV1}from"../../../🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs";
import{Database}from"bun:sqlite";
import windowsCases from "./windows-process.json" with {type:"json"};
import{GRAPH_MEMORY_POLICY_V1}from"../../🟨️.mjs";
import{planCeilingBytesV1,useSliceCeilingGateV1}from"../🟨️.mjs";

/** 🦶️ Contract: the memory of a graph read is bounded by its grant. The physical footprint of the parent (not the resident set, not the managed heap) is read without blocking, compared with the grant, the admission of new work slows before the grant is overrun, and the process settles after every published wave and says so in one line.
 * @see ../🧬️schema/🔣️.json */
const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8"));
const MB=1048576,holes=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
const clock=()=>{let at=0;return{now:()=>at,sleep:async(milliseconds:number)=>{at+=milliseconds;},advance:(milliseconds:number)=>{at+=milliseconds;}};};

test("policy and messages equal the schema file; both languages have the same keys and placeholders",()=>{
 expect(FOOTPRINT_POLICY_V1).toEqual(schema.definitions.Policy.const);expect(FOOTPRINT_MESSAGES_V1).toEqual(schema.definitions.Messages.const);
 expect(Object.keys(FOOTPRINT_MESSAGES_V1.en)).toEqual(Object.keys(FOOTPRINT_MESSAGES_V1.de));
 for(const key of Object.keys(FOOTPRINT_MESSAGES_V1.en))expect(holes(FOOTPRINT_MESSAGES_V1.de[key as"slowed"])).toEqual(holes(FOOTPRINT_MESSAGES_V1.en[key as"slowed"]));
 expect(renderFootprintMessageV1("waveSettled",{generation:7,before:9000,after:1200,managed:300,grant:10462},{SEMIO_LANGUAGE:"en,de"})).toEqual(["graph: wave 7 settled from 9000 MB to 1200 MB (managed 300 MB, grant 10462 MB)","Graph: Welle 7 beruhigt von 9000 MB auf 1200 MB (verwaltet 300 MB, Zusage 10462 MB)"]);
 expect(FOOTPRINT_POLICY_V1.softPercent).toBeLessThan(FOOTPRINT_POLICY_V1.hardPercent);
});

test("the decision law: run below soft, settle from soft, wait from hard, run without a known grant",()=>{
 const grantBytes=1000*MB,at=(percent:number)=>decideFootprintV1({footprintBytes:grantBytes*percent/100,grantBytes});
 expect([at(0),at(59),at(60),at(84),at(85),at(300)]).toEqual(["run","run","settle","settle","wait","wait"]);
 expect(decideFootprintV1({footprintBytes:50000*MB,grantBytes:undefined})).toBe("run");expect(decideFootprintV1({footprintBytes:50000*MB,grantBytes:0})).toBe("run");expect(decideFootprintV1({footprintBytes:Number.NaN,grantBytes})).toBe("run");
 expect(footprintHeadroomBytesV1({footprintBytes:800*MB,grantBytes})).toBe(50*MB);expect(footprintHeadroomBytesV1({footprintBytes:900*MB,grantBytes})).toBeLessThan(0);expect(footprintHeadroomBytesV1({footprintBytes:1,grantBytes:undefined})).toBe(Infinity);
 const ajv=new Ajv({strict:false});ajv.addSchema(schema,"s");for(const percent of[0,70,99])expect(ajv.validate({$ref:"s#/definitions/Decision"},at(percent))).toBe(true);
});

test("the grant of a capacity source is the declared limit, else the reserved megabytes, else none",()=>{
 expect(grantBytesOfV1(()=>({limit:{memoryMegabytes:4096},handle:{reservedMegabytes:1}}))).toBe(4096*MB);
 expect(grantBytesOfV1(()=>({handle:{reservedMegabytes:512}}))).toBe(512*MB);expect(grantBytesOfV1(()=>undefined)).toBeUndefined();expect(grantBytesOfV1(undefined)).toBeUndefined();expect(grantBytesOfV1(()=>({handle:{reservedMegabytes:0}}))).toBeUndefined();
});

test("the reader answers the footprint of one process, equal to the system tool, and never the resident set when its reading fails",async()=>{
 const own=await readProcessFootprintBytesV1(process.pid);expect(own).toBeGreaterThan(8*MB);
 if(process.platform==="darwin"){
  const text=execFileSync("footprint",["-f","bytes","--noCategories","-p",String(process.pid)],{encoding:"utf8"}),tool=Number(/phys_footprint:\s+(\d+)\s+B/u.exec(text)?.[1]??/Footprint:\s+(\d+)\s+B/u.exec(text)?.[1]);
  expect(Math.abs(own!-tool)).toBeLessThan(Math.max(64*MB,tool*0.5));
 }
 if(process.platform==="linux"){
  const text=readFileSync(`/proc/${process.pid}/smaps_rollup`,"utf8"),kilobytes=(name:string)=>Number(new RegExp(`^${name}:\\s+(\\d+) kB`,"mu").exec(text)?.[1]??0);
  expect(Math.abs(own!-(kilobytes("Pss")+kilobytes("SwapPss"))*1024)).toBeLessThan(Math.max(64*MB,own!*0.5));
 }
 const failing=async()=>{throw Error("no tool");};
 expect(await readProcessFootprintBytesV1(1,{platform:"darwin",run:failing,rss:()=>99})).toBeUndefined();expect(await readProcessFootprintBytesV1(1,{platform:"linux",read:failing,rss:()=>99})).toBeUndefined();
 expect(await readProcessFootprintBytesV1(1,{platform:"darwin",run:async()=>"bun [1]: 64-bit    Footprint: 123456789 B\n    phys_footprint: 123456789 B\n",rss:()=>99})).toBe(123456789);
 expect(await readProcessFootprintBytesV1(process.pid,{platform:"win32",rss:()=>99})).toBe(99);
 expect(await readProcessFootprintBytesV1(1,{platform:"win32",rss:()=>99,workingSet:()=>undefined})).toBeUndefined();
});

test("a Darwin footprint batch samples exact distinct PIDs and keeps valid readings when one process disappears",async()=>{
 const db=new Database(":memory:");
 try{
  const pids=[501,502,503,504,505,506,507,508,509,501];
  const rows=pids.slice(0,9).filter(pid=>pid!==504).map(pid=>({pid,bytes:pid*4096}));
  const expected=db.query("select json_extract(probe.value,'$.bytes') as bytes from json_each(?1) request left join json_each(?2) probe on json_extract(probe.value,'$.pid')=request.value order by request.key").all(JSON.stringify(pids),JSON.stringify(rows)) as {bytes:number|null}[];
  const calls:string[][]=[];
  const probe=createDarwinFootprintProbeV1(async(command,args)=>{
   expect(command).toBe("footprint");
   calls.push([...args]);
   return rows.map(row=>`child [${row.pid}]: 64-bit  Footprint: 32 B\n    phys_footprint: ${row.bytes} B\n`).join("");
  });
  expect(await Promise.all(pids.map(pid=>probe(pid)))).toEqual(expected.map(row=>row.bytes??undefined));
  expect(calls).toEqual([["-f","bytes","--noCategories",...[...new Set(pids)].flatMap(pid=>["-p",String(pid)])]]);
 }finally{db.close();}
});

test("each Darwin footprint request cancels independently and the final cancellation aborts its command",async()=>{
 const started=Promise.withResolvers<AbortSignal>();
 const a=new AbortController(),b=new AbortController(),c=new AbortController();
 let calls=0;
 const probe=createDarwinFootprintProbeV1((_command,_args,signal)=>{
  if(++calls>1)return Promise.resolve("child [701]: 64-bit  Footprint: 4096 B\n");
  started.resolve(signal!);
  return new Promise((_resolve,reject)=>signal!.addEventListener("abort",()=>reject(signal!.reason),{once:true}));
 });
 const first=probe(700,a.signal),second=probe(701,b.signal),cancelled=probe(702,c.signal);
 c.abort();expect(await cancelled).toBeUndefined();
 const shared=await started.promise;
 a.abort();expect(await first).toBeUndefined();expect(shared.aborted).toBe(false);
 b.abort();expect(await second).toBeUndefined();expect(shared.aborted).toBe(true);
 expect(await probe(701)).toBe(4096);
 expect(calls).toBe(2);
});

test("Darwin footprint requests arriving during a probe wait for a fresh bounded batch",async()=>{
 const started=Promise.withResolvers<void>(),firstAnswer=Promise.withResolvers<string>();
 const calls:string[][]=[];
 const probe=createDarwinFootprintProbeV1((_command,args)=>{
  calls.push([...args]);
  if(calls.length===1){started.resolve();return firstAnswer.promise;}
  return Promise.resolve("child [801]: 64-bit  Footprint: 8192 B\n");
 });
 const first=probe(801);
 await started.promise;
 const following=Array.from({length:FOOTPRINT_POLICY_V1.processProbeBatchRequests},()=>probe(801));
 expect(await probe(801)).toBeUndefined();
 expect(calls).toHaveLength(1);
 firstAnswer.resolve("child [801]: 64-bit  Footprint: 4096 B\n");
 expect(await first).toBe(4096);
 expect(new Set(await Promise.all(following))).toEqual(new Set([8192]));
 expect(calls).toEqual(Array.from({length:2},()=>["-f","bytes","--noCategories","-p","801"]));
});

test.skipIf(process.platform!=="darwin")("the Darwin footprint tool returns a live PID beside a missing PID",async()=>{
 const probe=createDarwinFootprintProbeV1();
 const [present,missing]=await Promise.all([probe(process.pid),probe(2147483647)]);
 expect(present).toBeGreaterThan(8*MB);
 expect(missing).toBeUndefined();
});

test("Windows footprint reads exactly the requested process working set",async()=>{
 const {readWindowsProcessWorkingSetV1}=await import("../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪟️native-memory/🟦️.ts");
 const db=new Database(":memory:");
 try{
  expect(new Ajv({strict:true}).validate(schema.definitions.WindowsProcessCases,windowsCases)).toBe(true);
  for(const row of windowsCases.cases){
   const asked:number[]=[],closed:bigint[]=[];
   const oracle=db.query("select case json_extract(?1,'$.mode') when 'counter' then json_extract(?1,'$.workingSet') end bytes").get(JSON.stringify(row)) as {bytes:number|null};
   expect(oracle.bytes).toBe(row.expected);
   const handle=BigInt(row.pid)+90n,kernel={
    OpenProcess:(access:number,inherit:number,pid:number)=>{expect(access).toBe(0x1000);expect(inherit).toBe(0);asked.push(pid);return row.mode==="open-failed"?0n:handle;},
    K32GetProcessMemoryInfo:(actual:bigint,buffer:Buffer,size:number)=>{
     expect(actual).toBe(handle);expect(size).toBe(80);expect(buffer.readUInt32LE(0)).toBe(80);
     if(row.mode==="counter-threw")throw Error("unavailable");
     if(row.mode==="counter-failed")return 0;
     buffer.writeBigUInt64LE(BigInt(row.workingSet!),16);
     buffer.writeBigUInt64LE(BigInt(row.privateBytes!),72);
     return 1;
    },
    CloseHandle:(actual:bigint)=>{closed.push(actual);return 1;},
   };
   const bytes=await readProcessFootprintBytesV1(row.pid,{
    platform:"win32",
    rss:()=>{throw Error("Reader's RSS is unrelated");},
    run:async()=>{throw Error("Windows does not need a command");},
    workingSet:pid=>readWindowsProcessWorkingSetV1(kernel,pid),
   });
   expect(bytes).toBe(oracle.bytes??undefined);
   expect(asked).toEqual(row.mode==="invalid"?[]:[row.pid]);
   expect(closed).toEqual(row.mode==="invalid"||row.mode==="open-failed"?[]:[handle]);
  }
 }finally{db.close();}
});

test("footprint cancellation stops before a read and ignores an aborted result",async()=>{
 const controller=new AbortController();
 let calls=0;
 const run=async(_command:string,_args:readonly string[],signal?:AbortSignal)=>{
  calls++;
  expect(signal).toBe(controller.signal);
  controller.abort();
  return "bun [301]: 64-bit    Footprint: 123456789 B\n    phys_footprint: 123456789 B\n";
 };
 expect(await readProcessFootprintBytesV1(301,{platform:"darwin",run,signal:controller.signal})).toBeUndefined();
 expect(await readProcessFootprintBytesV1(301,{platform:"darwin",run,signal:controller.signal})).toBeUndefined();
 expect(calls).toBe(1);
});

test("the meter answers at once, reads in the background once per ttl, ignores a failed reading and prefers the reading to the resident set",async()=>{
 const time=clock();let calls=0,fail=false,value=500*MB;
 const meter=createFootprintMeterV1({read:async()=>{calls++;if(fail)throw Error("x");return value;},rss:()=>7000*MB,now:time.now,ttlMilliseconds:5000});
 expect(meter.bytes()).toBe(7000*MB);expect(calls).toBe(1);await meter.refresh();expect(meter.bytes()).toBe(500*MB);expect(calls).toBe(2);
 meter.bytes();meter.bytes();expect(calls).toBe(2);time.advance(5000);meter.bytes();meter.bytes();expect(calls).toBe(3);await meter.refresh();expect(calls).toBe(4);
 fail=true;value=900*MB;time.advance(6000);await meter.refresh();expect(meter.bytes()).toBe(500*MB);expect(meter.peakBytes()).toBe(500*MB);
});

test("fresh() reuses a reading younger than the ttl and reads again after it",async()=>{
 const time=clock();let calls=0;const meter=createFootprintMeterV1({read:async()=>{calls++;return 100*MB;},now:time.now,ttlMilliseconds:5000});
 await meter.fresh();await meter.fresh();await meter.fresh();expect(calls).toBe(1);time.advance(5000);await meter.fresh();expect(calls).toBe(2);
});

test("a gate below the soft limit admits at once and touches nothing; without a grant it never does",async()=>{
 const time=clock();let reads=0,released=0;
 const meter=createFootprintMeterV1({read:async()=>{reads++;return 300*MB;},now:time.now});await meter.refresh();reads=0;
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,release:()=>{released++;},sleep:time.sleep,now:time.now});
 for(let index=0;index<50;index++)expect(await gate.admit()).toBe("run");
 expect(released).toBe(0);expect(gate.stats().waits).toBe(0);expect(gate.stats().admitted).toBe(50);
 expect(await createFootprintGateV1({meter,grantBytes:()=>undefined,release:()=>{released++;}}).admit()).toBe("run");expect(released).toBe(0);
});

test("between soft and hard the gate only collects and shrinks, at most once per minimum, and lets the work run; the relievers stay untouched",async()=>{
 const time=clock(),lines:string[]=[],order:string[]=[];let held=700*MB;
 const meter=createFootprintMeterV1({read:async()=>held,now:time.now,ttlMilliseconds:0});
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,release:()=>{order.push("release");held=200*MB;},sleep:time.sleep,now:time.now,report:line=>lines.push(line),heapCensus:async()=>undefined,environment:{SEMIO_LANGUAGE:"en"}});
 gate.relieve(()=>{order.push("reliever");});
 expect(await gate.admit()).toBe("run");expect(order).toEqual(["release"]);expect(gate.stats().settled).toBe(1);
 expect(lines).toEqual(["graph: the process held 700 MB of its grant of 1000 MB; collected and shrunk to 200 MB"]);
 held=700*MB;await meter.refresh();expect(await gate.admit()).toBe("settle");expect(gate.stats().settled).toBe(1);
 time.advance(FOOTPRINT_POLICY_V1.settleMinimumMilliseconds);expect(await gate.admit()).toBe("run");expect(gate.stats().settled).toBe(2);expect(order).toEqual(["release","release"]);
});

test("at the hard limit the relievers drop what a pass can re-read after the collection did not help, and the work runs when that is enough",async()=>{
 const time=clock(),order:string[]=[];let held=950*MB;
 const meter=createFootprintMeterV1({read:async()=>held,now:time.now,ttlMilliseconds:0});
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,release:()=>{order.push("release");},sleep:time.sleep,now:time.now});
 const off=gate.relieve(()=>{order.push("reliever");held=300*MB;});
 expect(await gate.admit()).toBe("run");expect(order).toEqual(["release","reliever","release"]);expect(gate.stats().waits).toBe(0);
 off();expect(gate.relieve(()=>undefined)).toBeInstanceOf(Function);
});

test("when nothing helps the gate waits in steps, at most the bound per call, never forever, repeats no wait within the cooldown and stops when the footprint falls",async()=>{
 const time=clock(),lines:string[]=[];let held=950*MB;
 const meter=createFootprintMeterV1({read:async()=>held,now:time.now,ttlMilliseconds:0});
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,release:()=>undefined,sleep:time.sleep,now:time.now,report:line=>lines.push(line),heapCensus:async()=>undefined,environment:{SEMIO_LANGUAGE:"en,de"}});
 const began=time.now();expect(await gate.admit()).toBe("wait");
 expect(time.now()-began).toBe(FOOTPRINT_POLICY_V1.waitBoundMilliseconds);expect(gate.stats().waits).toBe(1);expect(gate.stats().waitedMilliseconds).toBe(FOOTPRINT_POLICY_V1.waitBoundMilliseconds);
 expect(lines.some(line=>line.startsWith("graph: the process holds 950 MB of its grant of 1000 MB; new work waits up to 20 s"))).toBe(true);expect(lines.some(line=>line.startsWith("Graph: der Prozess hält 950 MB"))).toBe(true);
 const after=time.now();for(let index=0;index<5;index++)expect(await gate.admit()).toBe("wait");
 expect(time.now()).toBe(after);expect(gate.stats().waits).toBe(1);
 time.advance(FOOTPRINT_POLICY_V1.waitCooldownMilliseconds);expect(await gate.admit()).toBe("wait");expect(gate.stats().waits).toBe(2);
 const calm=createFootprintGateV1({meter,grantBytes:()=>1000*MB,release:()=>undefined,sleep:async milliseconds=>{time.advance(milliseconds);held=300*MB;},now:time.now});
 const start=time.now();expect(await calm.admit()).toBe("run");expect(time.now()-start).toBeLessThanOrEqual(FOOTPRINT_POLICY_V1.waitStepMilliseconds);
});

test("a wait is cancellable",async()=>{
 const meter=createFootprintMeterV1({read:async()=>990*MB,ttlMilliseconds:0}),controller=new AbortController();
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,release:()=>undefined});
 const asking=gate.admit(controller.signal);setTimeout(()=>controller.abort(Error("stop")),30);
 await expect(asking).rejects.toThrow("stop");
});

test("the governor starts no further owner while the footprint of the parent stands above its limit, and the first owner always runs",async()=>{
 let released=false,running=0,heldPeak=0,done=0;const started:string[]=[];
 const governor=createMemoryGovernorV1({workers:4,headroom:()=>1e12,hold:()=>!released,estimateBytes:MB,peaks:{read:()=>({k:[MB]}),write:()=>undefined}});
 const task=(name:string,first:boolean)=>async()=>{started.push(name);running++;if(!released)heldPeak=Math.max(heldPeak,running);await Bun.sleep(first?400:20);running--;done++;if(first)released=true;};
 await Promise.all([governor.run(task("a",true),{kind:"k"}),governor.run(task("b",false),{kind:"k"}),governor.run(task("c",false),{kind:"k"})]);governor.dispose();
 expect(done).toBe(3);expect(started[0]).toBe("a");expect(heldPeak).toBe(1);
});

test("the parent governor takes its hold and the service headroom from the footprint gate of the process, not from the resident set",async()=>{
 expect(footprintGateV1()).toBeUndefined();
 const meter=createFootprintMeterV1({read:async()=>9000*MB,ttlMilliseconds:0});await meter.refresh();
 const gate=createFootprintGateV1({meter,grantBytes:()=>10000*MB,release:()=>undefined});useFootprintGateV1(gate);
 try{
  const declared=serviceMemoryHeadroomBytesV1({environment:{SEMIO_GRAPH_MEMORY_LIMIT_MB:"10000"}});
  expect(declared).toBeLessThan(10000*MB-9000*MB);
  expect(gate.headroomBytes()).toBeLessThan(0);
  const governor=createGraphParentMemoryGovernorV1({workers:2,headroom:()=>1e12});expect(governor).toBeDefined();
 }finally{useFootprintGateV1(undefined);}
 expect(footprintGateV1()).toBeUndefined();
});

test("the wave settling logs ONE line per language with the footprint before and after and survives every failure",async()=>{
 const lines:string[]=[];let held=8000*MB,dropped=0;
 const meter=createFootprintMeterV1({read:async()=>held,ttlMilliseconds:0});
 const result=await settleWaveMemoryV1({generation:12,release:()=>{dropped++;},meter,managed:()=>300*MB,grantBytes:()=>10462*MB,collect:()=>{held=1500*MB;},heapCensus:async()=>undefined,report:line=>lines.push(line),environment:{SEMIO_LANGUAGE:"en,de"}});
 expect(result).toEqual({beforeMegabytes:8000,afterMegabytes:1500,managedMegabytes:300,grantMegabytes:10462});expect(dropped).toBe(1);
 expect(lines).toEqual(["graph: wave 12 settled from 8000 MB to 1500 MB (managed 300 MB, grant 10462 MB)","Graph: Welle 12 beruhigt von 8000 MB auf 1500 MB (verwaltet 300 MB, Zusage 10462 MB)"]);
 expect(await settleWaveMemoryV1({generation:1,release:()=>{throw Error("boom");},meter,report:()=>undefined})).toBeUndefined();
});

test("releaseRuntimeMemoryV1 collects, shrinks and collects again, in that order",()=>{
 const order:string[]=[];releaseRuntimeMemoryV1({collect:()=>order.push("collect"),shrink:()=>order.push("shrink")});expect(order).toEqual(["collect","shrink","collect"]);
});

const SIZE={projects:5000,waves:20,index:160*MB,rowsPerProject:300};
async function syntheticRead(settle:boolean){
 const meter=createFootprintMeterV1({ttlMilliseconds:0}),per=SIZE.projects/SIZE.waves,footprints:number[]=[],heaps:number[]=[];
 let kept:unknown[][]=[],persisted=0;
 for(let wave=1;wave<=SIZE.waves;wave++){
  const index=Buffer.alloc(SIZE.index,wave&255),patched=Buffer.from(index),packed=Buffer.from(patched.subarray(0,SIZE.index>>3));
  const rows:unknown[][]=[];
  for(let project=0;project<per;project++){const one:unknown[]=[];for(let row=0;row<SIZE.rowsPerProject;row++)one.push({kind:"file",path:`p${wave}/${project}/${row}/${"x".repeat(48)}`,fact:{size:row,digest:"d".repeat(40)+row}});rows.push(one);}
  persisted+=packed.length;
  kept=settle?rows:kept.concat(rows);
  if(settle)releaseRuntimeMemoryV1();
  footprints.push(Math.round((await meter.refresh())/MB));
  Bun.gc(true);heaps.push(Math.round(process.memoryUsage().heapUsed/MB));
 }
 return{footprints,heaps,persisted,kept:kept.length};
}

test("a synthetic read of 5000 projects over 20 waves in one process holds a flat footprint from wave 3 on once every wave settles",async()=>{
 const settled=await syntheticRead(true);
 const tail=settled.footprints.slice(2),floor=Math.min(...tail),ceiling=Math.max(...tail);
 console.log("[R35] settled footprint MB per wave:",settled.footprints.join(" "));console.log("[R35] settled managed heap MB per wave:",settled.heaps.join(" "));
 expect(settled.footprints.length).toBe(20);
 expect(ceiling-floor).toBeLessThan(Math.max(320,floor*1.5));
 expect(ceiling).toBeLessThan(900);
 expect(Math.max(...settled.heaps.slice(2))-Math.min(...settled.heaps.slice(2))).toBeLessThan(80);
},180000);

test("without the settling the same read keeps all waves and its managed heap grows with them (control)",async()=>{
 const retained=await syntheticRead(false);
 console.log("[R35] retained footprint MB per wave:",retained.footprints.join(" "));
 expect(retained.kept).toBe(SIZE.projects);
 console.log("[R35] retained managed heap MB per wave:",retained.heaps.join(" "));expect(retained.heaps[19]).toBeGreaterThan(100);expect(retained.heaps[19]).toBeGreaterThan(retained.heaps[2]*3);
},180000);

test("the closure settle asks the gate of the process before every derivation, registers a reliever that drops the pass round while it runs and removes it afterwards",async()=>{
 const asked:string[]=[],relievers=new Set<()=>unknown>();let dropped=0;
 const gate={admit:async()=>{asked.push("admit");return"run" as const;},relieve:(reliever:()=>unknown)=>{relievers.add(reliever);return()=>relievers.delete(reliever);},watch:()=>undefined,stop:()=>undefined,settle:async()=>0,headroomBytes:()=>Infinity,footprintBytes:()=>0,grantBytes:()=>undefined,stats:()=>({admitted:0,settled:0,waits:0,waitedMilliseconds:0,peakBytes:0})};
 useFootprintGateV1(gate);
 try{
  const rows:GenerationRowV1[]=Array.from({length:6},(_,index)=>({kind:"derived",path:`q${index}`,fact:"answer"})),names=rows.map(derivedOwnerNameV1);
  const index=createFactIndexV1(new Map([["holder",rows],...rows.map((row,at)=>[names[at],[row,{kind:"file",path:`s${at}`,fact:"h"}]] as[string,GenerationRowV1[]])]));
  let seen=0;
  const derive=async(_row:unknown,_signal:unknown,pass:{round?:Map<string,unknown>})=>{pass.round??=new Map([["k",1]]);seen++;if(seen===3){for(const reliever of relievers)reliever();dropped=pass.round===undefined?1:0;}return{fact:"answer",rows:[{kind:"file",path:"s",fact:"h"}]};};
  await createClosureSettlerV1().settle({pseudoRows:suspectRowsV1(index,new Set(names)),derive:derive as never,hold:async()=>true});
  expect(asked.length).toBeGreaterThanOrEqual(rows.length);expect(dropped).toBe(1);expect(relievers.size).toBe(0);
 }finally{useFootprintGateV1(undefined);}
});

test("binding a capacity grant to the process installs the gate with that grant, unbinding removes it",()=>{
 try{
  useCapacityGrantV1(()=>({handle:{reservedMegabytes:2048},limit:{memoryMegabytes:1024}}));
  expect(footprintGateV1()?.grantBytes()).toBe(1024*MB);
  useCapacityGrantV1(undefined);expect(footprintGateV1()).toBeUndefined();
 }finally{useCapacityGrantV1(undefined);}
});

test("the census names the heap by counters without walking it, in both languages, and survives a runtime without counters",async()=>{
 const stats=async()=>({heapSize:300*MB,extraMemorySize:120*MB,objectCount:5000000,objectTypeCounts:{Array:3000000,Map:900,String:1200000,Object:800000,Set:10,Function:5}});
 expect(await heapCensusV1({stats,types:3})).toEqual({heapMegabytes:300,extraMegabytes:120,objects:5000000,types:"Array 3000000, String 1200000, Object 800000"});
 expect(await heapCensusV1({stats:async()=>{throw Error("none");}})).toBeUndefined();
 const real=await heapCensusV1();expect(real?.objects).toBeGreaterThan(0);
 const sample=JSON.parse(readFileSync(new URL("./census-settle.json",import.meta.url),"utf8"));
 const db=new Database(":memory:");
 const {megabytes}=db.query("select value ->> 'megabytes' as megabytes from json_each(?) where value ->> 'phase' = ?").get(JSON.stringify(sample.measurements),sample.censusPhase) as {megabytes:number};
 db.close();
 const lines:string[]=[];let held=sample.measurements[0].megabytes*MB;
 const meter=createFootprintMeterV1({read:async()=>held,ttlMilliseconds:0});
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,release:()=>{held=sample.measurements[1].megabytes*MB;},heapCensus:()=>heapCensusV1({stats}),report:line=>lines.push(line),environment:{SEMIO_LANGUAGE:"en,de"}});
 await gate.admit();
 expect(lines).toContain(`graph: heap census at ${megabytes} MB: managed 300 MB, external 120 MB, 5000000 objects; most numerous: Array 3000000, String 1200000, Object 800000, Map 900, Set 10, Function 5`);
 expect(lines.some(line=>line.startsWith(`Graph: Heap-Zählung bei ${megabytes} MB: verwaltet 300 MB, extern 120 MB, 5000000 Objekte; am zahlreichsten: Array 3000000`))).toBe(true);
});

test("the watcher settles a process whose footprint passed the soft limit while no derivation asks, and stops with the gate",async()=>{
 const time=clock(),calls:number[]=[];let held=700*MB,tick:(()=>unknown)|undefined,timerCleared=0;
 const meter=createFootprintMeterV1({read:async()=>held,now:time.now,ttlMilliseconds:0});await meter.refresh();
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,release:()=>{calls.push(held);held=200*MB;},now:time.now,setTimer:(callback:()=>unknown)=>{tick=callback;return{unref:()=>undefined};},clearTimer:()=>{timerCleared++;}});
 gate.watch();gate.watch();expect(typeof tick).toBe("function");
 await tick!();expect(calls).toEqual([700*MB]);
 held=900*MB;await meter.refresh();await tick!();expect(calls.length).toBe(1);
 time.advance(FOOTPRINT_POLICY_V1.settleMinimumMilliseconds);await tick!();expect(calls.length).toBe(2);
 gate.stop();gate.stop();expect(timerCleared).toBe(1);
 useFootprintGateV1(gate);useFootprintGateV1(undefined);
});

test("the trace appends every line with its time to a file of this process, stops at its bound and never throws",()=>{
 const directory=mkdtempSync(join(tmpdir(),"footprint-trace-"));
 try{
  const trace=createFootprintTraceV1({directory:join(directory,"t"),pid:4242,maximumBytes:120,now:()=>"2026-11-15T09:00:00.000Z"});
  trace("graph: wave 1 settled from 9000 MB to 900 MB");trace("graph: wave 2 settled from 9000 MB to 900 MB");trace("graph: wave 3 settled from 9000 MB to 900 MB");trace("graph: wave 4 settled from 9000 MB to 900 MB");
  const text=readFileSync(trace.path,"utf8");
  expect(trace.path.endsWith("4242.log")).toBe(true);expect(text.startsWith("2026-11-15T09:00:00.000Z graph: wave 1 settled")).toBe(true);expect(text.split("\n").filter(Boolean).length).toBeLessThan(4);
  expect(()=>createFootprintTraceV1({directory:"/proc/none/x"})("x")).not.toThrow();
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the watcher writes a heartbeat row every interval, also while the footprint is far below the soft limit",async()=>{
 const time=clock(),rows:{footprintBytes:number;grantBytes:number}[]=[];let tick:(()=>unknown)|undefined;
 const meter=createFootprintMeterV1({read:async()=>100*MB,now:time.now,ttlMilliseconds:0});await meter.refresh();
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,now:time.now,heartbeat:row=>rows.push(row),setTimer:(callback:()=>unknown)=>{tick=callback;return{unref:()=>undefined};},clearTimer:()=>undefined});
 gate.watch();await tick!();await tick!();expect(rows.length).toBe(1);expect(rows[0]!.footprintBytes).toBe(100*MB);expect(rows[0]!.grantBytes).toBe(1000*MB);
 time.advance(FOOTPRINT_POLICY_V1.heartbeatMilliseconds);await tick!();expect(rows.length).toBe(2);gate.stop();
});

test("the sampler writes a row on the first look, on growth, on a blocked loop and after the quiet interval, with the 15 most numerous types, and nothing in between",async()=>{
 const P=FOOTPRINT_POLICY_V1,time=clock(),rows:Record<string,string|number>[]=[];let rss=500*MB,footprint=400*MB;
 const counts=Object.fromEntries(Array.from({length:20},(_,index)=>["T"+index,1000-index]));
 const sampler=createHeapSamplerV1({meter:{bytes:()=>footprint},stats:async()=>({heapSize:300*MB,extraMemorySize:50*MB,objectCount:777,objectTypeCounts:counts}),usage:()=>({rss,arrayBuffers:3*MB}),now:time.now,phase:()=>"step-a",emit:fields=>rows.push(fields)});
 await sampler.tick();expect(rows.length).toBe(1);
 expect(rows[0]).toMatchObject({phase:"step-a",footprint:400,rss:500,heap:300,extra:50,buffers:3,objects:777,lag:0});
 expect(String(rows[0]!.types).split(", ").length).toBe(P.sampleTypes);expect(String(rows[0]!.types).startsWith("T0 1000, T1 999")).toBe(true);
 time.advance(P.sampleMilliseconds);await sampler.tick();expect(rows.length).toBe(1);
 rss+=P.sampleGrowthBytes;time.advance(P.sampleMilliseconds);await sampler.tick();expect(rows.length).toBe(2);expect(rows[1]!.moved).toBe(P.sampleGrowthBytes/MB);
 rss-=P.sampleGrowthBytes;time.advance(P.sampleMilliseconds);await sampler.tick();expect(rows.length).toBe(3);
 time.advance(P.sampleMilliseconds+P.sampleLagMilliseconds);await sampler.tick();expect(rows.length).toBe(4);expect(rows[3]!.lag).toBe(P.sampleLagMilliseconds);
 time.advance(P.sampleQuietMilliseconds);await sampler.tick();expect(rows.length).toBe(5);
});

test("the sampler survives a runtime without counters and never overlaps two countings",async()=>{
 const time=clock(),rows:unknown[]=[];let release:(()=>void)|undefined,calls=0;
 const failing=createHeapSamplerV1({meter:{bytes:()=>1},stats:async()=>{throw Error("none");},usage:()=>({rss:1}),now:time.now,emit:fields=>rows.push(fields)});
 await expect(failing.tick()).resolves.toBeUndefined();expect(rows.length).toBe(0);
 const slow=createHeapSamplerV1({meter:{bytes:()=>1},stats:()=>{calls++;return new Promise(resolve=>{release=()=>resolve({heapSize:1});});},usage:()=>({rss:1}),now:time.now,emit:fields=>rows.push(fields)});
 const first=slow.tick();await slow.tick();expect(calls).toBe(1);release!();await first;expect(rows.length).toBe(1);
});

test("the real runtime counters of the sampler equal the managed heap the runtime reports (independent reading)",async()=>{
 const rows:Record<string,string|number>[]=[],keep=Array.from({length:50000},(_,index)=>({index,text:"s"+index}));
 const sampler=createHeapSamplerV1({meter:{bytes:()=>process.memoryUsage().rss},emit:fields=>rows.push(fields)});
 await sampler.tick();const usage=process.memoryUsage();
 expect(rows.length).toBe(1);expect(Math.abs(Number(rows[0]!.heap)-usage.heapUsed/MB)).toBeLessThan(64);expect(String(rows[0]!.types)).toContain("Object ");expect(keep.length).toBe(50000);
});

test("the gate with a sample port runs a second timer for the sampler, in both languages with the step, and stops both timers",async()=>{
 const lines:string[]=[],callbacks:(()=>unknown)[]=[];let cleared=0;
 const meter=createFootprintMeterV1({read:async()=>100*MB,ttlMilliseconds:0});await meter.refresh();
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,sample:line=>lines.push(line),phase:()=>"inference 7 of 4774",environment:{SEMIO_LANGUAGE:"en,de"},setTimer:(callback:()=>unknown)=>{callbacks.push(callback);return{unref:()=>undefined};},clearTimer:()=>{cleared++;}});
 gate.watch();gate.watch();expect(callbacks.length).toBe(2);
 await callbacks[1]!();await new Promise(resolve=>setTimeout(resolve,50));
 expect(lines.length).toBe(2);expect(lines[0]!.startsWith("graph: sample step inference 7 of 4774: footprint 100 MB")).toBe(true);expect(lines[1]!.startsWith("Graph: Probe Schritt inference 7 of 4774: Fußabdruck 100 MB")).toBe(true);
 gate.stop();expect(cleared).toBe(2);
});

test("a gate without a sample port starts no sampler",()=>{
 const callbacks:unknown[]=[];const gate=createFootprintGateV1({meter:createFootprintMeterV1({read:async()=>1}),grantBytes:()=>1000*MB,setTimer:(callback:unknown)=>{callbacks.push(callback);return{unref:()=>undefined};},clearTimer:()=>undefined});
 gate.watch();expect(callbacks.length).toBe(1);gate.stop();
});

test("a process that holds no grant object but runs inside a granted run installs its gate from the reservation of the environment, once, and follows a grown reservation",()=>{
 let reserved:number|undefined=4096,at=0;const binds:(()=>unknown)[]=[];
 try{
  expect(useEnvironmentGateV1({},{read:()=>reserved})).toBeUndefined();
  const env={SEMIO_HOST_CAPACITY_GRANT:"abc"},ports={read:()=>reserved,now:()=>at,bind:(source:()=>unknown)=>{binds.push(source);return bindProcessGateV1(source,{trace:()=>undefined,write:()=>undefined});}};
  const gate=useEnvironmentGateV1(env,ports);
  expect(gate?.grantBytes()).toBe(4096*MB);expect(footprintGateV1()).toBe(gate);
  expect(useEnvironmentGateV1(env,ports)).toBe(gate);expect(binds.length).toBe(1);
  reserved=8192;expect(gate?.grantBytes()).toBe(4096*MB);at+=FOOTPRINT_POLICY_V1.grantTtlMilliseconds;expect(gate?.grantBytes()).toBe(8192*MB);
  useFootprintGateV1(undefined);reserved=undefined;at+=1;expect(useEnvironmentGateV1(env,{...ports,read:()=>{throw Error("none");}})).toBeUndefined();
  expect(useEnvironmentGateV1(env,{...ports,read:()=>undefined})).toBeUndefined();
 }finally{useFootprintGateV1(undefined);}
});

//R36b
test("a cold part engages the farm, a warm part does not, and an engaged farm opens at the first miss while a not engaged one waits for 24 misses",async()=>{
 expect(partInferenceEngagementV1({cold:true})).toEqual({coldInference:true});expect(partInferenceEngagementV1({cold:false})).toEqual({});
 const asked:number[]=[];
 const open=(engage:boolean)=>openProjectFarmV1({workspaceRoot:"/w",context:{registry:null} as any,contracts:{},closureStore:undefined,projects:5000,commandInputs:undefined,lateClosures:true,projectsByRoot:new Map(),componentRoots:[],environment:{SEMIO_GRAPH_FARM:"on",SEMIO_LANGUAGE:"en"} as any,launch:null as any,cores:8,engage,request:((input:any)=>{asked.push(input.preferredJobs);return{granted:Promise.resolve(undefined),release:()=>undefined};}) as any,spawnProcess:(() => {throw Error("no process in this proof");}) as any,write:()=>undefined});
 try{
  const cold=open(true);expect(asked.length).toBe(1);await cold?.close();await closeProjectFarmsV1();
  asked.length=0;const warm=open(false);expect(asked.length).toBe(0);
  for(let miss=0;miss<23;miss++)await warm?.prepare({name:"p"+miss,root:"r"+miss,json:{},native:false,pipeline:false,nativeRoot:"r",selectedInputs:[],head:[],cacheKey:"k"});
  expect(asked.length).toBe(0);
  await warm?.close();
 }finally{await closeProjectFarmsV1().catch(()=>undefined);}
});
test("a slow stretch names itself in the trace, a fast one does not",()=>{
 const lines:string[]=[];
 expect(noteSlowV1("fast",Date.now(),2000,line=>lines.push(line))).toBeLessThan(2000);
 const spent=noteSlowV1("typescript-program x",Date.now()-2500,2000,line=>lines.push(line));
 expect(spent).toBeGreaterThanOrEqual(2500);expect(lines.length).toBe(1);expect(lines[0]).toMatch(/^slow typescript-program x 2[5-9]\d\d ms$/u);
});

test("the walk caches of the owners are released every declared interval whatever the footprint reads and whatever the governor's headroom said; only the shrink and the spill of entries follow the footprint",()=>{
 const graph={nodes:1},scripts=createScriptInputCache({workspaceRoot:"/w"}) as any,fill=()=>{scripts.files.set("a",1);scripts.closures.set("b",1);scripts.commandDependencies.set("c",1);scripts.walkNodes=new Map([["n",1]]);scripts.sharedBytes=7;scripts.closureGraph=graph;};
 const gate=(share:number)=>({grantBytes:()=>1000*MB,footprintBytes:()=>1000*MB*share/100});
 const interval=INFERENCE_LIMITS_V1.ownersPerRelease;
 fill();for(let owner=0;owner<interval-1;owner++)releaseOwnerStateV1(scripts,{shrink:true,gate:gate(30)});
 expect([scripts.files.size,scripts.closures.size,scripts.commandDependencies.size,scripts.walkNodes.size,scripts.sharedBytes]).toEqual([1,1,1,1,7]);
 releaseOwnerStateV1(scripts,{shrink:true,gate:gate(30)});
 expect([scripts.files.size,scripts.closures.size,scripts.commandDependencies.size,scripts.walkNodes.size,scripts.sharedBytes]).toEqual([0,0,0,0,0]);expect(scripts.closureGraph,"the module graph outlives every release: the next owner's walks are lookups").toBe(graph);
 fill();for(let owner=0;owner<interval-1;owner++)releaseOwnerStateV1(scripts,{gate:gate(70)});
 expect(scripts.walkNodes.size).toBe(1);
 releaseOwnerStateV1(scripts,{gate:gate(70)});
 expect([scripts.files.size,scripts.walkNodes.size,scripts.sharedBytes]).toEqual([0,0,0]);expect(scripts.closureGraph).toBe(graph);
 fill();releaseOwnerStateV1(scripts,{gate:gate(90)});
 expect([scripts.files.size,scripts.closures.size,scripts.commandDependencies.size]).toEqual([0,0,0]);expect(scripts.walkNodes.size).toBe(1);
 fill();for(let owner=0;owner<40;owner++)releaseOwnerStateV1(scripts,{});
 expect(scripts.walkNodes.size).toBeLessThanOrEqual(1);
});

test("programs of different owners share their parsed source files, a changed file is parsed again and only it, and the closure follows the change",()=>{
 const root=join(tmpdir(),"footprint-shared-"+process.pid);mkdirSync(join(root,"a"),{recursive:true});mkdirSync(join(root,"b"),{recursive:true});
 const write=(path:string,text:string)=>writeFileSync(join(root,path),text);
 write("shared.ts","export const shared = 1;\n");write("other.ts","export const other = 1;\n");write("extra.ts","export const extra = 1;\n");
 for(const name of["a","b"]){write(name+"/tsconfig.json",JSON.stringify({compilerOptions:{noLib:true,types:[]},files:["x.ts","../shared.ts","../other.ts"]}));write(name+"/x.ts","import \"../shared\";\n");}
 const parsed:string[]=[],listing=(path:string)=>{try{return require("node:fs").readdirSync(path,{withFileTypes:true}).map((entry:any)=>({name:entry.name,directory:entry.isDirectory(),file:entry.isFile()}));}catch{return undefined;}};
 const compiler={...ts,createCompilerHost:(options:any,setParents:boolean)=>{const host=ts.createCompilerHost(options,setParents),original=host.getSourceFile.bind(host);host.getSourceFile=(name:string,...rest:any[])=>{parsed.push(name.split("/").pop()!);return(original as any)(name,...rest);};return host;}};
 const shared=createSharedSourceFilesV1(),run=(config:string)=>declaredTypeScriptProjectsV1(["{projectRoot}/"+config+"/tsconfig.json"],root,".",{compiler,host:ts.sys,readdir:listing,join,dirname:require("node:path").dirname,isAbsolute:require("node:path").isAbsolute,reuse:shared,release:()=>undefined});
 try{
  const first=run("a");expect(parsed.filter(name=>name==="shared.ts").length).toBe(1);
  parsed.length=0;const second=run("b");
  expect(parsed.includes("shared.ts")).toBe(false);expect(parsed.includes("other.ts")).toBe(false);expect(second.files.map((path:string)=>path.split("/").pop()).sort()).toEqual(first.files.map((path:string)=>path.split("/").pop()).sort().map((name:string)=>name));
  write("shared.ts","import \"./extra\";\nexport const shared = 2;\n");
  parsed.length=0;const third=run("b");
  expect(parsed.filter(name=>name==="shared.ts").length).toBe(1);expect(parsed.includes("other.ts")).toBe(false);
  expect(third.files.some((path:string)=>path.endsWith("/extra.ts"))).toBe(true);expect(first.files.some((path:string)=>path.endsWith("/extra.ts"))).toBe(false);
  shared.clear();parsed.length=0;run("a");expect(parsed.includes("other.ts")).toBe(true);
 }finally{require("node:fs").rmSync(root,{recursive:true,force:true});}
});

test("the shared source files are bounded: past the maximum they are dropped as a whole and parsing goes on correctly",()=>{
 const shared=createSharedSourceFilesV1({maximumFiles:2}),calls:string[]=[],original=(name:string)=>{calls.push(name);return{fileName:name,text:"t"};};
 const read=()=>"t",get=shared.sourceFile("c",original as any,read,"s");
 for(const name of["1","2","3","1"])get(name,99);
 expect(shared.size()).toBeLessThanOrEqual(2);expect(calls.length).toBeGreaterThanOrEqual(3);
});
test("a learned peak that garbage inflated is clamped to the ceiling, persisted clamped, and the owners of the kind are admitted together at once instead of one per wait bound",async()=>{
 const ceiling=2048*MB,written:Record<string,number[]>[]=[];let time=0;
 const governor=createMemoryGovernorV1({workers:4,headroom:()=>6000*MB,estimateBytes:256*MB,sleep:async(milliseconds:number)=>{time+=milliseconds;},now:()=>time,peaks:{read:()=>({project:[23101*MB,14*MB,9617*MB]}),write:value=>{written.push(value as Record<string,number[]>);}}});
 await Promise.all([1,2,3,4].map(index=>governor.run(async()=>undefined,{kind:"project",key:"p"+index})));
 expect(time).toBeLessThan(6000);
 expect(governor.stats().ownerCostMegabytes).toBeLessThanOrEqual(2048);
 expect(governor.stats().kinds.project).toBeLessThanOrEqual(2048);
 for(const row of written)for(const samples of Object.values(row))for(const bytes of samples)expect(bytes).toBeLessThanOrEqual(ceiling);
});
test("the process that runs a graph-reading command of the build tool carries the collector cap and the allocator purge, a command that never loads the graph does not",()=>{
 const base={PATH:"/bin",KEEP:"1"},cap=graphChildEnvironmentV1();
 for(const args of[["show","projects"],["build","some-project"],["test","some-project"]]){
  const row=implementationProcessEnvironmentV1(args,base,{});
  expect(row).toMatchObject({...cap,PATH:"/bin",KEEP:"1",BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"});
 }
 for(const args of[["doctor"],["tools","list"],["capacity"]]){
  const row=implementationProcessEnvironmentV1(args,base,{});
  for(const key of Object.keys(cap))expect(row[key]).toBeUndefined();
  expect(row.BUN_RUNTIME_TRANSPILER_CACHE_PATH).toBe("0");
 }
 expect(cap.BUN_JSC_forceRAMSize).toBe(String(512*1048576));
});

test("releaseIfGrownV1 releases once the managed memory grew by the threshold, moves its baseline to the live size and counts calls when asked to look rarely",()=>{
 let used=100*MB;const released:number[]=[];
 const call=(more:Record<string,unknown>={})=>releaseIfGrownV1({usage:()=>used,release:()=>{released.push(used);used=150*MB;},threshold:200*MB,...more});
 expect(call()).toBe(false);
 used=400*MB;expect(call()).toBe(true);expect(released).toEqual([400*MB]);
 used=300*MB;expect(call()).toBe(false);
 used=360*MB;expect(call()).toBe(true);expect(released.length).toBe(2);
 used=900*MB;let looks=0;const seen=()=>{looks++;return used;};
 for(let index=0;index<7;index++)releaseIfGrownV1({usage:seen,release:()=>undefined,threshold:200*MB,every:8});
 expect(looks).toBeLessThanOrEqual(1);
});

test("the declared TypeScript closure releases the runtime between its programs and after the last one, but never when a reuse store keeps the programs",()=>{
 const root=join(tmpdir(),"footprint-programs-"+process.pid);mkdirSync(join(root,"a"),{recursive:true});mkdirSync(join(root,"b"),{recursive:true});
 for(const name of["a","b"]){writeFileSync(join(root,name,"tsconfig.json"),JSON.stringify({compilerOptions:{noLib:true,types:[]},files:["x.ts"]}));writeFileSync(join(root,name,"x.ts"),"export const x = 1;\n");}
 try{
  const calls:string[]=[],listing=(path:string)=>{try{return require("node:fs").readdirSync(path,{withFileTypes:true}).map((entry:any)=>({name:entry.name,directory:entry.isDirectory(),file:entry.isFile()}));}catch{return undefined;}};
  const environment=(more:Record<string,unknown>)=>({compiler:ts,host:ts.sys,readdir:listing,join,dirname:require("node:path").dirname,isAbsolute:require("node:path").isAbsolute,...more});
    const result=declaredTypeScriptProjectsV1(["{projectRoot}/a/tsconfig.json","{projectRoot}/b/tsconfig.json"],root,".",environment({release:()=>{calls.push("release");}}));
  expect(result.configs.length).toBe(2);expect(calls.length).toBe(3);
  calls.length=0;
  declaredTypeScriptProjectsV1(["{projectRoot}/a/tsconfig.json"],root,".",environment({release:()=>{calls.push("release");},reuse:{sourceFile:(_c:string,original:unknown)=>original,oldProgram:()=>undefined,keep:()=>undefined}}));
  expect(calls.length).toBe(0);
 }finally{require("node:fs").rmSync(root,{recursive:true,force:true});}
});

test("releasing between the TypeScript programs keeps the managed memory flat where the control grows with every program (measured in a child process)",()=>{
 const script=`
  const ts=require(${JSON.stringify(ts.sys.getExecutingFilePath())}),fs=require("node:fs"),path=require("node:path"),os=require("node:os");
  (async()=>{
  const {declaredTypeScriptProjectsV1}=await import(${JSON.stringify(fileURLToPath(new URL("../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs",import.meta.url)))});
  const {releaseIfGrownV1}=await import(${JSON.stringify(fileURLToPath(new URL("../🟨️.mjs",import.meta.url)))});
  const root=fs.mkdtempSync(path.join(os.tmpdir(),"progs-"));fs.writeFileSync(path.join(root,"tsconfig.json"),JSON.stringify({compilerOptions:{types:[]},files:["x.ts"]}));fs.writeFileSync(path.join(root,"x.ts"),"export const x = 1;\\n");
  const listing=p=>{try{return fs.readdirSync(p,{withFileTypes:true}).map(e=>({name:e.name,directory:e.isDirectory(),file:e.isFile()}));}catch{return undefined;}};
  const mode=process.env.R36_MODE;let peak=0;
  for(let index=0;index<10;index++){
   declaredTypeScriptProjectsV1(["{projectRoot}/tsconfig.json"],root,".",{compiler:ts,host:ts.sys,readdir:listing,join:path.join,dirname:path.dirname,isAbsolute:path.isAbsolute,release:mode==="release"?()=>releaseIfGrownV1({threshold:64*1048576}):()=>undefined});
   const usage=process.memoryUsage();peak=Math.max(peak,usage.heapUsed+usage.external);
  }
  fs.rmSync(root,{recursive:true,force:true});console.log(Math.round(peak/1048576));})();`;
  const measure=(mode:string)=>{const run=spawnSync(process.execPath,[...(process.env.R5_PRELOAD?["--preload",process.env.R5_PRELOAD]:[]),"--eval",script],{encoding:"utf8",env:{...process.env,R36_MODE:mode}});expect(run.status,String(run.stderr)+String(run.error)).toBe(0);return Number(run.stdout.trim());};
 const control=measure("control"),released=measure("release");
 expect(control).toBeGreaterThan(0);expect(released).toBeGreaterThan(0);
 expect(released).toBeLessThan(control*0.9);
});

test("a starved loop makes the sampler speak from hot code: stack, counts and the blocked time, rate limited, and nothing while the loop runs",()=>{
 const time=clock(),rows:{key?:string;fields:Record<string,string|number>}[]=[];let tick:(()=>unknown)|undefined,now=1_000_000;
 const realNow=Date.now;
 const meter=createFootprintMeterV1({read:async()=>100*MB,ttlMilliseconds:0});
 const gate=createFootprintGateV1({meter,grantBytes:()=>1000*MB,sample:line=>rows.push({fields:{line}}),phase:()=>"owners 0 of 5102",now:()=>now,environment:{SEMIO_LANGUAGE:"en,de"},setTimer:(callback:()=>unknown)=>{tick=callback;return{unref:()=>undefined};},clearTimer:()=>undefined});
 try{
  gate.watch();expect(typeof tick).toBe("function");
  Date.now=()=>now;
  for(let index=0;index<1024;index++)footprintPulseV1();
  expect(rows.length).toBe(0);
  now+=FOOTPRINT_POLICY_V1.pulseMilliseconds+1;
  for(let index=0;index<256;index++)footprintPulseV1();
  expect(rows.length).toBe(2);
  expect(rows[0]!.fields.line).toContain("graph: starved sample step owners 0 of 5102: blocked ");expect(rows[0]!.fields.line).toContain("stack ");expect(String(rows[1]!.fields.line).startsWith("Graph: Hungerprobe Schritt owners 0 of 5102")).toBe(true);
  for(let index=0;index<512;index++)footprintPulseV1();
  expect(rows.length).toBe(2);
  now+=FOOTPRINT_POLICY_V1.pulseMilliseconds;for(let index=0;index<256;index++)footprintPulseV1();
  expect(rows.length).toBe(4);
 }finally{Date.now=realNow;gate.stop();for(let index=0;index<512;index++)footprintPulseV1();expect(time.now()).toBe(0);}
});

test("the sampler starved row carries the real runtime counts and a stack of at most pulseFrames frames",()=>{
 const rows:Record<string,string|number>[]=[];let at=10_000;
 const sampler=createHeapSamplerV1({meter:{bytes:()=>1},now:()=>0,emit:fields=>rows.push(fields)});
 sampler.starved(at);expect(rows.length).toBe(1);
 expect(Number(rows[0]!.objects)).toBeGreaterThan(0);expect(Number(rows[0]!.blocked)).toBe(at);expect(String(rows[0]!.stack).split(" < ").length).toBeLessThanOrEqual(FOOTPRINT_POLICY_V1.pulseFrames);
 sampler.starved(0);expect(rows.length).toBe(1);
});

test("the step label names the running graph phase beside the profile phase and is cut",()=>{
 try{
  noteFootprintStepV1("build:graph-phase:owners:4:5102");expect(footprintStepV1().endsWith("build:graph-phase:owners:4:5102")).toBe(true);
  noteFootprintStepV1("x".repeat(500));expect(footprintStepV1().length).toBeLessThan(260);
 }finally{noteFootprintStepV1(undefined);}
});

test("R57 law: the heap census takes at most censusDutyPercent of the wall time however long the loop stays starved, every starved row is still written and carries the last counts",()=>{
 const P=FOOTPRINT_POLICY_V1,time=clock(),rows:Record<string,string|number>[]=[];let counted=0,spent=0;
 const costMilliseconds=2000,sampler=createHeapSamplerV1({meter:{bytes:()=>1},statsSync:()=>{counted++;spent+=costMilliseconds;time.advance(costMilliseconds);return{heapSize:300*MB,extraMemorySize:50*MB,objectCount:1000+counted,objectTypeCounts:{Object:counted}};},stack:()=>"s",usage:()=>({rss:2*MB}),now:time.now,emit:fields=>rows.push(fields)});
 const began=time.now();let calls=0;
 for(;time.now()-began<600_000;calls++){time.advance(P.pulseMilliseconds);sampler.starved(time.now());}
 expect(rows.length).toBe(calls);
 expect(counted).toBeGreaterThanOrEqual(2);
 expect(spent/(time.now()-began)).toBeLessThanOrEqual(P.censusDutyPercent/100+0.005);
 expect(rows[0]!.objects).toBe(1001);expect(rows[1]!.objects).toBe(1001);expect(rows[rows.length-1]!.objects).toBe(1000+counted);
 expect(new Set(rows.map(row=>row.objects)).size).toBe(counted);
});

test("R57 law: a regular sample waits for the census quiet time instead of walking the heap again, and speaks once it passed",async()=>{
 const P=FOOTPRINT_POLICY_V1,time=clock(),rows:Record<string,string|number>[]=[];let counted=0,rss=100*MB;
 const sampler=createHeapSamplerV1({meter:{bytes:()=>1},stats:async()=>{counted++;time.advance(1000);return{heapSize:MB,objectCount:counted};},usage:()=>({rss}),now:time.now,emit:fields=>rows.push(fields)});
 await sampler.tick();expect(rows.length).toBe(1);expect(counted).toBe(1);
 rss+=P.sampleGrowthBytes;time.advance(P.sampleMilliseconds);await sampler.tick();
 expect(rows.length).toBe(1);expect(counted).toBe(1);
 time.advance(1000*(100-P.censusDutyPercent)/P.censusDutyPercent);await sampler.tick();
 expect(rows.length).toBe(2);expect(counted).toBe(2);
});

test("R57 law: a forced release is bounded to releaseDutyPercent of the wall time, never runs below the soft limit of the grant, and runs from it",()=>{
 const P=FOOTPRINT_POLICY_V1,state={quietUntil:-Infinity,releasedAt:0};let at=1_000_000,used=0,released=0,spent=0;
 const call=(more:Record<string,unknown>={})=>releaseIfGrownV1({usage:()=>used,release:()=>{released++;spent+=3000;at+=3000;},threshold:100*MB,now:()=>at,decision:()=>"settle",state,...more});
 const began=at;
 for(;at-began<600_000;at+=1000){used+=200*MB;call();}
 expect(released).toBeGreaterThanOrEqual(2);
 expect(spent/(at-began)).toBeLessThanOrEqual(P.releaseDutyPercent/100+0.01);
 const before=released;used+=500*MB;at+=1_000_000;expect(call({decision:()=>"run"})).toBe(false);expect(released).toBe(before);
 expect(call({decision:()=>"settle"})).toBe(true);expect(released).toBe(before+1);
 used+=500*MB;at+=1_000_000;expect(call({decision:()=>"wait"})).toBe(true);
});

test("R57 law: the release decision of a process reads the footprint gate it holds: below soft nothing, from soft a release",()=>{
 let footprint=100*MB;const state={quietUntil:-Infinity,releasedAt:0},gate={grantBytes:()=>1000*MB,footprintBytes:()=>footprint,stop:()=>undefined};let released=0,used=0,at=1_000_000;
 const call=()=>releaseIfGrownV1({usage:()=>used,release:()=>{released++;at+=5;},threshold:50*MB,state,now:()=>at});
 try{
  useFootprintGateV1(gate as never);
  used+=300*MB;expect(call()).toBe(false);expect(released).toBe(0);
  footprint=FOOTPRINT_POLICY_V1.softPercent*10*MB+MB;expect(call()).toBe(true);expect(released).toBe(1);
 }finally{useFootprintGateV1(undefined);}
 used+=300*MB;at+=1_000_000;expect(call()).toBe(true);
});

const shareOracle=(footprint:number,grant:number)=>{const database=new Database(":memory:"),row=database.query("select case when ?1*100.0/?2>=?3 then 'wait' when ?1*100.0/?2>=?4 then 'settle' else 'run' end as decision").get(footprint,grant,FOOTPRINT_POLICY_V1.hardPercent,FOOTPRINT_POLICY_V1.softPercent) as {decision:string};database.close();return row.decision;};

test("a slice worker told its plan ceiling gates the whole tree it holds against it and spills its owner state before the ceiling is reached; a worker told none stays ungated",async()=>{
 const name=GRAPH_MEMORY_POLICY_V1.planCeilingEnvironment;
 expect(planCeilingBytesV1({})).toBeUndefined();for(const nonsense of["","x","0","-5","Infinity"])expect(planCeilingBytesV1({[name]:nonsense})).toBeUndefined();
 expect(planCeilingBytesV1({[name]:"6553"})).toBe(6553*MB);
 expect(useSliceCeilingGateV1({})).toBeUndefined();expect(footprintGateV1()).toBeUndefined();
 let tree=300*MB;const asked:number[]=[],scripts=createScriptInputCache({workspaceRoot:"/w"}) as any,fill=()=>{scripts.files.set("a",1);scripts.closures.set("b",1);scripts.commandDependencies.set("c",1);};
 const gate=useSliceCeilingGateV1({[name]:"1000"},{pid:4242,read:async pid=>{asked.push(pid);return tree;},bind:(grant,ports)=>bindProcessGateV1(grant,{...ports,trace:()=>undefined,write:()=>undefined})})!;
 try{
  expect<unknown>(gate).toBe(footprintGateV1());expect(gate.grantBytes()).toBe(1000*MB);expect(useSliceCeilingGateV1({[name]:"2000"})).toBe(gate);
  for(const held of[300,599,600,849,850,1200]){tree=held*MB;await gate.settle(false);expect(gate.footprintBytes()).toBe(held*MB);expect<string>(decideFootprintV1({footprintBytes:gate.footprintBytes(),grantBytes:gate.grantBytes()!})).toBe(shareOracle(held*MB,1000*MB));}
  expect(new Set(asked)).toEqual(new Set([4242]));
  tree=900*MB;await gate.settle(false);fill();releaseOwnerStateV1(scripts);
  expect([scripts.files.size,scripts.closures.size,scripts.commandDependencies.size]).toEqual([0,0,0]);
  tree=300*MB;await gate.settle(false);
  const calm=createScriptInputCache({workspaceRoot:"/w"}) as any,interval=INFERENCE_LIMITS_V1.ownersPerRelease;
  calm.files.set("a",1);calm.closures.set("b",1);calm.commandDependencies.set("c",1);
  for(let owner=0;owner<interval-1;owner++)releaseOwnerStateV1(calm);
  expect([calm.files.size,calm.closures.size,calm.commandDependencies.size]).toEqual([1,1,1]);
  releaseOwnerStateV1(calm);
  expect([calm.files.size,calm.closures.size,calm.commandDependencies.size]).toEqual([0,0,0]);
 }finally{useFootprintGateV1(undefined);}
 expect(footprintGateV1()).toBeUndefined();
});
