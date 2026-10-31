import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv2020 from "ajv";
import {applyPatch} from "fast-json-patch";
import {Database} from "bun:sqlite";
import {between} from "../../../🧪️tests/🧫️fixtures/🟦️.ts";
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
 expect(source.includes("source.retire()")).toBe(true);expect(source.includes("SharedOriginalRelease::Original")).toBe(true);
 const observer=between(source,"pub fn original_aliases_are_returned(","\n");
 expect(observer.includes("->bool{")).toBe(true);
 expect(observer.split("source.aliases()").length-1).toBe(1);
 const sealed=source.replace(observer,"");
 for(const forbidden of ["Arc::downgrade","Arc::weak_count","Arc::strong_count","pub source:","pub fn into_arc","pub fn as_arc"]){expect(sealed.includes(forbidden)).toBe(false);}
 expect(valid({...fixture,cases:[{...fixture.cases[0],aliases:0}]})).toBe(false);
});

test("borrowed original strong parent witnesses one directly paid nonlast alias return",()=>{
 const fixture=JSON.parse(readFileSync(join(import.meta.dir,"🧫️fixtures/🔣️.json"),"utf8"));
 const schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));
 const valid=new Ajv2020({strict:true}).compile(schema);expect(valid(fixture)).toBe(true);
 expect(valid({...fixture,borrowedReturn:{...fixture.borrowedReturn,parentRemainsLive:false}})).toBe(false);
 const db=new Database(":memory:");try{db.run("CREATE TABLE custody(parent INTEGER,aliases INTEGER,items INTEGER,copy INTEGER,capacity INTEGER,released INTEGER)");for(const count of fixture.borrowedReturn.aliases){db.query("INSERT INTO custody VALUES(1,?,0,0,0,0)").run(count);for(let i=0;i<count;i++)db.query("UPDATE custody SET aliases=aliases-1,items=items+1 WHERE aliases>0 AND parent=1").run();expect(db.query("SELECT parent,aliases,items,copy,capacity,released FROM custody").all().at(-1)).toEqual({parent:1,aliases:0,items:count,copy:0,capacity:0,released:0});}}finally{db.close();}
 const source=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8"),body=between(source,"pub fn return_borrowed_alias(","pub fn terminal_is_empty(");expect(body.includes("original.identity()!=parent.identity()")).toBe(true);expect(body.includes("self.issuance!=witness.issuance")).toBe(true);expect(body.includes("source.retire()")).toBe(true);expect(body.includes("SharedOriginalRelease::Alias")).toBe(true);expect(body.includes("RetainedCloneProgress{copied_items:1,..Default::default()}")).toBe(true);for(const forbidden of["Arc::strong_count","Arc::weak_count","Arc::downgrade","birth_bytes()","Box::new"]){expect(body.includes(forbidden)).toBe(false)}
});

test("stale original slot refuses another issuance at the same numeric address",()=>{
 const corpus=JSON.parse(readFileSync(join(import.meta.dir,"🧫️fixtures/🔣️.json"),"utf8"));
 const input={candidate:{address:7,issuance:2},slot:{address:7,issuance:1}};
 const replay=applyPatch(input,[{op:"test",path:"/candidate/address",value:7},{op:"test",path:"/slot/address",value:7}],true,false).newDocument as typeof input;
 expect({sameAddress:replay.candidate.address===replay.slot.address,sameIssuance:replay.candidate.issuance===replay.slot.issuance,accepted:replay.candidate.address===replay.slot.address&&replay.candidate.issuance===replay.slot.issuance}).toEqual(corpus.staleSlot);
 const source=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8");expect(source.includes("self.issuance!=slot.issuance")).toBe(true);
});

test("original shared issuer declares its exact transfer body quote before header release",()=>{
 const fixture=JSON.parse(readFileSync(join(import.meta.dir,"🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));expect(new Ajv2020({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{expect(db.query("SELECT ? AS body,? AS releasedHeader").get(fixture.originalTransferQuote.sourceRootMetadataCopy,32)).toEqual({body:0,releasedHeader:32})}finally{db.close()}const source=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8");expect(source.includes("pub transfer_copy_bytes:fn(&T)->usize")).toBe(true);expect(source.includes("pub fn original_transfer_demand(")).toBe(true);expect(source.includes("self.issuer.transfer_copy_bytes")).toBe(true);
});

import stableLaw from "../../📦️original/🧪️tests/🧫️fixtures/🔣️.json";
import stableSchema from "../../📦️original/🧬️schema/🔣️.json";
import {sharedOriginalEvent} from "../../📦️original/🟦️.ts";
test("stable original aliases and field retirement agree with independent SQL",()=>{const valid=new Ajv2020({strict:true}).compile(stableSchema);expect(valid(stableLaw)).toBe(true);expect(valid({...stableLaw,denialRetainsPointer:false})).toBe(false);const db=new Database(":memory:");try{const oracle=db.query("SELECT CASE WHEN ? > 1 THEN 'alias' WHEN ? = 1 THEN 'owner' WHEN ? > 0 THEN 'field' WHEN ? > 1 THEN 'wait' ELSE 'release' END AS event");for(const row of stableLaw.cases){expect<string>(sharedOriginalEvent(row)).toBe(row.event);expect((oracle.get(row.aliases,row.aliases,row.remainingFields,row.physicalHandles)as{event:string}).event).toBe(row.event);}for(const bytes of stableLaw.fieldBytes)expect(bytes*stableLaw.fieldMoves).toBeLessThanOrEqual(stableLaw.maximumCopyBytes);expect(stableLaw.inlineBytes*stableLaw.fieldMoves).toBeGreaterThan(stableLaw.maximumCopyBytes);}finally{db.close();}});

import {sharedOriginalAdmitted} from "../../📦️original/🟦️.ts";
test("stable native field admission agrees with independent SQL",()=>{const db=new Database(":memory:");try{const oracle=db.query("SELECT ? AND ? AND NOT ? AND ? > 0 AND ? >= ? AND ? > 0 AS accepted");for(const row of stableLaw.admission){expect(sharedOriginalAdmitted(row)).toBe(row.accepted);expect(Boolean((oracle.get(Number(row.supported),Number(row.birthKnown),Number(row.customDrop),row.items,row.capacity,row.requiredCapacity,row.depth)as{accepted:number}).accepted)).toBe(row.accepted);}expect(stableLaw.fieldConstructorPanic.nextField).toBe(stableLaw.fieldConstructorPanic.transferredFields.length);expect(stableLaw.selectionInterleaving.originalEffects).toBe(0);}finally{db.close();}});
