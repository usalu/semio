import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./frame.schema.json";
import cases from "./frame.json";
/** 🧵️ A native turn enters only with its original three bound owners. */
test("native turn bindings",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? AND ? AND ?) AS entered");for(const c of cases){const r=q.get(+c.work,+c.construction,+c.recipient) as {entered:number};expect(Boolean(r.entered)).toBe(c.entered)}}finally{db.close()}});
