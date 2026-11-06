import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./settlement.schema.json";
import fixture from "./settlement.json";
/** 🧾️ Matching physical settlement and independent original refusals remain observable. */
test("Source and budget refusals preserve a matching physical receipt",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE quote(request INTEGER, spent INTEGER)");for(const row of fixture.cases){db.exec("DELETE FROM quote; INSERT INTO quote VALUES(1,0)");const accepted=db.query("UPDATE quote SET spent=? WHERE request=? RETURNING spent").get(row.actual,row.request) as {spent:number}|null;expect(accepted?.spent??0).toBe(row.spent);expect(accepted===null).toBe(row.pending);expect(!row.sourceValid).toBe(row.sourceRefusal);expect(accepted===null||!row.grantValid).toBe(row.budgetRefusal)}}finally{db.close()}});
