import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import{createScriptProcessEnvelope,createIssuedResourceControlV1,createRequestedIssuedResourceControlV1,readIssuedScriptProcessEnvelopeV1,withIssuedResourceControlV1,withIssuedResourceOperationV1,withScriptInvocationSignalV1,withScriptProcessEnvelope}from"../../🟦️.ts";
import type{ResourceLeaseControl}from"../../../../../🔒️leases/🎛️control/🟦️.ts";
import schema from"../../🧬️schema/🌊️drain.json";
import corpus from"./🔣️.json";

/** 🌊️ Pins that accepted work admitted beneath live owners while an Original or loan joins its work is joined, and that admission closes in the very step that observes the owner empty, so no admitted operation completes after settlement. @see ../../🟦️.ts */
test("a draining owner joins nested admissions and refuses every admission after its join, as an independent event log proves",async()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);
 for(const row of corpus.cases){
  const database=new Database(":memory:");database.run("create table events(seq integer primary key autoincrement,name text not null)");
  const note=(name:string)=>{database.run("insert into events(name) values(?)",[name]);};
  const start=(control:ResourceLeaseControl)=>{void withIssuedResourceOperationV1(control,async()=>{await new Promise<void>(done=>setImmediate(done));note("first-end");}).then(async()=>{
   for(let step=0;step<row.hops;step++)await null;
   try{await withIssuedResourceOperationV1(control,async()=>{await new Promise<void>(done=>setTimeout(done,20));note("late-end");});}catch{note("late-refused");}
  });};
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"fixture:drain",maximumElapsedMilliseconds:0},{fixture:"drain"},Date.now()),async original=>{
   if(row.scope==="process"){start(createIssuedResourceControlV1(original,"fixture:drain",0));return;}
   await withIssuedResourceControlV1(original,"fixture:drain-loan",0,async loan=>{start(createRequestedIssuedResourceControlV1(loan,0));});
   note("settled");
  });
  if(row.scope==="process")note("settled");
  await new Promise<void>(done=>setTimeout(done,80));
  const orphans=database.query("select count(*) as n from events late join events done on done.name='settled' where late.name='late-end' and late.seq>done.seq").get() as{n:number},outcomes=database.query("select group_concat(name) as names from events where name in ('late-end','late-refused')").get() as{names:string|null};
  expect(orphans.n,JSON.stringify(row)).toBe(0);
  expect(outcomes.names,JSON.stringify(row)).not.toBeNull();
  if(row.expected==="joined")expect(outcomes.names,JSON.stringify(row)).toBe("late-end");
  database.close();
 }
},60000);

/** 🚪️ Pins that a draining Original keeps admitting only beneath an owner that is still live and accepting: a new top-level loan, a new nested process envelope and a new loan beneath the Original itself stay refused. */
test("a draining Original refuses new top-level admissions and admits only beneath a live owner",async()=>{
 const outcomes:Record<string,string>={};
 await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"fixture:drain-top",maximumElapsedMilliseconds:0},{fixture:"drain"},Date.now()),async original=>{
  const owner=createIssuedResourceControlV1(original,"fixture:drain-owner",0),attempt=async(name:string,run:()=>unknown)=>{try{await run();outcomes[name]="admitted";}catch{outcomes[name]="refused";}};
  void withIssuedResourceOperationV1(owner,async()=>{
   await new Promise<void>(done=>setImmediate(done));
   await attempt("top-level control",()=>createIssuedResourceControlV1(original,"fixture:late-top",0));
   await attempt("top-level loan",()=>withIssuedResourceControlV1(original,"fixture:late-loan",0,async()=>{}));
   await attempt("nested envelope",()=>withScriptProcessEnvelope(readIssuedScriptProcessEnvelopeV1(original),async()=>{}));
   await attempt("owner operation",()=>withIssuedResourceOperationV1(owner,async()=>{}));
   await attempt("owner narrowing",()=>createRequestedIssuedResourceControlV1(owner,0));
  });
 });
 expect(outcomes).toEqual({"top-level control":"refused","top-level loan":"refused","nested envelope":"refused","owner operation":"admitted","owner narrowing":"admitted"});
},30000);

/** 🛑️ Pins that cancelling a draining Original closes admission beneath its live owners too: the accepted work is still joined, but no new operation is admitted once the cancellation signal fired. */
test("a cancelled draining Original admits nothing new beneath a live owner and still joins its accepted work",async()=>{
 const reason=Error("fixture cancellation"),cancellation=new AbortController(),events:string[]=[];let late:unknown;
 await expect(withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"fixture:drain-cancel",maximumElapsedMilliseconds:0},{fixture:"drain"},Date.now()),async original=>{
  await expect(withScriptInvocationSignalV1(original,cancellation.signal,async alias=>{
   const owner=createIssuedResourceControlV1(alias,"fixture:drain-cancel-owner",0);
   void withIssuedResourceOperationV1(owner,async()=>{
    await new Promise<void>(done=>setImmediate(done));cancellation.abort(reason);
    try{await withIssuedResourceOperationV1(owner,async()=>{events.push("admitted");});}catch(error){late=error;}
    events.push("first-end");
   }).catch(()=>{});
  })).rejects.toBe(reason);
 })).rejects.toBe(reason);
 expect(events).toEqual(["first-end"]);expect(late).toBe(reason);
},30000);
