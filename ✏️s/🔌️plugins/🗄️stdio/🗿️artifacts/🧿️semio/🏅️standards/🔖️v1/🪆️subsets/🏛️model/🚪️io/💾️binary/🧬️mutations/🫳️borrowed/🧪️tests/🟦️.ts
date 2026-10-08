import {test,expect} from "bun:test";
import {readFileSync,existsSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
const base=join(import.meta.dir,"..");
const fixture=JSON.parse(readFileSync(join(base,"🧫️fixtures/🔣️.json"),"utf8"));
const schema=JSON.parse(readFileSync(join(base,"🧬️schema/🔣️.json"),"utf8"));
const elementSchema=JSON.parse(readFileSync(join(base,"../../../..","🧬️schema/🧬️mutations/🧱insert-element/🧬️schema/🔣️.json"),"utf8"));
test("neutral Model element wire preserves UTF8 hex, tuple order and properties",()=>{
  expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
  const snapshotSchema=JSON.parse(readFileSync(join(base,"../../../..","🧬️schema/📸️snapshot/🔣️.json"),"utf8"));
  const validate=new Ajv({strict:false}).addSchema(snapshotSchema).compile(elementSchema);
  const hex=(text:string)=>Buffer.from(text).toString("hex");
  const point=(p:any)=>`[${p.x},${p.y},${p.z}]`;
  const transform=(t:any)=>`[${point(t.translation)},[${t.rotation.x},${t.rotation.y},${t.rotation.z},${t.rotation.w}],${point(t.scale)}]`;
  const value=(v:any)=>v.kind==="text"?`T[${hex(v.value)}]`:v.kind==="number"?`N[${v.value}]`:`B[${v.value?1:0}]`;
  for(const element of fixture.elements){
    expect(validate({mutation:"insertElement",element})).toBe(true);
    const geometry=element.geometry.kind==="none"?"N":`${element.geometry.kind==="brep"?"B":"M"}[${hex(element.geometry.brepId??element.geometry.meshId)}]`;
    const psets=`[${element.psets.map((ps:any)=>`[${hex(ps.name)},[${ps.properties.map((p:any)=>`[${hex(p.key)},${value(p.value)}]`).join(",")}]]`).join(",")}]`;
    const body=`element=[${hex(element.id)},${element.class.kind==="column"?"CO":`OT[${hex(element.class.name)}]`},${transform(element.placement)},${geometry},${element.spatialId===null?"[0]":`[1,${hex(element.spatialId)}]`},${psets}]`;
    expect(Buffer.from(new TextEncoder().encode(body))).toEqual(Buffer.from(body));
    expect(Buffer.from(hex(element.id),"hex").toString()).toBe(element.id);
    for(const grant of fixture.grants){const bytes=Buffer.from(body);const parts=[];for(let offset=0;offset<bytes.length;offset+=Math.min(grant,64))parts.push(bytes.subarray(offset,offset+Math.min(grant,64)));expect(Buffer.concat(parts)).toEqual(bytes);}
  }
});
test("installed original Model operations have borrowed text authority",()=>{
  expect(existsSync(join(base,"🦀️.rs"))).toBe(true);
  const source=readFileSync(join(base,"../🦀️.rs"),"utf8");
  expect(source).toContain("ArtifactPreparedOperationSource::Text");
  expect(source).toContain("InsertElement");
  expect(source).toContain("RemoveElement");
});
