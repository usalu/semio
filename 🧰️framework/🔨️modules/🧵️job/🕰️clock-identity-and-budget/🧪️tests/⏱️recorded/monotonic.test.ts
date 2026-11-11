import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./monotonic.schema.json";
import cases from "./monotonic.json";
/** 🧭️ Elapsed microseconds are the unsigned difference of two marks and never negative. */
test("Monotonic port elapsed microseconds",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT CASE WHEN CAST(? AS INTEGER) >= CAST(? AS INTEGER) THEN CAST(? AS TEXT) END AS elapsed");for(const c of cases){const r=q.get(c.now.toString(),c.origin.toString(),(BigInt(c.now)-BigInt(c.origin)).toString()) as {elapsed:string|null};expect(r.elapsed===null?null:Number(BigInt(r.elapsed))).toBe(c.elapsedMicros===null?null:Number(BigInt(c.elapsedMicros)))}}finally{db.close()}});
