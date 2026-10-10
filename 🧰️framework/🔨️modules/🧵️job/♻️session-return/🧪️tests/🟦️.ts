import {test,expect} from "bun:test";
import Ajv from "ajv";
import jsonPatch from "fast-json-patch";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json";
import corpus from "../🧫️fixtures/🔣️.json";
test("original session frame neutral custody agrees with independent owner removal",()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);
 for(const row of corpus.cases){
  const canReturn=row.phase==="terminal"&&row.aliases===1&&!row.waker&&row.releaseGrant>=row.frameBytes;
  const source={original:{bytes:row.frameBytes}};
  const oracle=canReturn?jsonPatch.applyPatch(structuredClone(source),[{op:"remove",path:"/original"}]).newDocument:source;
  expect(canReturn?"returned":"retained").toBe(row.expected);
  expect(Object.hasOwn(oracle,"original")).toBe(!canReturn);
 }
});
test("original session native authority preserves its pre-admitted frame and seals weak capability",()=>{
 const source=readFileSync(join(import.meta.dir,"../../🦀️.rs"),"utf8"),owner=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8"),handle=readFileSync(join(import.meta.dir,"../🔐️handle/🦀️.rs"),"utf8");
 expect(handle).toContain("inner:Arc<WorkerJobSessionInner<J>>");
 expect(handle).not.toContain("Weak");
 expect(owner).toContain("original worker session retains a registered wake owner");
 expect(owner).toContain("self.inner_returned=true");
 expect(owner).toContain("drop(retirement)");
 const slot=source.slice(source.indexOf("fn release_retirement_slot"),source.indexOf("impl<J:InteractiveJob+'static> Drop for WorkerJobSession"));
 expect(slot).not.toContain("drop(retirement)");
 const fallback=source.slice(source.indexOf("impl<J:InteractiveJob+'static> Drop for WorkerJobSession"),source.indexOf("pub struct WorkerJobOutcome"));
 expect(fallback).toContain("Box::into_raw(retirement)");
 expect(fallback).toContain("ManuallyDrop::drop(&mut self.inner)");
});
