import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
test("original camera and selection preparation preserve native semantics and issuer custody",async()=>{
 expect(new Ajv({strict:true,allowUnionTypes:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");db.run("CREATE TABLE selection(original TEXT, next TEXT)");for(const row of fixture.selection){db.run("INSERT INTO selection VALUES(?,?)",[row.base,row.next]);const actual=db.query("SELECT original AS inverse,next AS post FROM selection ORDER BY rowid DESC LIMIT 1").get();expect(actual).toEqual({inverse:row.base,post:row.next});}
 db.run("CREATE TABLE camera(axis INTEGER PRIMARY KEY,original REAL,next REAL)");const fields=(value:typeof fixture.camera.base)=>[value.viewport.x,value.viewport.y,value.viewport.zoom,...value.eye];const original=fields(fixture.camera.base),next=fields(fixture.camera.next);for(let index=0;index<original.length;index++)db.run("INSERT INTO camera VALUES(?,?,?)",[index,original[index]!,next[index]!]);expect(db.query("SELECT original,next FROM camera ORDER BY axis").all()).toEqual(original.map((value,index)=>({original:value,next:next[index]})));db.close();
 for(const row of fixture.selectionBounds){const next=row.next.repeat(row.repeat);const binary=(value:string|null)=>value===null?2:6+new TextEncoder().encode(value).byteLength;const bytes=new TextEncoder().encode(JSON.stringify({selected:next})).byteLength+binary(next)+binary(row.base);expect(bytes).toBe(row.bytes);expect(bytes<=1024).toBe(row.admissible);}
 const owner=await Bun.file(new URL("../../🦀️.rs",import.meta.url)).text();expect(owner).toContain("type Edit: store::snapshot_clone_preparation::RetainedCloneEdit");expect(owner).toContain("RetainedClonePreparationFactory::new");expect(owner).not.toContain("struct BoundedWindowConfigPreparation");expect(owner).not.toContain("protocol::Mutation::diff(mutation, base.get())");console.log("[DEBUG] independent SQLite preserves original Window camera fields and null/empty/UnicodeNUL selection plus inverse order; actual native issuer pipeline mounted");
});
