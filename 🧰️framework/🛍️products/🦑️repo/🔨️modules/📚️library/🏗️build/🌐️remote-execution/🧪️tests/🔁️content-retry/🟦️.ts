/** 🔁️ Contract: a busy answer (503 with Retry-After) of the shared content store is retried within the offload policy instead of sending the unit to the local compiler; a capacity refusal (507) and the exhausted retries are not. */
import {expect,test} from "bun:test";
import {RemoteExecutionContentErrorV1,httpRemoteExecutionContentV1} from "../../🗄️content/🟦️.ts";
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
