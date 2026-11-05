/** 📤️ Checks image export facts and lifecycle against neutral vectors and an independent XState machine. */
import {test,expect} from "bun:test";
import {createActor,createMachine,assign} from "xstate";
import fixture from "./🔣️.json";
import {IconExportLedger,cancelIconExport,iconExportTaskId,iconExportJobsSnapshot,subscribeIconExportJobs,runIconExportBatch,iconExportText,iconExportProgressText,type IconExportEvent} from "../📤️export/🟦️.ts";
for(const sample of fixture.cases)test(sample.id+" event oracle",()=>{
 const ledger=new IconExportLedger();
 const expectedLog=JSON.stringify(sample.events);
 const oracle=createActor(createMachine({
  initial:"idle",context:{completed:0},
  states:{
   idle:{on:{started:"active"}},
   active:{on:{progressed:{actions:assign({completed:({event})=>(event as unknown as {completed:number}).completed})},cancelRequested:"cancelling",finished:"finished"}},
   cancelling:{on:{finished:"finished"}},
   finished:{type:"final"}
  }
 }));
 oracle.start();
 for(let index=0;index<sample.events.length;index++){
  const event=sample.events[index] as IconExportEvent,expected=sample.expected[index]!;
  ledger.apply(event);oracle.send({...event,type:event.kind});
  const state=oracle.getSnapshot(),rows=ledger.snapshot();
  expect(state.value).toBe(expected.state);
  expect(state.context.completed).toBe(expected.completed);
  expect(rows.length).toBe(expected.state==="finished"?0:1);
  if(rows[0]){expect(rows[0]).toEqual({job:1,owner:"shooting",label:"shots",total:2,atMs:10,completed:state.context.completed,cancelling:state.value==="cancelling"});}
  expect(ledger.snapshot()).toBe(rows);
 }
 expect(JSON.stringify(ledger.log())).toBe(expectedLog);
 oracle.stop();
});
test("invalid progress and terminal transitions preserve rows",()=>{
 const ledger=new IconExportLedger();
 ledger.apply({kind:"started",job:1,owner:"test",label:"one",total:2,atMs:0});
 const rows=ledger.snapshot();
 for(const event of [{kind:"progressed",job:1,completed:3},{kind:"finished",job:1,outcome:"done"},{kind:"started",job:1,owner:"test",label:"duplicate",total:2,atMs:0}] as IconExportEvent[])expect(()=>ledger.apply(event)).toThrow();
 expect(ledger.snapshot()).toBe(rows);
 ledger.apply({kind:"cancelRequested",job:1});
 expect(()=>ledger.apply({kind:"progressed",job:1,completed:1})).toThrow();
 ledger.apply({kind:"finished",job:1,outcome:"cancelled"});
 expect(()=>ledger.apply({kind:"finished",job:1,outcome:"cancelled"})).toThrow();
});
const items=[{filename:"one.png",request:1},{filename:"two.svg",request:2}];
const host=(changes:Record<string,unknown>={})=>({owner:"shooting",locale:()=>"en",render:async(request:number)=>request,deliver:(_name:string,_result:number)=>{},yieldToHost:async()=>{},...changes});
test("ordered image delivery reports progress and terminal completion",async()=>{
 const delivered:string[]=[],progress:number[]=[];
 const stop=subscribeIconExportJobs(()=>{const row=iconExportJobsSnapshot()[0];if(row)progress.push(row.completed)});
 try{
  const outcome=await runIconExportBatch(items,host({deliver:(name:string)=>delivered.push(name)}));
  expect(outcome).toEqual({outcome:"done",completed:2});expect(delivered).toEqual(["one.png","two.svg"]);expect(progress).toContain(1);expect(progress).toContain(2);
  expect(iconExportJobsSnapshot()).toEqual([]);
 }finally{stop();}
});
test("render and delivery failures leave no running task and announce failure",async()=>{
 for(const changes of [{render:async()=>{throw Error("render failure")}},{deliver:()=>{throw Error("delivery failure")}}]){
  const notices:string[]=[];
  const outcome=await runIconExportBatch(items,host({...changes,announce:(text:string)=>notices.push(text)}));
  expect(outcome.outcome).toBe("failed");expect(outcome.completed).toBe(0);expect(outcome.reason).toContain("failure");
  expect(notices.at(-1)).toContain("failed");expect(iconExportJobsSnapshot()).toEqual([]);
 }
});
test("cancellation before work starts prevents rendering",async()=>{
 let renders=0;
 const outcome=await runIconExportBatch(items,host({yieldToHost:async()=>{const row=iconExportJobsSnapshot()[0]!;cancelIconExport(iconExportTaskId(row.job));},render:async()=>{renders++;return 1}}));
 expect(outcome.outcome).toBe("cancelled");expect(renders).toBe(0);expect(iconExportJobsSnapshot()).toEqual([]);
});
test("cancellation interrupts pending render and suppresses its later delivery",async()=>{
 let release!:(value:number)=>void,entered!:(value:void)=>void,delivered=0;
 const notices:string[]=[];
 const started=new Promise<void>(resolve=>{entered=resolve});
 const pending=runIconExportBatch(items,host({locale:()=>"de",announce:(text:string)=>notices.push(text),render:()=>{entered();return new Promise<number>(resolve=>{release=resolve})},deliver:()=>{delivered++}}));
 await started;
 const row=iconExportJobsSnapshot()[0]!;
 expect(cancelIconExport(iconExportTaskId(row.job))).toBe(true);
 expect(await pending).toEqual({outcome:"cancelled",completed:0});
 expect(notices.at(-1)).toBe("Bildexport abgebrochen.");
 release(1);await Promise.resolve();await Promise.resolve();
 expect(delivered).toBe(0);expect(iconExportJobsSnapshot()).toEqual([]);
 expect(cancelIconExport(iconExportTaskId(row.job))).toBe(false);
});
test("observer and announcement failures cannot prevent terminal cleanup",async()=>{
 const original=console.error,errors:unknown[]=[];console.error=(...args:unknown[])=>{errors.push(args)};
 const stop=subscribeIconExportJobs(()=>{throw Error("observer")});
 try{
  const result=await runIconExportBatch(items,host({announce:()=>{throw Error("notice")}}));
  expect(result.outcome).toBe("done");expect(iconExportJobsSnapshot()).toEqual([]);expect(errors.length).toBeGreaterThan(0);
 }finally{stop();console.error=original;}
});
test("explicit image export languages",()=>{
 expect(iconExportText("done","en")).toContain("exported");
 expect(iconExportText("cancelled","de")).toContain("abgebrochen");
 expect(iconExportProgressText(1,2,"de")).toBe("Bild 1 von 2");
 expect(()=>iconExportText("started","fr")).toThrow();
});
test("cancel between render admission and its microtask avoids starting work",async()=>{
 let renders=0;
 const stop=subscribeIconExportJobs(()=>{
  const row=iconExportJobsSnapshot()[0];
  if(row&&!row.cancelling&&row.completed===0)queueMicrotask(()=>queueMicrotask(()=>cancelIconExport(iconExportTaskId(row.job))));
 });
 try{
  const result=await runIconExportBatch(items,host({render:async()=>{renders++;return 1}}));
  expect(result.outcome).toBe("cancelled");expect(renders).toBe(0);expect(iconExportJobsSnapshot()).toEqual([]);
 }finally{stop();}
});
