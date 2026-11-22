/** 🪜️ Contract: after the owner pass every phase of a selected read and every wait inside it is a declared step: while it runs the progress line names it in both languages with the seconds it ran and its bound, and keeps naming it past the bound; a source file cannot use a step the schema does not declare
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{Database}from"bun:sqlite";
import{readyStepOverdueSecondsV1,readyStepWatchV1}from"../../../../🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🟦️.ts";
import samples from"./🔣️.json"with{type:"json"};
import{registerSliceReportSourceV1,sliceReadStepV1,describeSliceReadStepV1,sliceReadMovedAtV1,sliceReadActiveV1,startSliceProgressV1}from"../../🟨️.mjs";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};

const pause=(milliseconds:number)=>new Promise(resolve=>setTimeout(resolve,milliseconds));
const SOURCES=["../../../../🕸️graph/📥️load/🟦️.ts","../../../../🕸️graph/📥️load/🤝️selected-exchange/🟦️.ts","../../../../🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"];

test("a running step is named in both languages with its seconds and its bound, the innermost first, and ends with the step",async()=>{
 expect(describeSliceReadStepV1()).toBeUndefined();
 await sliceReadStepV1("owners-round",async()=>{
  await sliceReadStepV1("facts-hold",async()=>{
   const row=describeSliceReadStepV1(performance.now()+5000)!;
   expect(row.phase).toContain("checking that the observed source facts still hold");expect(row.phase).toContain("Prüfe, ob die beobachteten Quellfakten");
   expect(row.done).toBe(5);expect(row.total).toBe(600);
   expect(row.phase).not.toContain("elapsed seconds");expect(row.phase).not.toContain("verstrichene Sekunden");
  },{round:2});
  expect(describeSliceReadStepV1()!.phase).toContain("reading the owners (round 1)");
 });
 expect(describeSliceReadStepV1()).toBeUndefined();
});

test("a step past its bound is still named, with the note, and a step that throws is removed",async()=>{
 await expect(sliceReadStepV1("seal",async()=>{const row=describeSliceReadStepV1(performance.now()+700000)!;expect(row.done).toBe(700);expect(row.phase).not.toContain("beyond the expected");throw Error("x");})).rejects.toThrow("x");
 expect(describeSliceReadStepV1()).toBeUndefined();
});

test("the progress heartbeat tells the step at every tick, the owners again after it ended, and an undeclared step is refused",async()=>{
 const told:{phase:string;done:number;total:number}[]=[],stop=startSliceProgressV1({emit:event=>told.push(event),milliseconds:10});
 try{
  await sliceReadStepV1("exchange-wait",async()=>{await pause(60);},{who:"g1-m02",boundMilliseconds:30000});
  expect(told.length).toBeGreaterThan(2);expect(told.every(event=>event.phase.includes("waiting for g1-m02 to seal this selection")&&event.total===30)).toBe(true);
 }finally{stop();}
 await expect(sliceReadStepV1("nothing-declared",async()=>1)).rejects.toThrow("Declared slice read step required");
});

test("progress is the moment a step began or ended or the owners counted more, and idle past the declared bound is not active",async()=>{
 let ledger={done:1,total:9,running:0};const remove=registerSliceReportSourceV1("owners",()=>({ledger}));
 try{
  const first=sliceReadMovedAtV1();await pause(15);expect(sliceReadMovedAtV1()).toBe(first);
  ledger={done:2,total:9,running:0};expect(sliceReadMovedAtV1()).toBeGreaterThan(first);
  expect(sliceReadActiveV1()).toBe(true);expect(sliceReadActiveV1(performance.now()+schema.definitions.Steps.const.idleMilliseconds+1000)).toBe(false);
 }finally{remove();}
});

test("every step used by the load, the exchange and the owner pass is declared with both languages and a bound",()=>{
 const declared=schema.definitions.Steps.const.steps as Record<string,{en:string;de:string;boundMilliseconds:number}>,used=new Set<string>();
 for(const file of SOURCES){const text=readFileSync(new URL(file,import.meta.url),"utf8");for(const match of text.matchAll(/(?:sliceReadStepV1|step)\("([a-z-]+)"/gu))used.add(match[1]!);}
 for(const id of used){expect(declared[id],id).toBeDefined();expect(declared[id]!.en.length).toBeGreaterThan(8);expect(declared[id]!.de.length).toBeGreaterThan(8);expect(declared[id]!.boundMilliseconds).toBeGreaterThan(0);}
 for(const id of["inventory","owners-round","facts-hold","seal","exchange-look","exchange-wait","exchange-install","exchange-publish","heavy-wait","orphan-sweep","receipt-check"])expect(used.has(id),id).toBe(true);
});

test("during the owner round the count is told while it moves and the step is told while it stands still",async()=>{
 let ledger={done:3,total:9,running:1};const told:string[]=[],remove=registerSliceReportSourceV1("owners",()=>({ledger})),stop=startSliceProgressV1({emit:event=>told.push(event.phase),milliseconds:10});
 try{
  await sliceReadStepV1("owners-round",async()=>{await pause(40);ledger={done:5,total:9,running:1};await pause(40);await pause(60);},{round:1});
  expect(told[0]).toBe("taken");expect(told).toContain("taken");
  expect(told.at(-1)).toContain("reading the owners (round 1)");
  expect(told.at(-1)).toContain("Lese die Projekte (Runde 1)");
 }finally{stop();remove();}
});

test("inventory reports new source observations while a repeated sample keeps the same progress",async()=>{
 let count=0;
 const remove=registerSliceReportSourceV1("caches",()=>({interned:count}));
 try{
  await sliceReadStepV1("inventory",async()=>{
   const english=readyStepWatchV1(),german=readyStepWatchV1(),db=new Database(":memory:");
   try{
    for(const [index,sample] of samples.entries()){
     count=sample.rows;
     const oracle=db.query("select max(json_extract(value,'$.rows')) as peak from json_each(?1) where key<=?2").get(JSON.stringify(samples),index);
     expect(oracle).toEqual({peak:sample.peak});
     const row=describeSliceReadStepV1()!;
     expect(row.phase).toContain(`${sample.peak} distinct source observations`);
     expect(row.phase).toContain(`${sample.peak} verschiedene Quellbeobachtungen`);
     const line=`Reading the project graph slice (${index} s, expected ${row.total} s): ${row.phase}.`;
     const translated=`Lese den Projektgraph-Ausschnitt (${index} s, erwartet ${row.total} s): ${row.phase}.`;
     for(const result of [english.observe([line],index*1000),german.observe([translated],index*1000)]){
      expect(result.step).toBeDefined();
      expect(result.quietMs).toBe(sample.quietMs);
     }
    }
   }finally{db.close();}
  });
 }finally{remove();}
});

test("live inventory aggregation emits completed checks in the existing step progress",async()=>{
 let count=0,phase="scoped-discovery";
 const events:{phase:string;done:number;total:number}[]=[],stop=startSliceProgressV1({emit:event=>events.push(event),milliseconds:10});
 const diagnostics=()=>{
  const discovery={phase:"scoped-discovery",completedOwners:count};
  return{active:phase==="scoped-discovery"?discovery:{phase},completed:phase==="scoped-discovery"?[]:[discovery]};
 };
 const next=async(index:number)=>{const until=performance.now()+1500;while(events.length<=index&&performance.now()<until)await pause(5);expect(events[index]).toBeDefined();return events[index]!;};
 try{
  await sliceReadStepV1("inventory",async()=>{
   expect((await next(0)).phase).not.toContain("completed owner checks");
   count=177;const first=await next(events.length);expect(first.total).toBe(600);expect(first.phase).toContain("177 completed owner checks");
   phase="memo-publish";const repeated=await next(events.length);expect(repeated.phase).toBe(first.phase);
  },{},diagnostics);
 }finally{stop();}
});
