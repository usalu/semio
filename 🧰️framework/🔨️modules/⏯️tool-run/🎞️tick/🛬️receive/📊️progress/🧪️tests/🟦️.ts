/** 📊️ The existing Progress wire enters a controlled final native slot without assembled payload movement. */
import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import {existsSync,readFileSync} from "node:fs";
import schema from "../🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";
import ticks from "../../../../🧪️tests/🧫️fixtures/🎞️ticks.json";
test("original Progress uses required scalar fields, exact10-byte counters and one controlled final native slot",()=>{
 const validate=new Ajv({strict:true}).compile(schema);
 expect(validate(law)).toBe(true);
 expect(validate({...law,grant:[1,18,65536,262144,64]})).toBe(false);
 expect(validate({...law,requiredFields:[1,2,3,4,5,9]})).toBe(false);
 const db=new Database(":memory:");
 try{
  const ceiling=law.allocationCeiling;
  expect(db.query("SELECT (?1 + ?2) = ?3 AS existing, (?1 + ?2 + ?4) > ?3 AS denied").get(ceiling.sourceBytes,ceiling.frameBytes,ceiling.maximumTotalAllocationBytes,ceiling.progressBytes)).toEqual({existing:1,denied:1});
  for(const row of law.cases){
  const bytes=Buffer.from(row.counterHex,"hex");
  expect(db.query("SELECT length(?1) / ?2 AS counters, length(?1) % ?2 AS retained").get(bytes,law.counterBytes)).toEqual({counters:row.counters.length,retained:0});
  expect(Array.from({length:bytes.length/10},(_,index)=>({counter:bytes.readUInt16LE(index*10),value:Number(bytes.readBigUInt64LE(index*10+2))}))).toEqual(row.counters);
  expect(Math.fround(Buffer.from(row.unitsHex,"hex").readDoubleLE())).toBe(row.unitsPerSecond);
  expect(db.query("SELECT ? AS rate, ? AS fields").get(row.unitsPerSecond,law.requiredFields.reduce((mask,id)=>mask|(1<<(id-1)),0))).toEqual({rate:row.unitsPerSecond,fields:415});
  expect(db.query("SELECT 0 AS copied, 0 AS capacity, 0 AS released").get()).toEqual({copied:0,capacity:0,released:0});
 }}finally{db.close()}
 const canonical=ticks.ticks[0]!.tick.progress!;
 const full=law.cases[1]!;
 const canonicalRow=[canonical.sequence,1,canonical.stage,canonical.completed,canonical.total,canonical.counters,canonical.unitsPerSecond,canonical.conflicts,canonical.steps.length];
 expect([full.sequence,full.state,full.stage,full.completed,full.total,full.counters,full.unitsPerSecond,full.conflicts,full.stepCount]).toEqual<typeof canonicalRow>(canonicalRow);
 const path=new URL("../🦀️.rs",import.meta.url);
 expect(existsSync(path)).toBe(true);
 const source=readFileSync(path,"utf8");
 for(const proof of["ToolRunProgressReceiving","MaybeUninit","original_range()","allocated_bytes()","maximum_total_allocation_bytes","take_receipt","terminal_is_empty()","maximum_copy_bytes","maximum_capacity_bytes","maximum_release_bytes","maximum_depth"])expect(source).toContain(proof);
 for(const cold of["progress_from_record(","RecordValue","DecodeOptions::default","collect::<Vec","to_vec()","root_now()","StepContext::new"])expect(source).not.toContain(cold);
 const child=readFileSync(new URL("../../📝️step/🦀️.rs",import.meta.url),"utf8");
 const start=child.indexOf("pub fn allocated_bytes(&self)");
 expect(start).toBeGreaterThanOrEqual(0);
 const quote=child.slice(start,child.indexOf("pub fn ",start+8));
 expect(quote).toContain("self.step.as_ref()");
 expect(quote).toContain("step.args.capacity()");
 for(const mutation of["take()","reserve(","new(","charge("])expect(quote).not.toContain(mutation);
 expect(source).toContain("ToolRunStepReceiving::allocated_bytes");
 expect(ticks.ticks[0]!.hex).toContain(full.bodyHex.slice(2));
 expect(Buffer.from(full.bodyHex,"hex").indexOf(Buffer.from("0a0c010d","hex"))).toBeGreaterThan(0);
 expect(source).toContain("10=>12");
});
