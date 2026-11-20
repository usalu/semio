/** 💽️ Contract: a stage whose declared need does not fit starts the collection of the registry (priority order, progress lines in both languages, cancellation) before it waits; it starts once the need fits; when automatic collection did not free enough space it waits with the typed refusal that lists the largest consumers and the exact command for each; a stage that already runs is never gated; the need is derived from measured growth when the machine has it. */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
import{READY_POLICY_V1 as P,createReadyLeaseV1,emptyReadyStateV1,readyDigestsV1,readyDiskNeedV1,readyDiskShortV1,runReadyV1,type ReadyFilesystemFactsV1,type ReadyLocalStateV1,type ReadyPortsV1,type ReadyRunResultV1,type ReadyStatusV1,type ReadyTimingV1}from"../../🟦️.ts";
import{recoverNeedV1,type DiskWriterV1}from"../../../../../🌡️host-capacity/🧹️disk-recovery/🟦️.ts";
import{DISK_WRITERS_V1}from"../../../../../🌡️host-capacity/🧹️disk-recovery/🟦️.ts";
import{CAS_RESERVE_FACTOR_V1}from"../../../../../🗄️cas/🧬️schema/🟦️.ts";

const scratch:string[]=[];afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const MB=1048576,PATH="/ws/.cache";
type Volume={freeMegabytes:number};
const factsOf=(volume:Volume)=>(path:string):ReadyFilesystemFactsV1|null=>path===PATH?{freeBytes:volume.freeMegabytes*MB,freeInodes:5_000_000}:null;
const rowOf=(id:string)=>DISK_WRITERS_V1.find(row=>row.id===id)!;
const writer=(volume:Volume,id:string,holds:number,gives:number,log:string[],options:{fail?:boolean;slow?:AbortSignal}={}):DiskWriterV1=>({row:rowOf(id),root:"/ws/"+id,measure:()=>holds*MB,collect:async({signal})=>{log.push(id);if(options.fail)throw Error("boom "+id);if(options.slow){await new Promise((_,reject)=>signal?.addEventListener("abort",()=>reject(signal.reason),{once:true}));}volume.freeMegabytes+=gives;holds=Math.max(0,holds-gives);return gives*MB;}});

function world(volume:Volume,writers:readonly DiskWriterV1[],options:Readonly<{stage?:string;stopAfterSleeps?:number;inflight?:string;external?:(round:number)=>void;aborting?:boolean}>={}){
 const shared={state:emptyReadyStateV1() as ReadyLocalStateV1,marker:false,now:1_000_000},controller=new AbortController(),sleeps:number[]=[],said:string[]=[],updates:Partial<ReadyStatusV1>[]=[],calls:string[]=[];
 const digest=readyDigestsV1({platform:"linux",hash:path=>path==="bun.lock"?"a".repeat(64):"b".repeat(64)}),directory=mkdtempSync(join(tmpdir(),"ready-disk-"));scratch.push(directory);
 const facts=factsOf(volume),stage=options.stage??"workspace";
 const ports:ReadyPortsV1={
  server:false,now:()=>shared.now,jitter:0,cores:4,
  sleep:async(milliseconds,signal)=>{sleeps.push(milliseconds);shared.now+=milliseconds;options.external?.(sleeps.length);await new Promise<void>(done=>setTimeout(done,1));if(sleeps.length>=(options.stopAfterSleeps??40)||signal.aborted)controller.abort();},
  digests:async()=>digest,
  state:{read:()=>structuredClone(shared.state),write:state=>{shared.state=structuredClone(state);}},
  marker:{exists:()=>shared.marker,write:()=>{shared.marker=true;}},
  repoPresent:()=>true,human:()=>false,
  lease:createReadyLeaseV1({directory,owner:"one",now:()=>Date.now()}),
  memory:()=>({availableMegabytes:8192,ok:true}),
  ...options.inflight===undefined?{}:{inflight:()=>options.inflight},
  disk:name=>name===stage?readyDiskShortV1({stage:name,paths:[PATH],facts}):null,
  recover:async request=>{
   const reading=()=>({partition:PATH,totalBytes:250_000*MB,freeBytes:volume.freeMegabytes*MB});
   const outcome=await recoverNeedV1({facts:reading,needBytes:request.short.needed*MB,writers,language:"both",report:request.report,signal:request.signal});
   return{fits:outcome.verdict!=="refuse",consumers:outcome.consumers};
  },
  run:async row=>{calls.push(row.name);return{exit:0,timedOut:false,spawnFailed:false,cancelled:false,stalled:false,detached:false,startedMs:shared.now,peakMegabytes:512,tail:[]} as ReadyRunResultV1;},
  status:{update:patch=>{updates.push(patch);}},say:line=>{said.push(line);},
 };
 return{ports,controller,sleeps,said,updates,calls};
}

test("2993 MB free, 4096 MB needed: the collectors run in priority order until the need fits, the stage starts, the cheaper ones first and the image slack never",async()=>{
 const volume={freeMegabytes:2993},log:string[]=[];
 const w=world(volume,[writer(volume,"image-slack",30_000,5000,log),writer(volume,"cas-lru",900,600,log),writer(volume,"leaked-staging",800,800,log),writer(volume,"tool-generations",9000,0,log)]);
 await runReadyV1(w.ports,w.controller.signal);
 expect(log).toEqual(["leaked-staging","cas-lru"]);
 expect(volume.freeMegabytes).toBe(2993+800+600);
 expect(w.calls).toContain("workspace");
 expect(w.sleeps.filter(value=>value===P.memoryPollMilliseconds)).toHaveLength(0);
 const progress=w.said.filter(line=>line.includes("leaked-staging")||line.includes("cas-lru"));
 expect(progress.length).toBeGreaterThanOrEqual(2);for(const line of progress)expect(line).toContain(" / ");
 expect(w.updates.some(update=>update.refusal?.code==="disk-collecting"&&update.refusal.en.includes("4096")&&update.refusal.de.includes("4096"))).toBe(true);
});

test("automatic collection left the volume short: the stage waits with the typed refusal that lists the largest consumers and the exact command for each, in both languages, and starts when space returns",async()=>{
 const volume={freeMegabytes:2993},log:string[]=[];
 const w=world(volume,[writer(volume,"cas-lru",5000,0,log),writer(volume,"image-slack",12_000,0,log,{fail:true}),writer(volume,"read-verdicts",300,0,log),writer(volume,"replica-lost",40,0,log)],{stopAfterSleeps:6,external:round=>{if(round===3)volume.freeMegabytes=4200;}});
 await runReadyV1(w.ports,w.controller.signal);
 const refusals=w.updates.map(update=>update.refusal).filter(refusal=>refusal?.code==="disk-consumers");
 expect(refusals.length).toBeGreaterThanOrEqual(2);
 const first=refusals[0]!;
 for(const text of[first.en,first.de]){expect(text).toContain("4096");expect(text).toContain("2993");expect(text).toContain("/ws/image-slack");expect(text).toContain("/ws/cas-lru");expect(text).toContain("`");expect(text).not.toContain("{");}
 expect(first.en.indexOf("/ws/image-slack")).toBeLessThan(first.en.indexOf("/ws/cas-lru"));
 expect(first.en).toContain(rowOf("cas-lru").command);expect(first.en).toContain("repo");
 expect(first.en).not.toBe(first.de);
 expect(first.en).toContain("after automatic collection");expect(first.de).toContain("nach der automatischen Bereinigung");
 expect(w.said.some(line=>line.includes("boom image-slack"))).toBe(true);
 expect(w.calls).toContain("workspace");
 expect(log.filter(id=>id==="cas-lru").length).toBeGreaterThanOrEqual(2);
});

test("a failing collector never stops the next and the collection is cancelled by the signal without starting the stage",async()=>{
 const volume={freeMegabytes:2993},log:string[]=[];
 const failing=world(volume,[writer(volume,"leaked-staging",100,0,log,{fail:true}),writer(volume,"cas-lru",2000,1500,log)]);
 await runReadyV1(failing.ports,failing.controller.signal);
 expect(log.slice(0,2)).toEqual(["leaked-staging","cas-lru"]);expect(failing.calls).toContain("workspace");
 expect(failing.said.some(line=>line.includes("boom leaked-staging"))).toBe(true);
 const hung={freeMegabytes:2993},hungLog:string[]=[];
 const cancel=world(hung,[writer(hung,"leaked-staging",100,0,hungLog,{slow:new AbortController().signal})]);
 setTimeout(()=>cancel.controller.abort(),50);
 await runReadyV1(cancel.ports,cancel.controller.signal);
 expect(hungLog).toEqual(["leaked-staging"]);expect(cancel.calls.includes("workspace")).toBe(false);
});

test("a stage that is already running is not gated by the disk check, a short disk for another stage still is",async()=>{
 const volume={freeMegabytes:100},log:string[]=[];
 const w=world(volume,[writer(volume,"cas-lru",5000,0,log)],{inflight:"workspace",stopAfterSleeps:3});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.calls).toContain("workspace");expect(log).toEqual([]);
 expect(w.updates.some(update=>update.refusal?.code==="disk-consumers"||update.refusal?.code==="disk-short")).toBe(false);
});

test("the need is the declared estimate until the machine has measured the stage, then the largest net growth times the safety factor of the scale family, never below the floor, and equals an independent model",()=>{
 const run=(stage:string,growth:number|null|undefined,outcome:ReadyTimingV1["outcome"]="ok"):ReadyTimingV1=>({stage,startedMs:1,endedMs:2,seconds:1,exit:0,outcome,peakMegabytes:1,attempts:0,digest:"d",...growth===undefined?{}:{growthMegabytes:growth}});
 expect(readyDiskNeedV1("workspace",[])).toEqual({megabytes:P.diskRequirements.workspace.megabytes,inodes:P.diskRequirements.workspace.inodes,source:"declared"});
 expect(readyDiskNeedV1("workspace",[run("workspace",undefined),run("workspace",null),run("workspace",900,"stage-failed")]).source).toBe("declared");
 const cases:[ReadyTimingV1[],number][]=[[[run("workspace",1500)],3000],[[run("workspace",100)],P.diskGrowth.floorMegabytes],[[run("workspace",1000),run("workspace",2600),run("tools",9000)],5200],[[...Array.from({length:6},(_,index)=>run("workspace",index===0?9000:700))],1400]];
 for(const [runs,expected] of cases){expect(readyDiskNeedV1("workspace",runs).megabytes).toBe(expected);expect(readyDiskNeedV1("workspace",runs).source).toBe("measured");}
 expect(CAS_RESERVE_FACTOR_V1).toBe(2);
 const model=(runs:ReadyTimingV1[])=>Number(spawnSync("/usr/bin/python3",["-I",join(import.meta.dir,"🐍️.py"),JSON.stringify(runs.map(item=>[item.stage,item.outcome,item.growthMegabytes??null])),String(P.diskGrowth.keptRuns),String(P.diskGrowth.floorMegabytes),String(CAS_RESERVE_FACTOR_V1)],{encoding:"utf8"}).stdout.trim());
 for(const [runs] of cases)expect(readyDiskNeedV1("workspace",runs).megabytes).toBe(model(runs));
 expect(readyDiskShortV1({stage:"workspace",paths:[PATH],facts:factsOf({freeMegabytes:3100}),runs:[run("workspace",1500)]})).toBeNull();
 expect(readyDiskShortV1({stage:"workspace",paths:[PATH],facts:factsOf({freeMegabytes:3100})})?.needed).toBe(4096);
});
