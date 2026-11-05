/** 🧾️ The host continuation executes only current resident native arguments and carries the exact captured revision. */
import {expect,test} from "bun:test";
import {StagedActionHost,type StagedActionSource,type StagedActionLease} from "./session.ts";
import type {StagedOwnedSnapshot,StagedOwnedSubscription} from "../../🛠️ShellHelpers/🪟️window-panes/📝️staged-args/🛡️intents/owned.ts";
import lifecycle from "./lifecycle-cases.json" with {type:"json"};
import {STAGED_FORM_STATUS_EXTENSION} from "@semio-tech/framework/manifest/staging";
import fixture from "../../../../../../../../🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/commit-cases.json" with {type:"json"};
const tick=async()=>{for(let index=0;index<16;index++)await Promise.resolve();};
function deferred(){let resolve!:()=>void;const promise=new Promise<void>(accept=>{resolve=accept;});return {promise,resolve};}
function fixtureSource(locale:"en"|"de",ready:boolean,commitArgs:unknown,execute:StagedActionSource["execute"],failFirstRelease?:string){
 const view=Object.freeze({revision:1,root:0,hash:"original-staged-status"});
 const props={token:fixture.token,ready,commitArgs,diagnostics:[],diagnosticOffset:"0",diagnosticTotal:"0"};
 const snapshot:StagedOwnedSnapshot={version:1,record:{id:0,key:"status",disabled:false,component:{type:"extension",extension:STAGED_FORM_STATUS_EXTENSION,props}}};
 const reads=new Set<StagedOwnedSubscription>(),abort=new AbortController();let closed=false,closeCalls=0;
 const lease:StagedActionLease={
  signal:abort.signal,current:()=>!closed,
  source:{get view(){return view;},subscribeView:()=>{throw new Error("status fixture has no view subscriber");},subscribeNode:(id)=>{if(id!==0)throw new Error("foreign node");const read={snapshot};reads.add(read);return read;},acknowledgeRead:()=>{},unsubscribeNode:read=>{reads.delete(read);}},
  close:async()=>{closeCalls++;closed=true;abort.abort();expect(reads.size).toBe(0);if(failFirstRelease&&closeCalls===1)throw new Error(failFirstRelease);},
 };
 const source:StagedActionSource<StagedActionLease>={owner:{token:fixture.token,locale,controllerId:"test"},surface:"7:typed-status",current:()=>!closed,revision:()=>fixture.token.revision,subscribe:()=>()=>{},query:async request=>{expect(request.args).toBe(fixture.draft);return lease;},execute};
 return {source,view,closeCalls:()=>closeCalls,publish:(host:StagedActionHost<StagedActionLease>)=>{
  const controls=host.snapshot()?.controls;if(!controls)throw new Error("original controls absent");
  const read=controls.reads.subscribeNode(0,()=>{});return controls.publishStatus(read,read.snapshot!,view);
 }};
}
for(const locale of ["en","de"] as const)test(locale+": initial native readiness executes typed arguments once and keeps the original lease until settlement",async()=>{
 const host=new StagedActionHost<StagedActionLease>(),settle=deferred(),calls:{args:unknown;revision:string;signal:AbortSignal}[]=[];
 const owner=fixtureSource(locale,true,fixture.commitArgs,async(args,revision,signal)=>{calls.push({args,revision,signal});await settle.promise;});
 const opened=host.open(owner.source,fixture.draft);
 try{
  await tick();expect(host.snapshot()?.locale).toBe(locale);expect(owner.publish(host)).toBe(true);await tick();
  expect(calls).toHaveLength(1);expect(calls[0]!.args).toBe(fixture.commitArgs);expect(calls[0]!.args).not.toEqual(fixture.draft);expect(calls[0]!.revision).toBe(fixture.token.revision);
  expect(host.snapshot()?.executing).toBe(true);expect(owner.closeCalls()).toBe(0);
  host.execute();expect(calls).toHaveLength(1);
  settle.resolve();expect(await opened).toBe("executed");expect(owner.closeCalls()).toBe(1);expect(host.snapshot()).toBeNull();
 }finally{settle.resolve();await host.close();await opened;}
});
test("cancel during execution aborts that exact native continuation and waits for its settlement",async()=>{
 const host=new StagedActionHost<StagedActionLease>(),settle=deferred();let signal:AbortSignal|undefined;
 const owner=fixtureSource("en",true,fixture.commitArgs,async(_args,_revision,original)=>{signal=original;await settle.promise;if(original.aborted)throw original.reason;});
 const opened=host.open(owner.source,fixture.draft);
 try{
  await tick();expect(owner.publish(host)).toBe(true);await tick();let closed=false;
  const closing=host.close().then(()=>{closed=true;});await tick();
  expect(signal?.aborted).toBe(true);expect(closed).toBe(false);expect(owner.closeCalls()).toBe(0);
  settle.resolve();await closing;expect(await opened).toBe("cancelled");expect(owner.closeCalls()).toBe(1);
 }finally{settle.resolve();await host.close();await opened;}
});
for(const row of fixture.cases.filter(row=>!row.allowed||!row.ready))test("native status cannot execute "+JSON.stringify({ready:row.ready,commitArgs:row.commitArgs}),async()=>{
 const host=new StagedActionHost<StagedActionLease>();let executed=0;
 const owner=fixtureSource("en",row.ready,row.commitArgs,async()=>{executed++;});
 const opened=host.open(owner.source,fixture.draft);
 try{
  await tick();expect(owner.publish(host)).toBe(row.allowed);host.execute();await tick();expect(executed).toBe(0);expect(host.snapshot()?.ready).toBe(false);
  await host.close();expect(await opened).toBe("cancelled");expect(owner.closeCalls()).toBe(1);
 }finally{await host.close();await opened;}
});

test(lifecycle.committedRelease.id,async()=>{
 const host=new StagedActionHost<StagedActionLease>();let finished=false;
 const owner=fixtureSource("en",true,fixture.commitArgs,async()=>{},lifecycle.retirement.error);
 const opened=host.open(owner.source,fixture.draft);void opened.then(()=>{finished=true;});
 try{
  await tick();expect(owner.publish(host)).toBe(true);await tick();
  expect(finished).toBe(lifecycle.committedRelease.firstFinished);expect(host.snapshot()?.error).toBe(lifecycle.retirement.error);
  host.cancel();expect<string>(await opened).toBe(lifecycle.committedRelease.result);expect(owner.closeCalls()).toBe(lifecycle.committedRelease.closeCalls);
 }finally{await host.close().catch(()=>host.close());await opened;}
});

test("synchronous cancellation of the executing notification cannot enter native dispatch",async()=>{
 const host=new StagedActionHost<StagedActionLease>();let calls=0;
 const owner=fixtureSource("en",true,fixture.commitArgs,async()=>{calls++;});
 const unsubscribe=host.subscribe(()=>{if(host.snapshot()?.executing)host.cancel();}),opened=host.open(owner.source,fixture.draft);
 try{
  await tick();expect(owner.publish(host)).toBe(true);expect(await opened).toBe("cancelled");expect(calls).toBe(0);expect(owner.closeCalls()).toBe(1);
 }finally{unsubscribe();await host.close();await opened;}
});
