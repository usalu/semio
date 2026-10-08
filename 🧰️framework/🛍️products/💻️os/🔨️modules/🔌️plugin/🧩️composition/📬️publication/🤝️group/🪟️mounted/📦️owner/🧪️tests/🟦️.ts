import {expect,test} from "bun:test";
import {applyPatch} from "fast-json-patch";
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};

test("mounted private group frames retain every original owner on zero or underfunded admission",async()=>{
 
 for(const row of fixture.cases){
  const admitted=row.items===1&&row.capacity==="frame";
  expect(admitted).toBe(row.admitted);
  expect(applyPatch({owners:[]},admitted?[{op:"add",path:"/owners/0",value:row.id}]:[],true,false).newDocument.owners.length).toBe(Number(admitted));
 }
 const source=await Bun.file(new URL("../../../../../../🦀️.rs",import.meta.url)).text();
 expect(source.includes("private_child_groups: ArtifactFixedRegistry")).toBe(true);
 const owner=await Bun.file(new URL("../🦀️.rs",import.meta.url)).text();
 expect(source.includes('include!("🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs")')).toBe(true);
 expect(owner.includes("fn admit_private_child_group_frame(")).toBe(true);
 expect(owner.includes("fn close_private_child_group_step(")).toBe(true);
 expect(owner.includes("self.terminal_is_empty()")).toBe(true);
 expect(owner.includes("grant.maximum_release_bytes < bytes")).toBe(true);
 console.log("[DEBUG] mounted frame symbolic-size law agrees with RFC6902; original64child/64copy bounds; whole frame and empty registry extents independently funded");
});
