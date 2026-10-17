import {expect,test} from "bun:test";
import {applyPatch} from "fast-json-patch";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

test("mounted private group frames retain every original owner on zero or underfunded admission",async()=>{
 const policy={type:"object",additionalProperties:false,required:["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes","maximumDepth"],properties:{maximumItems:{const:1},maximumCopyBytes:{const:64},maximumCapacityBytes:{const:262144},maximumReleaseBytes:{const:262144},maximumDepth:{const:64}}};
 expect(new Ajv({strict:true}).compile({type:"object",additionalProperties:false,required:["generation","clock","deadline","fuel","source"],properties:{generation:{const:1},clock:{const:1},deadline:{const:1000000},fuel:{const:1},source:policy}})(fixture.caller)).toBe(true);
 for(const row of fixture.cases){
  const admitted=row.items===1&&row.capacity==="frame";
  expect(admitted).toBe(row.admitted);
  expect(applyPatch({owners:[]},admitted?[{op:"add",path:"/owners/0",value:row.id}]:[],true,false).newDocument.owners.length).toBe(Number(admitted));
 }
 const source=await Bun.file(new URL("../../🦀️.rs",import.meta.url)).text();
 const owner=await Bun.file(new URL("../🦀️.rs",import.meta.url)).text();
 const instance=await Bun.file(new URL("../../../../../../🧵️instance/🪆️children/📥️staging/🤝️group/🦀️.rs",import.meta.url)).text();
 const mounted=await (await Promise.all(["🦀️.rs","📨️dispatch/🦀️.rs","🖼️render/🦀️.rs","🧷️command-routes/🦀️.rs","🧾️typed-operations/🦀️.rs","🔌️plugin-app-impl/🦀️.rs","🪆️children/📥️staging/🧱️components/🦀️.rs"].map(s=>Bun.file(new URL("../../../../../../🧵️instance/"+s,import.meta.url)).text()))).join("\n");
 expect(mounted.includes("private_child_groups: ArtifactFixedRegistry")).toBe(true);
 expect(source.includes('#[path = "📦️owner/🦀️.rs"]')).toBe(true);
 expect(instance.includes("fn admit_private_child_group_frame(")).toBe(true);
 expect(instance.includes("fn close_private_child_group_step<")).toBe(true);
 expect(owner.includes("self.terminal_is_empty()")).toBe(true);
 expect(instance.includes("grant.maximum_capacity_bytes<bytes")).toBe(true);
 expect(owner.includes("StepContextOwner")).toBe(false);
 expect(instance.includes("owner.close_step(&mut stores.store,&mut self.children,")).toBe(true);
});
