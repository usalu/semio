import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {decodeProtobufV1,encodeProtobufV1,REAPI_V1} from "../../📡️wire/📦️protobuf/🟦️.ts";
import type {RemoteExecutionContentV1} from "../../🗄️content/🟦️.ts";
import {RemoteExecutionSchedulerV1,type RemoteExecutionSchedulerOptionsV1,type RemoteExecutionSnapshotV1} from "../🟦️.ts";
import type {RemoteExecutionDigestV1,RemoteExecutionWorkerRegistrationV1,RemoteExecutionTaskResultV1} from "../../🟦️.ts";

const sha=(data:Uint8Array)=>createHash("sha256").update(data).digest("hex");

class Memory implements RemoteExecutionContentV1{
 blobs=new Map<string,Uint8Array>();results=new Map<string,Uint8Array>();
 put(bytes:Uint8Array):RemoteExecutionDigestV1{const digest={hash:sha(bytes),size:bytes.length};this.blobs.set(digest.hash,bytes);return digest;}
 async has(_:string,hash:string){return this.blobs.has(hash);}
 async missing(_:string,digests:readonly RemoteExecutionDigestV1[]){return digests.filter(digest=>!this.blobs.has(digest.hash));}
 async read(_:string,digest:RemoteExecutionDigestV1){return this.blobs.get(digest.hash);}
 async stream(_:string,digest:RemoteExecutionDigestV1){const bytes=this.blobs.get(digest.hash);return bytes?(async function*(){yield bytes;})():undefined;}
 async write(_:string,digest:RemoteExecutionDigestV1,source:Uint8Array|AsyncIterable<Uint8Array>){this.blobs.set(digest.hash,source instanceof Uint8Array?source:new Uint8Array());}
 async readActionResult(_:string,hash:string){return this.results.get(hash);}
 async writeActionResult(_:string,hash:string,bytes:Uint8Array){this.results.set(hash,bytes);}
 action(properties:{name:string;value:string}[]=[],salt="",timeout=0):RemoteExecutionDigestV1{
  const command=this.put(encodeProtobufV1(REAPI_V1.command,{arguments:["/bin/true"],platform:{properties}}));
  return this.put(encodeProtobufV1(REAPI_V1.action,{command_digest:{hash:command.hash,size_bytes:command.size},salt:new TextEncoder().encode(salt),...(timeout?{timeout:{seconds:timeout}}:{})}));
 }
}

const profile=(name:string,properties:{name:string;value:string}[]=[{name:"OSFamily",value:"macos"}],concurrency=1):RemoteExecutionWorkerRegistrationV1=>({version:1,name,host:"h",platform:"darwin-arm64",properties,concurrency,sandbox:"isolated-root",runtime:"test"});
const resultOf=(exit=0,extra:object={}):RemoteExecutionTaskResultV1=>({version:1,outcome:"completed",status:{code:0,message:""},actionResult:Buffer.from(encodeProtobufV1(REAPI_V1.actionResult,{exit_code:exit,...extra})).toString("base64")});

function harness(extra:Partial<RemoteExecutionSchedulerOptionsV1>={}){
 let now=1_000_000;const memory=new Memory(),events:string[]=[];
 const scheduler=new RemoteExecutionSchedulerV1({content:memory,unmatchedSeconds:30,lingerSeconds:0.05,leaseSeconds:10,maximumAttempts:3,defaultActionSeconds:60,maximumQueued:100,heartbeatSeconds:2,pollSeconds:5,cache:{url:"http://c",relative:false},language:"none",sweepMilliseconds:0,now:()=>now,onEvent:event=>events.push(`${event.stage} ${event.detail}`),...extra});
 return {memory,scheduler,events,advance:(seconds:number)=>{now+=seconds*1000;scheduler.sweep();}};
}
const drain=async(stream:AsyncGenerator<RemoteExecutionSnapshotV1>)=>{const rows:RemoteExecutionSnapshotV1[]=[];for await(const row of stream)rows.push(row);return rows;};
const submit=(scheduler:RemoteExecutionSchedulerV1,action:RemoteExecutionDigestV1,abort=new AbortController())=>{
 const rows:RemoteExecutionSnapshotV1[]=[],done=(async()=>{for await(const row of scheduler.execute({instance:"main",action,skipCacheLookup:false},abort.signal))rows.push(row);})();
 return {rows,done,abort};
};
const polled=async(scheduler:RemoteExecutionSchedulerV1,id:string,milliseconds=1000)=>scheduler.poll(id,milliseconds,new AbortController().signal);

test("dispatch is first-in-first-out per platform, matches by property subset and honours a worker's concurrency",async()=>{
 const {memory,scheduler}=harness(),a=memory.action([],"a"),b=memory.action([],"b"),special=memory.action([{name:"pool",value:"gpu"}],"gpu");
 const one=submit(scheduler,a),two=submit(scheduler,b),three=submit(scheduler,special);
 const plain=scheduler.register(profile("plain")),gpu=scheduler.register(profile("gpu",[{name:"OSFamily",value:"macos"},{name:"pool",value:"gpu"}],2));
 await Bun.sleep(20);
 const first=await polled(scheduler,plain.id);expect(first.task?.action).toEqual(a);
 const blocked=await polled(scheduler,plain.id,30);expect(blocked.task).toBeNull();
 const second=await polled(scheduler,gpu.id);expect(second.task?.action).toEqual(b);
 const third=await polled(scheduler,gpu.id);expect(third.task?.action).toEqual(special);
 expect(scheduler.report(plain.id,first.task!.id,resultOf())).toBe(true);expect(scheduler.report(gpu.id,second.task!.id,resultOf())).toBe(true);expect(scheduler.report(gpu.id,third.task!.id,resultOf())).toBe(true);
 await Promise.all([one.done,two.done,three.done]);
 expect(one.rows.map(row=>row.stage)).toEqual(["CACHE_CHECK","QUEUED","EXECUTING","COMPLETED"]);
 expect(scheduler.status().counters).toMatchObject({completed:3,failed:0});
});

test("a worker that stops heartbeating loses its work; the task is retried with a new attempt and fails after the last one",async()=>{
 const {memory,scheduler,advance,events}=harness({maximumAttempts:2}),action=memory.action([],"lease"),run=submit(scheduler,action);
 const first=scheduler.register(profile("one"));await Bun.sleep(20);
 const task=(await polled(scheduler,first.id)).task!;expect(task.attempt).toBe(1);
 advance(11);expect(scheduler.status().workers).toHaveLength(0);
 expect((await polled(scheduler,first.id,10)).known).toBe(false);
 const second=scheduler.register(profile("two"));const retry=(await polled(scheduler,second.id)).task!;
 expect([retry.attempt,retry.id.endsWith(".2")]).toEqual([2,true]);
 expect(scheduler.report(first.id,task.id,resultOf())).toBe(false);
 advance(11);await run.done;
 expect(run.rows.at(-1)!.outcome!.status).toMatchObject({code:14});expect(run.rows.at(-1)!.outcome!.status.message).toMatch(/gave up after 2 attempts/u);
 expect(events.filter(row=>row.startsWith("worker-lost"))).toHaveLength(2);expect(events.some(row=>row.startsWith("retried"))).toBe(true);
});

test("heartbeats keep a worker alive, deliver cancellations and reconcile tasks the worker no longer knows",async()=>{
 const {memory,scheduler,advance}=harness(),run=submit(scheduler,memory.action([],"hb")),worker=scheduler.register(profile("w"));await Bun.sleep(20);
 const task=(await polled(scheduler,worker.id)).task!;
 for(let index=0;index<5;index++){advance(8);expect(scheduler.heartbeat(worker.id,[task.id])).toEqual({version:1,known:true,cancel:[]});}
 expect(scheduler.status().workers).toHaveLength(1);
 expect(scheduler.heartbeat(worker.id,[])).toMatchObject({known:true});
 const again=(await polled(scheduler,worker.id)).task!;expect(again.attempt).toBe(2);
 run.abort.abort();await run.done;await Bun.sleep(120);
 expect(scheduler.heartbeat(worker.id,[again.id]).cancel).toEqual([again.id]);
 expect(scheduler.report(worker.id,again.id,{version:1,outcome:"cancelled",status:{code:1,message:"Cancelled"},actionResult:null})).toBe(true);
 await Bun.sleep(10);expect(scheduler.status().counters.cancelled).toBe(1);
});

test("an action nobody can run fails after the grace period; one the cache already knows is answered without a worker; the queue is bounded",async()=>{
 const {memory,scheduler,advance}=harness({maximumQueued:2,unmatchedSeconds:5}),odd=memory.action([{name:"OSFamily",value:"amiga"}],"odd"),run=submit(scheduler,odd);
 await Bun.sleep(20);scheduler.register(profile("w"));advance(3);expect(run.rows.at(-1)!.stage).toBe("QUEUED");advance(3);await run.done;
 expect(run.rows.at(-1)!.outcome!.status.code).toBe(9);expect(run.rows.at(-1)!.outcome!.status.message).toMatch(/OSFamily=amiga/u);
 const known=memory.action([],"known");memory.results.set(known.hash,encodeProtobufV1(REAPI_V1.actionResult,{exit_code:0,stdout_raw:new TextEncoder().encode("from cache")}));
 const rows=await drain(scheduler.execute({instance:"main",action:known,skipCacheLookup:false},new AbortController().signal));
 expect(rows.at(-1)!.outcome).toMatchObject({cached:true,status:{code:0}});expect(scheduler.status().counters.cached).toBe(1);
 const first=submit(scheduler,memory.action([{name:"x",value:"1"}],"q1")),second=submit(scheduler,memory.action([{name:"x",value:"1"}],"q2")),third=submit(scheduler,memory.action([{name:"x",value:"1"}],"q3"));
 await third.done;expect(third.rows.at(-1)!.outcome!.status.code).toBe(8);first.abort.abort();second.abort.abort();await Promise.all([first.done,second.done]);
});

test("only successful results enter the action cache; malformed results and results from the wrong worker or attempt are refused",async()=>{
 const {memory,scheduler}=harness(),worker=scheduler.register(profile("w",[],4)),other=scheduler.register(profile("other",[],4));
 const good=memory.action([],"good"),bad=memory.action([],"bad"),junk=memory.action([],"junk"),a=submit(scheduler,good),b=submit(scheduler,bad),c=submit(scheduler,junk);await Bun.sleep(20);
 const tasks=[await polled(scheduler,worker.id),await polled(scheduler,worker.id),await polled(scheduler,worker.id)].map(row=>row.task!);
 expect(scheduler.report(other.id,tasks[0]!.id,resultOf())).toBe(false);
 expect(scheduler.report(worker.id,tasks[0]!.id,resultOf(0,{stdout_raw:new TextEncoder().encode("ok")}))).toBe(true);
 expect(scheduler.report(worker.id,tasks[1]!.id,resultOf(2))).toBe(true);
 expect(scheduler.report(worker.id,tasks[2]!.id,{version:1,outcome:"completed",status:{code:0,message:""},actionResult:Buffer.from([0x0a,0x05,0x61]).toString("base64")})).toBe(true);
 await Promise.all([a.done,b.done,c.done]);
 expect(memory.results.has(good.hash)).toBe(true);expect(memory.results.has(bad.hash)).toBe(false);expect(memory.results.has(junk.hash)).toBe(false);
 expect(decodeProtobufV1(REAPI_V1.actionResult,memory.results.get(good.hash)!).stdout_raw).toEqual(new TextEncoder().encode("ok"));
 expect([a,b,c].map(row=>row.rows.at(-1)!.outcome!.status.code)).toEqual([0,0,13]);
 expect(scheduler.report(worker.id,tasks[0]!.id,resultOf())).toBe(false);
});

test("a missing input reported by a worker is passed through; infrastructure failures are retried elsewhere",async()=>{
 const {memory,scheduler}=harness(),worker=scheduler.register(profile("w")),run=submit(scheduler,memory.action([],"flaky"));await Bun.sleep(20);
 const first=(await polled(scheduler,worker.id)).task!;
 scheduler.report(worker.id,first.id,{version:1,outcome:"failed",status:{code:14,message:"store unreachable"},actionResult:null});
 const second=(await polled(scheduler,worker.id)).task!;expect(second.attempt).toBe(2);
 const missing={hash:"b".repeat(64),size:3};
 scheduler.report(worker.id,second.id,{version:1,outcome:"failed",status:{code:9,message:"Missing blob",missing:[missing]},actionResult:null});
 await run.done;expect(run.rows.at(-1)!.outcome!.status).toEqual({code:9,message:"Missing blob",missing:[missing]});
});
