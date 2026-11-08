import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./terminal-refusals.schema.json";
import cases from "./terminal-refusals.json";
/** ⚠️ Refusals remain separate owned causes across terminal settlement. */
test("independent terminal refusals",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? + 2 * ? + 4 * ? + 8 * ?) AS mask");for(const c of cases){const r=q.get(+c.quote,+c.work,+c.overrun,+c.post) as {mask:number};expect(r.mask).toBe(c.mask)}}finally{db.close()}});
