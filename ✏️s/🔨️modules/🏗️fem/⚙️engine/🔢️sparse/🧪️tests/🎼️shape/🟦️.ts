import assert from "node:assert/strict";
import {Database} from "bun:sqlite";
import law from "../🧫️fixtures/🎼️shape/🔣️.json";
import {subspaceShapeAdmitted} from "../../🎼️shape/🟦️.ts";

/** 🎼️ Independent SQLite checks exact integer quotients for native layout admission across both pointer widths. */
export function testSubspaceShapeOracle():void{
 const database=new Database(":memory:");
 try{
  const oracle=database.query<{admitted:number},[number,number,number,bigint]>("WITH input(n,p,maxn,maxcells) AS (VALUES (?,?,?,?)) SELECT n>0 AND p>0 AND p<=n AND n<=maxn AND maxcells/n>=n AS admitted FROM input");
  for(const wordBits of [32,64]as const){
   for(const row of law.cases){
    const expected=oracle.get(row.order,row.modes,2**law.indexBits-1,((1n<<BigInt(wordBits))-1n)/BigInt(law.scalarBytes))!.admitted===1;
    assert.equal(expected,wordBits===32?row.admitted32:row.admitted64);assert.equal(subspaceShapeAdmitted(row.order,row.modes,wordBits),expected);
   }
   for(const [order,modes]of [[-1,1],[1,-1],[NaN,1],[1,NaN],[Infinity,1],[Number.MAX_SAFE_INTEGER+1,1]])assert.equal(subspaceShapeAdmitted(order!,modes!,wordBits),false);
  }
  console.log(`[DEBUG] ${law.cases.length*2} sparse subspace layouts match independent SQLite exact integer pricing for 32/64-bit owners; multi-page problems pass logical admission`);
 }finally{database.close();}
}
