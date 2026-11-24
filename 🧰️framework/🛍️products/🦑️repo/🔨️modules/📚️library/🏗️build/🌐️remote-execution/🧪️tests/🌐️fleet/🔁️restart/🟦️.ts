import {afterEach,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {createServer} from "node:net";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {startRemoteCacheServerV1} from "../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {connectGrpcV1} from "../../../📡️wire/🚪️grpc/🟦️.ts";
import {executeOffloadV1} from "../../../🚀️offload/🔌️client/🟦️.ts";
import {clientHelpersV1,eventuallyV1} from "../../🧰️cluster/🟦️.ts";
process.env.SEMIO_REMOTE_EXECUTION_WORKER_ACTIONS="any";process.env.SEMIO_REMOTE_EXECUTION_UNCONFINED_ANY="I-ACCEPT-THE-RISK";
const {runRemoteExecutionWorkerV1}=await import("../../../👷️worker/🟦️.ts");

const bun=process.execPath,serve=join(import.meta.dir,"🚀️serve.ts"),scratch:string[]=[],children:ReturnType<typeof Bun.spawn>[]=[],aborts:AbortController[]=[];
afterEach(async()=>{for(const abort of aborts.splice(0))abort.abort();for(const child of children.splice(0))child.kill("SIGKILL");await Bun.sleep(100);for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});
const free=():Promise<number>=>new Promise(resolve=>{const server=createServer();server.listen(0,"127.0.0.1",()=>{const port=(server.address() as {port:number}).port;server.close(()=>resolve(port));});});
const start=async(environment:Record<string,string>)=>{
 const child=Bun.spawn([bun,serve],{stdout:"pipe",stderr:"inherit",env:{...process.env,...environment}});children.push(child);
 const reader=child.stdout.getReader();let text="";
 while(!text.includes("listening")){const row=await reader.read();if(row.done)throw Error("the scheduler did not start");text+=new TextDecoder().decode(row.value);}
 reader.releaseLock();return child;
};

test("a scheduler killed while a worker runs an action: the worker joins the new scheduler with the running task, the requester asks again and gets the one run; nothing is computed twice",async()=>{
 const root=mkdtempSync(join(tmpdir(),"sched-restart-"));scratch.push(root);
 const [cachePort,grpcPort,workerPort]=[await free(),await free(),await free()];
 const environment={CACHE_DIRECTORY:join(root,"cache"),CACHE_PORT:String(cachePort),GRPC_PORT:String(grpcPort),WORKER_PORT:String(workerPort),WITH_CACHE:"1"};
 let scheduler=await start(environment);
 const abort=new AbortController();aborts.push(abort);const events:string[]=[];
 const worker=runRemoteExecutionWorkerV1({scheduler:`http://127.0.0.1:${workerPort}`,token:undefined,cacheToken:undefined,concurrency:2,directory:join(root,"worker"),sandbox:"auto",actions:"any",unconfinedForTests:true,materialize:"clone",maximumBlobBytes:1<<30,inlineOutputBytes:4096,signal:abort.signal,name:"w1",onEvent:event=>{events.push(`${event.stage} ${event.detail}`);}});
 await eventuallyV1(()=>events.some(line=>line.startsWith("registered")),15000,"worker registered");
 const client=connectGrpcV1(`http://127.0.0.1:${grpcPort}`),{action}=clientHelpersV1(client,{});
 const digest=await action({arguments:["/bin/sh","-c","sleep 9; echo done > out.txt"],outputFiles:["out.txt"],outputPaths:["out.txt"]});
 const input={endpoint:`http://127.0.0.1:${grpcPort}`,token:undefined,instance:"main",action:digest,queueMilliseconds:20000,executeMilliseconds:60000};
 const stages:string[]=[];
 const first=executeOffloadV1({...input,rideThroughMilliseconds:30000,onStage:stage=>stages.push(stage)});
 await eventuallyV1(()=>stages.includes("EXECUTING")&&events.some(line=>line.startsWith("started")),15000,"the action is running");
 scheduler.kill("SIGKILL");await scheduler.exited;
 await Bun.sleep(1500);
 scheduler=await start(environment);
 const outcome=await first;
 expect(outcome.kind,JSON.stringify(outcome)).toBe("done");
 expect(events.filter(line=>line.startsWith("started")).length,"the worker started the action exactly once although the scheduler was replaced while it ran").toBe(1);
 expect(events.filter(line=>line.startsWith("registered")).length).toBeGreaterThanOrEqual(2);
 client.close();abort.abort();await worker;
},120000);

test("a requester that cannot reach the scheduler for longer than its ride-through budget gives up with an infrastructure decline that names the outage, never throws",async()=>{
 const port=await free();
 const outcome=await executeOffloadV1({endpoint:`http://127.0.0.1:${port}`,token:undefined,instance:"main",action:{hash:"a".repeat(64),size_bytes:1},connectMilliseconds:500,rideThroughMilliseconds:1500});
 expect(outcome.kind).toBe("declined");expect(outcome.kind==="declined"&&outcome.reason).toBe("infrastructure");expect(outcome.kind==="declined"&&outcome.detail).toMatch(/stayed away/u);
});

const boot=async(extra:Record<string,string>={},workers=1)=>{
 const root=mkdtempSync(join(tmpdir(),"sched-ride-"));scratch.push(root);
 const [cachePort,grpcPort,workerPort]=[await free(),await free(),await free()];
 const environment={CACHE_DIRECTORY:join(root,"cache"),CACHE_PORT:String(cachePort),GRPC_PORT:String(grpcPort),WORKER_PORT:String(workerPort),WITH_CACHE:"1",...extra};
 const abort=new AbortController();aborts.push(abort);const events:string[]=[];
 const agents=Array.from({length:workers},(_,index)=>runRemoteExecutionWorkerV1({scheduler:`http://127.0.0.1:${workerPort}`,token:undefined,cacheToken:undefined,concurrency:2,directory:join(root,`worker${index}`),sandbox:"auto",actions:"any",unconfinedForTests:true,materialize:"clone",maximumBlobBytes:1<<30,inlineOutputBytes:4096,signal:abort.signal,name:`w${index}`,onEvent:event=>{events.push(`${event.stage} ${event.detail}`);}}));
 return {root,environment,abort,events,agents,grpcPort,workerPort,endpoint:`http://127.0.0.1:${grpcPort}`};
};
const counters=async(workerPort:number)=>(await (await fetch(`http://127.0.0.1:${workerPort}/status`)).json() as {counters:Record<string,number>}).counters;

test("the ride-through budget counts the outage, not the run: an action that ran for longer than the whole budget before the scheduler restarted is re-attached by its name and finishes on the one worker that kept it, the restarted scheduler having restored it from its journal",async()=>{
 const rig=await boot({JOURNAL:"",RESUME_GRACE:"10"});
 rig.environment.JOURNAL=join(rig.root,"journal.json");
 let scheduler=await start(rig.environment);
 await eventuallyV1(()=>rig.events.some(line=>line.startsWith("registered")),15000,"worker registered");
 const client=connectGrpcV1(rig.endpoint),{action}=clientHelpersV1(client,{}),digest=await action({arguments:["/bin/sh","-c","sleep 9; echo done > out.txt"],outputFiles:["out.txt"],outputPaths:["out.txt"]});
 const stages:string[]=[],first=executeOffloadV1({endpoint:rig.endpoint,token:undefined,instance:"main",action:digest,queueMilliseconds:20000,executeMilliseconds:90000,rideThroughMilliseconds:6000,onStage:stage=>stages.push(stage)});
 await eventuallyV1(()=>stages.includes("EXECUTING")&&rig.events.some(line=>line.startsWith("started")),15000,"the action is running");
 await Bun.sleep(4500);
 scheduler.kill("SIGKILL");await scheduler.exited;await Bun.sleep(1200);
 scheduler=await start(rig.environment);
 const outcome=await first;
 expect(outcome.kind,JSON.stringify(outcome)).toBe("done");
 expect(rig.events.filter(line=>line.startsWith("started")).length,"the action was started once").toBe(1);
 expect(await counters(rig.workerPort)).toMatchObject({restored:1,resumed:1});
 client.close();rig.abort.abort();await Promise.all(rig.agents);
},120000);

test("an operation the restarted scheduler does not know is submitted again and the finished result comes from the action cache; nothing runs again",async()=>{
 const rig=await boot({},1);
 let scheduler=await start(rig.environment);
 await eventuallyV1(()=>rig.events.some(line=>line.startsWith("registered")),15000,"worker registered");
 const client=connectGrpcV1(rig.endpoint),{action}=clientHelpersV1(client,{}),digest=await action({arguments:["/bin/sh","-c","sleep 60"],outputFiles:[],outputPaths:[]});
 const stages:string[]=[],first=executeOffloadV1({endpoint:rig.endpoint,token:undefined,instance:"main",action:digest,queueMilliseconds:20000,executeMilliseconds:90000,rideThroughMilliseconds:30000,onStage:stage=>stages.push(stage)});
 await eventuallyV1(()=>stages.includes("EXECUTING")&&rig.events.some(line=>line.startsWith("started")),15000,"the action is running");
 rig.abort.abort();await Promise.all(rig.agents);
 await client.unary("build.bazel.remote.execution.v2.ActionCache/UpdateActionResult",{instance_name:"main",action_digest:digest,action_result:{exit_code:0}},{metadata:{}});
 scheduler.kill("SIGKILL");await scheduler.exited;await Bun.sleep(800);
 scheduler=await start(rig.environment);
 const outcome=await first;
 expect(outcome.kind,JSON.stringify(outcome)).toBe("done");expect(outcome.kind==="done"&&outcome.cached).toBe(true);
 expect(rig.events.filter(line=>line.startsWith("started")).length).toBe(1);
 client.close();
},120000);

test("a worker keeps the result of a finished action while the scheduler is away for longer than the old eight attempts lasted and reports it to the one that comes back",async()=>{
 const cachePort=await free(),cacheRoot=mkdtempSync(join(tmpdir(),"sched-cache-"));scratch.push(cacheRoot);
 const cache=await startRemoteCacheServerV1({directory:cacheRoot,host:"127.0.0.1",port:cachePort,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{}});
 const rig=await boot({JOURNAL:"",RESUME_GRACE:"10",CACHE_PORT:String(cachePort),WITH_CACHE:"0"},1);
 rig.environment.JOURNAL=join(rig.root,"journal.json");
 let scheduler=await start(rig.environment);
 await eventuallyV1(()=>rig.events.some(line=>line.startsWith("registered")),15000,"worker registered");
 const client=connectGrpcV1(rig.endpoint),{action}=clientHelpersV1(client,{}),digest=await action({arguments:["/bin/sh","-c","sleep 2; echo done > out.txt"],outputFiles:["out.txt"],outputPaths:["out.txt"]});
 const stages:string[]=[],first=executeOffloadV1({endpoint:rig.endpoint,token:undefined,instance:"main",action:digest,queueMilliseconds:20000,executeMilliseconds:90000,rideThroughMilliseconds:60000,onStage:stage=>stages.push(stage)});
 await eventuallyV1(()=>stages.includes("EXECUTING")&&rig.events.some(line=>line.startsWith("started")),15000,"the action is running");
 scheduler.kill("SIGKILL");await scheduler.exited;
 await eventuallyV1(()=>rig.events.some(line=>line.includes("report of")&&line.includes("kept")),20000,"the worker holds its result");
 await Bun.sleep(14000);
 scheduler=await start(rig.environment);
 const outcome=await first;
 expect(outcome.kind,JSON.stringify(outcome)).toBe("done");
 expect(rig.events.filter(line=>line.startsWith("started")).length).toBe(1);
 expect(rig.events.some(line=>line.includes("report lost"))).toBe(false);
 client.close();rig.abort.abort();await Promise.all(rig.agents);await cache.stop();
},120000);
