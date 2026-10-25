import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
test("sealed original command wire accounts for wrapper bytes and rejects donor substitution",()=>{
 const law=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 expect(new Ajv({strict:true}).validate({type:"object",required:["schema","commands","cases","mutable","cloneable"],additionalProperties:false,properties:{schema:{const:"semio.plugin.original-command-wire.v1"},commands:{type:"array",minItems:4,items:{type:"object",required:["channel","text"],additionalProperties:false,properties:{channel:{type:"integer",minimum:0,maximum:1},text:{type:"string"}}}},cases:{type:"array",minItems:4,items:{type:"object",required:["relation","extent","admitted"],additionalProperties:false,properties:{relation:{enum:["same","differentWire","differentSource"]},extent:{enum:["exact","minusOne"]},admitted:{type:"boolean"}}}},mutable:{const:false},cloneable:{const:false}}},law)).toBe(true);
 for(const command of law.commands){
  const bytes=Buffer.concat([Buffer.from([command.channel]),Buffer.from(JSON.stringify(command.text),"utf8")]);
  const originalWire={bytes};const originalSource={id:7};const witness={wire:originalWire,source:originalSource,bytes:bytes.length};
  const manual=1+[...JSON.stringify(command.text)].reduce((sum,c)=>sum+(c.codePointAt(0)!<128?1:c.codePointAt(0)!<2048?2:c.codePointAt(0)!<65536?3:4),0);
  expect(witness.bytes).toBe(manual);
  for(const c of law.cases){
   const candidate=applyPatch({limit:witness.bytes},c.extent==="minusOne"?[{op:"replace",path:"/limit",value:witness.bytes-1}]:[],true,false).newDocument;
   const wire=c.relation==="differentWire"?{bytes:Buffer.from(bytes)}:originalWire,source=c.relation==="differentSource"?{id:7}:originalSource;
   expect(witness.wire===wire&&witness.source===source&&witness.bytes<=candidate.limit).toBe(c.admitted);
  }
 }
 const owner=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");
 for(const required of ["pub struct ArtifactOriginalToolCommand","pub(crate) fn from_admitted_op_binary","pub fn admitted_encoded_bytes","first_page","source_binding.matches(cx)","impl<C> std::ops::Deref","impl<C:semio_framework_value::retirement::RetireOwned>"])expect(owner).toContain(required);
 expect(owner).not.toContain("DerefMut");expect(owner).not.toContain("#[derive(Clone");
});
