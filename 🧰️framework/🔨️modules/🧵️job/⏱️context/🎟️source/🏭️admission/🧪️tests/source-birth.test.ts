import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./source-birth.schema.json";
import cases from "./source-birth.json";
/** 🎟️ Source birth retains its actual inputs until successful physical admission. */
test("original fixture Source birth",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (NOT ?) AS original, (? AND ?) AS valid_birth, (NOT ? OR ?) AS valid_clock");for(const c of cases){const r=q.get(+c.born,+c.admitted,+c.performed,+c.clock,+c.born) as {original:number;valid_birth:number;valid_clock:number};expect(Boolean(r.original)).toBe(c.original);expect(!c.born||Boolean(r.valid_birth)).toBe(true);expect(Boolean(r.valid_clock)).toBe(true)}}finally{db.close()}});
