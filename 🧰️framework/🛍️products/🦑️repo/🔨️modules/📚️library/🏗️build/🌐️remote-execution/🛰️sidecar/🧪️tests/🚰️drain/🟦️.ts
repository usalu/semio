/** 🚰️ Contract: a sidecar whose code changed finishes the work in flight before it restarts. `drainWhenIdleV1` calls its callback once, and only after the drain signal fired and nothing is in flight; without a drain signal, or after it was stopped, never. */
import {afterEach,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {drainWhenIdleV1,runRemoteExecutionAgentV1} from "../../../🛰️agent/🟦️.ts";
import {startClusterV1,eventuallyV1,type ClusterV1} from "../../../🧪️tests/🧰️cluster/🟦️.ts";

let cluster:ClusterV1|undefined;const scratch:string[]=[];
afterEach(async()=>{await cluster?.stop().catch(()=>{});cluster=undefined;for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});
const temporary=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`drain-${label}-`));scratch.push(path);return path;};

const wait=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));

test("without a drain signal nothing is ever called",async()=>{
 let calls=0;const stop=drainWhenIdleV1(undefined,()=>0,()=>{calls++;},2);
 await wait(20);stop();expect(calls).toBe(0);
});

test("the drain waits for the work in flight and then calls once",async()=>{
 let running=2,calls=0;const drain=new AbortController();
 drainWhenIdleV1(drain.signal,()=>running,()=>{calls++;},2);
 await wait(15);expect(calls).toBe(0);
 drain.abort();await wait(20);expect(calls,"still two in flight").toBe(0);
 running=1;await wait(20);expect(calls).toBe(0);
 running=0;await wait(20);expect(calls).toBe(1);
 await wait(20);expect(calls,"called once").toBe(1);
});

test("a drain that fired before the watch started with nothing in flight calls at once",()=>{
 const drain=new AbortController();drain.abort();let calls=0;
 drainWhenIdleV1(drain.signal,()=>0,()=>{calls++;},50);
 expect(calls).toBe(1);
});

test("a stopped watch never calls",async()=>{
 let running=1,calls=0;const drain=new AbortController();
 const stop=drainWhenIdleV1(drain.signal,()=>running,()=>{calls++;},2);
 drain.abort();await wait(10);stop();running=0;await wait(20);expect(calls).toBe(0);
});

test("the busy probe is asked again at every check, so work that starts during the drain is waited for",async()=>{
 let asked=0,running=1,calls=0;const drain=new AbortController();
 drainWhenIdleV1(drain.signal,()=>{asked++;return running;},()=>{calls++;},2);
 drain.abort();await wait(20);running=0;await wait(20);
 expect(asked).toBeGreaterThan(2);expect(calls).toBe(1);
});

test("an agent that is told to drain finishes the action in flight (a complete result reaches the requester), stops taking work and ends by itself, long before the grace is over",async()=>{
 cluster=await startClusterV1({workers:0,root:temporary("cluster"),scheduler:{refusalCooldownSeconds:1}});
 const root=temporary("agent"),controller=new AbortController(),drain=new AbortController();
 const host={version:1,atMs:Date.now()+86_400_000,platform:"darwin",cores:8,totalMegabytes:65536,availableMegabytes:60000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0.5,openFiles:1000,openFilesMaximum:200000,attachedImages:0};
 let ended=false;
 const agent=runRemoteExecutionAgentV1({root,environment:{SEMIO_UNIT_CACHE_TOOLCHAIN:"rustc 1.0.0-nightly (abcdef 2026-07-20)\nhost: aarch64-apple-darwin\n",SEMIO_FLEET_ORIGIN:"g1-drain"},origin:"g1-drain",scheduler:cluster.server.workerUrl,concurrency:2,directory:join(root,"work"),sandbox:"isolated-root",materialize:"copy",maximumBlobBytes:1048576,inlineOutputBytes:0,loadIntervalSeconds:1,signal:controller.signal,drain:drain.signal,view:async()=>({host,grants:[],waiters:[]}) as never,admitRetryMilliseconds:300,actions:"any",unconfinedForTests:true,network:"deny",confinement:"best-effort"}).then(()=>{ended=true;});
 const load=async():Promise<any>=>(await fetch(cluster!.server.workerUrl+"/v1/fleet/load")).json();
 const row=async():Promise<any>=>(await load()).machines.find((machine:any)=>machine.origin==="g1-drain");
 await eventuallyV1(async()=>(await row())?.worker?.state==="idle",30000,"the worker is registered and idle");
 const action=await cluster.action({arguments:["/bin/sh","-c","sleep 3; echo done > out.txt"],outputPaths:["out.txt"]});
 const run=cluster.execute(action);
 await eventuallyV1(async()=>(await row())?.worker?.state==="busy",30000,"the worker runs the action");
 const told=Date.now();drain.abort();
 await Bun.sleep(1000);
 expect(ended,"the agent waits for the action in flight").toBe(false);
 const result=await run;
 expect(result.response.status.code).toBe(0);expect(result.response.result.exit_code,"the action in flight was finished, not cancelled").toBe(0);expect(result.response.result.output_files).toHaveLength(1);
 await eventuallyV1(()=>ended,15000,"the agent ends by itself once the action is done");
 expect(Date.now()-told,"within seconds, not within the grace").toBeLessThan(20000);
 controller.abort();await agent;
},120000);
