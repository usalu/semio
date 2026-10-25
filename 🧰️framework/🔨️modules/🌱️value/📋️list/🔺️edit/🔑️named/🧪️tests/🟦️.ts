import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {NamedEditPlan,type NamedEditSource,type NamedEditGrant,observeContiguousNamedKey} from "../🟦️.ts";
import {IndexedOutputCursor} from "../../🟦️.ts";

test("named edit plan preserves original ordinals and matches SQLite binary keys",()=>{
 expect(new Ajv2020({strict:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");db.exec("CREATE TABLE names(idx INTEGER PRIMARY KEY,key TEXT COLLATE BINARY)");
 try{for(const law of fixture.cases){
  db.exec("DELETE FROM names");for(const [index,key]of law.base.entries())db.query("INSERT INTO names VALUES(?,?)").run(index,key);
  const lookup=(key:string)=>db.query<{idx:number},[string]>("SELECT idx FROM names WHERE key=? ORDER BY idx LIMIT 1").get(key)?.idx;
  if(law.expected){expect(law.removed.map(lookup)).toEqual(law.expected.removed);expect(law.modified.map(lookup)).toEqual(law.expected.modified);}
  const encoder={encode:(text:string)=>new Uint8Array(Buffer.from(text,"utf8"))},keys={base:law.base.map(key=>encoder.encode(key)),removed:law.removed.map(key=>encoder.encode(key)),modified:law.modified.map(key=>encoder.encode(key)),added:law.added.map(key=>encoder.encode(key))};
  const source:NamedEditSource={originalBase:keys.base,shape:{baseLen:law.base.length,removed:law.removed.length,modified:law.modified.length,added:law.added.length},keyDemand:()=>({copyBytes:64,capacityBytes:0,depth:1}),observeKey:(lane,ordinal,grant)=>observeContiguousNamedKey(()=>keys[lane][ordinal]!,grant),addedIndex:ordinal=>law.indices[ordinal]!};
  const owner=new NamedEditPlan(source.shape);let turns=0;
  while(!owner.settled){const demand=owner.demand(source,owner.fixedCopyBytes+128),grant:NamedEditGrant={maximumItems:1,maximumCopyBytes:demand.copyBytes,maximumCapacityBytes:demand.capacityBytes,maximumDepth:demand.depth};expect(owner.advance(source,{...grant,maximumItems:0}).copiedBytes).toBe(0);expect(owner.advance(source,{...grant,maximumCopyBytes:owner.fixedCopyBytes-1}).copiedBytes).toBe(0);const receipt=owner.advance(source,grant);expect(receipt.copiedBytes).toBeLessThanOrEqual(grant.maximumCopyBytes);expect(++turns).toBeLessThan(2000);}
  expect(owner.ready).toBe(law.expected!==null);
  if(law.expected){expect(law.modified.map((_,ordinal)=>{const d=owner.modifiedIndexDemand(ordinal),grant={maximumItems:1,maximumCopyBytes:d.copyBytes,maximumCapacityBytes:d.capacityBytes,maximumDepth:d.depth};expect(d.capacityBytes).toBe(fixture.observation.capacityBytes);for(const currency of fixture.observation.deniedCurrencies){const denied={...grant,...(currency==="items"?{maximumItems:0}:currency==="copy"?{maximumCopyBytes:0}:{maximumDepth:0})};expect(owner.observeModifiedIndex(ordinal,denied)).toBeUndefined();}const output=owner.observeModifiedIndex(ordinal,grant)!;expect(output.receipt.copiedBytes).toBe(d.copyBytes);expect(output.receipt.retainedCapacityBytes).toBe(0);expect(output.index).toBe(lookup(law.modified[ordinal]!)!);return output.index;})).toEqual(law.expected.modified);const cursor=new IndexedOutputCursor(),output:string[]=[];while(true){const row=cursor.next(owner.plan);if(row.kind==="complete")break;if(row.kind==="base")output.push(law.base[row.index]!);else if(row.kind==="added")output.push(law.added[row.ordinal]!);cursor.advance(owner.plan);}expect(output).toEqual(law.expected.output);}
  owner.beginClose();while(!owner.terminalIsEmpty){const d=owner.closeDemand(4096);const p=owner.closeStep({maximumItems:1,maximumCopyBytes:d.copyBytes,maximumCapacityBytes:d.capacityBytes,maximumDepth:d.depth});expect(p.copiedBytes).toBeLessThanOrEqual(d.copyBytes);}expect(owner.terminalIsEmpty).toBe(true);
 }
 }finally{db.close();}
});
test("named keys with long common prefixes yield and cancel at original frontiers",()=>{
 const encoder={encode:(text:string)=>new Uint8Array(Buffer.from(text,"utf8"))},prefix="k".repeat(fixture.wide.prefixLength),strings={base:[prefix+"é",prefix+"ö"],removed:[prefix+"é"],modified:[prefix+"ö"],added:[prefix+"é"]},keys={base:strings.base.map(k=>encoder.encode(k)),removed:strings.removed.map(k=>encoder.encode(k)),modified:strings.modified.map(k=>encoder.encode(k)),added:strings.added.map(k=>encoder.encode(k))};
 const source:NamedEditSource={originalBase:keys.base,shape:{baseLen:2,removed:1,modified:1,added:1},keyDemand:()=>({copyBytes:64,capacityBytes:0,depth:1}),observeKey:(lane,ordinal,grant)=>observeContiguousNamedKey(()=>keys[lane][ordinal]!,grant),addedIndex:()=>1};
 const original=keys.base[0]!.buffer;
 for(const cut of [0,1,7,128,25000,null]){
  const owner=new NamedEditPlan(source.shape);let turns=0;
  while(!owner.settled&&(cut===null||turns<cut)){const d=owner.demand(source,owner.fixedCopyBytes+64+2*fixture.wide.pageBytes);const p=owner.advance(source,{maximumItems:1,maximumCopyBytes:d.copyBytes,maximumCapacityBytes:d.capacityBytes,maximumDepth:d.depth});if(p.copiedBytes>d.copyBytes)throw Error("unfunded original key page");if(++turns>fixture.wide.prefixLength*16)throw Error("named key pages stalled");}
  if(cut===null){expect(owner.ready).toBe(true);expect(owner.modifiedIndex(0)).toBe(1);}
  owner.beginClose();let closing=0;while(!owner.terminalIsEmpty){const d=owner.closeDemand(4096);owner.closeStep({maximumItems:1,maximumCopyBytes:d.copyBytes,maximumCapacityBytes:d.capacityBytes,maximumDepth:d.depth});expect(++closing).toBeLessThan(128);}expect(keys.base[0]!.buffer).toBe(original);
 }
});
