/** 🏁️ Contract: when the owners of a read are done, the progress line says so with the final count: the last value the timer happened to see (588 of 590) is not what a worker shows for the rest of the read, which goes on through admission, assembly and repair rounds with no owner source left. */
import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
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
  const step=(age:number)=>`Reading the project graph slice (${fixture.receiptSeconds+age} s, expected 3600 s): ${phase}.`;
  const finalLine=`Reading the project graph slice (22 s): owners done ${final[0]!.done} of ${final[0]!.total}.`;
  const watch=readyStepWatchV1();
  const quiet=fixture.quietMilliseconds;
  watch.observe([step(1)],0);
  expect(watch.observe([step(2),finalLine],quiet).overdue).toBe(false);
  expect(watch.observe([step(3),finalLine],2*quiet).overdue).toBe(true);
 },{round:1});}finally{remove();stop();}
});

test("certified owner progress distinguishes two passes with identical numeric counters",async()=>{
 const {readyStepWatchV1,readyStallTrackerV1}=await import("../../../../🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🟦️.ts");
 const db=new Database(":memory:"),told:{phase:string;done:number;total:number}[]=[];
 const row=fixture.repeatedPass;
 let ledger={done:row.counters[0]!,total:row.total},pass=row.first;
 const remove=registerSliceReportSourceV1("owners",()=>({pass,ledger})),stop=startSliceProgressV1({emit:event=>told.push(event),milliseconds:5});
 const change=async(next:number,done:number)=>{pass=next;ledger={done,total:row.total};await pause(35);return told.at(-1)!;};
 const render=(value:typeof told[number],outer:number)=>`Reading the project graph slice (${outer} s): ${value.phase} ${value.done} of ${value.total}.`;
 const step=`Reading the project graph slice (${row.elapsed} s, expected 3600 s): reading the owners (round 2) / Lese die Projekte (Runde 2).`;
 try{
  const first=await change(row.first,row.counters[0]!),second=await change(row.first,row.counters[1]!);
  const old=[render(first,row.elapsed),render(second,row.elapsed+1)];
  const watch=readyStepWatchV1(),stall=readyStallTrackerV1();
  watch.observe([...old,step],0);expect(stall.observe(old)).toBe(true);
  const changed=await change(row.second,row.counters[0]!),advanced=await change(row.second,row.counters[1]!);
  const current=[render(changed,row.elapsed+2),render(advanced,row.elapsed+3)];
  const sql=db.query("select count(distinct pass || ':' || counter) as count from (select ?1 pass, ?3 counter union all select ?1, ?4 union all select ?2, ?3 union all select ?2, ?4)");
  expect((sql.get(row.first,row.second,...row.counters) as {count:number}).count).toBe(4);
  expect(stall.observe(current)).toBe(true);
  const quiet=fixture.quietMilliseconds;
  expect(watch.observe([step,...current],quiet).overdue).toBe(false);
  expect(stall.observe([render(advanced,row.elapsed+4)])).toBe(false);
  expect(watch.observe([step,render(advanced,row.elapsed+4)],2*quiet).overdue).toBe(true);
  remove();await pause(35);
  expect(told.at(-1)).toEqual({phase:"owners done (pass 2)",done:row.counters[1],total:row.total});
 }finally{remove();stop();db.close();}
});
