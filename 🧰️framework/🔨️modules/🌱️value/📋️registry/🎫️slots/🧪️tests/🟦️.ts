import {test,expect} from "bun:test";
import {FixedKeySlots} from "../🟦️.ts";
import law from "./🧫️fixtures/🔣️.json";
import fc from "fast-check";
import {Database} from "bun:sqlite";

test("portable slot outputs agree with independent SQLite uniqueness and compare-and-release",()=>{
 const database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE slots(slot INTEGER PRIMARY KEY, owner INTEGER UNIQUE, generation INTEGER NOT NULL)");
  const insert=database.query("INSERT INTO slots(slot,owner,generation) VALUES (?,NULL,0)");for(let index=0;index<law.capacity;index++)insert.run(index);
  const lookup=database.query<{slot:number;generation:number},[number]>("SELECT slot,generation FROM slots WHERE owner=?"),free=database.query<{slot:number;generation:number},[]>("SELECT slot,generation FROM slots WHERE owner IS NULL ORDER BY slot LIMIT 1"),claim=database.query("UPDATE slots SET owner=?,generation=generation+1 WHERE slot=? AND owner IS NULL"),release=database.query("UPDATE slots SET owner=NULL WHERE owner=? AND slot=? AND generation=?");
  for(const action of law.actions){
   if(action.kind==="claim"){
    if(!lookup.get(action.key)){const vacant=free.get();if(vacant)expect(claim.run(action.key,vacant.slot).changes).toBe(1);}
    const row=lookup.get(action.key);expect(row?.slot??null).toBe(action.index);if(row)expect(action.generation).toBe(String(row.generation));
   }else if(action.kind==="lookup"){const row=lookup.get(action.key);expect(row?.slot??null).toBe(action.index);if(row)expect(action.generation).toBe(String(row.generation));}
   else expect(release.run(action.key,action.index!,Number(action.generation)).changes===1).toBe(action.accepted!);
  }
  expect(database.query<{count:number},[]>("SELECT count(*) AS count FROM slots WHERE owner IS NOT NULL").get()!.count).toBe(0);
  console.log(`[DEBUG] ${law.actions.length} portable exact-slot outputs match independent SQLite unique-key admission and atomic generation release`);
 }finally{database.close();}
});

test("exact fixed slots preserve colliding keys and reject stale or foreign leases",()=>{
 const actual=new FixedKeySlots<number>(law.capacity),oracle=new Map<number,{index:number;generation:bigint}>(),generations=Array<bigint>(law.capacity).fill(0n),leases=new Map<string,ReturnType<typeof actual.claim>>();
 for(const action of law.actions){
  const key=action.key;
  if(action.kind==="claim"){
   let expected=oracle.get(key);
   if(!expected){const index=generations.findIndex((_,index)=>![...oracle.values()].some(row=>row.index===index));if(index>=0){expected={index,generation:++generations[index]!};oracle.set(key,expected);}}
   const lease=actual.claim(key);expect(lease?.index??null).toBe(action.index);expect(lease?.generation).toBe(expected?.generation);
   if(lease)leases.set(`${key}:${lease.generation}`,lease);
  }else if(action.kind==="lookup"){
   expect(actual.lookup(key)?.index??null).toBe(action.index);expect(actual.lookup(key)?.generation).toBe(oracle.get(key)?.generation);
  }else{
   const lease=leases.get(`${key}:${action.generation}`)??leases.get(`33:${action.generation}`)!;
   const expected=oracle.get(key),accepted=expected!==undefined&&expected.index===action.index&&expected.generation===BigInt(action.generation!);
   expect(actual.release(key,lease!)).toBe(accepted);expect(action.accepted).toBe(accepted);if(accepted)oracle.delete(key);
  }
  expect(actual.size).toBe(oracle.size);
 }
 expect(actual.size).toBe(0);
 const a=new FixedKeySlots<number>(1),b=new FixedKeySlots<number>(1),own=a.claim(1)!,foreign=b.claim(1)!;
 expect(a.release(1,foreign)).toBe(false);expect(a.lookup(1)).toEqual(own);expect(a.release(1,own)).toBe(true);
 console.log(`[DEBUG] ${law.actions.length} exact slot transitions match an independent Map; stale and foreign releases preserve live owners`);
});

test("arbitrary claim and release schedules preserve exact fixed capacity",()=>{
 fc.assert(fc.property(fc.array(fc.tuple(fc.boolean(),fc.integer({min:0,max:15})),{maxLength:300}),actions=>{
  const actual=new FixedKeySlots<number>(8),oracle=new Set<number>();
  for(const [claim,key]of actions){
   if(claim){const admitted=oracle.has(key)||oracle.size<8;expect(actual.claim(key)!==undefined).toBe(admitted);if(admitted)oracle.add(key);}
   else{const lease=actual.lookup(key);if(lease)expect(actual.release(key,lease)).toBe(true);oracle.delete(key);}
   expect(actual.size).toBe(oracle.size);for(let id=0;id<16;id++)expect(actual.lookup(id)!==undefined).toBe(oracle.has(id));
  }
 }),{numRuns:100});
 console.log("[DEBUG] 100 generated slot schedules agree with fast-check and independent Set membership");
});
