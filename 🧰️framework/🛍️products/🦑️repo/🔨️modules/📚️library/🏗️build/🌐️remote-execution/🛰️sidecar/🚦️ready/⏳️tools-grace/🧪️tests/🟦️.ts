import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import owns from "../../../../../../⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️install.json" with {type:"json"};
import {parseLeaseProgressV1} from "../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts";
import {currentToolsCheckpoint,recordToolsProgress,toolsGraceActive,type ToolsSample,type ToolsCheckpoint} from "../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import aliases from "./🔣️.json" with {type:"json"};
const A="native-artifact-input-read", B="dependencies:selected-postorder";
const proof={pid:5,birth:100,parentSince:110,sourceSince:115};
const row=(done:number,updatedAt:number,phase=A,series=120,p=proof):ToolsSample=>({proof:p,phase,series,done,updatedAt});
test("fixed one-window Ready continuation requires two advancing samples of the same owned series",()=>{
 const db=new Database(":memory:"),valid=new Ajv({strict:true}).compile(schema.definitions.ToolsGraceNatural);
 const at=(state:ToolsCheckpoint|null,now:number,stage="tools",sampleLimit=15)=>toolsGraceActive(state,{stage,started:100,now,soft:100,quiet:45,sampleLimit});
 let current=recordToolsProgress(null,row(100,460),461);
 try{
 expect(at(current,461)).toBe(false);
 current=recordToolsProgress(current,row(101,470),471);
 expect(at(current,471)).toBe(false);
 current=recordToolsProgress(current,row(102,490),491);
 expect(at(current,491)).toBe(false);
 expect(at(current,501)).toBe(true);
 expect(at(current,506)).toBe(true);
 expect(at(current,507)).toBe(false);
 expect(at(current,501,"workspace")).toBe(false);
 const adopted=JSON.parse(JSON.stringify(current));
 expect(at(adopted,501)).toBe(true);
 expect(at(recordToolsProgress(current,undefined,500),501)).toBe(false);
 const interrupted=recordToolsProgress(current,undefined,495);
 expect(at(recordToolsProgress(interrupted,row(102,500),500),501)).toBe(false);
 expect(at(recordToolsProgress(interrupted,row(103,500),500),501)).toBe(false);
 expect(at(recordToolsProgress(current,row(199,480),500),501)).toBe(false);
 expect(at(recordToolsProgress(current,row(199,500,B),500),501)).toBe(false);
 expect(at(recordToolsProgress(current,row(99,500),500),501)).toBe(false);
 expect(at(recordToolsProgress(current,row(152,500,A,500),500),501)).toBe(false);
 expect(at(recordToolsProgress(current,row(152,500,A,120,{...proof,birth:101}),500),501)).toBe(false);
 expect(at(recordToolsProgress(current,row(102,501),501),517)).toBe(false);
 for(const now of [500,501,600,601]){
  expect(valid(now)).toBe(true);
  const updated={...current,sampleAt:now,registryAt:now,counters:[{...current.counters[0]!,previousAt:now-10,advancedAt:now-5}]};
  const oracle=db.query("SELECT ? > 500 AND ? <= 600 AS eligible").get(now,now) as {eligible:number};
  expect(at(updated,now)).toBe(Boolean(oracle.eligible));
 }
 }finally{db.close();}
});


test("funded ownership continues by exact advancing operation under the existing clock",()=>{
 const operations=owns.$defs.CurrentOwnershipProgress.properties.operation.enum;
 const family="dependencies:funded-ownership:";
 const validator=new Ajv({strict:true}).compile(schema.definitions.ToolsGracePhase);
 const remains=(state:ToolsCheckpoint,now:number)=>toolsGraceActive(state,{stage:"tools",started:100,now,soft:100,quiet:45,sampleLimit:15});
 expect(schema.definitions.ToolsGracePhase.enum.filter(phase=>phase.startsWith(family))).toEqual(operations.map(operation=>family+operation));
 for(const operation of operations){
  const phase=family+operation;
  expect(validator(phase)).toBe(true);
  expect(parseLeaseProgressV1(phase+":5832")).toEqual({phase,done:5832});
  const initial=recordToolsProgress(null,row(50,480,phase),480);
  const repeated=recordToolsProgress(recordToolsProgress(initial,row(50,491,phase),491),row(50,501,phase),501);
  expect(remains(repeated,501)).toBe(false);
  const once=recordToolsProgress(initial,row(51,492,phase),492);
  expect(remains(once,501)).toBe(false);
  const twice=recordToolsProgress(once,row(52,501,phase),501);
  expect(remains(twice,501)).toBe(true);
  const different=family+(operation==="state"?"list":"state");
  expect(remains(recordToolsProgress(twice,row(53,502,different),502),502)).toBe(false);
 }
 expect(validator(family+"unknown")).toBe(false);
});


test("a current unreset provider accepts only the immediately observed qualified Native read",()=>{
 const db=new Database(":memory:"),B="javascript-source-provider",C="dependencies:sync-bindings";
 const seal=(state:ToolsCheckpoint,now:number)=>toolsGraceActive(state,{stage:"tools",started:100,now,soft:100,quiet:45,sampleLimit:15});
 const seed=recordToolsProgress(null,row(100,460),461),step=recordToolsProgress(seed,row(101,470),471),qualified=recordToolsProgress(step,row(102,490),491);
 const switchTo=(state:ToolsCheckpoint,now=501)=>recordToolsProgress(state,row(20,now,B,now),now);
 try{
  const changed=switchTo(qualified);
  expect(seal(changed,501)).toBe(true);
  expect(currentToolsCheckpoint(changed,100,501)).toBe(true);
  expect(currentToolsCheckpoint({...changed,counters:[...changed.counters].reverse()},100,501)).toBe(false);
  expect(seal(switchTo(step),501)).toBe(false);
  expect(seal(recordToolsProgress(qualified,row(3,501,C,501),501),501)).toBe(false);
  expect(seal(switchTo(recordToolsProgress(qualified,row(3,500,C,500),500)),501)).toBe(false);
  expect(seal(switchTo(recordToolsProgress(qualified,row(1,500,A,500),500)),501)).toBe(false);
  expect(seal(recordToolsProgress(changed,row(19,502,B,502),502),502)).toBe(false);
  expect(seal(recordToolsProgress(qualified,row(20,501,B,501,{...proof,sourceSince:116}),501),501)).toBe(false);
  expect(seal(recordToolsProgress(qualified,row(20,501,B,501,{...proof,pid:6}),501),501)).toBe(false);
  expect(seal(recordToolsProgress(changed,undefined,502),502)).toBe(false);
  expect(seal(recordToolsProgress(recordToolsProgress(changed,undefined,502),row(21,503,B,501),503),503)).toBe(false);
  expect(seal(changed,517)).toBe(false);
  for(const now of [501,516,517,600,601]){
   const repeated=recordToolsProgress(changed,row(20,now,B,501),now);
   const allowed=db.query("SELECT ? - 471 <= 45 AND ? <= 600 AS allowed").get(now,now) as {allowed:number};
   expect(seal(repeated,now)).toBe(Boolean(allowed.allowed));
  }
  const afterAbsolute={...changed,sampleAt:601,registryAt:601,counters:changed.counters.map((c,i)=>i===0?{...c,previousAt:598,advancedAt:599}:c)};
  expect(seal(afterAbsolute,601)).toBe(false);
 }finally{db.close();}
});

test("historical Source alias scans qualify only while counts advance",()=>{
 const db=new Database(":memory:");
 const validPhase=new Ajv({strict:true}).compile(schema.definitions.ToolsGracePhase);
 try{
  expect(validPhase(aliases.phase)).toBe(true);
  expect(schema.definitions.ToolsGraceCheckpoint.properties.counters.maxItems).toBe(schema.definitions.ToolsGracePhase.enum.length);
  expect(validPhase("dependencies:source-aliases")).toBe(false);
  expect(parseLeaseProgressV1(aliases.phase+":9016")).toEqual({phase:aliases.phase,done:9016});
  const oracle=db.query("SELECT ? < ? AND ? < ? AS eligible");
  for(const {counters:[first,next,last],eligible} of aliases.series){
   expect(Boolean((oracle.get(first,next,next,last) as {eligible:number}).eligible)).toBe(eligible);
   const one=recordToolsProgress(null,row(first!,480,aliases.phase),480);
   const two=recordToolsProgress(one,row(next!,490,aliases.phase),490);
   const three=recordToolsProgress(two,row(last!,501,aliases.phase),501);
   const decision=(now:number)=>toolsGraceActive(three,{stage:"tools",started:100,now,soft:100,quiet:45,sampleLimit:15});
   expect(decision(501)).toBe(eligible);
   expect(decision(517)).toBe(false);
  }
 }finally{db.close();}
});
