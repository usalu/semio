/** 🧮️ Contract: the early stages of a worker never wait for an absolute number a small machine cannot reach. The memory a stage needs to start is a share of the machine (never above the declared floor) and a wait that outlasts `memoryWaitMaximumMilliseconds` ends with the stage starting under its own capacity gate; the inodes a stage needs are a share of the inode table, and a shortage of inodes collects every build-owned cache once (zero-touch) before the stage waits. Both cross-checked with an independent Python model. */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
import{READY_POLICY_V1 as P,createReadyLeaseV1,emptyReadyStateV1,readyDigestsV1,readyDiskNeedV1,readyDiskShortV1,readyMemoryFloorMegabytesV1,runReadyV1,type ReadyLocalStateV1,type ReadyPortsV1,type ReadyStatusV1}from"../../🟦️.ts";

const scratch:string[]=[];afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const model=(...args:string[]):unknown=>JSON.parse(spawnSync("/usr/bin/python3",["-I",join(import.meta.dir,"🐍️.py"),...args],{encoding:"utf8"}).stdout);
const MB=1048576;

type World={ports:ReadyPortsV1;controller:AbortController;sleeps:number[];updates:Partial<ReadyStatusV1>[];said:string[];calls:string[];shared:{state:ReadyLocalStateV1;marker:boolean;now:number}};
function world(options:Readonly<{memory?:()=>{availableMegabytes:number|null;ok:boolean;floorMegabytes?:number};disk?:(stage:string)=>ReturnType<typeof readyDiskShortV1>;sweep?:ReadyPortsV1["sweep"];stopAfterSleeps?:number}>={}):World{
 const shared={state:emptyReadyStateV1(),marker:false,now:1_000_000},controller=new AbortController(),sleeps:number[]=[],said:string[]=[],updates:Partial<ReadyStatusV1>[]=[],calls:string[]=[];
 const digest=readyDigestsV1({platform:"linux",hash:path=>path==="bun.lock"?"a".repeat(64):"b".repeat(64)}),directory=mkdtempSync(join(tmpdir(),"ready-early-"));scratch.push(directory);
 const ports:ReadyPortsV1={
  server:false,now:()=>shared.now,jitter:0,cores:4,
  sleep:async(milliseconds,signal)=>{sleeps.push(milliseconds);shared.now+=milliseconds;await new Promise<void>(done=>setTimeout(done,1));if(sleeps.length>=(options.stopAfterSleeps??60)||signal.aborted)controller.abort();},
  digests:async()=>digest,
  state:{read:()=>structuredClone(shared.state),write:state=>{shared.state=structuredClone(state);}},
  marker:{exists:()=>shared.marker,write:()=>{shared.marker=true;}},
  repoPresent:()=>true,human:()=>false,
  lease:createReadyLeaseV1({directory,owner:"one",now:()=>Date.now()}),
  memory:options.memory??(()=>({availableMegabytes:8192,ok:true})),
  ...options.disk===undefined?{}:{disk:options.disk},
  ...options.sweep===undefined?{}:{sweep:options.sweep},
  run:async stage=>{calls.push(stage.name);return{exit:0,timedOut:false,spawnFailed:false,cancelled:false,stalled:false,detached:false,startedMs:shared.now,peakMegabytes:512,tail:[]};},
  status:{update:patch=>{updates.push(patch);}},say:line=>{said.push(line);},
 };
 return{ports,controller,sleeps,updates,said,calls,shared};
}

test("the memory a stage needs to start is a share of the machine, capped by the declared floor, and equals an independent model",()=>{
 const hosts=[0,1024,2048,4096,8192,16384,32768,65536];
 expect(hosts.map(readyMemoryFloorMegabytesV1)).toEqual(model("memory",String(P.memoryFloorMegabytes),String(P.memoryFloorShareBasisPoints),String(P.memoryFloorMinimumMegabytes),JSON.stringify(hosts)) as number[]);
 expect(readyMemoryFloorMegabytesV1(16384)).toBeLessThan(1377);
 expect(readyMemoryFloorMegabytesV1(65536)).toBe(P.memoryFloorMegabytes);
});

test("the inodes a stage needs never exceed a share of the inode table and equal an independent model",()=>{
 const tables:(number|null)[]=[null,200_000,600_000,1_000_000,30_000_000];
 const declared=P.diskRequirements.dependencies.inodes;
 expect(tables.map(total=>readyDiskNeedV1("dependencies",[],null,total).inodes)).toEqual(model("inodes",String(declared),String(P.diskDefaultRequirement.inodes),String(P.diskGrowth.ceilingPercent),JSON.stringify(tables)) as number[]);
 expect(readyDiskNeedV1("dependencies",[],null,1_000_000).inodes).toBe(250_000);
 const short=readyDiskShortV1({stage:"dependencies",paths:["/p"],facts:()=>({freeBytes:80_000*MB,freeInodes:251_913,totalInodes:1_000_000})});
 expect(short).toBeNull();
});

test("a memory wait ends after the maximum and the stage starts under its own capacity gate; a host above the floor never waits",async()=>{
 const low=world({memory:()=>({availableMegabytes:1377,ok:false,floorMegabytes:1310}),stopAfterSleeps:40});
 await runReadyV1(low.ports,low.controller.signal);
 expect(low.calls[0]).toBe("native");
 const waited=low.sleeps.filter(value=>value===P.memoryPollMilliseconds).length;
 expect(waited).toBeGreaterThanOrEqual(Math.floor(P.memoryWaitMaximumMilliseconds/P.memoryPollMilliseconds));
 expect(waited).toBeLessThan(Math.floor(P.memoryWaitMaximumMilliseconds/P.memoryPollMilliseconds)+3);
 expect(low.updates.some(update=>update.refusal?.code==="memory-low"&&update.refusal.en.includes("1310"))).toBe(true);
 expect(low.said.filter(line=>line.includes("starts anyway")&&line.includes(" / ")).length).toBe(1);
 const fine=world();
 await runReadyV1(fine.ports,fine.controller.signal);
 expect(fine.sleeps.some(value=>value===P.memoryPollMilliseconds)).toBe(false);
});

test("a stage short of inodes collects every build-owned cache once, starts when that frees them and does not sweep again within the retry bound",async()=>{
 let free=false,sweeps=0;
 const short={path:"/home/deck/.cache",resource:"inodes" as const,needed:300000,available:251913};
 const w=world({disk:stage=>stage==="dependencies"&&!free?short:null,sweep:async request=>{sweeps++;request.report("collected");free=true;return{fits:true,consumers:[]};},stopAfterSleeps:6});
 await runReadyV1(w.ports,w.controller.signal);
 expect(sweeps).toBe(1);
 expect(w.calls).toContain("dependencies");
 const collecting=w.updates.map(update=>update.refusal).find(refusal=>refusal?.code==="disk-collecting");
 expect(collecting?.en).toContain("300000 inodes");expect(collecting?.de).toContain("300000 inodes");expect(collecting?.en).not.toContain("300000 MB");
 expect(w.said.some(line=>line.includes("lacks inodes")&&line.includes(" / ")&&line.includes("Inodes"))).toBe(true);
 let stuck=0;
 const never=world({disk:stage=>stage==="dependencies"?short:null,sweep:async()=>{stuck++;return{fits:false,consumers:[]};},stopAfterSleeps:20});
 await runReadyV1(never.ports,never.controller.signal);
 expect(stuck).toBe(1);
 expect(never.updates.some(update=>update.refusal?.code==="disk-short")).toBe(true);
 expect(never.calls).not.toContain("dependencies");
});
