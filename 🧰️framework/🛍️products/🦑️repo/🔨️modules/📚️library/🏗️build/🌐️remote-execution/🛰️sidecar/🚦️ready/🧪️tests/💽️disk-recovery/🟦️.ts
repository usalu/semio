/** 💽️ Contract: a stage whose declared need does not fit starts the collection of the registry (priority order, progress lines in both languages, cancellation) before it waits; it starts once the need fits; when automatic collection did not free enough space it waits with the typed refusal that lists the largest consumers and the exact command for each; a stage that already runs is never gated; the need is derived from measured growth when the machine has it. */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
import{READY_POLICY_V1 as P,createReadyLeaseV1,emptyReadyStateV1,readyDigestsV1,readyDiskNeedV1,readyDiskRefusalV1,readyDiskShortV1,readyNoSpaceV1,readyStoreFullV1,readyTailWithDiskCauseV1,runReadyV1,type ReadyFilesystemFactsV1,type ReadyLocalStateV1,type ReadyPortsV1,type ReadyRunResultV1,type ReadyStatusV1,type ReadyTimingV1}from"../../🟦️.ts";
import{recoverNeedV1,type DiskWriterV1}from"../../../../../🌡️host-capacity/🧹️disk-recovery/🟦️.ts";
import{DISK_WRITERS_V1}from"../../../../../🌡️host-capacity/🧹️disk-recovery/🟦️.ts";
import{CAS_RESERVE_FACTOR_V1}from"../../../../../🗄️cas/🧬️schema/🟦️.ts";

const scratch:string[]=[];afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const MB=1048576,PATH="/ws/.cache";
type Volume={freeMegabytes:number};
const factsOf=(volume:Volume)=>(path:string):ReadyFilesystemFactsV1|null=>path===PATH?{freeBytes:volume.freeMegabytes*MB,freeInodes:5_000_000}:null;
const rowOf=(id:string)=>DISK_WRITERS_V1.find(row=>row.id===id)!;
const writer=(volume:Volume,id:string,holds:number,gives:number,log:string[],options:{fail?:boolean;slow?:AbortSignal}={}):DiskWriterV1=>({row:rowOf(id),root:"/ws/"+id,measure:()=>holds*MB,collect:async({signal})=>{log.push(id);if(options.fail)throw Error("boom "+id);if(options.slow){await new Promise((_,reject)=>signal?.addEventListener("abort",()=>reject(signal.reason),{once:true}));}volume.freeMegabytes+=gives;holds=Math.max(0,holds-gives);return gives*MB;}});

function world(volume:Volume,writers:readonly DiskWriterV1[],options:Readonly<{stage?:string;stopAfterSleeps?:number;inflight?:string;external?:(round:number)=>void;aborting?:boolean;sweep?:ReadyPortsV1["sweep"];results?:Readonly<Record<string,readonly Partial<ReadyRunResultV1>[]>>}>={}){
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
  ...options.sweep===undefined?{}:{sweep:options.sweep},
  recover:async request=>{
   const reading=()=>({partition:PATH,totalBytes:250_000*MB,freeBytes:volume.freeMegabytes*MB});
   const outcome=await recoverNeedV1({facts:reading,needBytes:request.short.needed*MB,writers,language:"both",report:request.report,signal:request.signal});
   return{fits:outcome.verdict!=="refuse",consumers:outcome.consumers};
  },
  run:async row=>{calls.push(row.name);const planned=options.results?.[row.name]?.[calls.filter(name=>name===row.name).length-1];return{exit:0,timedOut:false,spawnFailed:false,cancelled:false,stalled:false,detached:false,startedMs:shared.now,peakMegabytes:512,tail:[],...planned} as ReadyRunResultV1;},
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

/** 📏️ Contract: the need of a stage has a ceiling relative to the volume (`diskGrowth.ceilingPercent`), declared or measured, so an observed 2x growth can never ask for more than a quarter of a 60 GB machine; the refusal says exactly how many megabytes are missing, in both languages; and a model in another language computes the same need. */
test("the need has a ceiling relative to the volume and the refusal names the missing megabytes in both languages, equal to an independent model",()=>{
 const run=(growth:number):ReadyTimingV1=>({stage:"workspace",startedMs:1,endedMs:2,seconds:1,exit:0,outcome:"ok",peakMegabytes:1,attempts:0,digest:"d",growthMegabytes:growth});
 const model=(runs:ReadyTimingV1[],totalMegabytes:number|"")=>Number(spawnSync("/usr/bin/python3",["-I",join(import.meta.dir,"🐍️.py"),JSON.stringify(runs.map(item=>[item.stage,item.outcome,item.growthMegabytes??null])),String(P.diskGrowth.keptRuns),String(P.diskGrowth.floorMegabytes),String(CAS_RESERVE_FACTOR_V1),String(totalMegabytes),String(P.diskGrowth.ceilingPercent),String(P.diskGrowth.measuredCeilingMegabytes)],{encoding:"utf8"}).stdout.trim());
 const GB=10**9;
 const cases:[ReadyTimingV1[],number|undefined,number][]=[[[run(40_000)],60*GB,Math.floor(60*GB/1048576*P.diskGrowth.ceilingPercent/100)],[[run(40_000)],undefined,P.diskGrowth.measuredCeilingMegabytes],[[run(1_500)],60*GB,3000],[[],60*GB,4096],[[],8*GB,Math.floor(8*GB/1048576*P.diskGrowth.ceilingPercent/100)],[[],1*GB,P.diskGrowth.floorMegabytes],[[run(9_000)],256*GB,P.diskGrowth.measuredCeilingMegabytes],[[run(6_000)],256*GB,12_000]];
 for(const[runs,total,expected]of cases){expect(readyDiskNeedV1("workspace",runs,total).megabytes,String(total)).toBe(expected);expect(model(runs,total===undefined?"":total/1048576),String(total)).toBe(expected);}
 expect(readyDiskNeedV1("dependencies",[],60*GB).megabytes).toBe(P.diskRequirements.dependencies.megabytes);
 const short=readyDiskShortV1({stage:"dependencies",paths:[PATH],facts:path=>path===PATH?{freeBytes:4937*MB,freeInodes:null,totalBytes:256*GB}:null});
 expect(short).toEqual({path:PATH,resource:"space",needed:6144,available:4937});
 const refusal=readyDiskRefusalV1("dependencies",short!);
 for(const text of[refusal.en,refusal.de]){expect(text).toContain("1207");expect(text).toContain("6144");expect(text).toContain("4937");expect(text).not.toContain("{");}
 expect(refusal.en).toContain("free at least 1207 MB more");expect(refusal.de).toContain("mindestens 1207 MB mehr");
 const withConsumers=readyDiskRefusalV1("dependencies",short!,[{id:"cas-lru",root:"/ws/cas",bytes:900*MB,command:"repo cache gc"}]);expect(withConsumers.en).toContain("1207");expect(withConsumers.de).toContain("1207");
});

/** 💥️ Contract: a stage that fails with the operating system's "no space left" collects every build-owned cache once and is retried once, without recording a failure; a second shortage in a row fails the stage as before with the full-disk code; a clean run re-arms the retry. */
test("no space left on device triggers one sweep and one immediate retry, never a failure on the first shortage, and agrees on the classification",async()=>{
 for(const lines of[["Unable to initialize workspace data while attempting to create lock file: No space left on device (os error 28)"],["write failed: ENOSPC"],["No space left (ENOSPC) in /ws/.cache/b/n/m: the disk"]])expect(readyNoSpaceV1(lines),lines[0]).toBe(true);
 for(const lines of[["permission denied"],[],["unable to open database file"]])expect(readyNoSpaceV1(lines),String(lines)).toBe(false);
 const full={exit:1,tail:["Unable to initialize workspace data while attempting to create lock file: No space left on device (os error 28)"]};
 const sweeps:string[]=[];
 const once=world({freeMegabytes:50_000},[],{stopAfterSleeps:6,results:{workspace:[full,{}]},sweep:async request=>{sweeps.push(request.stage+":"+request.neededMegabytes);request.report("swept");return{fits:true,consumers:[]};}});
 await runReadyV1(once.ports,once.controller.signal);
 expect(sweeps).toEqual(["workspace:"+P.diskRequirements.workspace.megabytes]);expect(once.calls.filter(name=>name==="workspace").length).toBe(2);
 expect(once.updates.some(update=>update.state==="backoff"||update.refusal?.code==="store-full")).toBe(false);
 expect(once.said.some(line=>line.includes("collecting every build-owned cache once")&&line.includes("Caches"))).toBe(true);
 const twice=world({freeMegabytes:50_000},[],{stopAfterSleeps:4,results:{workspace:[full,full,full]},sweep:async request=>{sweeps.push("second:"+request.stage);return{fits:true,consumers:[]};}});
 await runReadyV1(twice.ports,twice.controller.signal);
 expect(sweeps.filter(item=>item.startsWith("second:")).length).toBe(1);expect(twice.updates.some(update=>update.refusal?.code==="store-full"||update.state==="backoff")).toBe(true);
});

test("a lock-file shortage from Nx names the native shard that cannot rewrite its file map, every stage start relieves the shards first, and the retry keeps working",async()=>{
 const consumer={id:"21ff94dc4d63aba6596930a7e4dd621fb0c922bf",path:"/ws/.cache/b/n/m/21ff94dc4d63aba6596930a7e4dd621fb0c922bf/native/m",usedBytes:46_000_000,totalBytes:66_060_288,freeBytes:20_000_000,fileMapBytes:38_000_000,file:"nx_files.nxt"};
 const full={exit:1,tail:["Unable to initialize workspace data while attempting to create lock file:","No space left on device (os error 28)"]};
 const relieved:number[]=[];
 const base=world({freeMegabytes:50_000},[],{stopAfterSleeps:6,results:{workspace:[full,full,full]},sweep:async()=>({fits:true,consumers:[]})});
 const ports={...base.ports,native:()=>consumer,relieve:async()=>{relieved.push(base.calls.length);return 0;}};
 await runReadyV1(ports,base.controller.signal);
 expect(relieved.length).toBeGreaterThanOrEqual(2);expect(relieved[0]).toBe(0);expect(relieved[1]).toBe(1);
 const refusals=base.updates.filter(update=>update.refusal?.code==="store-full");
 expect(refusals.length).toBeGreaterThan(0);expect(JSON.stringify(refusals[0]!.refusal)).toContain(consumer.path);
 const stored=base.updates.map(update=>JSON.stringify(update)).join("\n");
 expect(stored).toContain("nx_files.nxt");expect(stored).toMatch(/Dateikarte/u);
 const quiet=world({freeMegabytes:50_000},[],{stopAfterSleeps:3,results:{workspace:[{exit:1,tail:["permission denied"]}]}});
 await runReadyV1({...quiet.ports,native:()=>consumer},quiet.controller.signal);
 expect(quiet.updates.some(update=>JSON.stringify(update).includes("nx_files.nxt"))).toBe(false);
 const named=readyTailWithDiskCauseV1(full.tail,null,consumer);
 expect(named.at(-2)).toBe("No space left (ENOSPC) in "+consumer.path+": the disk");expect(readyStoreFullV1(named)?.path).toBe(consumer.path);
 expect(readyTailWithDiskCauseV1(full.tail,null,null)).toEqual(full.tail);
 expect(readyTailWithDiskCauseV1(["No space left (ENOSPC) in /elsewhere: the disk"],null,consumer)).toEqual(["No space left (ENOSPC) in /elsewhere: the disk"]);
});
