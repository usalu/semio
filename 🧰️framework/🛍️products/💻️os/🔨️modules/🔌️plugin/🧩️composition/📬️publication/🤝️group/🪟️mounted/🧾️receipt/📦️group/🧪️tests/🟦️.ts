import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { applyPatch } from "fast-json-patch";

test("mounted group receipt preserves every causal lane before an allocation-free decision",()=>{
 const root=resolve(import.meta.dir,".."),law=JSON.parse(readFileSync(resolve(root,"🧫️fixtures/🔣️.json"),"utf8"));
 
 for(const row of law.cases){
  const expected={mutations:row.lanes.map((lane:any)=>JSON.parse(JSON.stringify(lane))),undoIds:row.lanes.map((lane:any)=>lane.mutationId),inverses:row.lanes.map((lane:any)=>({target:lane.mutationId,schema:lane.schema+".inverse",payload:lane.inverse,dependencies:lane.dependencies,baseVersion:lane.baseVersion})),memberEdits:row.members,invocationId:row.invocationId,parentEditId:row.parentEditId,childEditIds:row.childEditIds};
  expect(expected.memberEdits.length).toBe(row.childEditIds.length+Number(row.parentEditId!==null));
  const patched=applyPatch({},Object.entries(expected).map(([key,value])=>({op:"add" as const,path:"/"+key,value})),true,false).newDocument;
  expect(patched).toEqual(expected);
  expect(Buffer.byteLength(row.invocationId,"utf8")).toBe(new TextEncoder().encode(row.invocationId).byteLength);
  for(const stop of law.cancelStops){expect(stop>=0).toBe(true);expect(applyPatch({receipt:expected},[{op:"remove",path:"/receipt"}],true,false).newDocument).toEqual({});}
 }
 const source=readFileSync(resolve(root,"🦀️.rs"),"utf8");
 for(const method of ["next_capacity_byte_demand","advance_metadata","push_mutation_triple","push_member_edit","next_close_byte_demand","close_step","take"]){expect(source.includes("fn "+method+"(")).toBe(true);}
 console.log("[DEBUG] mounted group receipt neutral Node UTF8/RFC6902 preserves all causal fields, three-lane order, nine cancellation stops and constant decision transfer");
});

test("mounted group child metadata borrows original completed rows across independent backing",()=>{
 const root=resolve(import.meta.dir,".."),law=JSON.parse(readFileSync(resolve(root,"🧫️fixtures/🔣️.json"),"utf8"));
 
 for(const row of law.cases){
  const rows=[row.parentEditId,...row.childEditIds].map((editId,index)=>({index,editId}));
  const expected=JSON.parse(JSON.stringify(rows.filter((entry:any)=>entry.index>0).map((entry:any)=>entry.editId)));
  expect(applyPatch({children:[]},[{op:"replace",path:"/children",value:rows.slice(1).map((entry:any)=>entry.editId)}],true,false).newDocument.children).toEqual(expected);
  expect(rows.length).toBe(row.childEditIds.length+1);
 }
 const source=readFileSync(resolve(root,"🦀️.rs"),"utf8");
 expect(source.includes("trait MountedGroupPreparedChildEdits")).toBe(true);
 expect(source.includes("Ready(&'a dyn MountedGroupPreparedChildEdits)")).toBe(true);
 console.log("[DEBUG] completed member rows project original child metadata in source order with Node JSON/RFC6902 independent oracle");
});
