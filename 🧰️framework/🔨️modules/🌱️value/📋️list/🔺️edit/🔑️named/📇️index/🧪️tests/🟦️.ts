/** 🧪️ Admitted named coordinates agree with independent SQLite BINARY keys and original sparse edit outcomes. */
import {test,expect} from "bun:test";
import "../♻️transfer/🧪️tests/🟦️.ts";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import edits from "../../🧪️tests/🧫️fixtures/🔣️.json";
import {NamedKeyIndexPreparation} from "../🟦️.ts";
import {NamedEditPlan,type NamedEditSource,type NamedEditGrant,observeContiguousNamedKey} from "../../🟦️.ts";
const encode={encode:(text:string)=>new Uint8Array(Buffer.from(text,"utf8"))};
function source(keys:readonly string[],removed:readonly string[]=[],modified:readonly string[]=[],added:readonly string[]=[],indices:readonly number[]=[]):NamedEditSource{const lanes={base:keys.map(key=>encode.encode(key)),removed:removed.map(key=>encode.encode(key)),modified:modified.map(key=>encode.encode(key)),added:added.map(key=>encode.encode(key))};return{originalBase:keys,shape:{baseLen:keys.length,removed:removed.length,modified:modified.length,added:added.length},keyDemand:()=>({copyBytes:64,capacityBytes:0,depth:1}),observeKey:(lane,ordinal,grant)=>observeContiguousNamedKey(()=>lanes[lane][ordinal]!,grant),addedIndex:ordinal=>indices[ordinal]!};}
function admit(input:NamedEditSource){const owner=new NamedKeyIndexPreparation(input);for(let turn=0;!owner.settled;turn++){expect(turn).toBeLessThan(1000000);const d=owner.demand(4096);owner.advance({maximumItems:1,maximumCopyBytes:d.copyBytes,maximumCapacityBytes:d.capacityBytes,maximumDepth:d.depth});}const d=owner.takeDemand();return owner.take({maximumItems:1,maximumCopyBytes:d.copyBytes,maximumCapacityBytes:d.capacityBytes,maximumDepth:d.depth});}
function settle(plan:NamedEditPlan,input:NamedEditSource){for(let turn=0;!plan.settled;turn++){expect(turn).toBeLessThan(1000000);const d=plan.demand(input,4096),g={maximumItems:1,maximumCopyBytes:d.copyBytes,maximumCapacityBytes:d.capacityBytes,maximumDepth:d.depth};const before=plan.demand(input,4096);const p=plan.advance(input,{...g,maximumItems:0});expect(p.copiedItems).toBe(0);expect(plan.demand(input,4096)).toEqual(before);plan.advance(input,g);}return plan.ready;}
test("admitted original indexes preserve exact UTF8 identity and original ranks with a SQLite oracle",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,laws:{...fixture.laws,reuseBaseSort:true}})).toBe(false);
 const db=new Database(":memory:");try{db.run("CREATE TABLE keys(name TEXT PRIMARY KEY COLLATE BINARY,rank INTEGER)");for(const row of fixture.cases){db.run("DELETE FROM keys");let accepted=true;try{for(const[rank,text]of row.keys.entries())db.run("INSERT INTO keys VALUES(?,?)",[text,rank]);}catch{accepted=false;}const input=source(row.keys),index=admit(input);expect(index!==undefined).toBe(accepted);expect(index!==undefined).toBe(row.expected!==null);if(index){expect(row.expected!.length).toBe(row.queries.length);for(const[ordinal,text]of row.queries.entries()){const expected=db.query("SELECT rank FROM keys WHERE name=?").get(text)as{rank:number}|null;const query=source(row.keys,[],[text]);const joined=index.source(query);const plan=new NamedEditPlan(joined.shape,index);expect(settle(plan,joined)).toBe(expected!==null);expect(plan.ready?plan.modifiedIndex(0):null).toBe(expected?.rank??null);expect(expected?.rank??null).toBe(row.expected![ordinal]!);}expect(()=>index.source(source([...row.keys]))).toThrow();}}}finally{db.close();}
 console.log("[DEBUG] Admitted named original indices agree with SQLite BINARY Unicode/case/empty-key lookup and foreign original refusal");
});
test("the same named planner reuses admitted base order for every neutral original sparse edit",()=>{
 for(const row of edits.cases){const original=source(row.base),index=admit(original);const input=source(row.base,row.removed,row.modified,row.added,row.indices);const plain=new NamedEditPlan(input.shape);const accepted=settle(plain,input);if(!index){expect(accepted).toBe(false);continue;}const joined=index.source(input),plan=new NamedEditPlan(joined.shape,index);expect(settle(plan,joined)).toBe(accepted);if(accepted){expect(plan.plan.outputLength).toBe(plain.plan.outputLength);for(let ordinal=0;ordinal<row.modified.length;ordinal++)expect(plan.modifiedIndex(ordinal)).toBe(plain.modifiedIndex(ordinal));}expect(plan.baseAdmissionActions).toBe(0);}
 console.log("[DEBUG] Admitted base key order drives the original named/indexed planner for all neutral sparse edits with zero base admission actions");
});

test("retirement work bounds scale with every physical original row and retain a separate idle bound",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,retirementWork:{...fixture.retirementWork,turnsPerRow:1}})).toBe(false);
 const db=new Database(":memory:");try{for(const rows of fixture.lengths){const expected=fixture.retirementWork.fixedTurns+rows*fixture.retirementWork.turnsPerRow;expect(Number.isSafeInteger(expected)).toBe(true);expect(db.query("SELECT ? + ? * ? AS turns").get(fixture.retirementWork.fixedTurns,rows,fixture.retirementWork.turnsPerRow)).toEqual({turns:expected});expect(expected).toBeGreaterThan(rows*128);}expect(fixture.retirementWork.maximumIdleTurns).toBeLessThan(fixture.retirementWork.fixedTurns);}finally{db.close();}
 console.log("[DEBUG] Retirement work scales through all65537 original rows; unchanged public receipt/quote observations retain an independent finite idle bound");
});
