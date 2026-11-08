import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./terminal-retirement.schema.json";
import cases from "./terminal-retirement.json";
/** ♻️ Retirement retains the same paid Work state through physical settlement. */
test("terminal original retirement",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? = ?) AND (NOT ? OR ?) AND (NOT ? OR ?) AS valid");for(const c of cases){const r=q.get(c.work,+c.moved,+c.posted,+c.moved,+c.empty,+c.posted) as {valid:number};expect(r.valid).toBe(1)}}finally{db.close()}});
