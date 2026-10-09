import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {Buffer} from "node:buffer";
test("retained admission examples agree with the independent buffer allocation oracle",()=>{
 const root=join(import.meta.dir,"..");
 const fixture=JSON.parse(readFileSync(join(root,"🧫️fixtures/🔣️.json"),"utf8"));
 for(const row of fixture.cases){
  let refusal:string|null=null,capacity=0;
  if(row.maximumRawBytes===0||row.maximumWorkItems===0)refusal="capacity-is-zero";
  else {try{const original=Buffer.allocUnsafe(row.maximumRawBytes==="usize-max"?Number.MAX_VALUE:row.maximumRawBytes);capacity=original.length;}catch{refusal="raw-capacity-rejected";}}
  expect(refusal).toBe(row.expectedRefusal);expect(capacity).toBe(row.expectedRawCapacity);
 }
});
test("production payload admission has a retained fault result for every original capture",()=>{
 const source=readFileSync(join(import.meta.dir,"../../🦀️.rs"),"utf8");
 const start=source.indexOf("impl<A: ArtifactApp> ArtifactRetainedCommandPayload<A>");
 const body=source.slice(start,source.indexOf("//#endregion 🧳️Payload",start));
 expect(body).toContain("-> Self");expect(body).toContain("admission::admit_raw");expect(body).toContain("admission_refusal,work");expect(body).not.toContain("return Err");
 for(const owner of ["command","snapshot","config","history","interaction_state","interaction_hover","context","operation","completion"]){expect(body).toContain(owner);}
 expect(source).toContain("if payload.admission_refusal.is_some(){ArtifactRetainedCommandPhase::Fault}");
});
