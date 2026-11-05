/** 🪟️ Captured staged actions keep one current intent and await their original surface retirement. */
import {expect,test} from "bun:test";
import {Subject,asapScheduler,scheduled,switchMap,defer,lastValueFrom,of,retry,throwError} from "rxjs";
import {semioSchemaAjvV1} from "@semio-tech/framework/schema/oracles/validator";
import {StagedActionHost,type StagedActionSource} from "./session.ts";
import type {OwnedUiSurfaceLease} from "../../🔌️PluginRuntime/🧺️handle-ui-intake/lease.ts";
import fixture from "./lifecycle-cases.json" with {type:"json"};
import schema from "./lifecycle-schema.json" with {type:"json"};
const tick=async()=>{for(let index=0;index<16;index++)await Promise.resolve();};
function deferred(){let resolve!:()=>void;const promise=new Promise<void>(accept=>{resolve=accept;});return {promise,resolve};}
function harness(){
 const queried:string[]=[],listeners=new Set<()=>void>();let active=true;
 const make=(id:string,lease?:OwnedUiSurfaceLease):StagedActionSource=>({
  owner:{token:{action:"insertItem",window:id,generation:"0",revision:"a".repeat(64)},locale:"en",controllerId:"test"},
  surface:"7:staged-"+id,current:()=>active,revision:()=>"a".repeat(64),
  subscribe:notify=>{listeners.add(notify);return()=>{listeners.delete(notify);};},
  query:async(_request,signal)=>{queried.push(id);if(lease)return lease;return new Promise((_resolve,reject)=>{signal.addEventListener("abort",()=>reject(signal.reason),{once:true});});},
  execute:async()=>{throw new Error("an unready fixture must never execute");},
 });
 return {queried,make,notify:()=>{for(const notify of [...listeners])notify();},retire:()=>{active=false;for(const notify of [...listeners])notify();}};
}
function heldLease(released:Promise<void>){
 const controller=new AbortController();let closed=false,calls=0;
 const view=Object.freeze({revision:0,root:null,hash:null});
 const lease:OwnedUiSurfaceLease={
  signal:controller.signal,current:()=>!closed,
  source:{get view(){return view;},subscribeView:()=>{throw new Error("no DOM reader");},subscribeNode:()=>{throw new Error("no DOM reader");},acknowledgeRead:()=>{},unsubscribeNode:()=>{},retryNotification:()=>false},
  close:async()=>{calls++;closed=true;controller.abort();await released;},
 };
 return {lease,calls:()=>calls};
}
async function latestOracle(requests:readonly string[]):Promise<string[]>{
 const values:string[]=[],source=new Subject<string>(),subscription=source.pipe(switchMap(value=>scheduled([value],asapScheduler))).subscribe(value=>values.push(value));
 try{for(const request of requests)source.next(request);await tick();return values;}finally{subscription.unsubscribe();source.complete();}
}
test("neutral staged host lifecycles are closed and latest intent agrees with independent RxJS cancellation",async()=>{
 const validate=semioSchemaAjvV1({strict:true,allErrors:true}).compile(schema);
 expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 expect(validate({...fixture,extra:true})).toBe(false);
 for(const row of fixture.cases)expect(await latestOracle(row.requests),row.id).toEqual(row.queried);
 let attempts=0;const result=await lastValueFrom(defer(()=>++attempts===1?throwError(()=>new Error(fixture.retirement.error)):of(fixture.retirement.result)).pipe(retry(1)));
 expect({attempts,result}).toEqual({attempts:fixture.retirement.attempts,result:fixture.retirement.result});
});
for(const row of fixture.cases)test(row.id,async()=>{
 const host=new StagedActionHost(),owner=harness(),pending=row.requests.map(id=>host.open(owner.make(id),{}));
 try{
  await tick();expect(owner.queried).toEqual(row.queried);
  host.cancel();expect(await Promise.all(pending)).toEqual(row.requests.map(()=>"cancelled"));
 }finally{owner.retire();await host.close();await Promise.allSettled(pending);}
});
test(fixture.closing.id,async()=>{
 const host=new StagedActionHost(),owner=harness(),release=deferred(),held=heldLease(release.promise),opened=host.open(owner.make("first",held.lease),{});
 let first=false,second=false;
 try{
  await tick();expect(host.snapshot()?.controls).not.toBeNull();
  const close1=host.close().then(()=>{first=true;}),close2=host.close().then(()=>{second=true;});
  await tick();expect(!first&&!second).toBe(fixture.closing.pending);expect(held.calls()).toBe(fixture.closing.closeCalls);
  release.resolve();await Promise.all([close1,close2]);expect(await opened).toBe("cancelled");expect(host.snapshot()).toBeNull();
 }finally{release.resolve();owner.retire();await host.close();await opened;}
});
test(fixture.cancel.id,async()=>{
 const host=new StagedActionHost(),owner=harness(),release=deferred(),held=heldLease(release.promise);
 const first=host.open(owner.make("first",held.lease),{});await tick();
 const second=host.open(owner.make("second"),{}),third=host.open(owner.make("third"),{});
 try{
  await tick();host.cancel();release.resolve();await tick();
  expect(owner.queried).toEqual(fixture.cancel.queried);expect<readonly string[]>(await Promise.all([first,second,third])).toEqual(fixture.cancel.results);
 }finally{release.resolve();owner.retire();await host.close();await Promise.allSettled([first,second,third]);}
});

test(fixture.retirement.id,async()=>{
 const host=new StagedActionHost(),owner=harness(),held=heldLease(Promise.resolve());let attempts=0,finished=false;
 const lease={...held.lease,close:async()=>{attempts++;if(attempts===1)throw new Error(fixture.retirement.error);await held.lease.close();}};
 const opened=host.open(owner.make("first",lease),{});void opened.then(()=>{finished=true;});
 try{
  await tick();await expect(host.close()).rejects.toThrow(fixture.retirement.error);await tick();
  expect(finished).toBe(fixture.retirement.firstFinished);expect(host.snapshot()?.error).toBe(fixture.retirement.error);
  await host.close();expect(await opened).toBe("cancelled");expect(attempts).toBe(fixture.retirement.attempts);expect(host.snapshot()).toBeNull();
 }finally{owner.retire();await host.close();await opened;await lease.close();}
});

test("replacement starts from the live revision after old surface cleanup",async()=>{
 const host=new StagedActionHost(),owner=harness(),release=deferred(),held=heldLease(release.promise),tokens:{revision:string;generation:string}[]=[];
 const first=host.open(owner.make("first",held.lease),{});await tick();
 let revision=fixture.openingRevision.initial;
 const original=owner.make("second"),source={...original,revision:()=>revision,query:async(...args:Parameters<StagedActionSource["query"]>)=>{tokens.push(args[0].token);return original.query(...args);}};
 const second=host.open(source,{});
 try{
  await tick();revision=fixture.openingRevision.current;release.resolve();await tick();
  expect(tokens.map(({revision,generation})=>({revision,generation}))).toEqual([{revision:fixture.openingRevision.current,generation:fixture.openingRevision.generation}]);
  host.cancel();expect(await Promise.all([first,second])).toEqual(["cancelled","cancelled"]);
 }finally{release.resolve();owner.retire();await host.close();await Promise.allSettled([first,second]);}
});
test("an initially unavailable revision resumes when the same original revision becomes readable",async()=>{
 const host=new StagedActionHost(),owner=harness();let revision:string|null=null;
 const source={...owner.make("last"),revision:()=>revision},opened=host.open(source,{});
 try{
  await tick();expect(owner.queried).toEqual([]);
  revision=fixture.openingRevision.initial;owner.notify();await tick();expect(owner.queried).toEqual(["last"]);
  host.cancel();expect(await opened).toBe("cancelled");
 }finally{owner.retire();await host.close();await opened;}
});
test("notification failure cannot strand the original action",async()=>{
 const host=new StagedActionHost(),owner=harness();let notified=0;
 host.subscribe(()=>{throw new Error("presentation observer failed");});host.subscribe(()=>{notified++;});
 const opened=host.open(owner.make("last"),{}),observed=opened.then(result=>({result}),error=>({error}));
 try{
  await tick();expect(owner.queried).toEqual(["last"]);expect(notified).toBeGreaterThan(0);
  host.cancel();expect(await observed).toEqual({result:"cancelled"});
 }finally{owner.retire();await host.close();await observed;}
});

test("a late original lease remains retryable after its action was cancelled",async()=>{
 const host=new StagedActionHost(),owner=harness(),held=heldLease(Promise.resolve());let deliver!:(lease:OwnedUiSurfaceLease)=>void,attempts=0;
 const original=owner.make("late"),source={...original,query:()=>new Promise<OwnedUiSurfaceLease>(resolve=>{deliver=resolve;})};
 const lease={...held.lease,close:async()=>{attempts++;if(attempts===1)throw new Error(fixture.retirement.error);await held.lease.close();}};
 const opened=host.open(source,{});
 try{
  await tick();await host.close();expect<string>(await opened).toBe(fixture.lateLease.result);
  deliver(lease);await tick();expect(attempts).toBe(fixture.lateLease.firstAttempts);
  await host.close();expect(attempts).toBe(fixture.lateLease.finalAttempts);
  await host.close();expect(attempts).toBe(fixture.lateLease.finalAttempts);
 }finally{deliver?.(lease);owner.retire();await host.close().catch(()=>host.close());await opened;await lease.close();}
});

test("cancelling the pending notification never calls an already-aborted query",async()=>{
 const host=new StagedActionHost(),owner=harness();
 const unsubscribe=host.subscribe(()=>{if(host.snapshot()?.pending)host.cancel();}),opened=host.open(owner.make("last"),{});
 try{expect<string>(await opened).toBe(fixture.lateLease.result);expect(owner.queried).toEqual([]);}
 finally{unsubscribe();owner.retire();await host.close();await opened;}
});
