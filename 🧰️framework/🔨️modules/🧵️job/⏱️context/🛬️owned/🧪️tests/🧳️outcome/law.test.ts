import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import cases from "./cases.json";
import schema from "./schema.json";
test("Job return keeps output and independent original refusals",()=>{expect(new Ajv({strict:true}).validate(schema,cases)).toBe(true);const db=new Database(":memory:");try{for(const c of cases){const row=db.query("select ? + ? + ? + ? + ? as owners").get(Number(c.output),Number(c.operation),Number(c.context),Number(c.nativeReceipt),Number(c.returnReceipt)) as {owners:number};expect(row.owners).toBe(c.owners)}}finally{db.close()}});
