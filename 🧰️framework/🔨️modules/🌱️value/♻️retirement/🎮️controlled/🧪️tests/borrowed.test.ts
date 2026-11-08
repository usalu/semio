import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./borrowed.schema.json";
import cases from "./borrowed.json";
/** 🧵️ Controlled retirement recovers only the same unrecorded child invocation. */
test("controlled borrowed recovery",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? AND NOT ? AND ?) AS recover");for(const c of cases){const r=q.get(+c.entered,+c.recorded,+c.same) as {recover:number};expect(Boolean(r.recover)).toBe(c.recover)}}finally{db.close()}});
