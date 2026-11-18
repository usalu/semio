/** 🏁️ Contract: when the owners of a read are done, the progress line says so with the final count: the last value the timer happened to see (588 of 590) is not what a worker shows for the rest of the read, which goes on through admission, assembly and repair rounds with no owner source left. */
import{expect,test}from"bun:test";
import{registerSliceReportSourceV1,startSliceProgressV1}from"../../🟨️.mjs";

const pause=(milliseconds:number)=>new Promise(resolve=>setTimeout(resolve,milliseconds));

test("the final count of the owners is told when their source is removed between two ticks",async()=>{
 let ledger={done:588,total:590};const told:{phase:string;done:number;total:number}[]=[];
 const remove=registerSliceReportSourceV1("owners",()=>({ledger}));
 const stop=startSliceProgressV1({emit:event=>told.push(event),milliseconds:10});
 try{
  await pause(50);expect(told.at(-1)).toEqual({phase:"taken",done:588,total:590});
  ledger={done:590,total:590};remove();await pause(50);
  expect(told.at(-1)).toEqual({phase:"owners done",done:590,total:590});
  const count=told.length;await pause(40);expect(told.length,"told once").toBe(count);
 }finally{stop();}
});

test("a new pass of the owners is told as taken again after the final line",async()=>{
 let ledger={done:590,total:590};const told:{phase:string;done:number;total:number}[]=[];
 const first=registerSliceReportSourceV1("owners",()=>({ledger}));
 const stop=startSliceProgressV1({emit:event=>told.push(event),milliseconds:10});
 try{
  await pause(40);first();await pause(40);
  ledger={done:4,total:590};const second=registerSliceReportSourceV1("owners",()=>({ledger}));await pause(40);
  expect(told.at(-1)).toEqual({phase:"taken",done:4,total:590});second();
 }finally{stop();}
});
