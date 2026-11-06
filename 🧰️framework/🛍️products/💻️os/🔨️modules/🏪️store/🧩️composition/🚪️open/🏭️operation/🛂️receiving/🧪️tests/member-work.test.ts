import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {semioSchemaAjvV1} from "@semio-tech/framework/schema/oracles/validator";
import law from "./member-work.json" with {type:"json"};
import schema from "./member-work.schema.json" with {type:"json"};
test("original native work uses independent actual effects and preserves its semantic result",()=>{
 expect(semioSchemaAjvV1({strict:true}).validate(schema,law)).toBe(true);
 const db=new Database(":memory:");
 try{for(const row of law.cases){
  const value=db.query("SELECT max(?1,?2) AS posted,?1<?2 AS refused,?3 AS original").get(row.report,row.observed,Number(row.primary));
  expect(value).toEqual({posted:row.posted,refused:Number(row.refused),original:Number(row.primary)});
  expect(row.effects).toBe(Number(row.request<=law.budget));
 }}finally{db.close()}
});

test("original retirement admits its actual frame birth before owner transfer",()=>{
 const db=new Database(":memory:");
 try{for(const row of law.retirement.cases){
  const result=db.query("SELECT CASE WHEN ?1>=?2 THEN ?2 ELSE 0 END AS work, ?1<?2 AS owner, ?1>=?2 AS active").get(row.budget,law.retirement.quote);
  expect(result).toEqual({work:row.work,owner:Number(row.owner),active:Number(row.active)});
 }}finally{db.close()}
});
