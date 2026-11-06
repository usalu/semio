import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {semioSchemaAjvV1} from "@semio-tech/framework/schema/oracles/validator";
import law from "./history-receipt.json" with {type:"json"};
import schema from "./history-receipt.schema.json" with {type:"json"};
test("history receipt mismatch preserves the original failure and independent physical receipt",()=>{
 expect(semioSchemaAjvV1({strict:true}).validate(schema,law)).toBe(true);
 const db=new Database(":memory:");
 try{for(const row of law.cases){
  expect(db.query("SELECT ?1 AS primaryCause,NOT ?2 AS receiptRefused,?3 AS copiedBytes").get(law.primary,Number(row.grantMatches),row.copiedBytes)).toEqual({primaryCause:law.primary,receiptRefused:Number(row.receiptRefused),copiedBytes:row.copiedBytes});
 }}finally{db.close()}
});
