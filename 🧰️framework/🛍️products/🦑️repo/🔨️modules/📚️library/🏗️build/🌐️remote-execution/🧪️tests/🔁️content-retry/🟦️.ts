/** 🔁️ Contract: a busy answer (503 with Retry-After) of the shared content store is retried within the offload policy instead of sending the unit to the local compiler; a capacity refusal (507) and the exhausted retries are not. */
import {expect,test} from "bun:test";
import {RemoteExecutionContentErrorV1,httpRemoteExecutionContentV1,outageTolerantRemoteExecutionContentV1,retryRemoteExecutionContentOutageV1} from "../../🗄️content/🟦️.ts";
import {readRemoteExecutionPolicyV1} from "../../🟦️.ts";
import offload from "../../🚀️offload/🧬️schema/🔣️.json" with {type:"json"};

const HASH="b".repeat(64);
const scripted=(answers:readonly Response[])=>{let calls=0;return{calls:()=>calls,fetch:(async()=>answers[Math.min(calls++,answers.length-1)]!.clone())as unknown as typeof fetch};};
const busy=()=>new Response("Busy\n",{status:503,headers:{"retry-after":"1"}});

test("the offload policy retries the content store and caps how long it honours Retry-After",()=>{
 expect(offload.policy.limits.contentRetries).toBeGreaterThanOrEqual(1);
 expect(offload.policy.limits.contentRetryAfterCapMilliseconds).toBeLessThanOrEqual(5000);
});

test("a busy store is retried after the time it names and the write then succeeds",async()=>{
 const script=scripted([busy(),new Response("",{status:200})]);
 const store=httpRemoteExecutionContentV1({endpoint:"http://store.invalid",retries:2,retryAfterCapMilliseconds:200,fetch:script.fetch});
 const started=Date.now();
 await store.write("",{hash:HASH,size:1},new Uint8Array([1]));
 expect(script.calls()).toBe(2);
 expect(Date.now()-started).toBeGreaterThanOrEqual(190);
});

test("retries stop at the configured count and a capacity refusal is never retried",async()=>{
 const always=scripted([busy()]);
 const store=httpRemoteExecutionContentV1({endpoint:"http://store.invalid",retries:2,retryAfterCapMilliseconds:1,fetch:always.fetch});
 const error=await store.has("",HASH).catch(caught=>caught as RemoteExecutionContentErrorV1);
 expect(error).toBeInstanceOf(RemoteExecutionContentErrorV1);
 expect((error as RemoteExecutionContentErrorV1).message).toBe(`Content store answered 503 for cas/${HASH}`);
 expect(always.calls()).toBe(3);
 const full=scripted([new Response("Insufficient storage\n",{status:507,headers:{"retry-after":"5"}})]);
 const refused=await httpRemoteExecutionContentV1({endpoint:"http://store.invalid",retries:2,fetch:full.fetch}).write("",{hash:HASH,size:1},new Uint8Array([1])).catch(caught=>caught as RemoteExecutionContentErrorV1);
 expect((refused as RemoteExecutionContentErrorV1).code).toBe(8);
 expect(full.calls()).toBe(1);
});

const down=()=>new RemoteExecutionContentErrorV1(14,"Content store unreachable: http://store.invalid: Unable to connect");
const clock=()=>{let at=0;return{now:()=>at,sleep:async(milliseconds:number)=>{at+=milliseconds;},slept:[] as number[]};};

test("the authored outage keeps a finished action's outputs for minutes, not for the 350 ms of the ordinary retries",()=>{
 const outage=readRemoteExecutionPolicyV1().worker.outage;
 expect(outage.seconds).toBeGreaterThanOrEqual(300);
 expect(outage.initialMilliseconds).toBeLessThanOrEqual(outage.maximumMilliseconds);
});

test("an unreachable store is waited out with a doubling pause up to the outage and the work then succeeds; the pauses are capped at the maximum",async()=>{
 const time=clock(),slept:number[]=[],outage={seconds:60,initialMilliseconds:1000,maximumMilliseconds:8000};
 let calls=0;
 const value=await retryRemoteExecutionContentOutageV1(async()=>{if(calls++<6)throw down();return "stored";},outage,undefined,async milliseconds=>{slept.push(milliseconds);await time.sleep(milliseconds);},time.now);
 expect(value).toBe("stored");expect(calls).toBe(7);
 expect(slept).toEqual([1000,2000,4000,8000,8000,8000]);
});

test("the outage is bounded: after its seconds the last failure stands, and a refusal, an abort or an outage of zero seconds are never retried",async()=>{
 const time=clock(),outage={seconds:10,initialMilliseconds:1000,maximumMilliseconds:4000};
 let calls=0;
 const error=await retryRemoteExecutionContentOutageV1(async()=>{calls++;throw down();},outage,undefined,time.sleep,time.now).catch(caught=>caught as RemoteExecutionContentErrorV1);
 expect(error.code).toBe(14);expect(time.now()).toBe(10000);expect(calls).toBeGreaterThan(3);
 let refused=0;
 const full=await retryRemoteExecutionContentOutageV1(async()=>{refused++;throw new RemoteExecutionContentErrorV1(8,"Content store answered 507");},outage,undefined,time.sleep,time.now).catch(caught=>caught as RemoteExecutionContentErrorV1);
 expect(full.code).toBe(8);expect(refused).toBe(1);
 const controller=new AbortController();controller.abort();let aborted=0;
 await retryRemoteExecutionContentOutageV1(async()=>{aborted++;throw down();},outage,controller.signal,time.sleep,time.now).catch(()=>undefined);
 expect(aborted).toBe(1);
 let none=0;
 await retryRemoteExecutionContentOutageV1(async()=>{none++;throw down();},{...outage,seconds:0},undefined,time.sleep,time.now).catch(()=>undefined);
 expect(none).toBe(1);
});

test("the requester's reads and byte writes wait out a restart of the store, a stream is handed through once",async()=>{
 let reads=0,streams=0;
 const store=outageTolerantRemoteExecutionContentV1({
  has:async()=>true,missing:async()=>[],stream:async()=>undefined,readActionResult:async()=>undefined,writeActionResult:async()=>undefined,
  read:async()=>{if(reads++<2)throw down();return new Uint8Array([9]);},
  write:async(_i,_d,source)=>{if(!(source instanceof Uint8Array)){streams++;throw down();}},
 },{seconds:30,initialMilliseconds:1,maximumMilliseconds:2});
 expect([...(await store.read("",{hash:HASH,size:1}))!]).toEqual([9]);expect(reads).toBe(3);
 await store.write("",{hash:HASH,size:1},(async function*(){yield new Uint8Array([1]);})()).catch(()=>undefined);
 expect(streams).toBe(1);
});
