import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {join} from "node:path";
import {OBSERVATION_LIMITS_V1,observationsHold} from "../../🟨️.mjs";
import fixture from "./🔣️.json" with {type:"json"};

for(const {report} of fixture.cases)test("a large conflicting input is scanned once per check "+(report?"with":"without")+" a stale callback",async()=>{
 const data=[...Array.from({length:fixture.uniqueRows},(_,index)=>({index,fact:true})),...fixture.changes];
 expect(data.length).toBeGreaterThan(OBSERVATION_LIMITS_V1.consistencyMemoRows);
 let fetched=0;
 const rows=data.map(({index,fact})=>Object.freeze({kind:"exists",path:join(process.cwd(),"unprobed-conflict-fixture",String(index)),get fact(){fetched++;return fact;}}));
 const db=new Database(":memory:");
 try{
  const first=db.query<{at:number;index:number},[string]>(`WITH rows AS (
   SELECT CAST(key AS INTEGER) at,json_extract(value,'$.index') AS idx,json_extract(value,'$.fact') AS fact FROM json_each(?)
  ), compared AS (SELECT *, LAG(fact) OVER (PARTITION BY idx ORDER BY at) AS previous FROM rows)
  SELECT at,idx AS "index" FROM compared WHERE previous<>fact ORDER BY at LIMIT 1`).get(JSON.stringify(data))!;
  expect(first.index).toBe(fixture.firstConflict);
  expect(first.at+1).toBe(fixture.rowsRead);
  const conflicts:unknown[]=[];
  const check=()=>observationsHold(rows,undefined,undefined,undefined,undefined,report?row=>conflicts.push(row):undefined);
  expect(await check()).toBe(false);
  expect(fetched).toBe(fixture.rowsRead);
  expect(await check()).toBe(false);
  expect(fetched).toBe(2*fixture.rowsRead);
  expect(conflicts).toEqual(report?[rows[first.at],rows[first.at]]:[]);
  if(report)expect(conflicts[0]).toBe(rows[first.at]);
 }finally{db.close();}
});
