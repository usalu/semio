import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import fixture from "./🧫️.json";
import schema from "./🔣️.json";

test("original Flow geometry receives whole failure and exact five currencies once",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);for(const field of["wholeFailure","borrowedContext","paidEncoding"])expect(validate({...fixture,[field]:false})).toBe(false);
 const db=new Database(":memory:");try{for(let index=0;index<4;index++)expect(db.query("SELECT ?-? AS remaining").get(fixture.original[index]!,fixture.spent[index]!)).toEqual({remaining:fixture.remaining[index]!});expect(db.query("SELECT ?-? AS remaining").get(fixture.work.issued,fixture.work.spent)).toEqual({remaining:fixture.work.remaining});const bytes=Buffer.alloc(16);bytes.writeBigUInt64LE(BigInt(fixture.operation));bytes.writeBigUInt64LE(BigInt(fixture.generation),8);expect(bytes.readBigUInt64LE()).toBe(71631n);expect(bytes.readBigUInt64LE(8)).toBe(3n)}finally{db.close()}
 const protocol=readFileSync(new URL("../../../🕸️wasm/📡️protocol/🦀️.rs",import.meta.url),"utf8");expect(protocol).toContain("Geometry(crate::geometry::GeometryWorkFailure)");expect(protocol).toContain("pub fn from_geometry");expect(protocol).toContain("pub fn poll_on_original_caller");expect(protocol).toContain("operation.feature.step_on_original_caller(cx)");
 const source=readFileSync(new URL("../../../🕸️wasm/🦀️.rs",import.meta.url),"utf8"),action=source.slice(source.indexOf("struct FlowAction2599"),source.indexOf("struct FlowAction2600"));expect(action).toContain("operation:cx.operation().0,generation:cx.generation().0");expect(action).toContain("port.tessellate_step(identity,handle,tolerance,cx)");expect(action).toContain("FlowFailure::from_geometry");expect(action).toContain("ArtifactCanonicalJsonTreeWireCursor<semio_framework::MeshData>");expect(action).toContain("encoding.advance(grant)");expect(action).not.toContain("to_json_string(&mesh)");expect(action).not.toContain("StepContext::new");expect(action).not.toContain("default_now_us");console.log("[DEBUG] independent Ajv/SQLite/Buffer original Flow tessellation identity/full failure/CPU/paid canonical encoding caller contract; native and browser evidence remain separate");
});
