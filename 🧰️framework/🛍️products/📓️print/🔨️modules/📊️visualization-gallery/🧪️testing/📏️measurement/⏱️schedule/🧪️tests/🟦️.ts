import{expect,test}from"bun:test";
import{measurePrintItemsInOrderV1,printGalleryCapacityRequestV1}from"../🟦️.ts";
const host={cores:12,availableMegabytes:32768,pressure:"normal" as const,grantedJobs:1,grantedMegabytes:1638,unitMegabytes:2048};
test("Gallery capacity ignores the lone-task job grant but follows cores, memory, pressure and item count",()=>{
  expect(printGalleryCapacityRequestV1(host,320)).toEqual({kind:"available",maximumParallelism:6,minimumParallelism:1});
  expect(printGalleryCapacityRequestV1(host,2)).toEqual({kind:"available",maximumParallelism:2,minimumParallelism:1});
  expect(printGalleryCapacityRequestV1({...host,availableMegabytes:5000},320).maximumParallelism).toBe(1);
  expect(printGalleryCapacityRequestV1({...host,pressure:"critical"},320).maximumParallelism).toBe(1);
  expect(printGalleryCapacityRequestV1({...host,cores:1},0)).toEqual({kind:"available",maximumParallelism:1,minimumParallelism:1});
});
test("Measurements run concurrently up to the worker count, return in item order and report each completion",async()=>{
  const items=Array.from({length:12},(_,index)=>index),events:number[]=[],finished:number[]=[];let active=0,maximum=0;
  const rows=await measurePrintItemsInOrderV1(items,new AbortController().signal,3,async value=>{active++;maximum=Math.max(maximum,active);await new Promise(resolve=>setTimeout(resolve,(12-value)*2));finished.push(value);active--;return value*10;},progress=>{events.push(progress.done);expect(progress.total).toBe(12);});
  expect(rows).toEqual(items.map(value=>value*10));expect(maximum).toBe(3);expect(finished).not.toEqual(items);expect(events).toEqual(items.map(value=>value+1));
});
test("Measurements honour cancellation and a single worker is sequential",async()=>{
  const controller=new AbortController(),reason=new Error("cancelled");let started=0,active=0,maximum=0;
  let failure:unknown;try{await measurePrintItemsInOrderV1([1,2,3,4,5,6],controller.signal,1,async value=>{started++;active++;maximum=Math.max(maximum,active);await Promise.resolve();active--;if(value===2)controller.abort(reason);return value;});}catch(error){failure=error;}
  expect(failure).toBe(reason);expect(maximum).toBe(1);expect(started).toBeLessThan(6);
});
