import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./payload-birth.schema.json";
import cases from "./payload-birth.json";
/** 💾️ Payload backing records actual allocator effects and retains input until publication. */
test("payload birth custody",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (NOT ?) AS input, (? OR ? IS NULL) AS extent");for(const c of cases){const r=q.get(+c.published,+c.backing,c.actual) as {input:number;extent:number};expect(Boolean(r.input)).toBe(c.input);if(c.actual!==null&&c.actual>0)expect(c.backing).toBe(true);if(c.actual===0)expect(c.backing).toBe(false)}}finally{db.close()}});
