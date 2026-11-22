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

const profile=(name:string,properties:{name:string;value:string}[]=[{name:"OSFamily",value:"macos"}],concurrency=1):RemoteExecutionWorkerRegistrationV1=>({version:1,name,host:"h",platform:"darwin-arm64",properties,concurrency,sandbox:"isolated-root",runtime:"test",origin:name,capacity:{cores:8,memoryMegabytes:16384},confinement:"none",network:"unenforced"});
const resultOf=(exit=0,extra:object={}):RemoteExecutionTaskResultV1=>({version:1,outcome:"completed",status:{code:0,message:""},actionResult:Buffer.from(encodeProtobufV1(REAPI_V1.actionResult,{exit_code:exit,...extra})).toString("base64")});

function harness(extra:Partial<RemoteExecutionSchedulerOptionsV1>={}){
 let now=1_000_000;const memory=new Memory(),events:string[]=[];
 const scheduler=new RemoteExecutionSchedulerV1({content:memory,unmatchedSeconds:30,lingerSeconds:0.05,leaseSeconds:10,maximumAttempts:3,defaultActionSeconds:60,maximumActionSeconds:600,maximumQueued:100,heartbeatSeconds:2,pollSeconds:5,loadExpireSeconds:15,refusalCooldownSeconds:2,cache:{url:"http://c",relative:false},language:"none",sweepMilliseconds:0,now:()=>now,onEvent:event=>events.push(`${event.stage} ${event.detail}`),...extra});
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

test("an action whose worker falls silent past its deadline is rescheduled once on another worker and then fails back with its timeout; a refusal does not count as a failure",async()=>{
 const {memory,scheduler,advance,events}=harness({maximumAttempts:2,leaseSeconds:10}),requested=[{name:"semio-origin",value:"requester"}];
 const action=memory.action(requested,"deadline",5),run=submit(scheduler,action),one=scheduler.register(profile("one")),two=scheduler.register(profile("two"));
 await Bun.sleep(20);
 const first=await polled(scheduler,one.id);expect(first.task?.timeoutSeconds).toBe(5);
 const waiting=polled(scheduler,two.id,2000);
 for(let step=0;step<4;step++){advance(4);scheduler.heartbeat(one.id,[first.task!.id]);scheduler.heartbeat(two.id,[]);}
 const second=await waiting;expect(second.task?.attempt).toBe(2);expect(events.some(event=>event.startsWith("retried")&&event.includes("deadline"))).toBe(true);
 const third=polled(scheduler,one.id,2000);void third;
 for(let step=0;step<4;step++){advance(4);scheduler.heartbeat(two.id,[second.task!.id]);}
 await run.done;
 const outcome=run.rows.at(-1)!.outcome!;expect(outcome.status.code).toBe(4);expect(outcome.status.message).toMatch(/timeout/u);
 const refused=harness({maximumAttempts:2}),again=refused.memory.action([],"refused"),op=submit(refused.scheduler,again),only=refused.scheduler.register(profile("only"));
 await Bun.sleep(20);
 const given=await polled(refused.scheduler,only.id);
 expect(refused.scheduler.report(only.id,given.task!.id,{version:1,outcome:"refused",status:{code:8,message:"no headroom"},actionResult:null,refusal:"memory"})).toBe(true);
 expect(refused.scheduler.status().counters).toMatchObject({refused:1,retried:1});expect(refused.scheduler.status().workers[0]).toMatchObject({refused:1,headroom:false});
 refused.advance(3);
 const retry=await polled(refused.scheduler,only.id);expect(retry.task?.action).toEqual(again);expect(retry.task?.attempt).toBe(2);
 refused.scheduler.report(only.id,retry.task!.id,resultOf());await op.done;expect(op.rows.at(-1)!.outcome!.status.code).toBe(0);
});

test("an offloaded action that one worker's policy refuses runs on another worker; when every worker refuses it fails back at once with the refusal, and a non-offloaded action keeps the refusal as its result",async()=>{
 const {memory,scheduler}=harness({maximumAttempts:3}),requested=[{name:"semio-origin",value:"requester"}];
 const stale=scheduler.register(profile("stale")),fresh=scheduler.register(profile("fresh"));
 const run=submit(scheduler,memory.action(requested,"policy-1"));await Bun.sleep(20);
 const first=await polled(scheduler,stale.id);expect(first.task).not.toBeNull();
 scheduler.report(stale.id,first.task!.id,{version:1,outcome:"failed",status:{code:9,message:"Action refused by the worker policy: -Z embed-metadata=no is not allowed"},actionResult:null});
 const second=await polled(scheduler,fresh.id,2000);expect(second.task?.attempt).toBe(2);
 scheduler.report(fresh.id,second.task!.id,resultOf());
 await run.done;expect(run.rows.at(-1)!.outcome!.status.code).toBe(0);
 const all=harness({maximumAttempts:3}),only=all.scheduler.register(profile("only")),runAll=submit(all.scheduler,all.memory.action(requested,"policy-2"));await Bun.sleep(20);
 const given=await polled(all.scheduler,only.id);
 all.scheduler.report(only.id,given.task!.id,{version:1,outcome:"failed",status:{code:9,message:"Action refused by the worker policy: --sysroot is not allowed"},actionResult:null});
 await runAll.done;const status=runAll.rows.at(-1)!.outcome!.status;expect(status.message).toContain("Action refused by the worker policy: --sysroot is not allowed");expect(status.message).toContain("no other machine can take the action");
 const plain=harness({maximumAttempts:3}),one=plain.scheduler.register(profile("one")),runPlain=submit(plain.scheduler,plain.memory.action([],"policy-3"));await Bun.sleep(20);
 const taken=await polled(plain.scheduler,one.id);
 plain.scheduler.report(one.id,taken.task!.id,{version:1,outcome:"failed",status:{code:9,message:"Action refused by the worker policy: -o is not allowed"},actionResult:null});
 await runPlain.done;expect(runPlain.rows.at(-1)!.outcome!.status).toEqual({code:9,message:"Action refused by the worker policy: -o is not allowed"});
});

test("the idle answer the build door asks for is the answer the scheduler gives at submission: with a compatible worker that has headroom both say yes, and without one (no headroom, another toolchain build) both say no and the submission is declined with the counting in its event",async()=>{
 const T="aaaaaaaaaaaaaaaa",properties=(toolchain:string)=>[{name:"Arch",value:"arm64"},{name:"OSFamily",value:"macos"},{name:"semio-toolchain",value:toolchain},{name:"semio-target",value:"aarch64-apple-darwin"},{name:"semio-offload-format",value:"argv-1"}];
 const report=(origin:string,toolchain:string,headroom:boolean):Parameters<RemoteExecutionSchedulerV1["acceptLoad"]>[0]=>({version:1,origin,platform:{key:`macos-arm64-${toolchain}`,os:"macos",arch:"arm64",toolchain,target:"aarch64-apple-darwin"},capacity:{cores:8,memoryMegabytes:16384},running:0,queued:0,pressure:"normal",freeMegabytes:8000,load:1,headroom,wallMs:1_000_000,worker:{state:"idle",concurrency:2,running:0,sandbox:null,confinement:null,network:null},counters:{unit:{localHit:0,remoteHit:0,miss:0,stored:0},offload:{out:0,refused:0,fellBack:0}}});
 const control=[{name:"semio-offload",value:"if-idle"},{name:"semio-origin",value:"requester"},{name:"semio-offload-format",value:"argv-1"}];
 const answers:Record<string,{idle:number;declined:boolean;event:string}>={};
 for(const [label,workerToolchain,headroom] of [["ready",T,true],["no headroom",T,false],["other toolchain","bbbbbbbbbbbbbbbb",true]] as const){
  const {memory,scheduler,events}=harness();
  scheduler.acceptLoad(report("requester",T,true));scheduler.acceptLoad(report("worker-a",workerToolchain,headroom));
  scheduler.register({...profile("a",properties(workerToolchain),2),origin:"worker-a"} as RemoteExecutionWorkerRegistrationV1);
  const run=submit(scheduler,memory.action([...control],"truth-"+label));await Bun.sleep(30);
  const declined=run.rows.at(-1)?.outcome?.status.code===8;
  answers[label]={idle:scheduler.idleFor("requester").candidates,declined,event:events.find(line=>line.startsWith("declined"))??""};
  run.abort.abort();await run.done.catch(()=>undefined);
 }
 expect(answers.ready).toMatchObject({idle:1,declined:false});
 expect(answers["no headroom"]).toMatchObject({idle:0,declined:true});expect(answers["other toolchain"]).toMatchObject({idle:0,declined:true});
 expect(answers["no headroom"]!.event).toContain("1 workers, 1 compatible, 0 with headroom");expect(answers["other toolchain"]!.event).toContain("1 workers, 0 compatible, 1 with headroom");
});

test("an idle answer given to the door holds for 20 s: the units of that machine are queued, not declined, when the promised headroom was taken in between, while a machine that was never told is declined at once",async()=>{
 const T="aaaaaaaaaaaaaaaa",properties=[{name:"Arch",value:"arm64"},{name:"OSFamily",value:"macos"},{name:"semio-toolchain",value:T},{name:"semio-target",value:"aarch64-apple-darwin"},{name:"semio-offload-format",value:"argv-1"}];
 const report=(origin:string,headroom:boolean):Parameters<RemoteExecutionSchedulerV1["acceptLoad"]>[0]=>({version:1,origin,platform:{key:`macos-arm64-${T}`,os:"macos",arch:"arm64",toolchain:T,target:"aarch64-apple-darwin"},capacity:{cores:8,memoryMegabytes:16384},running:0,queued:0,pressure:"normal",freeMegabytes:8000,load:1,headroom,wallMs:1_000_000,worker:{state:"idle",concurrency:2,running:0,sandbox:null,confinement:null,network:null},counters:{unit:{localHit:0,remoteHit:0,miss:0,stored:0},offload:{out:0,refused:0,fellBack:0}}});
 const control=(origin:string)=>[{name:"semio-offload",value:"if-idle"},{name:"semio-origin",value:origin},{name:"semio-offload-format",value:"argv-1"}];
 const {memory,scheduler,advance}=harness();
 scheduler.acceptLoad(report("told",true));scheduler.acceptLoad(report("untold",true));scheduler.acceptLoad(report("worker-a",true));
 scheduler.register({...profile("a",properties,2),origin:"worker-a"} as RemoteExecutionWorkerRegistrationV1);
 expect(scheduler.idleFor("told").candidates).toBe(1);
 scheduler.acceptLoad(report("worker-a",false));
 const outcome=async(origin:string,salt:string)=>{const run=submit(scheduler,memory.action(control(origin),salt));await Bun.sleep(30);const code=run.rows.at(-1)?.outcome?.status.code;run.abort.abort();await run.done.catch(()=>undefined);return code;};
 expect(await outcome("untold","u1"),"declined").toBe(8);
 expect(await outcome("told","t1"),"queued, no outcome yet").toBeUndefined();
 advance(25);scheduler.acceptLoad(report("worker-a",false));
 expect(await outcome("told","t2"),"the hold ended").toBe(8);
});

test("an offloaded action that a worker refuses for lack of headroom goes to another machine that has some, and is declined only when nobody else qualifies",async()=>{
 const T="aaaaaaaaaaaaaaaa",properties=[{name:"Arch",value:"arm64"},{name:"OSFamily",value:"macos"},{name:"semio-toolchain",value:T},{name:"semio-target",value:"aarch64-apple-darwin"},{name:"semio-offload-format",value:"argv-1"}];
 const report=(origin:string):Parameters<RemoteExecutionSchedulerV1["acceptLoad"]>[0]=>({version:1,origin,platform:{key:`macos-arm64-${T}`,os:"macos",arch:"arm64",toolchain:T,target:"aarch64-apple-darwin"},capacity:{cores:8,memoryMegabytes:16384},running:0,queued:0,pressure:"normal",freeMegabytes:8000,load:1,headroom:true,wallMs:1_000_000,worker:{state:"idle",concurrency:2,running:0,sandbox:null,confinement:null,network:null},counters:{unit:{localHit:0,remoteHit:0,miss:0,stored:0},offload:{out:0,refused:0,fellBack:0}}});
 const control=[{name:"semio-offload",value:"if-idle"},{name:"semio-origin",value:"requester"},{name:"semio-offload-format",value:"argv-1"}];
 const {memory,scheduler}=harness();
 for(const origin of ["requester","one","two"])scheduler.acceptLoad(report(origin));
 const one=scheduler.register({...profile("one",properties,2),origin:"one"} as RemoteExecutionWorkerRegistrationV1),two=scheduler.register({...profile("two",properties,2),origin:"two"} as RemoteExecutionWorkerRegistrationV1);
 const run=submit(scheduler,memory.action(control,"refused-1"));await Bun.sleep(30);
 const first=await Promise.race([polled(scheduler,one.id,300),polled(scheduler,two.id,300)]),taker=first.task===null?undefined:(await polled(scheduler,one.id,10).then(()=>undefined)),given=first.task;
 expect(given).not.toBeNull();
 const holder=[one,two].find(worker=>scheduler.status().workers.find(row=>row.id===worker.id)!.running>0)!,other=holder.id===one.id?two:one;
 scheduler.report(holder.id,given!.id,{version:1,outcome:"refused",refusal:"memory",status:{code:0,message:""},actionResult:null} as never);
 const next=await polled(scheduler,other.id,2000);expect(next.task,"the other machine got it").not.toBeNull();expect(taker).toBeUndefined();
 scheduler.report(other.id,next.task!.id,{version:1,outcome:"refused",refusal:"memory",status:{code:0,message:""},actionResult:null} as never);
 await run.done;expect(run.rows.at(-1)!.outcome!.status.code,"nobody else: declined").toBe(8);
});
