import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import * as grace from "../../🟦️.ts";
import schema from "../../../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};

test("the exact tools hard-cap decision reports the closed cause it evaluated",()=>{
 const check=Reflect.get(grace,"toolsGraceDenial") as (state:grace.ToolsCheckpoint|null,input:{stage:string;eligible:boolean;progressing:boolean;now:number;started:number;soft:number;quiet:number;sampleLimit:number})=>string|undefined;
 expect(typeof check).toBe("function");
 const db=new Database(":memory:"),proof={pid:5,birth:100,parentSince:110,sourceSince:115};
 const phase="javascript-selected-payload",counter={phase,series:120,done:52,previousAt:471,advancedAt:491,reset:false};
 try{
  const sql=db.query(`select case
   when not ?1 or ?2 <= 500 then 'not-eligible'
   when ?2 > 600 then 'budget-exhausted'
   when ?3 > 45 then 'progress-quiet'
   when ?4 = 'missing' then 'sample-missing'
   when ?4 in ('unqualified','future') then 'sample-unqualified'
   when ?2 - ?5 > 15 then 'sample-stale'
   when ?4 = 'reset' then 'series-unqualified'
   when ?4 = 'once' or ?2 - ?6 > 45 then 'series-needs-advances'
   else null end as reason`);
  for(const row of fixture.cases){
   const sampleAt=row.sample==="future"?row.now+1:row.sample==="expired-progress"||row.sample==="at-budget"?row.now:491;
   const series=row.sample==="at-budget"?{...counter,previousAt:590,advancedAt:595}:row.sample==="once"?{...counter,previousAt:0}:row.sample==="reset"?{...counter,reset:true}:counter;
   const state=row.sample==="missing"?null:{proof,counters:[series],current:row.sample==="unqualified"?null:phase,sampleAt,registryAt:Math.min(sampleAt,row.now)};
   const {reason}=sql.get(Number(row.eligible),row.now,row.silent,row.sample,sampleAt,series.previousAt) as {reason:string|null};
   expect(reason).toBe(row.code);
   expect(check(state,{stage:"tools",eligible:row.eligible,progressing:row.silent<=45,now:row.now,started:100,soft:100,quiet:45,sampleLimit:15})??null).toBe(reason);
  }
 }finally{db.close();}
});

test("the inflight and timing contracts close the denial to tools hard-cap stops",()=>{
 const ajv=new Ajv({strict:false});
 const compile=(ref:string)=>ajv.compile({$ref:"#/definitions/"+ref,definitions:schema.definitions});
 const inflight=compile("Inflight"),watchdog=compile("TimingWatchdog"),timings=compile("Timings");
 const record={stage:"tools",pid:10,startedMs:100,exitFile:"exit",log:"log",launch:{version:1,platform:"macos",digest:"a".repeat(64),recipe:"b".repeat(64),prerequisites:[]},stop:"timeout",deadline:"hard-cap"};
 for(const decided of fixture.schema){
  expect(inflight({...record,toolsGraceDenial:decided})).toBe(true);
  expect(watchdog({deadline:"hard-cap",toolsGraceDenial:decided})).toBe(true);
 }
 expect(inflight({...record,toolsGraceDenial:"unknown"})).toBe(false);
 expect(inflight({...record,stage:"workspace",toolsGraceDenial:"sample-missing"})).toBe(false);
 expect(inflight({...record,stop:"stall",toolsGraceDenial:"sample-missing"})).toBe(false);
 expect(watchdog({deadline:"soft-cap-idle",toolsGraceDenial:"sample-missing"})).toBe(false);
 expect(watchdog({toolsGraceDenial:"sample-missing"})).toBe(false);
 expect(watchdog({deadline:"hard-cap"})).toBe(true);
 const run={stage:"tools",startedMs:100,endedMs:1200,seconds:1,exit:1,outcome:"stage-timeout",peakMegabytes:null,attempts:1,digest:"a".repeat(12),watchdog:{deadline:"hard-cap",toolsGraceDenial:"sample-missing"}};
 const document={version:1,origin:"fixture",updatedMs:1300,runs:[run]};
 expect(timings(document)).toBe(true);
 expect(timings({...document,runs:[{...run,stage:"workspace"}]})).toBe(false);
 expect(timings({...document,runs:[{...run,outcome:"stage-failed"}]})).toBe(false);
});
