import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./terminal-phases.schema.json";
import cases from "./terminal-phases.json";
/** 🧾️ Physical ownership stays hidden until its own original receipt is settled. */
test("terminal physical settlement",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (NOT ? OR ?) AND (NOT ? OR ?) AND (NOT ? OR ?) AND (? = ? + ?) AS valid");for(const c of cases){const r=q.get(+c.moved,+c.quoted,+c.posted,+c.moved,+c.outputs,+c.posted,c.work,+c.moved,+c.outputs) as {valid:number};expect(r.valid).toBe(1)}}finally{db.close()}});
