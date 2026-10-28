/** 🚦️ Contract: the real actions of a build run only as wide as the jobs its grant holds now. Every Bazel action birth first waits for a quiet grant to grow (the executor point of the first action that really runs) and then for a slot (the script's side is the births law); a partial growth therefore never runs at the full width Bazel was started with, a later growth widens it, and the width gate equals the independent Python model on seeded traces.
 * @see ./🐍️oracle/🐍️.py
 * @see ./🥒️.feature
 * @see ../../🟦️.ts
 * @see ../🧷️births/🟦️.ts */
import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {hostBootAtV1,newHostCapacityIdV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {bazelLiveHostJobsV1,createBazelActionWidthV1} from "../../🟦️.ts";
import oracle from "./🧫️fixtures/🔣️.json" with {type:"json"};

const flush=async()=>{for(let turn=0;turn<4;turn++)await Promise.resolve();};

test("The width gate equals the independent Python model on every seeded trace, and the traces reach every rule",async()=>{
 for(const key of ["waitedThenStarted","refusedByLeave","refusedByAbort","idleStartsOne","unboundedWide"])expect((oracle.coverage as Record<string,number>)[key]??0,key).toBeGreaterThan(0);
 for(const trace of oracle.traces){
  let jobs:number|undefined;const gate=createBazelActionWidthV1<string>(()=>jobs,3_600_000),started=new Set<string>(),left=new Set<string>(),refused=new Set<string>(),controllers=new Map<string,AbortController>();
  for(const [index,row] of trace.events.entries()){
   const value=(row as {value?:string|number|null}).value;
   if(row.event==="jobs")jobs=value===null?undefined:value as number;
   else if(row.event==="enter"){const controller=new AbortController();controllers.set(value as string,controller);gate.enter(value as string,controller.signal).then(()=>{started.add(value as string);},()=>{refused.add(value as string);});}
   else if(row.event==="leave"){if(started.has(value as string))left.add(value as string);gate.leave(value as string);}
   else if(row.event==="abort")controllers.get(value as string)?.abort(Error("aborted"));
   else gate.recheck();
   await flush();
   const running=[...started].filter(key=>!left.has(key)).sort(),waiting=[...controllers.keys()].filter(key=>!started.has(key)&&!refused.has(key)).sort();
   expect({running,waiting,refused:[...refused].sort()},`${trace.name} #${index} ${row.event}`).toEqual(row.expect);
   expect([gate.running(),gate.waiting()]).toEqual([running.length,waiting.length]);
  }
  for(const key of controllers.keys())gate.leave(key);
  await flush();
 }
});

test("A partial growth bounds the width and a later growth widens it, waking the waiters by itself",async()=>{
 let jobs=2;const gate=createBazelActionWidthV1<string>(()=>jobs,20),order:string[]=[];
 for(const key of ["a","b","c","d"])void gate.enter(key).then(()=>order.push(key));
 await flush();expect(order).toEqual(["a","b"]);
 jobs=4;await new Promise(done=>setTimeout(done,80));
 expect(order,"the poll admits the waiters once the grant holds more jobs").toEqual(["a","b","c","d"]);
 expect(gate.waiting()).toBe(0);
});

test("The live jobs follow the grant row: fewer after a partial growth, the carried plan while the grant is quiet, no bound without a grant",()=>{
 const directory=mkdtempSync(join(tmpdir(),"q16w-")),id=newHostCapacityIdV1(),now=Date.now();
 try{
  mkdirSync(join(directory,"grants"),{recursive:true});
  const plan=JSON.stringify({version:1,id,jobs:8,cpu:8,memoryMegabytes:4000,fixedMegabytes:500,floorMegabytes:2048}),environment={SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_HOST_CAPACITY_GRANT:id,SEMIO_HOST_CAPACITY_PLAN:plan};
  const row=(over:object)=>writeFileSync(join(directory,"grants",id+".json"),JSON.stringify({id,operation:"run",jobs:1,cpu:1,reservedMegabytes:600,admittedAtMs:now,observedMegabytes:null,pid:process.pid,bootAtMs:hostBootAtV1(directory,now),...over}));
  row({quiet:true,arrivedAtMs:now});expect(bazelLiveHostJobsV1(environment),"quiet: the full request's jobs the tool was started with").toBe(8);
  row({jobs:3,cpu:3,reservedMegabytes:2100,upgradedAtMs:now});expect(bazelLiveHostJobsV1(environment),"after a partial growth").toBe(3);
  expect(bazelLiveHostJobsV1({}),"no grant").toBeUndefined();
 }finally{rmSync(directory,{recursive:true,force:true});}
});
