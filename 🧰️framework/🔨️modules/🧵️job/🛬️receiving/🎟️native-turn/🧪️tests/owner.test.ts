import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./owner.schema.json";
import cases from "./owner.json";
/** 🧳️ A Job decoder owner admits only its original paid Source-bound components. */
test("resident native decoder owner",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? AND ? AND ? AND ? AND ? AND ? AND ?) AS admit");for(const c of cases){const r=q.get(+c.frame,+c.recipient,+c.work,+c.construction,+c.sameSource,+c.emptyOutput,+c.paid) as {admit:number};expect(Boolean(r.admit)).toBe(c.admit)}}finally{db.close()}});
