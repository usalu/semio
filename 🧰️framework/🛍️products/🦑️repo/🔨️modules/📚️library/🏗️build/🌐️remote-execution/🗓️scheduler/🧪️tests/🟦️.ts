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
 return {memory,scheduler,events,advance:(seconds:number)=>{now+=seconds*1000;scheduler.sweep();},elapse:(seconds:number)=>{now+=seconds*1000;}};
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

test("an action that waits for a machine (semio-offload=queue) is queued where an if-idle action is declined, and is given to a worker as soon as one has headroom",async()=>{
 const T="aaaaaaaaaaaaaaaa",properties=[{name:"Arch",value:"arm64"},{name:"OSFamily",value:"macos"},{name:"semio-toolchain",value:T},{name:"semio-target",value:"aarch64-apple-darwin"},{name:"semio-offload-format",value:"argv-1"}];
 const report=(origin:string,headroom:boolean):Parameters<RemoteExecutionSchedulerV1["acceptLoad"]>[0]=>({version:1,origin,platform:{key:`macos-arm64-${T}`,os:"macos",arch:"arm64",toolchain:T,target:"aarch64-apple-darwin"},capacity:{cores:8,memoryMegabytes:16384},running:0,queued:0,pressure:"normal",freeMegabytes:8000,load:1,headroom,wallMs:1_000_000,worker:{state:"idle",concurrency:2,running:0,sandbox:null,confinement:null,network:null},counters:{unit:{localHit:0,remoteHit:0,miss:0,stored:0},offload:{out:0,refused:0,fellBack:0}}});
 const control=(value:string)=>[{name:"semio-offload",value},{name:"semio-origin",value:"requester"},{name:"semio-offload-format",value:"argv-1"}];
 const {memory,scheduler,events}=harness();
 scheduler.acceptLoad(report("requester",true));scheduler.acceptLoad(report("worker-a",false));
 const worker=scheduler.register({...profile("a",properties,2),origin:"worker-a"} as RemoteExecutionWorkerRegistrationV1);
 const idle=submit(scheduler,memory.action(control("if-idle"),"idle"));await Bun.sleep(30);
 expect(idle.rows.at(-1)?.outcome?.status.code,"declined at once").toBe(8);
 const queued=submit(scheduler,memory.action(control("queue"),"queue"));await Bun.sleep(30);
 expect(queued.rows.at(-1)?.outcome,"no outcome while nobody has headroom").toBeUndefined();expect(queued.rows.at(-1)?.stage).toBe("QUEUED");
 expect(events.filter(line=>line.startsWith("declined")).length,"only the if-idle action was declined").toBe(1);
 scheduler.acceptLoad(report("worker-a",true));
 const given=await polled(scheduler,worker.id,2000);expect(given.task,"the waiting action goes out once a machine has headroom").not.toBeNull();
 queued.abort.abort();await queued.done.catch(()=>undefined);
 const never=submit(scheduler,memory.action([...control("queue"),{name:"OSFamily",value:"plan9"}],"never"));await Bun.sleep(30);
 expect(never.rows.at(-1)?.outcome?.status.code,"a queued action that no machine could ever take fails back at once").toBe(14);
 never.abort.abort();await never.done.catch(()=>undefined);
});

test("a pinned action (semio-pin=<origin>) goes only to the worker that offers that origin, never to its requester, and fails back at once when that worker is gone or is the requester",async()=>{
 const T="aaaaaaaaaaaaaaaa",properties=(origin:string)=>[{name:"Arch",value:"arm64"},{name:"OSFamily",value:"macos"},{name:"semio-toolchain",value:T},{name:"semio-target",value:"aarch64-apple-darwin"},{name:"semio-offload-format",value:"argv-1"},{name:"semio-pin",value:origin}];
 const report=(origin:string):Parameters<RemoteExecutionSchedulerV1["acceptLoad"]>[0]=>({version:1,origin,platform:{key:`macos-arm64-${T}`,os:"macos",arch:"arm64",toolchain:T,target:"aarch64-apple-darwin"},capacity:{cores:8,memoryMegabytes:16384},running:0,queued:0,pressure:"normal",freeMegabytes:8000,load:1,headroom:true,wallMs:1_000_000,worker:{state:"idle",concurrency:2,running:0,sandbox:null,confinement:null,network:null},counters:{unit:{localHit:0,remoteHit:0,miss:0,stored:0},offload:{out:0,refused:0,fellBack:0}}});
 const control=(pin:string)=>[{name:"semio-offload",value:"queue"},{name:"semio-origin",value:"requester"},{name:"semio-offload-format",value:"argv-1"},{name:"semio-pin",value:pin}];
 const {memory,scheduler}=harness();
 for(const origin of ["requester","worker-a","worker-b"])scheduler.acceptLoad(report(origin));
 const a=scheduler.register({...profile("a",properties("worker-a"),2),origin:"worker-a"} as RemoteExecutionWorkerRegistrationV1),b=scheduler.register({...profile("b",properties("worker-b"),2),origin:"worker-b"} as RemoteExecutionWorkerRegistrationV1);
 const run=submit(scheduler,memory.action(control("worker-b"),"pin-b"));await Bun.sleep(30);
 expect((await polled(scheduler,a.id,150)).task,"the other machine is not given a pinned action").toBeNull();
 expect((await polled(scheduler,b.id,2000)).task,"the pinned machine is").not.toBeNull();
 run.abort.abort();await run.done.catch(()=>undefined);
 for(const pin of ["worker-z","requester"]){
  const failed=submit(scheduler,memory.action(control(pin),"pin-"+pin));await Bun.sleep(30);
  expect(failed.rows.at(-1)?.outcome?.status.code,pin).toBe(14);
  failed.abort.abort();await failed.done.catch(()=>undefined);
 }
});

test("a machine that registers again under the same name keeps its one entry and its id: no orphan entry that nothing heartbeats is left to be reported as lost",async()=>{
 const {scheduler,events,advance}=harness(),first=scheduler.register(profile("one")),second=scheduler.register(profile("one")),third=scheduler.register(profile("one"));
 expect(second.id).toBe(first.id);expect(third.id).toBe(first.id);
 expect(scheduler.status().workers).toHaveLength(1);
 expect(scheduler.status().counters.workersReused).toBe(2);
 expect(events.filter(event=>event.startsWith("worker-joined"))).toHaveLength(1);
 scheduler.heartbeat(first.id,[]);advance(5);scheduler.heartbeat(first.id,[]);advance(5);
 expect(events.filter(event=>event.startsWith("worker-lost"))).toHaveLength(0);
 const other=scheduler.register(profile("two"));expect(other.id).not.toBe(first.id);expect(scheduler.status().workers).toHaveLength(2);
});

test("a beat of the sweep timer that came back far too late judges no worker, because the heartbeats wait unread in a stalled process; the next beat judges on fresh marks",async()=>{
 const {scheduler,events,elapse}=harness(),alive=scheduler.register(profile("alive")),silent=scheduler.register(profile("silent"));
 elapse(40);scheduler.tick(1000);
 expect(scheduler.status().workers).toHaveLength(2);expect(scheduler.status().counters.sweepStalls).toBe(1);expect(events.filter(event=>event.startsWith("worker-lost"))).toHaveLength(0);
 scheduler.heartbeat(alive.id,[]);elapse(1);scheduler.tick(1000);
 expect(scheduler.status().workers.map(row=>row.id)).toEqual([alive.id]);expect(silent.id).not.toBe(alive.id);
 expect(events.filter(event=>event.startsWith("worker-lost"))).toHaveLength(1);expect(scheduler.status().counters.sweepStalls).toBe(1);
});

test("an action whose requester is gone is not placed during the linger and is counted as cancelled before start, not logged per cancel; the waits of placed actions are measured",async()=>{
 const {memory,scheduler,events,elapse}=harness({lingerSeconds:0.4}),gone=memory.action([],"gone"),kept=memory.action([],"kept"),run=submit(scheduler,gone);
 await Bun.sleep(20);run.abort.abort();await run.done;
 const worker=scheduler.register(profile("w"));
 expect((await polled(scheduler,worker.id,30)).task).toBeNull();
 await Bun.sleep(500);
 expect(scheduler.status().counters).toMatchObject({cancelled:1,cancelledQueued:1,cancelledExecuting:0});
 expect(events.some(event=>event.startsWith("cancelled"))).toBe(false);
 const next=submit(scheduler,kept);await Bun.sleep(20);elapse(3);
 const task=(await polled(scheduler,worker.id)).task!;expect(task.action).toEqual(kept);
 expect(scheduler.status().counters).toMatchObject({queueWaitSamples:1,queueWaitP50Milliseconds:3000,queueWaitP90Milliseconds:3000,queueWaitMaxMilliseconds:3000});
 expect(scheduler.report(worker.id,task.id,resultOf())).toBe(true);await next.done;
});

test("an operation that has nothing new to say is repeated to its client every keep-alive interval, so a silent server can be told from a patient one",async()=>{
 const {memory,scheduler}=harness({keepAliveSeconds:0.05}),run=submit(scheduler,memory.action([],"beat"));
 await Bun.sleep(400);run.abort.abort();await run.done;
 expect(run.rows.length).toBeGreaterThanOrEqual(5);
 expect(new Set(run.rows.slice(1).map(row=>row.stage))).toEqual(new Set(["QUEUED"]));
});

test("a client that joins an operation whose cancellation was already sent to the worker revokes it: the action keeps running and its result reaches the new client",async()=>{
 const {memory,scheduler}=harness(),action=memory.action([],"rejoin"),first=submit(scheduler,action),worker=scheduler.register(profile("w"));await Bun.sleep(20);
 const task=(await polled(scheduler,worker.id)).task!;
 first.abort.abort();await first.done;await Bun.sleep(150);
 expect(scheduler.status().counters.cancelledExecuting).toBe(1);
 const second=submit(scheduler,action);await Bun.sleep(20);
 expect(scheduler.heartbeat(worker.id,[task.id]).cancel).toEqual([]);
 expect(scheduler.report(worker.id,task.id,resultOf())).toBe(true);await second.done;
 expect(second.rows.at(-1)!.outcome!.status.code).toBe(0);
});

test("a cancellation that the worker does not acknowledge within the grace frees the operation, and a cancel entry the worker no longer holds neither spins its long poll nor starves it",async()=>{
 const {memory,scheduler,advance}=harness({cancelGraceSeconds:5}),run=submit(scheduler,memory.action([],"grace")),worker=scheduler.register(profile("w"));await Bun.sleep(20);
 const task=(await polled(scheduler,worker.id)).task!;
 run.abort.abort();await run.done;await Bun.sleep(150);
 expect((await polled(scheduler,worker.id,10)).cancel).toEqual([task.id]);
 const waited=Date.now();expect((await polled(scheduler,worker.id,120)).task).toBeNull();expect(Date.now()-waited,"announced once, then the poll waits").toBeGreaterThanOrEqual(90);
 advance(6);await Bun.sleep(20);
 expect(scheduler.status()).toMatchObject({counters:{cancelled:1},running:0});
 expect(scheduler.heartbeat(worker.id,[]).cancel).toEqual([]);
 const again=Date.now();expect((await polled(scheduler,worker.id,120)).task).toBeNull();expect(Date.now()-again).toBeGreaterThanOrEqual(90);
});

test("a heartbeat that was sent before the task reached the worker does not make the scheduler forget the task: when the worker is lost the action is still rescheduled",async()=>{
 const {memory,scheduler,advance,events}=harness({maximumAttempts:3}),run=submit(scheduler,memory.action([],"race")),worker=scheduler.register(profile("w"));await Bun.sleep(20);
 const task=(await polled(scheduler,worker.id)).task!;
 expect(scheduler.heartbeat(worker.id,[])).toMatchObject({known:true});
 expect(scheduler.status().workers[0]!.running).toBe(1);
 advance(11);
 expect(events.some(event=>event.startsWith("worker-lost"))).toBe(true);expect(events.some(event=>event.startsWith("retried"))).toBe(true);
 const other=scheduler.register(profile("w2")),retry=(await polled(scheduler,other.id)).task!;
 expect([retry.attempt,task.attempt]).toEqual([2,1]);
 expect(scheduler.report(other.id,retry.id,resultOf())).toBe(true);await run.done;
});

test("a worker that is lost with an offloaded action fails the action back to the waiting client at once when no other machine can take it",async()=>{
 const {memory,scheduler,advance}=harness({maximumAttempts:3}),platform=[{name:"semio-offload",value:"queue"},{name:"semio-origin",value:"req"}],run=submit(scheduler,memory.action(platform,"lost")),worker=scheduler.register(profile("w"));await Bun.sleep(20);
 expect((await polled(scheduler,worker.id)).task).not.toBeNull();
 advance(11);await run.done;
 expect(run.rows.at(-1)!.outcome!.status).toMatchObject({code:14});expect(run.rows.at(-1)!.outcome!.status.message).toMatch(/no other machine can take the action/u);
});

test("an operator ends one operation by a prefix of its id or action digest, or every operation that is older than the stuck limit: the clients hear a cancellation, the worker is told to stop, a prefix that is too short matches nothing",async()=>{
 const {memory,scheduler,advance}=harness({stuckSeconds:100,defaultActionSeconds:600,maximumActionSeconds:600}),one=memory.action([],"one"),two=memory.action([],"two"),a=submit(scheduler,one),b=submit(scheduler,two),worker=scheduler.register(profile("w"));await Bun.sleep(20);
 const task=(await polled(scheduler,worker.id)).task!;
 expect(scheduler.abort("short")).toEqual([]);expect(scheduler.abort("all-stuck")).toEqual([]);
 const ended=scheduler.abort(two.hash.slice(0,8));await b.done;
 expect(ended.map(row=>row.action)).toEqual([two.hash.slice(0,12)]);expect(b.rows.at(-1)!.outcome!.status.code).toBe(1);expect(b.rows.at(-1)!.outcome!.status.message).toMatch(/operator/u);
 for(let beat=0;beat<21;beat++){advance(5);scheduler.heartbeat(worker.id,[task.id]);}
 const stuck=scheduler.abort("all-stuck");await a.done;
 expect(stuck.map(row=>[row.stage,row.worker])).toEqual([["EXECUTING","w"]]);
 expect(scheduler.heartbeat(worker.id,[task.id]).cancel).toEqual([task.id]);
 expect(scheduler.report(worker.id,task.id,{version:1,outcome:"cancelled",status:{code:1,message:"Cancelled"},actionResult:null})).toBe(false);
 expect(scheduler.abort("all-stuck")).toEqual([]);
});

test("a restarted scheduler restores its operation table from the journal: the executing action is known by its name, nothing is placed while its worker is missing, the worker that returns with the task is bound to it and the action never runs twice",async()=>{
 const first=harness({lingerSeconds:30}),running=first.memory.action([],"running"),waiting=first.memory.action([],"waiting"),run=submit(first.scheduler,running),worker=first.scheduler.register(profile("w"));await Bun.sleep(20);
 const task=(await polled(first.scheduler,worker.id)).task!;
 const queued=submit(first.scheduler,waiting);await Bun.sleep(20);
 const rows=first.scheduler.journal();
 expect(rows.map(row=>[row.id===task.id.split(".")[0],row.stage,row.worker?.name??null])).toEqual([[true,"EXECUTING","w"],[false,"QUEUED",null]]);
 expect(JSON.parse(JSON.stringify(rows))).toEqual(rows);
 run.abort.abort();queued.abort.abort();
 const second=harness({lingerSeconds:30,restore:rows,resumeGraceSeconds:60}),name=`operations/${task.id.split(".")[0]}`;
 expect(second.events.some(event=>/^restored 2 operations/u.test(event))).toBe(true);
 const attach=new AbortController(),seen:RemoteExecutionSnapshotV1[]=[],stream=second.scheduler.wait(name,attach.signal);
 expect(stream).toBeDefined();
 const reading=(async()=>{for await(const row of stream!)seen.push(row);})();
 await Bun.sleep(20);expect(seen.at(-1)!.stage).toBe("EXECUTING");
 const fresh=second.memory.action([],"fresh"),other=submit(second.scheduler,fresh),bystander=second.scheduler.register(profile("bystander"));
 await Bun.sleep(20);
 expect((await polled(second.scheduler,bystander.id,50)).task,"nothing is placed while a restored action waits for its worker").toBeNull();
 const returned=second.scheduler.register({...profile("w"),resumed:[{task:task.id,instance:"main",action:{hash:running.hash,size:running.size}}]} as RemoteExecutionWorkerRegistrationV1);
 expect(second.scheduler.status().counters).toMatchObject({restored:2,resumed:1});
 expect(second.scheduler.report(returned.id,task.id,resultOf())).toBe(true);
 await reading;expect(seen.at(-1)!.stage).toBe("COMPLETED");expect(seen.at(-1)!.outcome!.status.code).toBe(0);
 const placed=(await polled(second.scheduler,bystander.id,200)).task;
 expect(placed?.action.hash,"the next placement is the new action, never the restored one").toBe(fresh.hash);
 attach.abort();other.abort.abort();
});

test("a restored executing action whose worker never returns is queued after the grace and placed again for the requester that re-attached; a journal older than its limit restores nothing",async()=>{
 const first=harness({lingerSeconds:30}),action=first.memory.action([],"lost-worker"),run=submit(first.scheduler,action),worker=first.scheduler.register(profile("gone"));await Bun.sleep(20);
 const task=(await polled(first.scheduler,worker.id)).task!,rows=first.scheduler.journal();run.abort.abort();
 const second=harness({lingerSeconds:30,restore:rows,resumeGraceSeconds:60}),attach=new AbortController(),seen:RemoteExecutionSnapshotV1[]=[];
 const reading=(async()=>{for await(const row of second.scheduler.wait(`operations/${task.id.split(".")[0]}`,attach.signal)!)seen.push(row);})();
 const spare=second.scheduler.register(profile("spare"));await Bun.sleep(20);
 second.advance(30);expect((await polled(second.scheduler,spare.id,30)).task).toBeNull();
 second.advance(40);
 const back=second.scheduler.register(profile("spare")),again=(await polled(second.scheduler,back.id,200)).task!;
 expect([again.id.split(".")[0],again.attempt]).toEqual([task.id.split(".")[0],2]);
 expect(second.scheduler.report(back.id,again.id,resultOf())).toBe(true);await reading;
 expect(seen.at(-1)!.stage).toBe("COMPLETED");attach.abort();
 const {readSchedulerJournalV1,writeSchedulerJournalV1}=await import("../📓️journal/🟦️.ts"),{mkdtempSync,rmSync}=await import("node:fs"),{tmpdir}=await import("node:os"),{join}=await import("node:path"),directory=mkdtempSync(join(tmpdir(),"journal-")),path=join(directory,"j.json");
 writeSchedulerJournalV1(path,rows,1000);
 expect(readSchedulerJournalV1(path,2000,5000)).toEqual(rows);expect(readSchedulerJournalV1(path,10_000,5000)).toEqual([]);expect(readSchedulerJournalV1(join(directory,"absent.json"),2000,5000)).toEqual([]);
 rmSync(directory,{recursive:true,force:true});
});

class Leasing extends Memory{
 leased:{lease:string|undefined;hashes:string[]}[]=[];released:{lease:string;hashes:string[]}[]=[];
 override async missing(_:string,digests:readonly RemoteExecutionDigestV1[],_signal?:AbortSignal,intent?:string){this.leased.push({lease:intent,hashes:digests.map(digest=>digest.hash)});return digests.filter(digest=>!this.blobs.has(digest.hash));}
 async release(_:string,hashes:readonly string[],intent:string){this.released.push({lease:intent,hashes:[...hashes]});}
 inputAction(files:Record<string,string>):{action:RemoteExecutionDigestV1;blobs:Set<string>}{
  const before=new Set(this.blobs.keys()),wire=(digest:RemoteExecutionDigestV1)=>({hash:digest.hash,size_bytes:digest.size});
  const tree=(entries:Record<string,string>):RemoteExecutionDigestV1=>{
   const direct:Record<string,string>={},nested:Record<string,Record<string,string>>={};
   for(const [path,text] of Object.entries(entries)){const at=path.indexOf("/");if(at<0)direct[path]=text;else(nested[path.slice(0,at)]??={})[path.slice(at+1)]=text;}
   return this.put(encodeProtobufV1(REAPI_V1.directory,{files:Object.entries(direct).sort().map(([name,text])=>({name,digest:wire(this.put(new TextEncoder().encode(text))),is_executable:false})),directories:Object.entries(nested).sort().map(([name,child])=>({name,digest:wire(tree(child))})),symlinks:[]}));
  };
  const root=tree(files),command=this.put(encodeProtobufV1(REAPI_V1.command,{arguments:["/bin/true"],platform:{properties:[]}}));
  const action=this.put(encodeProtobufV1(REAPI_V1.action,{command_digest:wire(command),input_root_digest:wire(root)}));
  return{action,blobs:new Set([...this.blobs.keys()].filter(hash=>!before.has(hash)))};
 }
}
const textDigest=(text:string):RemoteExecutionDigestV1=>({hash:sha(new TextEncoder().encode(text)),size:text.length});

test("an operation leases its whole input root when it is accepted, renews the lease for as long as it queues (longer than the store's lease) and ends it when it completes",async()=>{
 const leasing=new Leasing(),{scheduler,advance}=harness({content:leasing,inputLeaseRenewSeconds:600,unmatchedSeconds:1_000_000}),built=leasing.inputAction({"a.txt":"a","d/b.txt":"b","d/e/c.txt":"c"});
 const run=submit(scheduler,built.action);await Bun.sleep(30);
 expect(run.rows.at(-1)!.stage).toBe("QUEUED");
 const first=leasing.leased[0]!;expect(first.lease).toBe(`input:op-${built.action.hash.slice(0,32)}`);expect(new Set(first.hashes),"action, command, three directories and three files").toEqual(built.blobs);expect(built.blobs.size).toBe(8);
 for(let round=0;round<4;round++){advance(601);await Bun.sleep(20);}
 expect(leasing.leased.length,"one lease at acceptance and one renewal per period while queued (2404 s)").toBe(5);
 expect(leasing.leased.every(row=>row.lease===first.lease&&new Set(row.hashes).size===8)).toBe(true);
 expect(leasing.released).toEqual([]);
 const worker=scheduler.register(profile("w")),task=(await polled(scheduler,worker.id)).task!;
 scheduler.report(worker.id,task.id,resultOf());await run.done;
 expect(leasing.released).toHaveLength(1);expect(leasing.released[0]!.lease).toBe(first.lease);expect(new Set(leasing.released[0]!.hashes)).toEqual(built.blobs);
});

test("an operation whose input blobs are missing is answered at submission with every missing blob, never queued, and holds no lease",async()=>{
 const leasing=new Leasing(),{scheduler}=harness({content:leasing}),built=leasing.inputAction({"a.txt":"a","d/b.txt":"b","d/e/c.txt":"c"});
 leasing.blobs.delete(textDigest("a").hash);leasing.blobs.delete(textDigest("c").hash);
 const run=submit(scheduler,built.action);await run.done;
 const status=run.rows.at(-1)!.outcome!.status;
 expect(status.code).toBe(9);expect(new Set(status.missing!.map(digest=>digest.hash))).toEqual(new Set([textDigest("a").hash,textDigest("c").hash]));
 expect(status.message).toBe(`Missing blob ${status.missing![0]!.hash}/1 and 1 more`);
 expect(scheduler.status().queued).toBe(0);expect(leasing.released).toHaveLength(1);
 leasing.blobs.set(textDigest("a").hash,new TextEncoder().encode("a"));leasing.blobs.set(textDigest("c").hash,new TextEncoder().encode("c"));
 const again=submit(scheduler,built.action);await Bun.sleep(30);expect(again.rows.at(-1)!.stage,"the same action queues once the blobs are back").toBe("QUEUED");again.abort.abort();
});

test("a queued operation whose input blob the store loses despite the lease is answered at the next renewal with the lost blobs",async()=>{
 const leasing=new Leasing(),{scheduler,advance}=harness({content:leasing,inputLeaseRenewSeconds:600,unmatchedSeconds:1_000_000}),built=leasing.inputAction({"a.txt":"a","d/b.txt":"b"});
 const run=submit(scheduler,built.action);await Bun.sleep(30);expect(run.rows.at(-1)!.stage).toBe("QUEUED");
 leasing.blobs.delete(textDigest("b").hash);advance(601);await run.done;
 const status=run.rows.at(-1)!.outcome!.status;expect(status.code).toBe(9);expect(status.missing).toEqual([textDigest("b")]);
});
