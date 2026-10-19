import {test,expect} from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import {applyPatch} from "fast-json-patch";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {IndexedEditPlan} from "../../🟦️.ts";
import {IndexedCollectionCommit} from "../🟦️.ts";

test("indexed collection transfer matches independent JSON Patch and preserves exact survivor objects",()=>{
 expect(new Ajv2020({strict:true}).compile(schema)(fixture)).toBe(true);
 for(const law of fixture.cases){
  const plan=new IndexedEditPlan(law.base.length,law.removed,[],law.added.map(row=>row.index));while(!plan.settled)plan.step();expect(plan.ready).toBe(true);
  const patches=[...[...law.removed].sort((a,b)=>b-a).map(index=>({op:"remove"as const,path:"/"+index})),...[...law.added].sort((a,b)=>a.index-b.index).map(row=>({op:"add"as const,path:"/"+row.index,value:row.item}))];
  expect(applyPatch([...law.base],patches,true).newDocument).toEqual(law.expected);
  for(const cut of [...fixture.cancelCuts,null]){
   const base=law.base.map(text=>({text})),originals=[...base],target={value:base},commit=new IndexedCollectionCommit<{text:string}>(),returned:Array<{text:string}>=[];let turns=0;
   while(!commit.complete&&(cut===null||turns<cut)){commit.step(plan,target,ordinal=>({text:law.added[ordinal]!.item}),value=>returned.push(value));turns++;expect(turns).toBeLessThan(100);}
   if(commit.complete){expect(target.value.map(row=>row.text)).toEqual(law.expected);for(const [index,original]of originals.entries())if(!law.removed.includes(index))expect(target.value.includes(original)).toBe(true);}
   commit.beginClose();while(!commit.terminalEmpty)commit.closeStep(value=>returned.push(value));if(!commit.complete)for(const row of target.value)returned.push(row);
   for(const original of originals)expect((returned.filter(row=>row===original).length)+(commit.complete?target.value.filter(row=>row===original).length:0)).toBe(1);
  }
 }
 console.log("[DEBUG] five neutral collection streams agree with JSON Patch; original survivor identities and cancellation ownership are returned exactly once at seven frontiers");
});
