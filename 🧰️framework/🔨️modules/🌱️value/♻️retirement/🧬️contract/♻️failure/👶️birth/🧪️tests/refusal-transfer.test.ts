import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {semioSchemaAjvV1} from "@semio-tech/framework/schema/oracles/validator";
import law from "./refusal-transfer.json";
import schema from "./refusal-transfer-schema.json";
test("original refusal custody requires both an empty output and remaining performed work",()=>{expect(semioSchemaAjvV1({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{for(const row of law.cases){const result=db.query("select (? and not ? and ? < 1) transferred,case when (? and not ? and ? < 1) then 1 else 0 end work").get(Number(row.copy),Number(row.occupied),row.alreadySpent,Number(row.copy),Number(row.occupied),row.alreadySpent)as{transferred:number,work:number};expect(result).toEqual({transferred:Number(row.transferred),work:row.work})}expect(law.slots).toBe(2)}finally{db.close()}});
