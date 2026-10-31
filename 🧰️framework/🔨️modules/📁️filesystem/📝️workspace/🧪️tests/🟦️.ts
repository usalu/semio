import "../🎮️management/🕰️history/🧪️tests/🟦️.ts";
import "../🎮️management/🔏️fingerprint/🧪️tests/🟦️.ts";
import "../📬️commands/🧪️tests/🟦️.ts";
import "../🎮️management/🪪️namespace/🧪️tests/🟦️.ts";
import "../🎮️management/🧪️tests/🟦️.ts";
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};
import {Workspace,WorkspaceError,validateRelative,validateText,type Revision,type WorkspacePort} from "../🟦️.ts";

test("source workspace shared paths and text match schema and independent SQLite",()=>{
 const validate=new Ajv({strict:false}).compile(schema);expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);const db=new Database(":memory:");
 try{for(const row of fixture.cases){let code:string|undefined;try{validateRelative(row.path);validateText(row.text);}catch(error){code=(error as WorkspaceError).code;}expect(code===undefined,row.name).toBe(row.valid);if(!row.valid)expect(code).toBe(row.code);
 const binary=db.query("SELECT instr(?1,char(0))>0 AS binary,length(CAST(?1 AS BLOB)) AS bytes").get(row.text) as {binary:number;bytes:number};if(row.code==="not-text")expect(binary.binary).toBe(1);if(row.valid)expect(binary.bytes).toBe(Buffer.byteLength(row.text));}}
 finally{db.close();}
});

for(const scenario of ["save","conflict","final-conflict","cancel","publication-race","receipt-failure"]){
 test(`source workspace command model and independent event oracle: ${scenario}`,async()=>{
  let revision:Revision={identity:"1",text:"before"},staged="",retained:Revision|undefined;const events:string[]=[];let retired=false;const control=new AbortController();
  const port:WorkspacePort={list:async()=>[],read:async()=>revision,stage:async(_path,text)=>{staged=text;return "stage-1";},retire:async()=>{retired=true;},record:async event=>{events.push(event.kind);if(scenario==="receipt-failure"&&event.kind==="save-committed")throw Error("Injected receipt failure");},publish:async()=>{if(scenario==="publication-race")revision={identity:"2",text:"foreign"};retained=revision;revision={identity:"3",text:staged};return {predecessor:retained,current:revision,recoveryPath:"events/stage-1.before"};}};
  const workspace=new Workspace(port),document=await workspace.load("a.txt",control.signal,()=>{});if(scenario==="conflict")revision={identity:"2",text:"foreign"};
  let code:string|undefined,recovery:string|undefined;try{await workspace.save("a.txt",document.revision,"local",control.signal,progress=>{if(progress.stage==="publishing"){if(scenario==="cancel")control.abort();if(scenario==="final-conflict")revision={identity:"2",text:"foreign"};}});}catch(error){code=(error as WorkspaceError).code;recovery=(error as WorkspaceError).recoveryPath;}
  const db=new Database(":memory:");try{const expected=db.query("SELECT CASE ?1 WHEN 'save' THEN NULL WHEN 'cancel' THEN 'cancelled' WHEN 'receipt-failure' THEN 'io' ELSE 'conflict' END AS code,CASE WHEN ?1 IN ('save','publication-race','receipt-failure') THEN 'local' WHEN ?1='cancel' THEN 'before' ELSE 'foreign' END AS text").get(scenario) as {code:string|null;text:string};expect(code??null).toBe(expected.code);expect(revision.text).toBe(expected.text);}finally{db.close();}
  if(scenario==="publication-race"){expect(retained!.text).toBe("foreign");expect(recovery).toBe("events/stage-1.before");expect(events.at(-1)).toBe("save-conflict");}if(scenario==="cancel"||scenario==="final-conflict")expect(retired).toBe(true);if(scenario==="save")expect(events).toEqual(["save-intent","save-committed"]);
  if(scenario==="receipt-failure"){expect(retained!.text).toBe("before");expect(recovery).toBe("events/stage-1.before");expect(retired).toBe(false);}
 });
}
