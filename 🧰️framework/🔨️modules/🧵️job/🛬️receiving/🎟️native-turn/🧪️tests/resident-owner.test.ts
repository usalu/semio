import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./resident-owner.schema.json";
import cases from "./resident-owner.json";
/** 🧵️ Native actor birth moves only actual authenticated and paid owners. */
test("resident native actor birth",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? AND ? AND ? AND ? AND ? AND ? AND ?) AS moved");for(const c of cases){const r=q.get(+c.source,+c.work,+c.physical,+c.ready,+c.empty,+c.grant,+c.fuel) as {moved:number};expect(Boolean(r.moved)).toBe(c.moved)}}finally{db.close()}});
