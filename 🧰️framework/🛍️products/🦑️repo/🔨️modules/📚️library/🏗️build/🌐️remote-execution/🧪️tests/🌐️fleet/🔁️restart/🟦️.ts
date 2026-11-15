import {afterEach,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {createServer} from "node:net";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {connectGrpcV1} from "../../../📡️wire/🚪️grpc/🟦️.ts";
import {executeOffloadRidingThroughV1} from "../../../🚀️offload/🔌️client/🟦️.ts";
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
 const digest=await action({arguments:["/bin/sh","-c","sleep 5; echo done > out.txt"],outputFiles:["out.txt"],outputPaths:["out.txt"]});
 const input={endpoint:`http://127.0.0.1:${grpcPort}`,token:undefined,instance:"main",action:digest,queueMilliseconds:20000,executeMilliseconds:60000};
 const stages:string[]=[];
 const first=executeOffloadRidingThroughV1({...input,onStage:stage=>stages.push(stage)},30000);
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

test("a requester that cannot reach the scheduler for longer than its budget gives up and reports a transient decline, never throws",async()=>{
 const port=await free();
 const outcome=await executeOffloadRidingThroughV1({endpoint:`http://127.0.0.1:${port}`,token:undefined,instance:"main",action:{hash:"a".repeat(64),size_bytes:1},connectMilliseconds:500},1500);
 expect(outcome.kind).toBe("declined");expect(outcome.kind==="declined"&&outcome.transient).toBe(true);
});
