import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./deadline-binding.schema.json";
import cases from "./deadline-binding.json";
/** 🎯️ Source decisions use only the deadline bound to their original clock. */
test("Source deadline binding",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? AND ?) AS accepted");for(const c of cases){const r=q.get(+c.bound,+c.same) as {accepted:number};expect(Boolean(r.accepted)).toBe(c.expected)}}finally{db.close()}});
