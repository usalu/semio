import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
import{READY_POLICY_V1 as P,READY_STAGES_V1 as STAGES,createReadyLeaseV1,emptyReadyStateV1,readyDigestsV1,readyRefreshV1,readyRefreshingPatchV1,runReadyV1,type ReadyLocalStateV1,type ReadyPortsV1,type ReadyRunResultV1,type ReadyStatusV1}from"../../🟦️.ts";
import{readyForegroundBusyV1}from"../../🌲️process-tree/🟦️.ts";
import{readyWorkerLivenessV1,readySidecarSilenceV1}from"../../💔️silence/🟦️.ts";

/** 🔄️ Contract: Ready is a state per stage input digest. A landing that touches no stage input changes no digest; one that touches a stage input refreshes only the stages that declare it, in the background, while the worker keeps saying ready (`refreshing`); only a failed re-verification leaves Ready.
 * @see ../../🟦️.ts */
const scratch:string[]=[];afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`refresh-${label}-`));scratch.push(path);return path;};
const oracle=(mode:string,payload:unknown)=>{const run=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),mode,JSON.stringify(payload)],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const names=STAGES.map(stage=>stage.name);
const allPaths=[...new Set(STAGES.flatMap(stage=>stage.inputs))];
const hashesOf=(salt:Record<string,string>)=>Object.fromEntries(allPaths.map(path=>[path,salt[path]??"1".repeat(64)]));
const digestsOf=(hashes:Record<string,string>)=>readyDigestsV1({platform:"linux",hash:path=>hashes[path]});
const shared=(path:string)=>STAGES.filter(stage=>stage.inputs.includes(path)).map(stage=>stage.name);

test("the stages to refresh equal an independent Python implementation on a grid of states",()=>{
 const stages:[string,string][]=names.map((name,index)=>[name,`d${index}`]);
 const rows=[];
 const maximum=P.refreshMaximumAgeMilliseconds,now=10*maximum;
 for(const marker of[true,false])for(const repo of[true,false])for(const readyMs of[null,5])for(const failure of[null,{code:"stage-failed"},{code:"tooling-compiler"},{code:"stage-timeout"},{code:"store-full"}])for(const stale of[[],[0],[3],[1,4],[0,1,2,3,4]])for(const age of[0,maximum,maximum+1])for(const unverified of[false,true]){
  const done=Object.fromEntries(stages.filter(([,,],index)=>!(unverified&&stale.includes(index))).map(([name,digest],index)=>[name,stale.includes(index)?"old":digest]));
  const doneMs=Object.fromEntries(stages.map(([name],index)=>[name,stale.includes(index)?now-age:now]));
  rows.push({stages,done,doneMs,now,maximum,marker,repo,readyMs,failure});
 }
 const expected=oracle("refresh",rows) as (string[]|null)[];
 rows.forEach((row,index)=>expect(readyRefreshV1({stages:row.stages.map(([name,digest])=>({name,digest})),done:row.done,doneMs:row.doneMs,now:row.now,marker:row.marker,repo:row.repo,readyMs:row.readyMs,failure:row.failure}),JSON.stringify(row)).toEqual(expected[index]));
 expect(expected.some(row=>row===null)&&expected.some(row=>row!==null)).toBe(true);
 expect(rows.some((row,index)=>row.failure?.code==="stage-timeout"&&expected[index]!==null)).toBe(true);
});

test("law: a landing touching unrelated files keeps every stage digest, one touching a stage input changes exactly the stages that declare it, equal to an independent Python digest",()=>{
 const base=digestsOf(hashesOf({}));
 expect(digestsOf(hashesOf({"unrelated/file.ts":"2".repeat(64)})).stages).toEqual(base.stages);
 for(const path of allPaths){
  const touched=digestsOf(hashesOf({[path]:"2".repeat(64)})),changed=touched.stages.filter((row,index)=>row.digest!==base.stages[index]!.digest).map(row=>row.name);
  expect(changed,path).toEqual(shared(path));
 }
 const path=STAGES.find(stage=>stage.name==="workspace")!.inputs[0]!,hashes=hashesOf({[path]:"3".repeat(64)});
 const expected=oracle("digests",{platform:"linux",hashes,stages:STAGES.map(stage=>[stage.name,[...stage.inputs,...stage.bootstrapInput?[P.bootstrap.unix.join("/")]:[]]])}) as [string,string][];
 expect(digestsOf(hashes).stages.map(row=>[row.name,row.digest])).toEqual(expected);
});

type Outcome=Partial<ReadyRunResultV1>;
function world(options:Readonly<{state:ReadyLocalStateV1;outcomes?:Outcome[];landing?:Record<string,string>}>){
 const before=digestsOf(hashesOf({})),after=digestsOf(hashesOf(options.landing??{}));
 const store={state:structuredClone(options.state),marker:true,now:1_000_000,calls:[] as string[],seen:[] as Partial<ReadyStatusV1>[]};
 const controller=new AbortController(),updates:Partial<ReadyStatusV1>[]=[],queue=[...options.outcomes??[]];let sleeps=0;
 const ports:ReadyPortsV1={
  server:false,now:()=>store.now,jitter:0,cores:4,
  sleep:async(milliseconds,signal)=>{store.now+=milliseconds;await new Promise<void>(done=>setTimeout(done,2));if(++sleeps>=1||signal.aborted)controller.abort();},
  digests:async()=>after,
  state:{read:()=>structuredClone(store.state),write:state=>{store.state=structuredClone(state);}},
  marker:{exists:()=>store.marker,write:()=>{store.marker=true;}},repoPresent:()=>true,human:()=>false,
  lease:createReadyLeaseV1({directory:temp("lease"),owner:"one",now:()=>Date.now()}),
  memory:()=>({availableMegabytes:8192,ok:true}),
  run:async stage=>{store.calls.push(stage.name);store.seen.push(Object.assign({},...updates));await new Promise<void>(done=>setTimeout(done,4));return{exit:0,timedOut:false,spawnFailed:false,cancelled:false,stalled:false,detached:false,startedMs:store.now,peakMegabytes:512,tail:[],...queue.shift()};},
  status:{update:patch=>{updates.push(patch);}},say:()=>undefined,
 };
 return{ports,controller,store,updates,before,after,last:()=>Object.assign({},...updates) as Partial<ReadyStatusV1>};
}
const readyState=(digests:ReturnType<typeof digestsOf>,readyMs:number|null=5,doneMs:number=1_000_000-1):ReadyLocalStateV1=>({...emptyReadyStateV1(),stages:Object.fromEntries(digests.stages.map(row=>[row.name,{digest:row.digest,doneMs}])),readyMs});
const landingOn=(stage:string)=>STAGES.find(row=>row.name===stage)!.inputs.find(path=>shared(path).length===1)!;

test("a landing on a stage input keeps the worker ready: only that stage runs, the status says ready and refreshing meanwhile, and it ends plain ready with readyMs never cleared",async()=>{
 const path=landingOn("workspace"),before=digestsOf(hashesOf({}));
 const w=world({state:readyState(before),landing:{[path]:"2".repeat(64)}});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.store.calls).toEqual(["workspace"]);
 const during=w.store.seen[0]!;
 expect(during.state).toBe("ready");expect(during.ready).toBe(true);expect(during.refreshing).toBe(true);expect(during.stage).toBe("workspace");expect(during.reason?.code).toBe("refreshing");
 expect(during.reason?.en).toContain("workspace");expect(during.reason?.de).toContain("workspace");expect(during.reason?.en).not.toBe(during.reason?.de);
 expect(w.updates.some(update=>update.ready===false)).toBe(false);
 expect(w.last().state).toBe("ready");expect(w.last().ready).toBe(true);expect(w.last().refreshing).toBe(false);expect(w.last().stage).toBeNull();
 expect(w.store.state.readyMs).toBeGreaterThanOrEqual(5);expect(w.store.state.failure).toBeNull();
 expect(w.store.state.stages.workspace!.digest).toBe(w.after.stages.at(-1)!.digest);
});

test("a landing on a shared input refreshes the declaring stages in order and no other stage",async()=>{
 const path=allPaths.find(candidate=>shared(candidate).length>1)!,before=digestsOf(hashesOf({}));
 const w=world({state:readyState(before),landing:{[path]:"2".repeat(64)}});
 const ports={...w.ports,sleep:async(milliseconds:number,signal:AbortSignal)=>{w.store.now+=milliseconds;if(w.store.calls.length>=shared(path).length||signal.aborted)w.controller.abort();await new Promise<void>(done=>setTimeout(done,2));}};
 await runReadyV1(ports,w.controller.signal);
 expect(w.store.calls).toEqual(shared(path));
 expect(w.updates.some(update=>update.ready===false)).toBe(false);
});

test("a landing on no stage input changes nothing: no stage, ready, not refreshing",async()=>{
 const before=digestsOf(hashesOf({})),w=world({state:readyState(before),landing:{"unrelated/file.ts":"2".repeat(64)}});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.store.calls).toEqual([]);expect(w.last().ready).toBe(true);expect(w.last().refreshing).toBe(false);
});

test("a re-verification that fails leaves Ready: backoff, not ready, failure recorded, readyMs kept",async()=>{
 const path=landingOn("tools"),before=digestsOf(hashesOf({}));
 const w=world({state:readyState(before),landing:{[path]:"2".repeat(64)},outcomes:[{exit:1}]});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.store.calls).toEqual(["tools"]);
 expect(w.store.state.failure?.stage).toBe("tools");expect(w.store.state.readyMs).toBe(5);
 const last=w.last();expect(last.state).toBe("backoff");expect(last.ready).toBe(false);expect(last.refreshing).toBe(false);
});

test("a supervisor that adopts a running refresh keeps the worker ready and refreshing, and only a failed re-verification clears Ready",async()=>{
 const path=landingOn("workspace"),before=digestsOf(hashesOf({}));
 const w=world({state:readyState(before),landing:{[path]:"2".repeat(64)}});
 await runReadyV1({...w.ports,inflight:()=>"workspace"},w.controller.signal);
 const during=w.store.seen[0]!;
 expect(during.state).toBe("ready");expect(during.ready).toBe(true);expect(during.refreshing).toBe(true);expect(during.stage).toBe("workspace");
 expect(w.updates.some(update=>update.ready===false)).toBe(false);
 expect(w.store.state.readyMs).toBeGreaterThanOrEqual(5);
 const failing=world({state:readyState(before),landing:{[path]:"2".repeat(64)},outcomes:[{exit:1}]});
 let adopted=true;await runReadyV1({...failing.ports,inflight:()=>{const now=adopted?"workspace":null;return failing.store.calls.length>0?(adopted=false,null):now;}},failing.controller.signal);
 expect(failing.last().ready).toBe(false);expect(failing.store.state.failure?.stage).toBe("workspace");
});

test("a worker that never was ready reports the stage as work, not as ready",async()=>{
 const path=landingOn("workspace"),before=digestsOf(hashesOf({}));
 const w=world({state:readyState(before,null),landing:{[path]:"2".repeat(64)}});
 await runReadyV1(w.ports,w.controller.signal);
 const during=w.store.seen[0]!;expect(during.state).toBe("running");expect(during.ready).toBe(false);expect(during.refreshing).toBeFalsy();
});

test("the refreshing patch rewrites only working states and every consumer reads a refreshing record as ready but busy",()=>{
 expect(readyRefreshingPatchV1({tail:["x"]},"tools")).toEqual({tail:["x"]});
 expect(readyRefreshingPatchV1({state:"backoff",ready:false},null).refreshing).toBe(false);
 {const patch=readyRefreshingPatchV1({state:"backoff",ready:false,stage:"tools",refusal:{code:"stage-timeout"} as never},"tools");expect([patch.state,patch.ready,patch.refreshing,patch.refusal?.code]).toEqual(["ready",true,true,"stage-timeout"]);}
 for(const state of["running","waiting","observing"] as const){const patch=readyRefreshingPatchV1({state,ready:false,stage:"tools"},"tools");expect([patch.state,patch.ready,patch.refreshing,patch.refusal]).toEqual(["ready",true,true,null]);}
 const nowMs=1_000_000,record={state:"ready",ready:true,refreshing:true,stage:"tools",updatedMs:nowMs-1000,beatMilliseconds:P.beatByState.running};
 expect(readyForegroundBusyV1(record,nowMs)).toBe(true);
 expect(readyForegroundBusyV1({...record,refreshing:false},nowMs)).toBe(false);
 expect(readySidecarSilenceV1({status:record,nowMs}).state).toBe("fresh");
 expect(readySidecarSilenceV1({status:{...record,updatedMs:nowMs-10*P.statusHeartbeatMilliseconds},nowMs}).state).toBe("silent");
 expect(readyWorkerLivenessV1({status:record,heartbeatMs:nowMs,nowMs}).liveness).toBe("ready");
 expect(readyWorkerLivenessV1({status:{...record,updatedMs:nowMs-10*P.statusHeartbeatMilliseconds},heartbeatMs:nowMs,nowMs}).liveness).toBe("silent");
});

test("old evidence is no evidence: a stale stage last verified more than a day ago, or never verified here, is plain work and the worker is not ready",async()=>{
 const path=landingOn("workspace"),before=digestsOf(hashesOf({}));
 const old=world({state:readyState(before,5,1_000_000-P.refreshMaximumAgeMilliseconds-1),landing:{[path]:"2".repeat(64)}});
 await runReadyV1(old.ports,old.controller.signal);
 expect(old.store.seen[0]!.state).toBe("running");expect(old.store.seen[0]!.ready).toBe(false);
 const state=readyState(before),{workspace:_gone,...rest}=state.stages,fresh=world({state:{...state,stages:rest},landing:{}});
 await runReadyV1(fresh.ports,fresh.controller.signal);
 expect(fresh.store.seen[0]!.ready).toBe(false);expect(fresh.store.seen[0]!.refreshing).toBeFalsy();
});
