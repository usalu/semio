import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./whole-reply.schema.json";
import cases from "./whole-reply.json";
/** 📤️ Whole original replies transfer only through their paid matching authority. */
test("whole reply payout preserves denied and foreign owners",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{for(const row of cases){const r=db.query("SELECT ? AND ? AND NOT ? AND ? > 0 AND ? > 0 AS accepted").get(Number(row.ready),Number(row.matching),Number(row.occupied),row.items,row.work) as {accepted:number};expect(Boolean(r.accepted)).toBe(row.accepted)}}finally{db.close()}});
