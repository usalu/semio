/** 🏁️ Contract: when the owners of a read are done, the progress line says so with the final count: the last value the timer happened to see (588 of 590) is not what a worker shows for the rest of the read, which goes on through admission, assembly and repair rounds with no owner source left. */
import{expect,test}from"bun:test";
import{registerSliceReportSourceV1,sliceReadStepV1,startSliceProgressV1}from"../../🟨️.mjs";
import fixture from"./🔣️.json"with{type:"json"};

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

test("a newly settled owner count reaches Ready while the enclosing owner round drains",async()=>{
 const {readyStepWatchV1}=await import("../../../../🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🟦️.ts");
 const told:{phase:string;done:number;total:number}[]=[];
 let ledger={done:fixture.reported,total:fixture.total};
 const remove=registerSliceReportSourceV1("owners",()=>({ledger}));
 const stop=startSliceProgressV1({emit:event=>told.push(event),milliseconds:10});
 try{await sliceReadStepV1("owners-round",async()=>{
  await pause(60);expect(told[0]).toEqual({phase:"taken",done:fixture.reported,total:fixture.total});
  ledger={...ledger,done:fixture.settled};remove();
  await pause(60);
  const final=told.filter(event=>event.phase==="owners done");
  expect(final).toEqual([{phase:"owners done",done:fixture.settled,total:fixture.total}]);
  expect(told.at(-1)?.phase).toContain("reading the owners (round 1)");
  const phase=told.at(-1)!.phase;
  const step=(age:number)=>`Reading the project graph slice (${age} s): ${phase} ${fixture.receiptSeconds+age} of 3600.`;
  const finalLine=`Reading the project graph slice (22 s): owners done ${final[0]!.done} of ${final[0]!.total}.`;
  const watch=readyStepWatchV1();
  const quiet=fixture.quietMilliseconds;
  watch.observe([step(1)],0);
  expect(watch.observe([step(2),finalLine],quiet).overdue).toBe(false);
  expect(watch.observe([step(3),finalLine],2*quiet).overdue).toBe(true);
 },{round:1});}finally{remove();stop();}
});
