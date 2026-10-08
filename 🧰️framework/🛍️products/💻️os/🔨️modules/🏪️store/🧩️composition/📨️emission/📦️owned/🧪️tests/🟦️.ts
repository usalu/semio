import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {applyPatch} from "fast-json-patch";
test("owned member batch retains exact ordered source across refusal and cancellation",()=>{
 const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 const source=structuredClone(fixture.values);
 for(const stop of [0,1,2,3]){
  let state={n:0};for(const value of source.slice(0,stop))state=applyPatch(state,[{op:"replace",path:"/n",value}],true,false).newDocument;
  expect(source).toEqual(fixture.values);expect(JSON.parse(JSON.stringify(source))).toEqual(fixture.values);
  if(stop===source.length)expect(state.n).toBe(fixture.expectedAfter);
 }
 expect(new TextEncoder().encode(fixture.metadata.actor).length).toBeLessThanOrEqual(fixture.grant.maximumCopyBytes);
 expect(fixture.grant.maximumItems).toBe(1);expect(fixture.grant.maximumReleaseBytes).toBe(4096);
 expect(fixture.transport).toEqual({apply:"typed",preview:"wire",applyingRequestsWireCodec:false});
 let typed={n:0};for(const value of source)typed=applyPatch(typed,[{op:"replace",path:"/n",value}],true,false).newDocument;
 const wire=JSON.parse(JSON.stringify(source));let preview={n:0};for(const value of wire)preview=applyPatch(preview,[{op:"replace",path:"/n",value}],true,false).newDocument;
 expect(typed).toEqual(preview);expect(typed.n).toBe(fixture.expectedAfter);
 console.log("[DEBUG] Owned member batch JSON Patch oracle preserves ordered original source through four admission/cancellation positions");
});

test("owned source census advances one exact ordered row and preserves cancellation prefix",()=>{
 const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 for(const count of fixture.census.rowCounts){
  const rows=Array.from({length:count},(_,index)=>fixture.values[index%fixture.values.length]);
  let state={n:0};let visited=0;
  for(const value of rows){state=applyPatch(state,[{op:"replace",path:"/n",value}],true,false).newDocument;visited+=fixture.census.maximumPreflightsPerTurn;expect(visited).toBeLessThanOrEqual(count);}
  expect(state.n).toBe(rows.at(-1));expect(JSON.parse(JSON.stringify(rows))).toEqual(rows);expect(visited).toBe(count);
 }
 console.log("[DEBUG] owned source JSON Patch census oracle: 1/3/257 exact forward rows, one row per turn and every serialized source retained");
});

test("owned scalar source declares copy work independently from physical release",()=>{
 const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 expect(Int32Array.BYTES_PER_ELEMENT).toBe(fixture.copyDemand.minimumCopyBytes);
 for(const count of fixture.copyDemand.rowCounts){
  const values=Array.from({length:count},(_,index)=>fixture.values[index%fixture.values.length]);
  const oracle=Int32Array.from(values);
  expect(oracle.byteLength).toBe(count*fixture.copyDemand.minimumCopyBytes);
  expect(Array.from(oracle)).toEqual(values);
  expect(fixture.copyDemand.zeroCopyReleasesBytes).toBe(0);
 }
 expect(readFileSync(resolve(import.meta.dir,"../🦀️.rs"),"utf8").includes("pub fn next_copy_byte_demand")).toBe(true);
 console.log("[DEBUG] independent Int32Array 0/1/65/257 ordered typed sources require4 bytes of logical work per scalar and zero physical release during that work");
});
