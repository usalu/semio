import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {semioSchemaAjvV1} from "@semio-tech/framework/schema/oracles/validator";
import law from "./history-source-birth.json" with {type:"json"};
import schema from "./history-source-birth.schema.json" with {type:"json"};
test("original History source keeps input or publishes the born owner before settlement",()=>{
 expect(semioSchemaAjvV1({strict:true}).validate(schema,law)).toBe(true);
 const db=new Database(":memory:");
 try{for(const row of law.cases){
  expect(db.query("SELECT NOT ?1 AS input,?1 AS output,CASE WHEN ?2 THEN ?3 ELSE 0 END AS work").get(Number(row.allocated),Number(row.attempted),law.quoteWork)).toEqual({input:Number(row.input),output:Number(row.output),work:row.work});
 }}finally{db.close()}
});

test("History birth quotes the larger success or refused-input restoration copy frontier",()=>{const db=new Database(":memory:");try{for(const row of law.layoutCases)expect(db.query("SELECT max(2*?1+?2,2*?2) AS quote,2*?1+?2 AS success,2*?2 AS refused").get(row.sourceHeader,row.historyHeader)).toEqual({quote:row.quote,success:row.success,refused:row.refused});}finally{db.close()}});
