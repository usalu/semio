import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{READY_POLICY_V1 as P,READY_STAGES_V1 as STAGES,createReadyLeaseV1,emptyReadyStateV1,readyDiskRefusalV1,readyDiskShortV1,readyDigestsV1,readyFilesystemFactsV1,readyRefusalV1,readyTailWithDiskCauseV1,runReadyV1,type ReadyFilesystemFactsV1,type ReadyLocalStateV1,type ReadyPortsV1,type ReadyRunResultV1,type ReadyStatusV1}from"../../🟦️.ts";

/** 💽️ A stage that lacks disk is refused before it starts with a typed bilingual reason (path, need, free), a raw database error on a full disk is classified as full, and the readiness loop keeps going after a failing port and after the retry exit. */
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const MB=1048576;
const facts=(table:Record<string,ReadyFilesystemFactsV1|null>)=>(path:string):ReadyFilesystemFactsV1|null=>table[path]??null;

test("a stage lacking inodes or space is refused with path, need and free in both languages, unknown facts never block",()=>{
 const tools=P.diskRequirements.tools;
 const inodes=readyDiskShortV1({stage:"tools",paths:["/home/deck/.cache","/tmp"],facts:facts({"/home/deck/.cache":{freeBytes:50_000*MB,freeInodes:1},"/tmp":{freeBytes:50_000*MB,freeInodes:900_000}})});
 expect(inodes).toEqual({path:"/home/deck/.cache",resource:"inodes",needed:tools.inodes,available:1});
 const space=readyDiskShortV1({stage:"tools",paths:["/home/deck/.cache"],facts:facts({"/home/deck/.cache":{freeBytes:100*MB+17,freeInodes:5_000_000}})});
 expect(space).toEqual({path:"/home/deck/.cache",resource:"space",needed:tools.megabytes,available:100});
 const refusal=readyDiskRefusalV1("tools",inodes!);
 expect(refusal.code).toBe("disk-short");
 for(const text of[refusal.en,refusal.de]){expect(text).toContain("/home/deck/.cache");expect(text).toContain(String(tools.inodes));expect(text).toContain("inodes");expect(text).not.toContain("{");}
 expect(refusal.en).not.toBe(refusal.de);
 expect(readyDiskRefusalV1("tools",space!).de).toContain("MB");
 const windows="C:\\Users\\semio\\.cache";
 expect(readyDiskShortV1({stage:"tools",paths:[windows],facts:facts({[windows]:{freeBytes:80_000*MB,freeInodes:null}})}),"Windows reports no inodes").toBeNull();
 expect(readyDiskShortV1({stage:"tools",paths:[windows,"/tmp"],facts:facts({})}),"nothing known").toBeNull();
 expect(readyDiskShortV1({stage:"unknown-stage",paths:["/x"],facts:facts({"/x":{freeBytes:1024*MB-1,freeInodes:99_999_999}})})?.resource).toBe("space");
 for(const stage of STAGES)expect(P.diskRequirements[stage.name as keyof typeof P.diskRequirements],stage.name).toBeDefined();
});

test("the real facts provider reads this filesystem, falls back to an existing ancestor and reports unknown inodes as null",()=>{
 const real=readyFilesystemFactsV1(tmpdir());
 expect(real).not.toBeNull();expect(real!.freeBytes!).toBeGreaterThan(0);
 const deeper=readyFilesystemFactsV1(join(tmpdir(),"does","not","exist","yet"));
 expect(deeper).not.toBeNull();
 expect(real!.freeInodes===null||real!.freeInodes>=0).toBe(true);
});

test("a raw database open error on a full disk is named as full with the path, one on a healthy disk is left alone",()=>{
 const tail=["Retained dependency generation retired","Internal error: unable to open database file","repository:command failed with exit code 1"];
 const short={path:"/home/deck/.cache",resource:"inodes" as const,needed:150000,available:0};
 const named=readyTailWithDiskCauseV1(tail,short);
 expect(named.at(-1)).toBe("No space left (ENOSPC) in /home/deck/.cache: the disk");
 expect(readyTailWithDiskCauseV1(tail,null)).toEqual(tail);
 expect(readyTailWithDiskCauseV1(["other failure"],short)).toEqual(["other failure"]);
});

type World={ports:ReadyPortsV1;controller:AbortController;sleeps:number[];updates:Partial<ReadyStatusV1>[];said:string[];calls:string[];shared:{state:ReadyLocalStateV1;marker:boolean;now:number}};
function world(options:Readonly<{outcomes?:Partial<ReadyRunResultV1>[];digestFailures?:number;stateWriteFailures?:number;disk?:(stage:string)=>ReturnType<typeof readyDiskShortV1>;stopAfterSleeps?:number}>={}):World{
 const shared={state:emptyReadyStateV1(),marker:false,now:1_000_000},controller=new AbortController(),sleeps:number[]=[],said:string[]=[],updates:Partial<ReadyStatusV1>[]=[],calls:string[]=[],queue=[...options.outcomes??[]];
 let digestFailures=options.digestFailures??0,writeFailures=options.stateWriteFailures??0;
 const digest=readyDigestsV1({platform:"linux",hash:path=>path==="bun.lock"?"a".repeat(64):"b".repeat(64)});
 const directory=mkdtempSync(join(tmpdir(),"ready-resume-"));scratch.push(directory);
 const ports:ReadyPortsV1={
  server:false,now:()=>shared.now,jitter:0,cores:4,
  sleep:async(milliseconds,signal)=>{sleeps.push(milliseconds);shared.now+=milliseconds;await new Promise<void>(done=>setTimeout(done,1));if(sleeps.length>=(options.stopAfterSleeps??60)||signal.aborted)controller.abort();},
  digests:async()=>{if(digestFailures>0){digestFailures--;throw Error("EIO: the digest cannot be read\nsecond line");}return digest;},
  state:{read:()=>structuredClone(shared.state),write:state=>{if(writeFailures>0){writeFailures--;throw Error("ENOSPC: no space left on device, write");}shared.state=structuredClone(state);}},
  marker:{exists:()=>shared.marker,write:()=>{shared.marker=true;}},
  repoPresent:()=>true,human:()=>false,
  lease:createReadyLeaseV1({directory,owner:"one",now:()=>Date.now()}),
  memory:()=>({availableMegabytes:8192,ok:true}),
  ...options.disk===undefined?{}:{disk:options.disk},
  run:async stage=>{calls.push(stage.name);return{exit:0,timedOut:false,spawnFailed:false,cancelled:false,stalled:false,detached:false,startedMs:shared.now,peakMegabytes:512,tail:[],...queue.shift()};},
  status:{update:patch=>{updates.push(patch);}},say:line=>{said.push(line);},
 };
 return{ports,controller,sleeps,updates,said,calls,shared};
}

test("a stage that lacks disk does not start, says why every minute and starts when space is back",async()=>{
 let free=false;
 const short={path:"/home/deck/.cache",resource:"inodes" as const,needed:150000,available:0};
 const w=world({disk:stage=>stage==="tools"&&!free?short:null,stopAfterSleeps:12});
 const real=w.ports.sleep;
 (w.ports as {sleep:ReadyPortsV1["sleep"]}).sleep=async(milliseconds,signal)=>{await real(milliseconds,signal);if(w.sleeps.filter(value=>value===P.memoryPollMilliseconds).length>=3)free=true;};
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.sleeps.slice(0,3)).toEqual([P.memoryPollMilliseconds,P.memoryPollMilliseconds,P.memoryPollMilliseconds]);
 const refusals=w.updates.map(update=>update.refusal).filter(refusal=>refusal?.code==="disk-short");
 expect(refusals.length).toBeGreaterThanOrEqual(3);
 expect(refusals[0]!.en).toContain("/home/deck/.cache");expect(refusals[0]!.de).toContain("/home/deck/.cache");
 expect(w.calls.indexOf("tools")).toBeGreaterThan(-1);
 expect(w.calls.slice(0,w.calls.indexOf("tools")).every(name=>name!=="tools")).toBe(true);
 expect(w.said.some(line=>line.includes("inodes")&&line.includes(" / "))).toBe(true);
});

test("a stage that dies with a raw database error on a full disk is recorded as store-full and names the path",async()=>{
 const short={path:"/home/deck/.cache",resource:"inodes" as const,needed:150000,available:0};
 let stagesRun=0;
 const w=world({outcomes:[{},{},{},{exit:1,tail:["Internal error: unable to open database file"]}],stopAfterSleeps:5,disk:stage=>stage==="tools"&&stagesRun>0?short:null});
 const run=w.ports.run;
 (w.ports as {run:ReadyPortsV1["run"]}).run=async(stage,options)=>{const result=await run(stage,options);if(stage.name==="tools")stagesRun++;return result;};
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.shared.state.failure?.code).toBe("store-full");
 expect(w.shared.state.failure?.tail.at(-1)).toContain("/home/deck/.cache");
});

test("the readiness loop survives a failing port: it says so in both languages, waits the back-off and carries on to ready",async()=>{
 const w=world({digestFailures:3,stateWriteFailures:2,stopAfterSleeps:40});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.sleeps.slice(0,7)).toEqual(Array(7).fill(P.statusHeartbeatMilliseconds));
 const failed=w.updates.filter(update=>update.refusal?.code==="loop-failed");
 expect(failed.length).toBeGreaterThanOrEqual(3);
 expect(failed[0]!.state).toBe("backoff");expect(failed[0]!.refusal!.en).toContain("EIO");expect(failed[0]!.refusal!.de).toContain("Sekunden");
 expect(failed[0]!.refusal!.en).not.toContain("\n");
 expect(w.calls).toEqual(expect.arrayContaining(["native","repo-mcp","dependencies","tools","workspace"]));
 expect(w.updates.some(update=>update.ready===true)).toBe(true);
});

test("the loop never gives up: forty failures in a row still back off at most to the maximum and then run",async()=>{
 const w=world({digestFailures:40,stopAfterSleeps:3000});
 await runReadyV1(w.ports,w.controller.signal);
 const pauses=Array.from({length:40},(_,index)=>Math.min(P.backoffMaximumMilliseconds,P.backoffInitialMilliseconds*2**index));
 const slices=pauses.reduce((sum,value)=>sum+Math.ceil(value/P.statusHeartbeatMilliseconds),0);
 expect(w.sleeps.slice(0,slices)).toEqual(Array(slices).fill(P.statusHeartbeatMilliseconds));
 expect(w.updates.some(update=>update.ready===true)).toBe(true);
});

test("after the retry exit (75, graph too heavy for now) the stage is tried again every first back-off until it succeeds, the machine does not go silent",async()=>{
 const w=world({outcomes:[{},{},{},{},{exit:75},{exit:75},{exit:75},{exit:75},{exit:75},{exit:75},{exit:75},{exit:75}],stopAfterSleeps:16});
 await runReadyV1(w.ports,w.controller.signal);
 const retried=w.sleeps.filter(value=>value===readyRefusalDelay());
 expect(retried.length).toBeGreaterThanOrEqual(8);
 expect(w.calls.filter(name=>name==="workspace").length).toBeGreaterThanOrEqual(9);
 expect(w.updates.some(update=>update.ready===true)).toBe(true);
 function readyRefusalDelay():number{return P.backoffInitialMilliseconds;}
});
