/** 🧪️ Original chunk ordinals match independent SQLite concatenation and every neutral cut. */
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import{readFileSync}from"node:fs";
import{OriginalVectorConcat,type OriginalVectorConcatDemand,type OriginalVectorConcatGrant}from"../🟦️.ts";
const read=(path:string)=>JSON.parse(readFileSync(new URL(path,import.meta.url),"utf8"));
const grant=(d:OriginalVectorConcatDemand):OriginalVectorConcatGrant=>({maximumItems:1,maximumCopyBytes:d.copyBytes,maximumCapacityBytes:d.capacityBytes,maximumReleaseBytes:d.releaseBytes,maximumDepth:d.depth});
const denied=(d:OriginalVectorConcatDemand):OriginalVectorConcatGrant[]=>{const issued=grant(d),result=[{...issued,maximumItems:0}];for(const[currency,demand]of[["maximumCopyBytes",d.copyBytes],["maximumCapacityBytes",d.capacityBytes],["maximumReleaseBytes",d.releaseBytes],["maximumDepth",d.depth]]as const)if(demand>0)result.push({...issued,[currency]:demand-1});return result;};
test("owned vector concatenation keeps original row order under denial and cancellation",()=>{
 const law=read("🧫️fixtures/🔣️.json");expect(new Ajv({strict:true}).compile(read("../🧬️schema/🔣️.json"))(law)).toBe(true);
 const db=new Database(":memory:");db.run("CREATE TABLE original_rows(chunk INTEGER,ordinal INTEGER,payload TEXT,PRIMARY KEY(chunk,ordinal))");
 law.chunks.forEach((rows:string[],chunk:number)=>rows.forEach((payload,ordinal)=>db.run("INSERT INTO original_rows VALUES(?,?,?)",[chunk,ordinal,payload])));
 const expected=(db.query("SELECT payload FROM original_rows ORDER BY chunk,ordinal").all()as{payload:string}[]).map(row=>row.payload);
 expect(expected).toEqual(law.expected);
 for(const cut of[...law.cancelCuts,Infinity]){
  const owner=new OriginalVectorConcat<string>(24);let turns=0;
  for(const chunk of law.chunks){if(turns===cut)break;const original={rows:[...chunk]},d=owner.pushDemand();for(const refused of denied(d)){expect(owner.push(original,refused)).toBe(false);expect(original.rows).toEqual(chunk);}expect(owner.push(original,grant(d))).toBe(true);expect(original.rows).toEqual([]);turns++;}
  const finish=owner.finishDemand();for(const refused of denied(finish))expect(owner.finish(refused)).toBe(false);owner.finish(grant(finish));
  while(!owner.ready&&turns<cut){const d=owner.demand();for(const refused of denied(d))expect(owner.advance(refused)).toBe(false);owner.advance(grant(d));turns++;}
  if(cut===Infinity)expect(owner.take(grant(owner.takeDemand()))).toEqual(expected);
  owner.beginClose();while(!owner.terminal){const d=owner.closeDemand();for(const refused of denied(d))expect(owner.closeStep(refused)).toBe(false);owner.closeStep(grant(d));}
  expect(owner.terminal).toBe(true);
 }
 db.close();console.log("[DEBUG] original vector chunks match SQLite ordinals with six cancellation cuts and unfunded custody preservation");
});
