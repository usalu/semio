import{test,expect}from"bun:test";
import{readFile}from"node:fs/promises";
import{AsyncLocalStorage}from"node:async_hooks";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import schema from"./🧬️schema/🔣️.json";
import corpus from"./🔣️.json";
import{instantiateOwnerClosureV1,prepareOwnerClosureV1}from"../🧰️support/🔗️owner-closure/🟦️.ts";

type Round={outcome:"value"|"failure"|"conflict";message?:string;code?:string;undefined?:boolean;young?:boolean;holds?:boolean};

const SETTLE_STUBS_V1=["scopes","sourceCaptures","inferenceValidationScopes","retainObservationsV1","firstObservationConflictV1","observationsHold","conflictingInferenceErrorV1","RACY_MS"];

/** 🔁️ Derives the actual settlement owner and its module bindings from the observation source (or from SETTLE_SOURCE for a patched copy) and runs it over the closed decision corpus. */
async function settlementOwner(){
 return prepareOwnerClosureV1({moduleUrl:new URL("../../../👁️observation/🟨️.mjs",import.meta.url),owner:"settleObservedComputationV1",provided:SETTLE_STUBS_V1,source:process.env.SETTLE_SOURCE?await readFile(process.env.SETTLE_SOURCE,"utf8"):undefined});
}

test("Observed settlement stops repeating a failure that returns unchanged and never repeats a demanded restart",async()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);
 const closure=await settlementOwner(),db=new Database(":memory:");
 db.run("create table terminal(scenario text, outcome text, round integer)");
 try{
  for(const scenario of corpus.scenarios){
   const state={consistent:true,holds:true},scopes=new AsyncLocalStorage<{order:unknown[]}>(),bindings={scopes,sourceCaptures:new AsyncLocalStorage(),inferenceValidationScopes:new AsyncLocalStorage(),retainObservationsV1:()=>{},firstObservationConflictV1:()=>state.consistent?undefined:{row:Object.freeze({path:"modeled Source conflict"})},observationsHold:async()=>state.holds,conflictingInferenceErrorV1:()=>Object.freeze({path:"modeled Source conflict"}),RACY_MS:0},run=instantiateOwnerClosureV1(closure,bindings);
   let calls=0;const events:string[]=[];let settled:string,failure:unknown;
   try{
    await run(async()=>{
     const row=scenario.rounds[calls++] as Round;if(!row)throw Error("Scripted rounds exhausted: "+scenario.name);
     state.consistent=row.outcome!=="conflict";state.holds=row.holds??true;
     if(row.young??true)scopes.getStore()!.order.push({kind:"file",path:"/modeled/young",fact:{mtimeMs:Date.now()+15,ctimeMs:0}});
     if(row.outcome==="failure"){if(row.undefined)throw undefined;throw Object.assign(Error(row.message),row.code?{code:row.code}:{});}
     return "value";
    },{limit:corpus.limit,onRound:(event:{failure?:unknown;conflict?:string})=>{events.push(Object.hasOwn(event,"failure")?"failure":"conflict");}});
    settled="value";
   }catch(error){failure=error;settled=(error as {path?:string})?.path==="modeled Source conflict"?"conflict":"failure";}
   const expected=scenario.expect;
   expect({name:scenario.name,settled,rounds:calls,events}).toEqual({name:scenario.name,settled:expected.settled,rounds:expected.rounds,events:expected.events});
   if(expected.settled==="failure")expect(expected.failure===null?failure:(failure as Error).message).toBe(expected.failure===null?undefined:expected.failure);
   db.run("insert into terminal values (?,?,?)",[scenario.name,settled,calls]);
  }
  const worst=db.query("select max(round) r from terminal where outcome='failure' and scenario like 'repeated-%' or scenario in ('thrown-undefined-repeats','restart-required-is-never-retried')").get() as {r:number};
  expect(worst.r).toBeLessThanOrEqual(2);
 }finally{db.close();}
});
