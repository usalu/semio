import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./continuation-close.schema.json";
import cases from "./continuation-close.json";
/** 🧹️ Closing a continuation requires its settled owner, grant and actual Work. */
test("original encoder continuation close",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? AND ? AND ?) AS closed");for(const c of cases){const r=q.get(+c.ready,+c.grant,+c.work) as {closed:number};expect(Boolean(r.closed)).toBe(c.closed)}}finally{db.close()}});
