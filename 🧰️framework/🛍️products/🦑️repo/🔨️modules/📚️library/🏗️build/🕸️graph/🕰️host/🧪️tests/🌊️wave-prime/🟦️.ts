import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {fileURLToPath} from "node:url";
import cases from "./🧫️.json";

const fixture=fileURLToPath(new URL("./🧫️fixture.ts",import.meta.url));

for(const row of cases)test(row.name,async()=>{
 const child=Bun.spawn([process.execPath,fixture,JSON.stringify(row.waves)],{stdin:"ignore",stdout:"pipe",stderr:"pipe"});
 const [code,stdout,stderr]=await Promise.all([child.exited,new Response(child.stdout).text(),new Response(child.stderr).text()]);
 expect(code,stderr).toBe(0);
 const {outcomes,trace}=JSON.parse(stdout.trim().replace(/^\[result\] /u,""));
 const database=new Database(":memory:");
 try{
  const oracle=database.query("SELECT key, CASE WHEN value = 'cancelled' THEN 'cancelled' ELSE 'prepared' END AS outcome FROM json_each(?) ORDER BY key").all(JSON.stringify(row.waves)) as {key:number;outcome:string}[];
  expect(outcomes).toEqual(row.outcomes);
  expect(outcomes).toEqual(oracle.map(entry=>entry.outcome));
  expect(database.query("SELECT count(*) AS primes FROM json_each(?) WHERE value = 'prime'").get(JSON.stringify(trace))).toEqual({primes:0});
  expect(trace).toEqual(row.trace);
 }finally{database.close();}
},30000);
