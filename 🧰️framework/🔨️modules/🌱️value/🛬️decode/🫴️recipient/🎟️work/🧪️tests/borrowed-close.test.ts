import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./borrowed-close.schema.json";
import cases from "./borrowed-close.json";
/** 🫴️ The original close leaf changes ownership only after Work and grant admission. */
test("borrowed recipient close",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (?>=1 AND ?) AS changed");for(const c of cases){const r=q.get(c.remaining,+c.grant) as {changed:number};expect(Boolean(r.changed)).toBe(c.changed);expect(c.paid).toBe(c.changed)}}finally{db.close()}});
