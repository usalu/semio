/** 💾️ The durable closure store keeps the entries and layers of a whole graph: a store maximum below what the graph needs makes the oldest records of every run vanish while the project records still name them, so a resumed run re-infers the projects it had already finished. */
import { expect,test } from "bun:test";
import { mkdtempSync,readdirSync,realpathSync,rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { INFERENCE_LIMITS_V1 } from "../../🧬️schema/🟨️.mjs";
import { createClosureStoreV1 } from "../💾️entries/🟨️.mjs";

const PROJECTS=4754,ENTRIES_PER_PROJECT=12,LAYERS_PER_PROJECT=48;

test("the declared store maxima cover every entry and layer measured for the projects of the whole graph",()=>{
 expect(INFERENCE_LIMITS_V1.closureStoreMaximumEntries).toBeGreaterThanOrEqual(PROJECTS*ENTRIES_PER_PROJECT);
 expect(INFERENCE_LIMITS_V1.closureStoreMaximumLayers).toBeGreaterThanOrEqual(PROJECTS*LAYERS_PER_PROJECT);
});

test("a later process within the maxima keeps every earlier entry and layer, with a smaller maximum it removes the oldest",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"closure-store-limits-"));
 try{
  for(const [name,limits,kept] of [["within",INFERENCE_LIMITS_V1,6],["below",{...INFERENCE_LIMITS_V1,closureStoreMaximumEntries:3,closureStoreMaximumLayers:3},3]] as const){
   const directory=join(root,name),earlier=createClosureStoreV1({directory});
   for(let index=0;index<5;index++){await earlier.save("entry-"+name+index,{recipe:{fn:"fixture"},value:[index],rows:[],layers:[[{kind:"file",path:"/layer/"+name+index,fact:{size:index}}]]});await new Promise(resolve=>setTimeout(resolve,5));}
   const later=createClosureStoreV1({directory,limits});
   await later.save("entry-"+name+"later",{recipe:{fn:"fixture"},value:[9],rows:[],layers:[[{kind:"file",path:"/layer/"+name+"later",fact:{size:9}}]]});await later.settled();
   expect(readdirSync(join(directory,"e")).length,name+" entries").toBe(kept);
   expect(readdirSync(join(directory,"l")).length,name+" layers").toBe(kept);
  }
 }finally{rmSync(root,{recursive:true,force:true});}
});
