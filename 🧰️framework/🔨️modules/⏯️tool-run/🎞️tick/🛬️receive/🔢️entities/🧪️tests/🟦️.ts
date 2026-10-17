/** 🔢️ The original little-endian entity field enters its final native column under the captured policy. */
import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import {existsSync,readFileSync} from "node:fs";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

test("original Tick entity bytes preserve declared scalars, final cells and cumulative native receiving",()=>{
 const validate=new Ajv({strict:true}).compile(schema);
 expect(validate(fixture)).toBe(true);
 expect(validate({...fixture,grant:[1,18,65536,262144,64]})).toBe(false);
 const db=new Database(":memory:");
 try{expect(db.query("SELECT ?1 + ?2 AS total").get(fixture.allocationCeiling.sourceBytes,fixture.allocationCeiling.frameBytes)).toEqual({total:fixture.allocationCeiling.maximumTotalAllocationBytes});for(const row of fixture.cases){
  const bytes=Uint8Array.from(row.hex.match(/../g)??[],octet=>parseInt(octet,16));
  expect(db.query("SELECT lower(hex(?)) AS encoded").get(bytes)).toEqual({encoded:row.hex});
  const view=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength);
  expect(Array.from({length:bytes.byteLength/8},(_,at)=>Number(view.getBigUint64(at*8,true)))).toEqual(row.entities);
  for(const cut of row.interruptAfter){expect(db.query("SELECT ? / 8 AS complete, ? % 8 AS retained").get(cut,cut)).toEqual({complete:Math.floor(cut/8),retained:cut%8})}
 }}finally{db.close()}
 const path=new URL("../🦀️.rs",import.meta.url);
 expect(existsSync(path)).toBe(true);
 const source=readFileSync(path,"utf8");
 for(const marker of["ToolRunEntityColumnReceiving","original_range()","allocated_bytes()","maximum_total_allocation_bytes","maximum_items","maximum_copy_bytes","maximum_capacity_bytes","maximum_release_bytes","maximum_depth","next_reserve_depth_demand()","reserve_capacity_one_funded(self.entities,g)","terminal_is_empty()"])expect(source).toContain(marker);
 for(const cold of["ToolRunTick::decode","DecodeOptions::default","collect::<Vec","to_vec()","root_now()","StepContext::new"])expect(source).not.toContain(cold);
 const native=readFileSync(new URL("../../🏗️native/🦀️.rs",import.meta.url),"utf8");
 expect(native).toContain("source.allocated_bytes().checked_add(Self::birth_bytes())");
 expect(native).toContain("if self.entity_issuer||self.progress_issuer{return Default::default()}");
 expect(native).toContain("column.truncate_retired_last()");
 expect(native).toContain("column.release_empty_page(g.maximum_release_bytes)");
});
