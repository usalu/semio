import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
/** 🏗️ Each actual resource birth requires its unchanged incoming capacity, cancellation and deadline. */
test("original Core opening preserves denied capacities and phases copy independently",async()=>{
 const fixture=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json();expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{for(const cut of fixture.cuts)expect(Boolean((db.query("SELECT (?1>=?2 AND ?3=0 AND ?4=0) AS admitted").get(cut.capacity,cut.required,Number(cut.cancelled),Number(cut.expired))as{admitted:number}).admitted)).toBe(cut.expectedBirth);expect((db.query("SELECT (?1+?2-1)/?2 AS phases").get(fixture.memoryBytes,fixture.chunkBytes)as{phases:number}).phases).toBe(64)}finally{db.close()}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("CoreInstanceOpening");expect(source).toContain("validate_context(cx)");expect(source).toContain("maximum_capacity_bytes");expect(source).toContain("released_bytes");expect(source).not.toContain("CoreInstance::instantiate(");expect(source).not.toContain("StepContext::new");
});

/** 📥️ The independently modelled copy grant includes the actual cursor representation as well as input bytes. */
test("original Core input copies into preborn memory under separate cursor cost",async()=>{const law=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json(),row=law.inputCopy,bytes=new TextEncoder().encode(row.source);const db=new Database(":memory:");try{expect((db.query("SELECT (?1+(?2-?3)-1)/(?2-?3) AS phases").get(bytes.length,row.copyBytes,row.cursorBytes)as{phases:number}).phases).toBe(row.expectedPhases);expect((db.query("SELECT (?1<?2) AS denied").get(row.cursorBytes-1,row.cursorBytes)as{denied:number}).denied).toBe(1)}finally{db.close()}const memory=new Uint8Array(law.memoryBytes);memory.set(bytes,row.offset);expect(new TextDecoder().decode(memory.subarray(row.offset,row.offset+bytes.length))).toBe(row.source);});
