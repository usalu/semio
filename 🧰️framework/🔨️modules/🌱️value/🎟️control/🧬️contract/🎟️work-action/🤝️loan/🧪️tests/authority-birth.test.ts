import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./authority-birth.schema.json";
import cases from "./authority-birth.json";
/** 🏛️ An original authority birth retains its producer until paid admission. */
test("Work and construction authority birth",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? AND NOT ? AND ? AND ? > 0) AS admitted");for(const c of cases){const r=q.get(+c.owner,+c.output,+c.sameSource,c.work) as {admitted:number};expect(Boolean(r.admitted)).toBe(c.admitted)}}finally{db.close()}});
