import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./completion-retry.schema.json";
import cases from "./completion-retry.json";
/** 🔁️ Retrying a sealed completion requires its separately retained original payout refusal. */
test("original completion retry custody",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? AND ? AND ? AND ? AND ? AND ?) AS moved");for(const c of cases){const row=q.get(+c.priorPublished,+c.sameAction,+c.sameWork,+c.free,+c.grant,+c.work) as {moved:number};expect(Boolean(row.moved)).toBe(c.moved)}}finally{db.close()}});
