/** 🪜️ Contract: after the owner pass every phase of a selected read and every wait inside it is a declared step: while it runs the progress line names it in both languages with the seconds it ran and its bound, and keeps naming it past the bound; a source file cannot use a step the schema does not declare
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
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
   expect(row.phase).toContain("(elapsed seconds of expected seconds)");expect(row.phase).toContain("(verstrichene Sekunden von erwarteten Sekunden)");
  },{round:2});
  expect(describeSliceReadStepV1()!.phase).toContain("reading the owners (round 1)");
 });
 expect(describeSliceReadStepV1()).toBeUndefined();
});

test("a step past its bound is still named, with the note, and a step that throws is removed",async()=>{
 await expect(sliceReadStepV1("seal",async()=>{const row=describeSliceReadStepV1(performance.now()+700000)!;expect(row.phase).toContain("beyond the expected 600 s");expect(row.phase).toContain("über den erwarteten 600 s");throw Error("x");})).rejects.toThrow("x");
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
