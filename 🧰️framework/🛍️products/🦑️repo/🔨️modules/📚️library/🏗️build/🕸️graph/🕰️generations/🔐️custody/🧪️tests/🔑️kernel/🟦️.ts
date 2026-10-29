/** 🔑️ Contract: beneath a real issued lease control, the waits of the slice custody and of the prefetch claims go through the REAL queued lease kernel, which admits only the control's own signal: a waiter behind an overlapping claim, and a prefetch that joins a running peer claim, wait and move on when the holder releases (no derived abort signal ever reaches the kernel), and cancelling the control ends a waiter promptly and leaves no claim behind. */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync} from "node:fs";
import {join} from "node:path";
import {acquireQueuedResourceLease,acquireResourceLease,acquireFreshResourceLeaseV1} from "@semio-tech/process/leases";
import{resourceLeasePathV1}from"@semio-tech/process/leases";
import{Database}from"bun:sqlite";
import {createIssuedResourceControlV1} from "@semio-tech/process/routing";
import {declareBuildTestEnvelopeV1,withBuildTestInvocationV1} from "@semio-tech/process/testing/invocation";
import {createNodeFilePortV1} from "../../../🟨️.mjs";
import {createClaimRegistryV1,createQueuedClaimLeasePortV1,waitAnyLeaseV1} from "../../../🧩️claims/🟨️.mjs";
import {createSliceCustodyV1} from "../../🟨️.mjs";

const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));
const settled=async(promise:Promise<unknown>,ms:number)=>Promise.race([promise.then(()=>true,(error:unknown)=>String((error as Error)?.message??error)),sleep(ms).then(()=>false)]);
declareBuildTestEnvelopeV1({owner:"test:build-workspace-graph-laws",maximumElapsedMilliseconds:300000,capabilities:{}});
type Original=Parameters<Parameters<typeof withBuildTestInvocationV1>[0]>[0];
/** 🎛️ One control issued beneath the real test invocation (as every repository command issues one per lease user) and the kernel-backed lease port bound to it; aborting `controller` cancels that control only. */
function issued(original:Original,directory:string,name:string){
 const controller=new AbortController(),control=createIssuedResourceControlV1(original,"test:custody-kernel:"+name,120000,controller.signal);
 return{controller,control,lease:createQueuedClaimLeasePortV1({acquire:acquireQueuedResourceLease as never,tryAcquire:acquireResourceLease as never,acquireFresh:acquireFreshResourceLeaseV1 as never,directory,control})};
}
const scratch=()=>{const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Owned test artifacts directory required");mkdirSync(artifacts,{recursive:true});const root=realpathSync(mkdtempSync(join(artifacts,"custody-kernel-")));roots.push(root);return root;};

test("fresh custody waits only for directory admission and preserves immediate real collision refusal",async()=>withBuildTestInvocationV1(async original=>{
 const root=scratch(),directory=join(root,"leases"),controller=new AbortController(),control=createIssuedResourceControlV1(original,"test:fresh-custody-admission",30000,controller.signal),lease=createQueuedClaimLeasePortV1({directory,control,acquire:acquireQueuedResourceLease as never,tryAcquire:acquireResourceLease as never,acquireFresh:acquireFreshResourceLeaseV1 as never});
 (await lease.hold("initialize")).release();const gate=new Database(resourceLeasePathV1(directory,"resource-directory-admission:v1"),{readwrite:true,create:false});let locked=false,held:any;
 try{gate.exec("PRAGMA busy_timeout=0;BEGIN EXCLUSIVE");locked=true;const custody=createSliceCustodyV1({root:join(root,"generations"),fs:createNodeFilePortV1(),lease}),pending=custody.hold({owners:["fresh"],signal:control.signal}).then(value=>({value}),error=>({error})),early=await Promise.race([pending.then(()=>true),sleep(50).then(()=>false)]);gate.exec("ROLLBACK");locked=false;const outcome=await pending;held="value"in outcome?outcome.value:undefined;expect(early).toBe(false);if("error"in outcome)throw outcome.error;expect(held.owners).toEqual(["fresh"]);await held.release();held=undefined;const collision=await lease.tryHold("real-collision",{fresh:true});expect(collision).toBeDefined();try{const peer=issued(original,directory,"fresh-peer");expect(await peer.lease.tryHold("real-collision",{fresh:true})).toBeUndefined();}finally{collision!.release();}}
 finally{if(locked)gate.exec("ROLLBACK");gate.close(true);await held?.release();}
}),60000);

test("fresh directory admission observes its original cancellation and leaves no claim",async()=>withBuildTestInvocationV1(async original=>{
 const root=scratch(),directory=join(root,"leases"),requester=issued(original,directory,"fresh-cancel"),observer=issued(original,directory,"fresh-observer"),make=(lease:ReturnType<typeof issued>["lease"])=>createSliceCustodyV1({root:join(root,"generations"),fs:createNodeFilePortV1(),lease});
 (await requester.lease.hold("initialize")).release();const gate=new Database(resourceLeasePathV1(directory,"resource-directory-admission:v1"),{readwrite:true,create:false}),reason=Error("Fresh admission cancelled");let locked=false;
 try{gate.exec("PRAGMA busy_timeout=0;BEGIN EXCLUSIVE");locked=true;const pending=make(requester.lease).hold({owners:["cancelled"],signal:requester.control.signal}).then(value=>({value}),error=>({error}));await sleep(25);const started=performance.now();requester.controller.abort(reason);const outcome=await pending;expect(performance.now()-started).toBeLessThan(1000);if("value"in outcome){await outcome.value.release();throw Error("Cancelled fresh claim admitted");}expect(outcome.error).toBe(reason);gate.exec("ROLLBACK");locked=false;expect(await make(observer.lease).list()).toEqual([]);}
 finally{if(locked)gate.exec("ROLLBACK");gate.close(true);}
}),60000);

test("an overlapping claim waits through the real kernel with the control's own signal and holds once the first is released",async()=>withBuildTestInvocationV1(async original=>{
 const root=scratch(),leases=join(root,"leases"),first=issued(original,leases,"c1"),second=issued(original,leases,"c2");
 const one=createSliceCustodyV1({root:join(root,"generations"),fs:createNodeFilePortV1(),lease:first.lease}),two=createSliceCustodyV1({root:join(root,"generations"),fs:createNodeFilePortV1(),lease:second.lease});
 const a=await one.hold({owners:["x","y"],signal:first.control.signal});
 const waiting=two.hold({owners:["y"],signal:second.control.signal});
 expect(await settled(waiting,1500)).toBe(false);
 const disjoint=await two.hold({owners:["z"],signal:second.control.signal});
 await a.release();
 const b=await waiting;expect(b.owners).toEqual(["y"]);
 await b.release();await disjoint.release();
}),60000);

test("the shared wait helper answers true when a held lease is released and false after its timeout, through the real kernel",async()=>withBuildTestInvocationV1(async original=>{
 const root=scratch(),leases=join(root,"leases"),holder=issued(original,leases,"c3"),waiter=issued(original,leases,"c4");
 const held=await holder.lease.tryHold("graph-custody:law:held");expect(held).toBeDefined();
 expect(await waitAnyLeaseV1(waiter.lease,["graph-custody:law:held"],{signal:waiter.control.signal,timeoutMs:300})).toBe(false);
 const later=waitAnyLeaseV1(waiter.lease,["graph-custody:law:held","graph-custody:law:other-held"],{signal:waiter.control.signal,timeoutMs:20000});
 const other=await holder.lease.tryHold("graph-custody:law:other-held");expect(other).toBeDefined();
 await sleep(100);held!.release();
 expect(await later).toBe(true);other!.release();
}),60000);

test("a prefetch that joins a running peer claim waits through the real kernel instead of failing",async()=>withBuildTestInvocationV1(async original=>{
 const root=scratch(),leases=join(root,"leases"),a=issued(original,leases,"c5"),b=issued(original,leases,"c6");let release!:()=>void;const gate=new Promise<void>(resolve=>{release=resolve;});
 const registry=(port:ReturnType<typeof issued>)=>createClaimRegistryV1({root:join(root,"generations"),fs:createNodeFilePortV1(),lease:port.lease});
 const ran:string[]=[];
 const first=registry(a).prefetch({base:1,implementation:"a".repeat(64),wanted:["p"],slots:2,signal:a.control.signal,run:async owners=>{ran.push("A:"+owners.join(","));await gate;}});
 await sleep(300);
 const second=registry(b).prefetch({base:1,implementation:"a".repeat(64),wanted:["p"],slots:2,signal:b.control.signal,run:async owners=>{ran.push("B:"+owners.join(","));}});
 expect(await settled(second,800)).toBe(false);
 release();
 expect(await first).toEqual({computed:1,served:false});expect(await second).toEqual({computed:0,served:false});expect(ran).toEqual(["A:p"]);
}),60000);

test("cancelling the control ends a waiting claim promptly and leaves only the holder's claim",async()=>withBuildTestInvocationV1(async original=>{
 const root=scratch(),leases=join(root,"leases"),first=issued(original,leases,"c7"),second=issued(original,leases,"c8");
 const one=createSliceCustodyV1({root:join(root,"generations"),fs:createNodeFilePortV1(),lease:first.lease}),two=createSliceCustodyV1({root:join(root,"generations"),fs:createNodeFilePortV1(),lease:second.lease});
 const a=await one.hold({owners:["x"],signal:first.control.signal}),waiting=two.hold({owners:["x"],signal:second.control.signal});
 expect(await settled(waiting,500)).toBe(false);
 const started=Date.now();second.controller.abort(new Error("stop"));
 await expect(waiting).rejects.toThrow();expect(Date.now()-started).toBeLessThan(5000);
 expect((await one.list()).map(claim=>claim.token)).toEqual([a.token]);
 await a.release();
}),60000);

test("nonwaiting custody probes preserve concurrent extend and alone decisions beneath the real kernel",async()=>withBuildTestInvocationV1(async original=>{
 const root=scratch(),leases=join(root,"leases"),a=issued(original,leases,"extend-a"),b=issued(original,leases,"extend-b"),make=(port:ReturnType<typeof issued>)=>createSliceCustodyV1({root:join(root,"generations"),fs:createNodeFilePortV1(),lease:port.lease});
 const one=make(a),two=make(b),[first,second]=await Promise.all([one.hold({owners:["a"],signal:a.control.signal}),two.hold({owners:["b"],signal:b.control.signal})]);
 try{const results=await Promise.all([first.extend({owners:["b"],signal:a.control.signal}),second.alone({signal:b.control.signal})]);expect(results[0].held).toBe(false);expect(results[1]).toBe(false);expect(first.owners).toEqual(["a"]);expect(second.structural).toBe(false);await second.release();expect((await first.extend({owners:["b"],signal:a.control.signal})).held).toBe(true);expect(await first.alone({signal:a.control.signal})).toBe(true);expect(first.structural).toBe(true);}finally{await first.release();await second.release();}
}),60000);
