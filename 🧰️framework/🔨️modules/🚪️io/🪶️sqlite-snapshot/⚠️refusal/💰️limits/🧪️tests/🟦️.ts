import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import fixture from "../🧫️fixtures/🔣️.json";
import {SqliteAllocationControl,ValueError} from "../../../🟦️.ts";

test("original SQLite fixed ceilings are independently constrained",()=>{
 const database=new Database(":memory:");
 try{
  database.run(fixture.database.sql);
  database.run("INSERT INTO original_value VALUES(?,?)",[fixture.database.id,fixture.database.value]);
  const extent=database.query("SELECT count(*) AS rows,sum(8+length(CAST(value AS BLOB))) AS bytes FROM original_value").get() as {rows:number;bytes:number};
  expect(extent).toEqual({rows:1,bytes:fixture.database.expectedValueBytes});
  database.run("CREATE TABLE original_ceiling(id TEXT PRIMARY KEY,operation TEXT NOT NULL,count INTEGER NOT NULL,maximum INTEGER NOT NULL,initial INTEGER NOT NULL,narrowed INTEGER)");
  for(const row of fixture.cases)database.run("INSERT INTO original_ceiling VALUES(?,?,?,?,?,?)",[row.id,row.operation,row.count,row.maximum,row.initial,row.narrowedMaximum??null]);
  const decisions=database.query("SELECT id,CASE WHEN operation LIKE 'narrow%' THEN initial<=narrowed WHEN operation='databaseRows' THEN ?<=maximum WHEN operation='databaseValues' THEN ?<=maximum ELSE initial+count<=maximum END AS accepted FROM original_ceiling ORDER BY id").all(extent.rows,extent.bytes) as {id:string;accepted:number}[];
  expect(decisions).toEqual(fixture.cases.map(row=>({id:row.id,accepted:Number(row.accepted)})).sort((a,b)=>a.id.localeCompare(b.id)));
  const refusals=database.query("WITH decision AS (SELECT *,CASE WHEN operation LIKE 'narrow%' THEN initial<=narrowed WHEN operation='databaseRows' THEN ?<=maximum WHEN operation='databaseValues' THEN ?<=maximum ELSE initial+count<=maximum END AS accepted FROM original_ceiling) SELECT id,CASE WHEN accepted THEN NULL WHEN operation IN ('rows','databaseRows') THEN 'workLimit' ELSE 'ownershipLimit' END AS kind,CASE WHEN accepted THEN NULL WHEN operation IN ('rows','databaseRows') THEN 'relational SQLite resource limit: rows' WHEN operation IN ('value','databaseValues') THEN 'relational SQLite resource limit: value bytes' WHEN operation='reconstruction' THEN 'native reconstruction ownership exceeds caller limit' WHEN operation='scalar' THEN 'SQLite original scalar values exceed their ceiling' WHEN operation LIKE 'narrow%' THEN 'relational SQLite resource limit: narrowed original snapshot allowance' ELSE 'relational SQLite resource limit: allocation bytes' END AS message FROM decision ORDER BY id").all(extent.rows,extent.bytes);
  expect(refusals).toEqual(fixture.cases.map(row=>({id:row.id,kind:row.expectedKind,message:row.expectedMessage})).sort((a,b)=>a.id.localeCompare(b.id)));
  for(const row of fixture.cases.filter(row=>row.operation==='allocation'||row.operation==='nativeAllocation')){
   const control=new SqliteAllocationControl({maxAllocationBytes:row.maximum});let error:unknown;
   try{control.admit(row.count);}catch(cause){error=cause;}
   expect(error===undefined,row.id).toBe(row.accepted);
   if(error!==undefined){expect(error).toBeInstanceOf(ValueError);expect(row.expectedKind).toBe((error as ValueError).kind);}
   expect(control.ownedBytes).toBe(row.accepted?row.count:0);
  }
 }finally{database.close();}
 for(const profile of fixture.overflowProfiles)expect(BigInt(profile.maximum)+1n).toBe(1n<<BigInt(profile.pointerBits));
 console.log("[DEBUG] Original SQLite fixed limits closed "+fixture.cases.length+" numeric/borrowed database cases, independent SQLite UTF8 extent and cumulative/narrowed decisions, actual TypeScript allocation boundaries, two authored pointer profiles; Native literal and allocator custody remain separate");
});

import {existsSync} from "node:fs";
test("original receiving and SQLite ceiling examples cannot own trial schemas",()=>{expect(existsSync(new URL("../🧬️schema/🔣️.json",import.meta.url))).toBe(false);expect(existsSync(new URL("../../../../../🌱️value/🫴️receiving/🎟️turn/🧬️schema/🔣️.json",import.meta.url))).toBe(false);});
