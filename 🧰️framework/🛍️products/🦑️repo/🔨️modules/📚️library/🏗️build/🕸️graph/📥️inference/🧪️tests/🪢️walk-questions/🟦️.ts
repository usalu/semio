/** 🪢️ Contract: the walk questions a stored closure entry names for its layers are one string each, however many layers and entries name them; the heap a store holds for them is bounded by the distinct questions, never by the layers times the entries. */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{heapStats}from"bun:jsc";
import{createClosureStoreV1}from"../../🧭️script-inputs/💾️entries/🟨️.mjs";

const QUESTIONS=40,LAYERS=300,ENTRIES=4,questions=Array.from({length:QUESTIONS},(_,at)=>JSON.stringify({walk:1,roots:["/w/"+at+"/"+"x".repeat(300)]}));
const strings=()=>{Bun.gc(true);return heapStats().objectTypeCounts.string??0;};

test("the questions of the layers of every loaded entry are shared strings",async()=>{
 const directory=mkdtempSync(join(realpathSync(tmpdir()),"walk-questions-"));
 try{
  const store=createClosureStoreV1({directory}),layers=Array.from({length:LAYERS},(_,at)=>[{kind:"file",path:"/w/"+at,fact:at}]),sidecar={facts:layers.map(()=>null),members:layers.map((_,at)=>questions.slice(0,1+at%QUESTIONS))};
  for(let at=0;at<ENTRIES;at++)await store.save("entry-"+at,{recipe:{fn:"probe",at},value:[],rows:[],layers,sidecar});
  const before=strings(),held=[];
  for(let at=0;at<ENTRIES;at++)held.push(await store.load("entry-"+at));
  const grown=strings()-before,named=held.reduce((sum,entry)=>sum+entry.sidecar.members.reduce((count:number,walks:string[]|null)=>count+(walks?.length??0),0),0);
  expect(held.every(entry=>entry?.sidecar.members.length===LAYERS)).toBe(true);
  expect(named).toBeGreaterThan(ENTRIES*LAYERS*10);
  expect(grown).toBeLessThan(named/2);
  expect(held[0].sidecar.members[LAYERS-1]).toEqual(questions.slice(0,1+(LAYERS-1)%QUESTIONS));
 }finally{rmSync(directory,{recursive:true,force:true});}
});
