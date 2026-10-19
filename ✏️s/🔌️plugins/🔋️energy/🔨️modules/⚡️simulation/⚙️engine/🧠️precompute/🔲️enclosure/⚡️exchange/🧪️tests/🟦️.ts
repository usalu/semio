import assert from "node:assert/strict";
import Ajv from "ajv";
import {inv,multiply} from "mathjs";
import corpus from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import retirementCorpus from "../../../♻️retirement/🧪️tests/🧫️fixtures/🔣️.json";
import retirementSchema from "../../../♻️retirement/🧬️schema/🔣️.json";
import {EnclosureExchangeCursor} from "../🟦️.ts";
/** 🧮️ Reference numeric rows are validated at the third-party boundary. */
function numericRows(value:unknown):number[][]{assert(Array.isArray(value));return value.map(row=>{assert(Array.isArray(row));return row.map(scalar=>{assert.equal(typeof scalar,"number");return scalar as number;});});}
/** 🧪️ Actual independent matrix inversion validates portable coefficients and every original scalar continuation. */
export function verifyEnclosureExchangeOracle():void{
 const validate=new Ajv({allErrors:true,strict:true}).compile(schema);assert(validate(corpus),JSON.stringify(validate.errors));const retirementValidate=new Ajv({allErrors:true,strict:true}).compile(retirementSchema);assert(retirementValidate(retirementCorpus),JSON.stringify(retirementValidate.errors));let totalTurns=0;
 for(const row of corpus.cases){
  const {areas,emissivities,view,expected}=row,n=areas.length,matrix=Array(n*n).fill(0) as number[],inverse=Array(n*n).fill(0) as number[],exchange=Array(n*n).fill(0) as number[];
  let cursor=new EnclosureExchangeCursor(n,emissivities.length,view.length,matrix.length,inverse.length,exchange.length),turns=0,terminal;
  for(;;){
   const demand=cursor.demand();if(demand.items!==0&&(n<=4||turns%1024===0)){
    const before=cursor.snapshot(),spans=[matrix.slice(),inverse.slice(),exchange.slice()];
    for(const grant of [{...demand,items:0},{...demand,copyBytes:Math.max(0,demand.copyBytes-1)},{...demand,depth:0}]){
     if(grant.items>=demand.items&&grant.copyBytes>=demand.copyBytes&&grant.depth>=demand.depth)continue;
     const turn=cursor.step(areas,emissivities,view,matrix,inverse,exchange,grant);assert.equal(turn.status.kind,"refused");assert.deepEqual(cursor.snapshot(),before);assert.deepEqual([matrix,inverse,exchange],spans);assert.equal(turn.receipt.items,0);
    }
    const words=(JSON.parse(JSON.stringify(before.map(String))) as string[]).map(BigInt);cursor=EnclosureExchangeCursor.fromSnapshot(words);assert.deepEqual(cursor.snapshot(),before);
   }
   const turn=cursor.step(areas,emissivities,view,matrix,inverse,exchange,demand);turns++;assert(turn.receipt.items<=1&&turn.receipt.copyBytes<=40&&turn.receipt.depth<=1&&turn.receipt.capacityBytes===0);assert(turn.receipt.copyBytes<=demand.copyBytes);
   if(turn.status.kind==="complete"||turn.status.kind==="fault"){terminal=turn.status;break;}assert.equal(turn.status.kind,"pending");assert(turns<2_000_000);
  }
  if(typeof expected==="string"){assert.equal(terminal.kind,"fault",row.name);if(terminal.kind==="fault")assert.equal(terminal.fault,expected,row.name);}else{
   assert.equal(terminal.kind,"complete",row.name);let reference:number[];
   if(n===0||emissivities.every(e=>e===0))reference=Array(n*n).fill(0) as number[];else{const f=Array.from({length:n},(_,i)=>view.slice(i*n,(i+1)*n)),d=f.map((r,i)=>r.map((v,j)=>(i===j?1:0)-(1-emissivities[i]!)*v)),inverseReference=inv(d),product=numericRows(multiply(f,inverseReference));reference=product.flatMap((r,i)=>r.map((v,j)=>i===j?0:emissivities[i]!*v*emissivities[j]!));}
   for(let i=0;i<n;i++)for(let j=0;j<n;j++){const index=i*n+j;assert(Math.abs(exchange[index]!-expected[index]!)<=corpus.tolerance,row.name+" fixture coefficient "+index);assert(Math.abs(exchange[index]!-reference[index]!)<=corpus.tolerance,row.name+" actual Mathjs coefficient "+index);assert(Math.abs(areas[i]!*exchange[index]!-areas[j]!*exchange[j*n+i]!)<=1e-9,row.name+" reciprocal pair "+index);}
   const temperatures=Array.from({length:n},(_,i)=>290.15+i),net=areas.reduce((sum,area,i)=>sum+area*temperatures.reduce((flux,t,j)=>flux+exchange[i*n+j]!*(t**4-temperatures[i]!**4),0),0);const netWatts=5.670374419e-8*net;assert(Math.abs(netWatts)<=1e-9,row.name+" actual closed energy watts "+netWatts);
  }
  totalTurns+=turns;
 }
 const original=new EnclosureExchangeCursor(2,2,4,4,4,4).snapshot();for(const [index,value]of [[0,2n],[6,99n],[10,1n],[25,1n],[3,3n],[11,2n]] as const){const forged=original.slice();forged[index]=value;assert.throws(()=>EnclosureExchangeCursor.fromSnapshot(forged),/InvalidSnapshot|invalid-snapshot/);}
 const early=original.slice();early[6]=7n;early[3]=2n;early[7]=0n;assert.throws(()=>EnclosureExchangeCursor.fromSnapshot(early),/InvalidSnapshot/);
 console.log("[DEBUG] retained enclosure exchange 15 portable cases matched actual Mathjs radiosity and closed pair energy across "+totalTurns+" original scalar turns");
}
