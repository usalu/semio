import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import globalsClose from "./🧫️fixtures/globals-close.json";

test("Core globals are scalar values within one backing", () => {
 const { globals, value, returnItems } = globalsClose;
 const declaration = [0x7f, 0, 0x41, value, 0x0b];
 const core = new Uint8Array([
  0, 97, 115, 109, 1, 0, 0, 0,
  6, 1 + declaration.length * globals, globals,
  ...Array.from({ length: globals }, () => declaration).flat(),
  7, 9, 1, 5, 118, 97, 108, 117, 101, 3, 0,
 ]);
 const instance = new WebAssembly.Instance(new WebAssembly.Module(core));
 expect((instance.exports.value as WebAssembly.Global).value).toBe(value);
 const db = new Database(":memory:");
 try {
  const row = db.query("SELECT COUNT(DISTINCT backing) AS items FROM json_each(?) CROSS JOIN (SELECT 'globals' AS backing)")
   .get(JSON.stringify(Array(globals).fill(value)));
  expect(row).toEqual({ items: returnItems });
 } finally {
  db.close();
 }
});
/** 🏗️ Each actual resource birth requires its unchanged incoming capacity, cancellation and deadline. */
test("original Core opening preserves denied capacities and phases copy independently",async()=>{
 const fixture=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json();expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{for(const cut of fixture.cuts)expect(Boolean((db.query("SELECT (?1>=?2 AND ?3=0 AND ?4=0) AS admitted").get(cut.capacity,cut.required,Number(cut.cancelled),Number(cut.expired))as{admitted:number}).admitted)).toBe(cut.expectedBirth);expect((db.query("SELECT (?1+?2-1)/?2 AS phases").get(fixture.memoryBytes,fixture.chunkBytes)as{phases:number}).phases).toBe(64)}finally{db.close()}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("CoreInstanceOpening");expect(source).toContain("validate_context(cx)");expect(source).toContain("charge_retained_item(copy,capacity,release)");expect(source).toContain("charge_retirement_retained_item(copy,capacity,release)");expect(source).not.toContain("CoreInstance::instantiate(");expect(source).not.toContain("StepContext::new");
});

/** 📥️ The independently modelled copy grant includes the actual cursor representation as well as input bytes. */
test("original Core input copies into preborn memory under separate cursor cost",async()=>{const law=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json(),row=law.inputCopy,bytes=new TextEncoder().encode(row.source);const db=new Database(":memory:");try{expect((db.query("SELECT (?1+(?2-?3)-1)/(?2-?3) AS phases").get(bytes.length,row.copyBytes,row.cursorBytes)as{phases:number}).phases).toBe(row.expectedPhases);expect((db.query("SELECT (?1<?2) AS denied").get(row.cursorBytes-1,row.cursorBytes)as{denied:number}).denied).toBe(1)}finally{db.close()}const memory=new Uint8Array(law.memoryBytes);memory.set(bytes,row.offset);expect(new TextDecoder().decode(memory.subarray(row.offset,row.offset+bytes.length))).toBe(row.source);});
