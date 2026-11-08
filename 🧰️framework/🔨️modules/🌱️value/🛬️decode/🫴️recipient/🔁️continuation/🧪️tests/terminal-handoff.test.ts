import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./terminal-handoff.schema.json";
import cases from "./terminal-handoff.json";
/** 🧷️ A terminal handoff needs the original owners, settled actions and paid header transfer. */
test("terminal frame handoff",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? AND ? AND ? AND ? AND ? AND ? AND ?) AS moved");for(const c of cases){const row=q.get(+c.sameRecipient,+c.sameWork,+c.settled,+c.empty,+c.free,+c.grant,+c.work) as {moved:number};expect(Boolean(row.moved)).toBe(c.moved)}}finally{db.close()}});
