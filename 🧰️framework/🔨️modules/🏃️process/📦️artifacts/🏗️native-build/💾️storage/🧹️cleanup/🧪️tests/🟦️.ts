import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import corpus from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";

test("neutral accepted image cleanup contracts agree with independent Ajv admission",()=>{
 const oracle=new Ajv({strict:true}).compile(schema),values:unknown[]=[corpus,{...corpus,extra:true},{version:1,cases:[{...corpus.cases[0],budgetMilliseconds:10001}]},{version:1,cases:[{...corpus.cases[0],identity:"receipt-only"}]},{version:1,cases:[{...corpus.cases[0],expected:{...corpus.cases[0]!.expected,authority:{}}}]}];
 const own=values.map(value=>validateJsonSchemaSubset(schema,value).length===0),independent=values.map(value=>oracle(value));
 expect(own).toEqual([true,false,false,false,false]);expect(independent).toEqual(own);console.log("[DEBUG] Image cleanup evidence contracts agree with independent Ajv; physical cleanup remains separately unqualified");
});

test("neutral accepted image cleanup decisions have an independent relational oracle",()=>{
 const database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE cases(id TEXT,lifecycle TEXT,identity TEXT,physical TEXT,settled INTEGER,budget INTEGER,deactivation TEXT,publication TEXT,primary_failure INTEGER)");
  const insert=database.query("INSERT INTO cases VALUES(?,?,?,?,?,?,?,?,?)");for(const row of corpus.cases)insert.run(row.id,row.lifecycle,row.identity,row.physical,Number(row.commandSettled),row.budgetMilliseconds,row.deactivation,row.publication,Number(row.primaryFailure));
  const rows=database.query("WITH effects AS (SELECT *,CASE WHEN settled=0 THEN 'await-command' WHEN lifecycle='published-persistent' AND publication='accepted' THEN 'release-reader' WHEN lifecycle='published-persistent' OR identity!='exact' OR physical='unknown' OR budget=0 THEN 'retain-unresolved' WHEN physical='inactive' THEN 'remove-owned' WHEN deactivation='complete' THEN 'deactivate-and-remove' ELSE 'deactivate-and-retain' END AS effect FROM cases) SELECT id,effect,CASE effect WHEN 'release-reader' THEN 'persistent' WHEN 'await-command' THEN 'held' WHEN 'remove-owned' THEN 'settled' WHEN 'deactivate-and-remove' THEN 'settled' ELSE 'unresolved' END AS custody,primary_failure AS primaryFailure,CASE WHEN effect IN ('retain-unresolved','deactivate-and-retain') THEN 1 ELSE 0 END AS cleanupFailure FROM effects ORDER BY id").all() as {id:string;effect:string;custody:string;primaryFailure:number;cleanupFailure:number}[];
  expect(rows.map(({id,...row})=>({id,expected:{...row,primaryFailure:Boolean(row.primaryFailure),cleanupFailure:Boolean(row.cleanupFailure)}}))).toEqual(corpus.cases.map(row=>({id:row.id,expected:row.expected})).sort((left,right)=>left.id<right.id?-1:left.id>right.id?1:0));
  console.log("[DEBUG] Independent SQLite agrees with neutral cleanup scenario outputs; this contract test performs no image operations");
 }finally{database.close();}
});
