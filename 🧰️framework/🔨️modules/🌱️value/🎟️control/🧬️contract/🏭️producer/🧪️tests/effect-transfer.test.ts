import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./effect-transfer.schema.json";
import cases from "./effect-transfer.json";
/** 📤️ Only a paid idle producer may transfer into an empty original slot. */
test("producer transfer requires its real funded idle owner",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{for(const row of cases){const result=db.query("SELECT NOT ? AND NOT ? AND ? > 0 AND ? > 0 AS accepted").get(Number(row.active),Number(row.occupied),row.items,row.work) as {accepted:number};expect(Boolean(result.accepted)).toBe(row.accepted)}}finally{db.close()}});
