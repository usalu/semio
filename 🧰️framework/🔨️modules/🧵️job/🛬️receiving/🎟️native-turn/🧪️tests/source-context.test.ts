import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./source-context.schema.json";
import cases from "./source-context.json";
/** 🧭️ A child Source callback requires its actual validated context and no held parent lock. */
test("Source child context",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? AND ?) AS callback");for(const c of cases){const r=q.get(+c.actualSource,+c.lockReleased) as {callback:number};expect(Boolean(r.callback)).toBe(c.callback)}}finally{db.close()}});
