import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{heapStats}from"bun:jsc";
import{createClosureSettlerV1,CLOSURE_SETTLE_V1}from"../../../../🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs";
import{createNodeStoreV1}from"../../../../🧮️derived-nodes/💾️store/🟨️.mjs";

/** 🧮️ Settles `questions` generated questions of `perQuestion` distinct supporting rows each, in a fresh process so no earlier test shapes the heap, and prints the heap (MB) after every chunk (`mode` chunks, through a node store), the growth after the result is dropped (`mode` growth) or the heap still taken while the held supports of unchanged answers are kept (`mode` held, rows of one pool of 30000 shared by every question, each question reading its own copies). */
const[mode,questions,perQuestion,chunk]=process.argv.slice(2).map((value,at)=>at===0?value:Number(value));
const used=()=>(heapStats().heapSize+heapStats().extraMemorySize)/1048576,tick=async()=>{for(let round=0;round<4;round++){await new Promise(resolve=>setTimeout(resolve,10));Bun.gc(true);}};
let armed=false;
const root=mkdtempSync(join(tmpdir(),"semio-heap-child-")),after=[];
const pseudoRows=new Map();for(let at=0;at<questions;at++)pseudoRows.set("q"+String(at).padStart(5,"0"),{kind:"derived",path:JSON.stringify({walk:1,question:at}),fact:mode==="held"?"new"+at:"old"});
const derive=async row=>{const at=Number(JSON.parse(row.path).question);return{fact:"new"+at,rows:Array.from({length:perQuestion},(_,item)=>({kind:"file",path:"/w/"+"🧰️framework/".repeat(3)+"d".repeat(40)+((at*7+item*13)%30000)+".ts",fact:mode==="held"?"h":"h"+(mode==="chunks"?at+"-"+item:(at+item)%5)}))};};
const run=async()=>{
 const settler=createClosureSettlerV1(mode==="chunks"||mode==="held"?{store:createNodeStoreV1({root}),limits:{...CLOSURE_SETTLE_V1,settleChunkClosures:chunk}}:{});
 const settled=await settler.settle({pseudoRows,derive,hold:async()=>true,progress:async event=>{if(event.phase==="flush"&&event.done===1)armed=true;else if(mode==="chunks"&&armed&&event.phase==="verification"){armed=false;Bun.gc(true);after.push(used());}}});
 return{outcome:{changed:settled.changed.size,answers:settled.answers.size},held:settled.held.size,keep:settled};
};
await tick();const before=used();
const{outcome,held,keep}=await run();await tick();const retained=used()-before;void keep;
rmSync(root,{recursive:true,force:true});
console.log(JSON.stringify({outcome,held,retained,growth:used()-before,after}));
process.exit(0);
