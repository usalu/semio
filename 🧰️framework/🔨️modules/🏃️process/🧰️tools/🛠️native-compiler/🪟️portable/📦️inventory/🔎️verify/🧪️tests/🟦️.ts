import {expect,test} from "bun:test";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {mkdtemp,mkdir,rm,writeFile} from "node:fs/promises";
import {join,resolve} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import cases from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {inventoryAllocationV1,inventoryProgressDueV1,inventoryQueueV1} from "../🚦️queue/🟦️.ts";
import {inspectPortableMsvcTreeV1} from "../🟦️.ts";

test("neutral allocation vectors agree with the independent schema oracle",()=>{
 const ajv=new Ajv(),validate=ajv.compile(schema.definitions.Protocol),oracle=ajv.compile({oneOf:cases.weights.map(row=>({const:row}))});
 expect(validate(schema.definitions.Protocol.const)).toBe(true);
 for(const row of cases.weights)expect(oracle({bytes:row.bytes,allocation:inventoryAllocationV1(row.bytes)})).toBe(true);
 expect(()=>inventoryAllocationV1(268435457)).toThrow();
});

test("neutral progress vectors report completed work at the authored cadence",()=>{
 for(const row of cases.cadence){const counts=Array.from({length:row.total},(_,index)=>index+1).filter(count=>inventoryProgressDueV1(count,row.total));expect(counts).toEqual(row.counts);}
});

test("weighted work overlaps, drains and admits no late work after failure",async()=>{
 const release:Array<()=>void>=[],started:number[]=[],finished:number[]=[],queue=inventoryQueueV1();
 const work=(index:number)=>async()=>{started.push(index);await new Promise<void>(resolve=>release[index]=resolve);finished.push(index);};
 await queue.submit(cases.admission.bytes[0]!,work(0));await queue.submit(cases.admission.bytes[1]!,work(1));const third=queue.submit(cases.admission.bytes[2]!,work(2));
 expect(started).toEqual(cases.admission.firstBatch);
 release[0]!();await third;expect(started).toEqual([0,1,2]);
 release[1]!();release[2]!();await queue.drain();expect(finished.sort()).toEqual([0,1,2]);
});

test("first failure and cancellation drain started work before returning",async()=>{
 let finish!:()=>void,late=false;const failure=Error("first"),queue=inventoryQueueV1();
 await queue.submit(1,async()=>{await new Promise<void>(resolve=>finish=resolve);late=true;});
 await queue.submit(1,async()=>{throw failure;});
 await new Promise(resolve=>setTimeout(resolve,0));let done=false;const draining=queue.drain().catch(error=>{expect(error).toBe(failure);done=true;});
 await new Promise(resolve=>setTimeout(resolve,0));expect(done).toBe(false);finish();await draining;expect(late).toBe(true);await expect(queue.submit(1,async()=>{})).rejects.toBe(failure);
 const controller=new AbortController(),cancelled=inventoryQueueV1(controller.signal);controller.abort(failure);await expect(cancelled.submit(1,async()=>{})).rejects.toBe(failure);await expect(cancelled.drain()).rejects.toBe(failure);
});

test("cancellation stops waiting admission and drains active work",async()=>{
 const controller=new AbortController(),queue=inventoryQueueV1(controller.signal),failure=Error("cancelled");let release!:()=>void,completed=false,late=false;
 await queue.submit(268435456,async()=>{await new Promise<void>(resolve=>release=resolve);completed=true;});
 const waiting=queue.submit(268435456,async()=>{late=true;});controller.abort(failure);await expect(waiting).rejects.toBe(failure);
 let settled=false;const drain=queue.drain().catch(error=>{expect(error).toBe(failure);settled=true;});await new Promise(resolve=>setTimeout(resolve,0));expect(settled).toBe(false);release();await drain;expect(completed).toBe(true);expect(late).toBe(false);
});

test("worker count bounds admission and failure keeps pending work unstarted",async()=>{
 const queue=inventoryQueueV1(),release:Array<()=>void>=[],failure=Error("work failed");let started=0;
 for(let index=0;index<4;index++)await queue.submit(1,async()=>{started++;await new Promise<void>(resolve=>release[index]=resolve);if(index===0)throw failure;});
 const pending=queue.submit(1,async()=>{started++;});expect(started).toBe(4);release[0]!();await expect(pending).rejects.toBe(failure);
 let drained=false;const done=queue.drain().catch(error=>{expect(error).toBe(failure);drained=true;});await new Promise(resolve=>setTimeout(resolve,0));expect(drained).toBe(false);
 for(let index=1;index<4;index++)release[index]!();await done;expect(started).toBe(4);
});

test("a failure or abort before a deferred start prevents work",async()=>{
 for(const mode of ["failure","abort"] as const){
  const controller=new AbortController(),queue=inventoryQueueV1(controller.signal),reason=Error(mode);let started=false;
  const submitted=queue.submit(1,async()=>{started=true;});if(mode==="abort")controller.abort(reason);else queue.fail(reason);
  await submitted;await expect(queue.drain()).rejects.toBe(reason);expect(started).toBe(false);
 }
});

test("drain closes admission immediately and after settlement",async()=>{
 const queue=inventoryQueueV1();let release!:()=>void,started=false;await queue.submit(1,async()=>{await new Promise<void>(resolve=>release=resolve);});
 const draining=queue.drain();await expect(queue.submit(1,async()=>{started=true;})).rejects.toThrow();release();await draining;
 await expect(queue.submit(1,async()=>{started=true;})).rejects.toThrow();expect(started).toBe(false);
});

test("physical tree validates bytes and membership and detects a concurrent edit",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Test artifact directory required");
 const root=await mkdtemp(join(resolve(output),"inventory-")),controller=new AbortController(),trace:string[]=[],control={signal:controller.signal,remainingMilliseconds:()=>null,publish:(value:{resource:string})=>{trace.push(value.resource);},yieldContinuation:async()=>{}};
 try{
  await mkdir(join(root,"VC"));const files=["a","b","c","d"].map(name=>({path:`VC/${name}`,bytes:1,sha256:createHash("sha256").update(name).digest("hex")}));
  for(const row of files)await writeFile(join(root,...row.path.split("/")),row.path.at(-1)!);
  const inventory={version:1,recipeSha256:"a".repeat(64),files},first=await inspectPortableMsvcTreeV1(root,inventory,control);expect(trace).toEqual(cases.progress);trace.length=0;const second=await inspectPortableMsvcTreeV1(root,inventory,control);
  expect(first.files).toBe(4);expect(first.bytes).toBe(4);expect(first.identitySha256).toBe(second.identitySha256);
  await writeFile(join(root,"VC","extra"),"x");await expect(inspectPortableMsvcTreeV1(root,inventory,control)).rejects.toThrow();await rm(join(root,"VC","extra"));
  let edited=false;const racing={...control,yieldContinuation:async()=>{if(!edited){edited=true;await writeFile(join(root,"VC","a"),"z");}}};
  await expect(inspectPortableMsvcTreeV1(root,inventory,racing)).rejects.toThrow();expect(edited).toBe(true);
  await writeFile(join(root,"VC","a"),"a");const failedTrace:string[]=[],changed={...control,publish:async(value:{resource:string})=>{failedTrace.push(value.resource);if(value.resource===cases.progress[0])await writeFile(join(root,"VC","a"),"z");}};
  await expect(inspectPortableMsvcTreeV1(root,inventory,changed)).rejects.toThrow();expect(failedTrace).toEqual([cases.progress[0]]);
 }finally{await rm(root,{recursive:true,force:true});}
},120000);
