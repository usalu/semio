/** 🎟️ The neutral storage protocol keeps independent grants and original logical owners. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import corpus from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {PackStackStorage,PackEagerCapacity,PackSymbolStorage} from "../🟦️.ts";
test("original Pack value stack storage closes the declared roots without merging grants",()=>{
 const validate=new Ajv().compile(schema);expect(validate(corpus)).toBe(true);
 const oracle=new Database(":memory:");oracle.run("CREATE TABLE ownership(kind TEXT, ordinal INTEGER)");
 for(const row of corpus.cases){
  oracle.run("DELETE FROM ownership");for(let n=0;n<row.roots;n++)oracle.run("INSERT INTO ownership VALUES ('frame', ?)",[n]);
  if(row.pending)oracle.run("INSERT INTO ownership VALUES ('pending', -1)");oracle.run("INSERT INTO ownership VALUES ('backing', 0)");
  const model=new PackStackStorage(64,{vector:24,frame:64,length:8,pending:24});const admission=model.admissionDemand();
  for(const denied of ["items","copy","capacity","depth"] as const){const grant={...admission,[denied]:admission[denied]-1};expect(model.admit(grant)).toEqual({items:0,copy:0,capacity:0,release:0,depth:0});expect(model.admissionDemand()).toEqual(admission);}
  expect(model.admit(admission)).toEqual(admission);model.initialize(row.roots,row.pending);
  const actual:string[]=[];
  for(;;){const next=oracle.query<{kind:string;ordinal:number},[]>("SELECT kind,ordinal FROM ownership ORDER BY CASE kind WHEN 'pending' THEN 0 WHEN 'frame' THEN 1 ELSE 2 END, ordinal DESC LIMIT 1").get();if(!next){actual.push("complete");break;}
   const demand=model.retirementDemand();expect(model.stage()).toBe(next.kind);
   for(const denied of ["items","copy","release","depth"] as const){if(demand[denied]===0)continue;const grant={...demand,[denied]:demand[denied]-1};expect(model.retire(grant)).toEqual({items:0,copy:0,capacity:0,release:0,depth:0});expect(model.retirementDemand()).toEqual(demand);expect(oracle.query("SELECT count(*) AS count FROM ownership").get()).toEqual({count:row.roots+(row.pending?1:0)+1-actual.length});}
   expect(model.retire(demand)).toEqual(demand);
   actual.push(next.kind);oracle.run("DELETE FROM ownership WHERE kind=? AND ordinal=?",[next.kind,next.ordinal]);
  }
  expect(actual).toEqual(row.close);
  const terminal=model.retirementDemand();expect(model.retire(terminal)).toEqual(terminal);expect(model.terminal()).toBe(true);
 }
 oracle.close();
});

test("original partial Unicode symbol owners agree with SQLite codepoints and UTF8",()=>{
 const oracle=new Database(":memory:");oracle.run("CREATE TABLE symbols(ordinal INTEGER, text TEXT)");for(const[index,text]of corpus.symbols.entries())oracle.run("INSERT INTO symbols VALUES(?,?)",[index,text]);
 for(const stop of corpus.interruptions){const model=new PackSymbolStorage(4,8,16);let consumed=0;
  symbols:for(const text of corpus.symbols){for(const character of text){if(consumed===stop)break symbols;model.append(character);consumed++;}model.publish();}
  expect(model.scalarCount()).toBe(stop);
  for(let index=0;index<model.symbolCount();index++){const row=oracle.query<{scalars:number;bytes:number;text:string},[number]>("SELECT length(text) AS scalars, length(CAST(text AS BLOB)) AS bytes,text FROM symbols WHERE ordinal=?").get(index)!;const chars=Array.from(row.text);expect(chars.length).toBe(row.scalars);expect(new TextEncoder().encode(row.text).length).toBe(row.bytes);for(const[ordinal,character]of chars.entries())expect(model.symbolChar(index,ordinal)).toBe(character);expect(model.symbolChar(index,chars.length)).toBeUndefined();}
  while(!model.terminal()){const grant={items:1,copy:1,capacity:0,release:0,depth:1};for(const axis of ["items","copy","depth"] as const){const before=[model.scalarCount(),model.symbolCount()];expect(model.close({...grant,[axis]:0})).toBe(false);expect([model.scalarCount(),model.symbolCount()]).toEqual(before);}expect(model.close(grant)).toBe(true);}
 }
 oracle.close();
});

test("eager Pack symbol capacity retains an original live entry through reserved leaves",()=>{
 const oracle=new Database(":memory:");oracle.run("CREATE TABLE pages(capacity INTEGER)");const model=new PackEagerCapacity(512,16384);const original={value:73};let captured=false;
 for(const target of corpus.capacityTargets){
  while(model.next(target)!==undefined){const grant={items:1,copy:1,capacity:1,release:0,depth:1};const before=model.capacity();
   for(const axis of ["items","copy","capacity","depth"] as const){expect(model.reserve(target,{...grant,[axis]:0})).toBe(false);expect(model.capacity()).toBe(before);if(captured)expect(model.borrowed()).toBe(original);}
   const stage=model.next(target);expect(model.reserve(target,grant)).toBe(true);if(stage==="payload")oracle.run("INSERT INTO pages VALUES(512)");
  }
  if(!captured){model.capture(original);captured=true;}expect(model.borrowed()).toBe(original);expect(model.capacity()).toBe(oracle.query<{capacity:number},[]>("SELECT SUM(capacity) AS capacity FROM pages").get()!.capacity);expect(model.capacity()).toBe(Math.ceil(target/512)*512);
 }
 expect(model.reserve(model.capacity(),{items:0,copy:0,capacity:0,release:0,depth:0})).toBe(false);expect(model.borrowed()).toBe(original);oracle.close();
});
