import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./selection.schema.json";
import cases from "./selection.json";
/** 🧵️ The same resident source chooses each paid text action before entering its producer. */
test("resident paged encoder selects the original operation",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{for(const row of cases){const answer=db.query("select case when ? = 0 then ? when ? = 1 then ? when ? = 2 and ? = 0 then ? when ? = 2 and ? = 0 then ? when ? = 2 then ? else ? end as operation").get(Number(row.bound),"bind",row.phase,"validate",row.phase,Number(row.slot),"reserve",row.phase,Number(row.finished),"copy",row.phase,"finish","publish") as {operation:string};expect(answer.operation).toBe(row.operation)}}finally{db.close()}});
