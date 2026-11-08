/** 🧪️ Retry restores the frame only through both original published owners. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import cases from "./retry.json";
import schema from "./retry.schema.json";
test("retry preserves the exact refused action and Source",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const sql=db.query("select (?1 and ?2 and ?3 and ?4 and ?5 and ?6) allowed");for(const c of cases){const row=sql.get(+c.issuer,+c.work,+c.refusalPublished,+c.retryPublished,+c.sameRefusal,+c.sameFrame) as {allowed:number};expect(Boolean(row.allowed)).toBe(c.restore)}}finally{db.close()}});
