import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import fixture from"./🧫️fixtures/🔣️.json"with{type:"json"};
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
test("guest holds, observed work and producer forfeits stay disjoint from host cleanup and transit",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");
 try{for(const row of fixture.cases){const oracle=fixture.total.map((total,index)=>(db.query("SELECT ?-?-?-?-?-? AS available").get(total,fixture.cleanup[index],fixture.transit[index],row.observed[index],row.forfeited[index],row.held?fixture.hold[index]:0)as{available:number}).available);expect(oracle,row.id).toEqual(row.expected);}}finally{db.close();}
});

test("purpose scopes share one monotonically decreasing total without spending client quota on infrastructure",()=>{
 const db=new Database(":memory:");try{for(const row of fixture.purposes.cases){const remaining=(purpose:"semantic"|"infrastructure")=>fixture.purposes[purpose].map((v,i)=>v-row[purpose==="semantic"?"semanticSpent":"infrastructureSpent"][i]);const semantic=remaining("semantic"),infrastructure=remaining("infrastructure");const header=semantic.map((v,i)=>v+infrastructure[i]);const oracle=semantic.map((_,i)=>(db.query("SELECT ?-?+?-? AS total").get(fixture.purposes.semantic[i],row.semanticSpent[i],fixture.purposes.infrastructure[i],row.infrastructureSpent[i])as{total:number}).total);expect(header,row.id).toEqual(oracle);expect(header,row.id).toEqual(row.expectedHeader);expect(remaining(row.purpose==="semantic"?"semantic":"infrastructure"),row.id).toEqual(row.expectedScope);}}finally{db.close();}
});

import cleanup from"./🧫️fixtures/cleanup.json"with{type:"json"};
test("cancelled infrastructure ends before the reserved host cleanup window",()=>{const db=new Database(":memory:");try{for(const row of cleanup.cases){const deadline=(db.query("SELECT min(?+?,?+?-(?-?)) AS deadline").get(row.now,cleanup.guestWindow,cleanup.birth,cleanup.lifetime,cleanup.hostReserve,cleanup.guestWindow)as{deadline:number}).deadline;expect(deadline>row.now).toBe(row.allowed);if(row.allowed)expect(deadline).toBe(row.deadline);}expect(cleanup.semanticHeader).toBe(0);expect(cleanup.cancelled).toBe(true);}finally{db.close();}});

import settlement from"./🧫️fixtures/settlement.json"with{type:"json"};
import settlementSchema from"../🧬️schema/settlement.json"with{type:"json"};
test("settlement witness is bound to its exact producer and receipt",()=>{expect(new Ajv({strict:true}).compile(settlementSchema)(settlement)).toBe(true);const db=new Database(":memory:");try{for(const row of settlement){const actual=(db.query("SELECT ?=1 AND ?=3 AND ?=1 AND ?=2 AND ?=4 AND ?=7 AS accepted").get(row.serial,row.epoch,row.items,row.copy,row.work,row.producer)as{accepted:number}).accepted;expect(Boolean(actual)).toBe(row.accepted);}}finally{db.close();}});

import coreWidth from "./🧫️fixtures/core-width.json" with {type:"json"};
import coreWidthSchema from "../🧬️schema/core-width.json" with {type:"json"};
test("fixed Core grants preserve wasm32 integers with wider host stock",()=>{
 const validate=new Ajv({strict:true}).compile(coreWidthSchema);
 expect(validate(coreWidth),JSON.stringify(validate.errors)).toBe(true);
 const db=new Database(":memory:");
 try{
  const semanticCeiling=db.query<{n:number},[number,number]>("SELECT min(?, (1 << 32)-1-?) AS n");
  const sum=db.query<{n:number},[number,number]>("SELECT ?+? AS n");
  const fitsCore=(value:number)=>BigInt.asUintN(32,BigInt(value))===BigInt(value);
  for(const row of coreWidth.cases){
   const semantic=row.requested.map((grant,index)=>semanticCeiling.get(grant,index<4?coreWidth.infrastructure[index]:0)!.n);
   const header=semantic.map((grant,index)=>index<4?sum.get(grant,coreWidth.infrastructure[index])!.n:Math.max(grant,coreWidth.infrastructure[index]));
   expect(semantic,row.name).toEqual(row.semantic);
   expect(header,row.name).toEqual(row.header);
   expect([...semantic,...header].every(fitsCore),row.name).toBe(true);
  }
  expect(coreWidth.physical.every((grant)=>!fitsCore(grant))).toBe(true);
  for(const rejected of coreWidth.invalidInfrastructure)expect(rejected.every(fitsCore)).toBe(false);
 }finally{db.close();}
});

import cancellation from "./🧫️fixtures/cancel-crossing.json" with {type:"json"};
test("cancelled operations refuse new crossings and retain their cleanup allocation",()=>{
 const db=new Database(":memory:");
 try{
  for(const cleanup of [false,true]){
   const row=db.query("select case when ? and not ? then 'canceled' else 'admitted' end as state").get(Number(cancellation.canceled),Number(cleanup)) as {state:string};
   expect(row.state).toBe(cleanup?cancellation.cleanup:cancellation.ordinary);
  }
 }finally{db.close();}
});
