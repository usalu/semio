import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./work-nested.schema.json";
import fixture from "./work-nested.json";
/** 🪆️ A nested producer gets exactly the live remaining work; reusing the parent's whole ceiling is rejected once the receipt is smaller or work was spent. */
test("nested ceiling is the live remaining work of effect and receipt",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{for(const row of fixture.cases){const result=db.query("SELECT CASE WHEN ?1 <= ?2 THEN MIN(?2 - ?1, ?3 - ?1) ELSE NULL END AS remaining").get(row.spent,row.parent,row.receipt) as {remaining:number|null};const remaining=result.remaining;expect(remaining!==null&&row.parent<=remaining).toBe(row.wholeCeilingAdmitted);expect(remaining??0).toBe(row.nestedCeiling)}}finally{db.close()}});
