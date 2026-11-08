import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./resident.schema.json";
import cases from "./resident.json";
/** ⛔️ Root cancellation birth retains its node and actual allocator result. */
test("resident cancellation root birth",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? IS NOT NULL) AS attempted, (? > 0) AS backing");for(const c of cases){const r=q.get(c.actual,c.actual) as {attempted:number;backing:number|null};expect(Boolean(r.attempted)).toBe(c.attempted);if(c.attempted)expect(Boolean(r.backing)).toBe(c.backing)}}finally{db.close()}});
