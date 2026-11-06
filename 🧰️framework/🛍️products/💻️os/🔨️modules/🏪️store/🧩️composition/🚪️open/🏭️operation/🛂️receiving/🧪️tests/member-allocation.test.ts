import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {semioSchemaAjvV1} from "@semio-tech/framework/schema/oracles/validator";
import law from "./member-allocation.json" with {type:"json"};
import schema from "./member-allocation.schema.json" with {type:"json"};
test("original Member allocation quotes and actual physical settlements remain separate",()=>{
 expect(semioSchemaAjvV1({strict:true}).validate(schema,law)).toBe(true);
 const db=new Database(":memory:");
 try{for(const row of law.cases){
  const result=db.query("SELECT ?1>?2 AS quoteRefused,CASE WHEN ?1>?2 THEN 0 WHEN ?3 IN ('foreign','duplicate') THEN 1 WHEN ?3='post' AND (?4>?1 OR ?4>?2) THEN 1 ELSE 0 END AS settlementRefused,CASE WHEN ?1<=?2 AND ?3='post' THEN ?4 ELSE 0 END AS spent,?1<=?2 AND ?3 IN ('foreign','duplicate') AS pending").get(row.bytes,row.maximum,row.action,row.actual);
  expect(result).toEqual({quoteRefused:Number(row.quoteRefused),settlementRefused:Number(row.settlementRefused),spent:row.spent,pending:Number(row.pending)});
 }}finally{db.close()}
});
