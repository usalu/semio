import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./physical-payout.schema.json";
import cases from "./physical-payout.json";
/** 🧾️ The original physical refusal is visible only after its real payout. */
test("physical receipt publication requires the actual payout",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{for(const row of cases){const result=db.query("select (? = 'published') and ? as visible").get(row.phase,Number(row.payout)) as {visible:number};expect(Boolean(result.visible)).toBe(row.visible)}}finally{db.close()}});
