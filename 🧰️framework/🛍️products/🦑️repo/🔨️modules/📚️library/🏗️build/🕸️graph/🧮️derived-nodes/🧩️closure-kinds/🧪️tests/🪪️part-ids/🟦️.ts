/** 🪪️ Law of the closure parts decided from row ids: a settle over a stored manifest decides every part from the dictionary ids of its rows (no part is read, each distinct row is probed once), reports `decisions N of M` and stops at cancellation. The expectations are computed here from the generated data, not from the settler. */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{CLOSURE_SETTLE_V1,createClosureSettlerV1}from"../../🟨️.mjs";
import{createNodeStoreV1}from"../../../💾️store/🟨️.mjs";
import{createGraphPhaseClockV1}from"../../../../⏱️phases/🟨️.mjs";
import{createGenerationProgressV1}from"../../../../🕰️generations/🔌️load/🟨️.mjs";

type Row={kind:string;path:string;fact:string};
const PARTS=90,SPAN=240,STEP=80,CLOSURES=400;
const rowAt=(at:number):Row=>({kind:at%5===0?"exists":"lstat",path:"/ws/tree/d"+Math.floor(at/40)+"/f"+at+".ts",fact:at%3===0?"file":"directory"});
const partRows=(part:number)=>Array.from({length:SPAN},(_all,at)=>rowAt(part*STEP+at));
const partsOf=(at:number)=>[(at*7)%PARTS,(at*13+5)%PARTS];
const names=Array.from({length:CLOSURES},(_all,at)=>"q"+String(at).padStart(4,"0"));
const text=(row:Row)=>JSON.stringify([row.kind,row.path,row.fact]);
const usedParts=new Set(names.flatMap((_name,at)=>partsOf(at)));
const distinctRows=new Set([...usedParts].flatMap(part=>partRows(part).map(text)));

const harness=(root:string,options:{partIds?:false|{directory:string};stale?:Set<string>;abortAfterRows?:number;controller?:AbortController}={})=>{
 const probes=new Map<string,number>(),derived:string[]=[],reads:string[]=[],progress:{phase:string;done:number;total:number}[]=[],base=createNodeStoreV1({root});
 const store={...base,get:async(id:string)=>{if(id.startsWith("closure-part\0"))reads.push(id);return base.get(id);},put:base.put,flush:base.flush};
 const settler=createClosureSettlerV1({store:store as never,limits:{...CLOSURE_SETTLE_V1,deriveConcurrency:4},farm:false,...options.partIds===undefined?{partIds:{directory:root+"-ids"}}:{partIds:options.partIds}} as never);
 let probed=0;
 const settle=()=>settler.settle({pseudoRows:new Map(names.map(name=>[name,{kind:"derived",path:name,fact:"old"}])),derive:async(row:{path:string})=>{derived.push(row.path);const at=Number(row.path.slice(1));return{fact:"old",parts:partsOf(at).map(partRows)};},hold:async(rows:Row[])=>{for(const row of rows){probes.set(text(row),(probes.get(text(row))??0)+1);probed++;}if(options.abortAfterRows!==undefined&&probed>=options.abortAfterRows)options.controller?.abort();return rows.every(row=>!options.stale?.has(text(row)));},signal:options.controller?.signal,progress:(row:{phase:string;done:number;total:number})=>{progress.push(row);}} as never) as Promise<any>;
 return{settle,probes,derived,reads,progress,settler};
};

const scratch=()=>realpathSync(mkdtempSync(join(tmpdir(),"part-ids-")));

/** 🧪️ A later settler over the stored nodes decides each part from its ids: no part record is read, every distinct row is probed once, the verdicts equal those of the part-by-part path. */
test("parts are decided from row ids, each distinct row probed once and no part read",async()=>{
 const root=scratch();
 try{
  const cold=harness(root);const first=await cold.settle();
  expect(first.failed).toBe(0);expect(cold.derived.length).toBe(CLOSURES);
  await cold.settler.markIndexed({held:first.held});await cold.settler.release?.();
  const warm=harness(root);const second=await warm.settle();
  expect(second.failed).toBe(0);expect(warm.derived,"nothing is walked while every row holds").toEqual([]);
  expect(warm.reads,"no closure part record is read").toEqual([]);
  expect(warm.probes.size).toBe(distinctRows.size);
  expect([...warm.probes.values()].every(count=>count===1),"each distinct row is probed once").toBe(true);
  expect(second.parts.byIds).toBe(usedParts.size);expect(second.parts.loaded).toBe(0);
 }finally{rmSync(root,{recursive:true,force:true});rmSync(root+"-ids",{recursive:true,force:true});}
},120000);

/** 🧪️ A row that no longer holds stales exactly the closures whose parts name it, the same set as without the index. */
test("a stale row stales exactly the closures that name it, with and without the index",async()=>{
 const stale=new Set([text(rowAt(2*STEP+7))]);
 const expected=names.filter((_name,at)=>partsOf(at).some(part=>partRows(part).some(row=>stale.has(text(row))))).sort();
 expect(expected.length).toBeGreaterThan(0);expect(expected.length).toBeLessThan(CLOSURES);
 const runs:string[][]=[];
 for(const indexed of[true,false]){
  const root=scratch();
  try{
   const cold=harness(root,indexed?{}:{partIds:false});const first=await cold.settle();
   await cold.settler.markIndexed({held:first.held});await cold.settler.release?.();
   const warm=harness(root,{...indexed?{}:{partIds:false},stale});await warm.settle();
   runs.push([...warm.derived].sort());
  }finally{rmSync(root,{recursive:true,force:true});rmSync(root+"-ids",{recursive:true,force:true});}
 }
 expect(runs[0]).toEqual(expected);expect(runs[1]).toEqual(expected);
},120000);

/** 🧪️ The settle names its work: `decisions N of M` rises to every part asked and ends at it. */
test("the part decisions report decisions counters",async()=>{
 const root=scratch();
 try{
  const cold=harness(root);const first=await cold.settle();
  await cold.settler.markIndexed({held:first.held});await cold.settler.release?.();
  const warm=harness(root);await warm.settle();
  const lines=warm.progress.filter(row=>row.phase==="decisions");
  expect(lines.length).toBeGreaterThan(0);
  for(let at=1;at<lines.length;at++)expect(lines[at]!.done).toBeGreaterThanOrEqual(lines[at-1]!.done);
  expect(lines.every(row=>row.done<=row.total)).toBe(true);
  expect(lines.at(-1)).toEqual({phase:"decisions",done:usedParts.size,total:usedParts.size});
 }finally{rmSync(root,{recursive:true,force:true});rmSync(root+"-ids",{recursive:true,force:true});}
},120000);

/** 🧪️ Cancelling in the middle of the part decisions stops the settle before the rest of the rows is probed. */
test("cancellation stops the part decisions",async()=>{
 const root=scratch();
 try{
  const cold=harness(root);const first=await cold.settle();
  await cold.settler.markIndexed({held:first.held});await cold.settler.release?.();
  const controller=new AbortController(),warm=harness(root,{controller,abortAfterRows:distinctRows.size>>3});
  await expect(warm.settle()).rejects.toThrow();
  expect(warm.probes.size).toBeLessThan(distinctRows.size>>1);
 }finally{rmSync(root,{recursive:true,force:true});rmSync(root+"-ids",{recursive:true,force:true});}
},120000);

/** 🧪️ Every phase a settle reports is accepted by the graph phase counters of a generation load (a word of lower-case letters, begun once with a finite total): a name the counters refuse ends the whole slice read with "Closed graph phase and finite total required" (g1-m10, `flush parts`, reference 3a9fba88). The events of a cold settle, a warm settle and a settle with every closure stale go through the real generation progress and a real clock. */
test("every phase of a settle is accepted by the graph phase counters",async()=>{
 const root=scratch();
 try{
  const cold=harness(root);const first=await cold.settle();
  await cold.settler.markIndexed({held:first.held});await cold.settler.release?.();
  const warm=harness(root);await warm.settle();
  const stale=harness(root,{stale:new Set([text(rowAt(2*STEP+7)),text(rowAt(10*STEP+3))])});await stale.settle();
  const events=[...cold.progress,...warm.progress,...stale.progress],phases=new Set(events.map(row=>row.phase));
  expect([...phases].every(phase=>/^[a-z]+$/u.test(phase)),[...phases].join(",")).toBe(true);
  expect(phases.has("decisions")).toBe(true);
  for(const run of [cold,warm,stale]){
   const published:string[]=[],report=createGenerationProgressV1(createGraphPhaseClockV1({publish:resource=>{published.push(resource);},minimumMilliseconds:0}) as never);
   for(const event of run.progress)await report(event);
   expect(published.length).toBeGreaterThan(0);
  }
 }finally{rmSync(root,{recursive:true,force:true});rmSync(root+"-ids",{recursive:true,force:true});}
},120000);
