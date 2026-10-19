import { test } from "bun:test";
import assert from "node:assert/strict";
import Ajv from "ajv";
import {inv} from "mathjs";
import {DenseInverseCursor} from "../🟦️.ts";
import type {DenseSolveStatus} from "../../🟦️.ts";
import corpus from "./🧫️fixtures/🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
const relative=(actual:number,expected:number):void=>assert(expected===0?Math.abs(actual)<=1e-12:Math.abs(actual-expected)<=corpus.relativeTolerance*Math.abs(expected),actual+" expected "+expected);
function retained(n:number,source:number[],checkDenied:boolean):{status:DenseSolveStatus;inverse:number[];turns:number}{const matrix=[...source],inverse=Array(n*n).fill(7),cursor=new DenseInverseCursor(n,matrix.length,inverse.length);for(let turn=0;turn<1000000;turn++){const d=cursor.demand(),before=cursor.snapshot();if(checkDenied&&d.items>0){const a=[...matrix],b=[...inverse];for(const budget of[{...d,items:0},{...d,depth:0},...(d.copyBytes>0?[{...d,copyBytes:d.copyBytes-1}]:[])]){assert.equal(cursor.step(matrix,inverse,budget).status.kind,"refused");assert.deepEqual(cursor.snapshot(),before);assert.deepEqual(matrix,a);assert.deepEqual(inverse,b);}}const step=cursor.step(matrix,inverse,{items:1,copyBytes:32,depth:1,capacityBytes:0});assert(step.receipt.copyBytes<=32&&step.receipt.items<=1);assert.equal(step.receipt.capacityBytes,0);const restored=DenseInverseCursor.fromSnapshot(JSON.parse(JSON.stringify(cursor.snapshot().map(String))).map(BigInt));assert.deepEqual(restored.snapshot(),cursor.snapshot());if(step.status.kind!=="pending")return{status:step.status,inverse,turns:turn+1};}throw Error("inverse budget exhausted");}

/** 🧪️ Original inverse/checkpoint values agree with independent Mathjs inversion and every identity residual. */
export function verifyRetainedInverseOracle():void{
 const validate=new Ajv({strict:true}).compile(schema),admitted:boolean=validate(corpus);assert(admitted,JSON.stringify(validate.errors));let turns=0;
 for(const row of corpus.cases){const result=retained(row.order,row.matrix,true);turns+=result.turns;if("fault"in row){assert.equal(result.status.kind,"fault");if(result.status.kind==="fault")assert.equal(result.status.fault,row.fault);}else{assert.equal(result.status.kind,"complete");result.inverse.forEach((value,i)=>relative(value,row.inverse![i]!));if(row.order>0){const scale=Math.max(...row.matrix.map(Math.abs)),matrix=Array.from({length:row.order},(_,i)=>row.matrix.slice(i*row.order,(i+1)*row.order).map(x=>x/scale)),independent=(inv(matrix) as number[][]).flat().map(x=>x/scale);independent.forEach((value,i)=>relative(result.inverse[i]!,value));}}}
 const forged=new DenseInverseCursor(2,4,4).snapshot();forged[7]=12n;assert.throws(()=>DenseInverseCursor.fromSnapshot(forged),/invalid-snapshot/);forged[7]=6n;assert.throws(()=>DenseInverseCursor.fromSnapshot(forged),/invalid-snapshot/);
 const law=corpus.large,n=law.order,matrix=Array.from({length:n},(_,i)=>Array.from({length:n},(_,j)=>i===j?law.diagonal:Math.abs(i-j)===1?law.adjacent:0)),result=retained(n,matrix.flat(),false),independent=inv(matrix) as number[][];assert.equal(result.status.kind,"complete");turns+=result.turns;for(let i=0;i<n;i++){relative(result.inverse[i*n+i]!,law.samples.diagonal[i]!);relative(result.inverse[i]!,law.samples.first[i]!);relative(result.inverse[(n-1)*n+i]!,law.samples.last[i]!);for(let j=0;j<n;j++)relative(result.inverse[i*n+j]!,independent[i]![j]!);}
 for(let i=0;i<n;i++)for(let j=0;j<n;j++){let residual=-(i===j?1:0);for(let k=0;k<n;k++)residual+=matrix[i]![k]!*result.inverse[k*n+j]!;assert(Math.abs(residual)<=law.residualTolerance);}
 console.log("[DEBUG] shared retained inverse "+corpus.cases.length+" portable cases and49x49 full Mathjs inverse/identity residuals matched across "+turns+" scalar turns");
}

test("verifyRetainedInverseOracle",()=>verifyRetainedInverseOracle(),30000);
