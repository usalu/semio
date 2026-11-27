/** 🧺️ Contract: the store phase of a generation (the fragments of every returned owner) runs in bounded batches and asks the footprint gate before each one, so a read that already stands near its grant slows down and releases memory between batches instead of allocating the compressed text of every owner on top of it; it never runs more than one batch of jobs at once, keeps the order of the answers and stops between batches when it is cancelled.
 * @see ../🟨️.mjs */
import {expect,test} from "bun:test";
import {GENERATION_POLICY_V1} from "../../🟨️.mjs";
import {mapBatchesV1} from "../🟨️.mjs";
import vectors from "./🔣️.json" with {type:"json"};

const tick=()=>new Promise<void>(resolve=>setTimeout(resolve,0));

test("the declared batch is a positive count that never exceeds the read concurrency",()=>{
 expect(Number.isSafeInteger(GENERATION_POLICY_V1.fragmentBatch)).toBe(true);
 expect(GENERATION_POLICY_V1.fragmentBatch).toBeGreaterThan(0);
 expect(GENERATION_POLICY_V1.fragmentBatch).toBeLessThanOrEqual(GENERATION_POLICY_V1.readConcurrency);
});

test("the answers equal a plain map in the order of the items and no more than one batch runs at once",async()=>{
 const items=Array.from({length:23},(_unused,at)=>at*3+1);let running=0,peak=0;
 const visit=async(item:number,at:number)=>{running++;peak=Math.max(peak,running);await tick();running--;return item*2+at;};
 const batched=await mapBatchesV1(items,{batch:5,visit});
 expect(batched).toEqual(await Promise.all(items.map((item,at)=>item*2+at)));
 expect(peak).toBe(5);expect(running).toBe(0);
});

test("the batch sizes of the language-agnostic vectors are the ones every implementation must produce",async()=>{
 for(const row of vectors.cases){
  const sizes:number[]=[];let before=0,count=0;
  await mapBatchesV1(Array.from({length:row.items},(_unused,at)=>at),{batch:row.batch,visit:async()=>{count++;},after:()=>{sizes.push(count-before);before=count;}});
  expect(sizes,row.name).toEqual(row.sizes);
 }
});

test("the gate is asked once before every batch and a batch does not start while the gate waits",async()=>{
 const events:string[]=[];let release!:()=>void;const held=new Promise<void>(resolve=>{release=resolve;});
 const admit=async(_signal?:AbortSignal)=>{events.push("admit");if(events.filter(event=>event==="admit").length===2)await held;return"run";};
 const visit=async(item:number)=>{events.push("job"+item);};
 const done=mapBatchesV1([0,1,2,3,4],{batch:2,visit,admit});
 await tick();await tick();
 expect(events).toEqual(["admit","job0","job1","admit"]);
 release();await done;
 expect(events).toEqual(["admit","job0","job1","admit","job2","job3","admit","job4"]);
});

test("a cancelled run starts no further batch and names the reason of the signal",async()=>{
 const controller=new AbortController(),started:number[]=[];
 const visit=async(item:number)=>{started.push(item);if(item===1)controller.abort(new Error("cancelled by the reader"));};
 await expect(mapBatchesV1([0,1,2,3,4,5],{batch:2,visit,signal:controller.signal})).rejects.toThrow("cancelled by the reader");
 expect(started).toEqual([0,1]);
});

test("the progress callback reports every finished batch with the running total",async()=>{
 const seen:[number,number][]=[];
 await mapBatchesV1([1,2,3,4,5,6,7],{batch:3,visit:async()=>undefined,after:(done:number,total:number)=>{seen.push([done,total]);}});
 expect(seen).toEqual([[3,7],[6,7],[7,7]]);
});

test("an empty list asks the gate for nothing",async()=>{
 let asked=0;
 expect(await mapBatchesV1([],{batch:4,visit:async()=>1,admit:async()=>{asked++;return"run";}})).toEqual([]);
 expect(asked).toBe(0);
});
