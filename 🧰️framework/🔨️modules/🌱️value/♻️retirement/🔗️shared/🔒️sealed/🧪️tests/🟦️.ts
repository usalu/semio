import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
import {applyPatch} from "fast-json-patch";
test("sealed original alias custody agrees with independent patch and schema oracles",()=>{
 const fixture=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 const schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));
 const valid=new Ajv2020({strict:true,allErrors:true}).compile(schema);
 expect(valid(fixture),JSON.stringify(valid.errors)).toBe(true);
 for(const row of fixture.cases){
  let oracle={handles:Array.from({length:row.aliases},(_,index)=>index),original:row.text};
  let originalPayloadOwners=0,releasedHeaders=0,terminalAliases=0;
  for(let index=0;index<row.aliases;index++){
   oracle=applyPatch(oracle,[{op:"remove",path:"/handles/0"}],true,false).newDocument;
   if(oracle.handles.length===0){originalPayloadOwners++;releasedHeaders++;}
   terminalAliases++;
   expect(oracle.original).toBe(row.text);
  }
  expect({originalPayloadOwners,releasedHeaders,terminalAliases}).toEqual(row.expected);
 }
 const source=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8");
 expect(source.includes("Arc::into_inner(source)")).toBe(true);
 for(const forbidden of ["Arc::downgrade","Arc::weak_count","Arc::strong_count","pub source:","pub fn into_arc","pub fn as_arc"]){expect(source.includes(forbidden)).toBe(false);}
 expect(valid({...fixture,cases:[{...fixture.cases[0],aliases:0}]})).toBe(false);
});

test("stale original slot refuses another issuance at the same numeric address",()=>{
 const corpus=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 const input={candidate:{address:7,issuance:2},slot:{address:7,issuance:1}};
 const replay=applyPatch(input,[{op:"test",path:"/candidate/address",value:7},{op:"test",path:"/slot/address",value:7}],true,false).newDocument as typeof input;
 expect({sameAddress:replay.candidate.address===replay.slot.address,sameIssuance:replay.candidate.issuance===replay.slot.issuance,accepted:replay.candidate.address===replay.slot.address&&replay.candidate.issuance===replay.slot.issuance}).toEqual(corpus.staleSlot);
 const source=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8");expect(source.includes("self.issuance!=slot.issuance")).toBe(true);
});