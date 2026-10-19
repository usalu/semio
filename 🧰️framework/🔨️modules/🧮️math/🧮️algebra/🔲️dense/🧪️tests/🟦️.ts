import { test } from "bun:test";
import assert from "node:assert/strict";
import Ajv from "ajv";
import { lusolve } from "mathjs";
import { DenseSolveCursor, type DenseSolveBudget } from "../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

export function verifyRetainedDenseOracle(): void {
 const validate=new Ajv({strict:true}).compile(schema); assert(validate(fixture),JSON.stringify(validate.errors));
 const generous:DenseSolveBudget={items:1,copyBytes:64,depth:1,capacityBytes:0}; let total=0;
 for(const row of fixture.cases){
  const matrix=[...row.matrix],rhs=[...row.rhs]; let cursor=new DenseSolveCursor(row.order,matrix.length,rhs.length); let turns=0; assert.deepEqual(cursor.snapshot().map(word=>word.toString(16).padStart(16,"0")),row.initialSnapshot,row.id+" initial portable words");
  for(;;){
   const saved=cursor.snapshot(),a=[...matrix],b=[...rhs],demand=cursor.demand();
   if(demand.items){for(const budget of [{...generous,items:0},{...generous,depth:0},...(demand.copyBytes?[{...generous,copyBytes:demand.copyBytes-1}]:[])]){
    assert.equal(cursor.step(matrix,rhs,budget).status.kind,"refused"); assert.deepEqual(cursor.snapshot(),saved);assert.deepEqual(matrix,a);assert.deepEqual(rhs,b);
   }}
   const turn=cursor.step(matrix,rhs,generous); assert(turn.receipt.items<=generous.items);assert(turn.receipt.copyBytes<=generous.copyBytes);assert.equal(turn.receipt.capacityBytes,0);
   const restored=DenseSolveCursor.fromSnapshot(cursor.snapshot());assert.deepEqual(restored.snapshot(),cursor.snapshot());cursor=restored;turns++;
   if(turn.status.kind==="fault"){assert.equal(turn.status.fault,(row as {fault?:string}).fault,row.id);break;}
   if(turn.status.kind==="complete"){
    assert(!("fault" in row),row.id);for(let i=0;i<rhs.length;i++)assert(Math.abs(rhs[i]!-row.solution![i]!)<=fixture.tolerance*Math.max(1,Math.abs(row.solution![i]!)),row.id);
    if(row.order){const scale=Math.max(...row.matrix.map(Math.abs));const original=Array.from({length:row.order},(_,i)=>row.matrix.slice(i*row.order,(i+1)*row.order).map(x=>x/scale));const expected=(lusolve(original,row.rhs.map(x=>x/scale)) as number[][]).flat();for(let i=0;i<rhs.length;i++)assert(Math.abs(rhs[i]!-expected[i]!)<=fixture.tolerance*Math.max(1,Math.abs(expected[i]!)),row.id+" mathjs");}
    break;
   }
   assert.equal(turn.status.kind,"pending");assert(turns<10000,row.id);
  }total+=turns;
 }
 for(const bad of [NaN,Infinity,-Infinity]){const a=[bad],b=[2],cursor=new DenseSolveCursor(1,1,1);assert.equal(cursor.step(a,b,generous).status.kind,"fault");assert(Object.is(a[0],bad));assert.deepEqual(b,[2]);}
 const overflow=new DenseSolveCursor(1,1,1),a=[1e-308],b=[1e308];let final;do{final=overflow.step(a,b,generous);}while(final.status.kind==="pending");assert.deepEqual(final.status,{kind:"fault",fault:"non-finite-arithmetic"});assert.deepEqual(b,[1e308]);
 assert.throws(()=>DenseSolveCursor.fromSnapshot(Array(12).fill(0)));
 console.log("[DEBUG] shared dense solver "+fixture.cases.length+" portable cases, "+total+" bounded turns, all refusal axes, exact snapshots and original nonfinite sources verified against Mathjs");
}

test("verifyRetainedDenseOracle",()=>verifyRetainedDenseOracle(),30000);
