import assert from "node:assert/strict";
import Ajv from "ajv";
import {lusolve} from "mathjs";
import {DenseSolveCursor} from "../../../../../../../../../🧰️framework/🔨️modules/🧮️math/🧮️algebra/🔲️dense/🟦️.ts";
import fixture from "../🧫️fixtures/🌡️thermal/🔣️.json" with {type:"json"};
import schema from "../🧫️fixtures/🌡️thermal/schema.json" with {type:"json"};

function retained(matrix:number[][],rhs:number[]):number[]{const a=matrix.flat(),b=[...rhs],cursor=new DenseSolveCursor(b.length,a.length,b.length);for(let turn=0;turn<1000;turn++){const before=cursor.snapshot(),restored=DenseSolveCursor.fromSnapshot(JSON.parse(JSON.stringify(before.map(String))).map(BigInt));assert.deepEqual(restored.snapshot(),before);const denied=cursor.step(a,b,{items:0,copyBytes:64,depth:1,capacityBytes:0});assert.equal(denied.status.kind,"refused");assert.deepEqual(cursor.snapshot(),before);const step=cursor.step(a,b,{items:1,copyBytes:64,depth:1,capacityBytes:0});assert.notDeepEqual(cursor.snapshot(),before,"funded dense prefix continuation");if(step.status.kind==="complete")return b;assert.equal(step.status.kind,"pending");}throw Error("dense thermal progress exhausted");}

export function verifyThermalBalanceOracle():void{
 const validate=new Ajv({strict:true}).compile(schema);assert(validate(fixture),JSON.stringify(validate.errors));
 for(const row of fixture.cases){
  const matrix=[[13,-3],[-3,23]],free=retained(matrix,[180,440]);
  const required=row.target===null?0:retained([[13,-3,0],[-3,23,-1],[0,1,0]],[180,440,row.target])[2]!;
  const settled=retained(matrix,[180,440+row.delivered]);
  for(const [actual,expected] of [...free.map((x,i)=>[x,row.free[i]!] as const),[required,row.required] as const,...settled.map((x,i)=>[x,row.settled[i]!] as const)])assert(Math.abs(actual-expected)<=fixture.tolerance,row.id);
  const independent=(lusolve(matrix,[180,440+row.delivered]) as number[][]).flat();for(let i=0;i<2;i++)assert(Math.abs(independent[i]!-settled[i]!)<=fixture.tolerance,row.id+" Mathjs");
  assert(Math.abs(13*settled[0]!-3*settled[1]!-180)<=fixture.tolerance,row.id+" face residual");assert(Math.abs(-3*settled[0]!+23*settled[1]!-440-row.delivered)<=fixture.tolerance,row.id+" air residual");
 }
 console.log("[DEBUG] retained Energy thermal "+fixture.cases.length+" free/constrained/partial-delivery cases matched actual Mathjs and both source balance residuals");
}
