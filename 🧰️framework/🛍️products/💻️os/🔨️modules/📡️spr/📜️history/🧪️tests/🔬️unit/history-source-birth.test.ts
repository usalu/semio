import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {semioSchemaAjvV1} from "@semio-tech/framework/schema/oracles/validator";
import law from "./history-source-birth.json" with {type:"json"};
import schema from "./history-source-birth.schema.json" with {type:"json"};
test("original History source keeps input or publishes the born owner before settlement",()=>{
 expect(semioSchemaAjvV1({strict:true}).validate(schema,law)).toBe(true);
 const db=new Database(":memory:");
 try{for(const row of law.cases){
  expect(db.query("SELECT NOT ?1 AS input,?1 AS output,CASE WHEN ?1 THEN ?3 WHEN ?2 THEN ?4 ELSE 0 END AS work").get(Number(row.allocated),Number(row.attempted),law.quoteWork,law.refusedWork)).toEqual({input:Number(row.input),output:Number(row.output),work:row.work});
 }}finally{db.close()}
});

test("History birth quotes the issued shared admission plus both fixed owner moves",()=>{const db=new Database(":memory:");try{for(const row of law.layoutCases)expect(db.query("SELECT ?2+2*?1 AS quote").get(row.sourceHeader,row.admission)).toEqual({quote:row.quote});}finally{db.close()}});
