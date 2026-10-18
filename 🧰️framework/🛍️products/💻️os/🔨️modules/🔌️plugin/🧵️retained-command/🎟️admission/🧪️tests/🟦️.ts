import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {readModuleTreeSource} from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import {Buffer} from "node:buffer";
test("retained admission examples agree with the independent buffer allocation oracle",()=>{
 const root=join(import.meta.dir,"..");
 const fixture=JSON.parse(readFileSync(join(root,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8"));
 const bytes=(value:number|string)=>value==="usize-max"?Number.MAX_VALUE:Number(value);
 for(const row of fixture.limits){
  const refusal=row.maximumRawBytes===0||row.maximumWorkItems===0?"capacity-is-zero":null;
  expect(refusal).toBe(row.expectedRefusal);
 }
 for(const row of fixture.raw){
  let refusal:string|null=null,capacity=0;
  if(bytes(row.declaredBytes)>bytes(row.maximumRawBytes))refusal="raw-capacity-rejected";
  else {try{const original=Buffer.allocUnsafe(bytes(row.declaredBytes));capacity=original.length;}catch{refusal="raw-capacity-rejected";}}
  expect(refusal).toBe(row.expectedRefusal);expect(capacity).toBe(row.expectedRawCapacity);
 }
});
test("production payload admission has a retained fault result for every original capture",()=>{
 const source=readModuleTreeSource(join(import.meta.dir,"../../🦀️.rs"));
 const start=source.indexOf("impl<A: ArtifactApp> ArtifactRetainedCommandPayload<A>");
 const body=source.slice(start,source.indexOf("//#endregion 🧳️Payload",start));
 expect(body).toContain("-> Self");expect(body).toContain("admission::admit_limits");expect(body).toContain("admission_refusal,work");expect(body).not.toContain("return Err");
 for(const owner of ["command","snapshot","config","history","interaction_state","interaction_hover","context","operation","completion"]){expect(body).toContain(owner);}
 expect(source).toContain("if admission_refusal.is_some(){ArtifactRetainedCommandPhase::Fault}");expect(source).toContain("admission::admit_raw(input.declared_bytes(),payload.maximum_raw_bytes)");
});
