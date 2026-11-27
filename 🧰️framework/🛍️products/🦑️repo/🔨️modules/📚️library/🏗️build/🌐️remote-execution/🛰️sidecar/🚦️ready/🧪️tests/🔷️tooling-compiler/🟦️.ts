import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createReadyLeaseV1,emptyReadyStateV1,readyDigestsV1,runReadyV1,type ReadyLocalStateV1,type ReadyPortsV1,type ReadyStageV1,type ReadyStatusV1}from"../../🟦️.ts";
import{inspectTypescriptCompilerV1}from"../../../../../🚀️bootstrap/🔷️typescript-compiler/🟨️.mjs";

/** 🔷️ The stage `dependencies` fails with the typed compiler reason when the tooling compiler is not the declared one. */
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const stage=(name:string):ReadyStageV1=>({name,kind:"repo",arguments:[name],timeoutMilliseconds:1000,inputs:[],bootstrapInput:false});
const wrong={en:"Compiler EN: resolved /x/node_modules/typescript, found 7.0.2, wanted 5.9.3.",de:"Compiler DE: aufgelöst /x/node_modules/typescript, gefunden 7.0.2, verlangt 5.9.3."};

function world(options:Readonly<{names:readonly string[];compiler?:()=>typeof wrong|null;done?:readonly string[];discard?:()=>Promise<{receipts:number;generations:number}>}>){
 const lease=mkdtempSync(join(tmpdir(),"ready-tc-"));scratch.push(lease);
 const stages=options.names.map(stage),digest=readyDigestsV1({platform:"linux",hash:()=>"b".repeat(64),stages});
 const shared={state:{...emptyReadyStateV1(),stages:Object.fromEntries(digest.stages.filter(row=>options.done?.includes(row.name)).map(row=>[row.name,{digest:row.digest,doneMs:1}]))} as ReadyLocalStateV1,marker:false,now:1_000_000,calls:[] as string[]};
 const controller=new AbortController(),updates:Partial<ReadyStatusV1>[]=[];let sleeps=0;
 const ports:ReadyPortsV1={
  server:false,stages,now:()=>shared.now,jitter:0,cores:4,
  sleep:async(milliseconds,signal)=>{shared.now+=milliseconds;await new Promise<void>(done=>setTimeout(done,2));if(++sleeps>=1||signal.aborted)controller.abort();},
  digests:async()=>digest,
  state:{read:()=>structuredClone(shared.state),write:state=>{shared.state=structuredClone(state);}},
  marker:{exists:()=>shared.marker,write:()=>{shared.marker=true;}},
  repoPresent:()=>true,human:()=>false,
  lease:createReadyLeaseV1({directory:lease,owner:"one",now:()=>Date.now()}),
  memory:()=>({availableMegabytes:8192,ok:true}),
  run:async row=>{shared.calls.push(row.name);return{exit:0,timedOut:false,spawnFailed:false,cancelled:false,stalled:false,detached:false,startedMs:shared.now,peakMegabytes:1,tail:["done"]};},
  status:{update:patch=>{updates.push(patch);}},say:()=>{},
  ...options.compiler===undefined?{}:{compiler:options.compiler},
  ...options.discard===undefined?{}:{discardDependencies:options.discard},
 };
 return{ports,controller,shared,updates,last:()=>Object.assign({},...updates) as Partial<ReadyStatusV1>};
}

test("dependencies that exit 0 with a wrong tooling compiler is a failure of that stage, typed, with the reason in both languages in the tail",async()=>{
 const w=world({names:["dependencies"],compiler:()=>wrong});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.shared.calls).toEqual(["dependencies"]);
 expect(w.shared.state.stages.dependencies).toBeUndefined();
 expect(w.shared.state.failure?.code).toBe("tooling-compiler");expect(w.shared.state.failure?.stage).toBe("dependencies");
 expect(w.shared.state.failure?.tail).toEqual([wrong.en,wrong.de]);
 expect(w.last().refusal?.code).toBe("tooling-compiler");expect(w.last().refusal?.en).toContain("dependencies");expect(w.last().refusal?.de).toContain("dependencies");
 expect(w.last().ready).toBe(false);
});

test("a declared compiler lets the stage complete, and a stage other than dependencies never asks",async()=>{
 let asked=0;
 const ok=world({names:["dependencies"],compiler:()=>{asked++;return null;}});
 await runReadyV1(ok.ports,ok.controller.signal);
 expect(ok.shared.state.stages.dependencies).toBeDefined();expect(ok.shared.state.failure).toBeNull();expect(asked).toBeGreaterThanOrEqual(1);
 asked=0;
 const other=world({names:["tools"],compiler:()=>{asked++;return wrong;}});
 await runReadyV1(other.ports,other.controller.signal);
 expect(other.shared.state.stages.tools).toBeDefined();expect(asked).toBe(0);
});

test("a machine whose dependencies stage was recorded done before the check existed is caught on the next turn without running anything first",async()=>{
 const w=world({names:["dependencies"],done:["dependencies"],compiler:()=>wrong});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.shared.calls).toEqual([]);
 expect(w.shared.state.stages.dependencies).toBeUndefined();
 expect(w.shared.state.failure?.code).toBe("tooling-compiler");expect(w.shared.state.failure?.tail).toEqual([wrong.en,wrong.de]);
 expect(w.last().refusal?.code).toBe("tooling-compiler");
});

test("the real inspection of this checkout declares and finds the compiler, so the real port returns nothing to refuse",()=>{
 const seen=inspectTypescriptCompilerV1();
 expect(seen.ok).toBe(true);
});

test("after a compiler failure the dependency receipts are discarded once per digest so the next stage rebuilds instead of repeating the same verdict, on a fresh failure and on a stage recorded done",async()=>{
 let discards=0;const discard=async()=>{discards++;return{receipts:2,generations:6};};
 const fresh=world({names:["dependencies"],compiler:()=>wrong,discard});
 await runReadyV1(fresh.ports,fresh.controller.signal);
 expect(discards).toBe(1);expect(fresh.shared.state.failure?.code).toBe("tooling-compiler");
 const recorded=world({names:["dependencies"],done:["dependencies"],compiler:()=>wrong,discard});
 await runReadyV1(recorded.ports,recorded.controller.signal);
 expect(discards).toBe(2);
 const healthy=world({names:["dependencies"],compiler:()=>null,discard});
 await runReadyV1(healthy.ports,healthy.controller.signal);
 expect(discards).toBe(2);
});
