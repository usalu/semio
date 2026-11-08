import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./settlement-verdict.schema.json";
import cases from "./settlement-verdict.json";
/** 🧾️ A settled verdict is available only from a recorded original reply. */
test("construction settlement verdict",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? = (? IS NOT NULL)) AS valid");for(const c of cases){const r=q.get(+c.recorded,c.succeeded===null?null:+c.succeeded) as {valid:number};expect(r.valid).toBe(1)}}finally{db.close()}});
