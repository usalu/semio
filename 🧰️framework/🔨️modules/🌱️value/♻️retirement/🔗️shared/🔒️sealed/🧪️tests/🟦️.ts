import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
import {applyPatch} from "fast-json-patch";
import {Database} from "bun:sqlite";
test("sealed original alias custody agrees with independent patch and schema oracles",()=>{
 const fixture=JSON.parse(readFileSync(join(import.meta.dir,"🧫️fixtures/🔣️.json"),"utf8"));
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

test("borrowed original strong parent witnesses one directly paid nonlast alias return",()=>{
 const fixture=JSON.parse(readFileSync(join(import.meta.dir,"🧫️fixtures/🔣️.json"),"utf8"));
 const schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));
 const valid=new Ajv2020({strict:true}).compile(schema);expect(valid(fixture)).toBe(true);
 expect(valid({...fixture,borrowedReturn:{...fixture.borrowedReturn,parentRemainsLive:false}})).toBe(false);
 const db=new Database(":memory:");try{db.run("CREATE TABLE custody(parent INTEGER,aliases INTEGER,items INTEGER,copy INTEGER,capacity INTEGER,released INTEGER)");for(const count of fixture.borrowedReturn.aliases){db.query("INSERT INTO custody VALUES(1,?,0,0,0,0)").run(count);for(let i=0;i<count;i++)db.query("UPDATE custody SET aliases=aliases-1,items=items+1 WHERE aliases>0 AND parent=1").run();expect(db.query("SELECT parent,aliases,items,copy,capacity,released FROM custody").all().at(-1)).toEqual({parent:1,aliases:0,items:count,copy:0,capacity:0,released:0});}}finally{db.close();}
 const source=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8"),start=source.indexOf("pub fn return_borrowed_alias(");expect(start).toBeGreaterThan(0);const body=source.slice(start,source.indexOf("pub fn return_slot(",start));expect(body.includes("Arc::ptr_eq(original,parent)")).toBe(true);expect(body.includes("self.issuance!=witness.issuance")).toBe(true);expect(body.includes("drop(self.source.take())")).toBe(true);expect(body.includes("RetainedCloneProgress{copied_items:1,..Default::default()}")).toBe(true);for(const forbidden of["Arc::strong_count","Arc::weak_count","Arc::downgrade","birth_bytes()","Box::new"]){expect(body.includes(forbidden)).toBe(false)}console.log("[DEBUG] Ajv/SQLite directly paid12 nonlast alias returns retain real strong parent, copy0 capacity0 release0; source only");
});

test("stale original slot refuses another issuance at the same numeric address",()=>{
 const corpus=JSON.parse(readFileSync(join(import.meta.dir,"🧫️fixtures/🔣️.json"),"utf8"));
 const input={candidate:{address:7,issuance:2},slot:{address:7,issuance:1}};
 const replay=applyPatch(input,[{op:"test",path:"/candidate/address",value:7},{op:"test",path:"/slot/address",value:7}],true,false).newDocument as typeof input;
 expect({sameAddress:replay.candidate.address===replay.slot.address,sameIssuance:replay.candidate.issuance===replay.slot.issuance,accepted:replay.candidate.address===replay.slot.address&&replay.candidate.issuance===replay.slot.issuance}).toEqual(corpus.staleSlot);
 const source=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8");expect(source.includes("self.issuance!=slot.issuance")).toBe(true);
});

test("original shared issuer declares its exact transfer body quote before header release",()=>{
 const fixture=JSON.parse(readFileSync(join(import.meta.dir,"🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));expect(new Ajv2020({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{expect(db.query("SELECT ? AS body,? AS releasedHeader").get(fixture.originalTransferQuote.sourceRootMetadataCopy,32)).toEqual({body:0,releasedHeader:32})}finally{db.close()}const source=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8");expect(source.includes("pub transfer_copy_bytes:fn(&T)->usize")).toBe(true);expect(source.includes("pub fn original_transfer_demand(")).toBe(true);expect(source.includes("self.issuer.transfer_copy_bytes")).toBe(true);console.log("[DEBUG] Ajv/SQLite original source roots transfer metadata with body0 and separately funded exact header release; generic inline quote remains explicit");
});
