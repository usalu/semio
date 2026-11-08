import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./handback-extent.schema.json";
import cases from "./handback-extent.json";
/** 📏️ A paid original handback quotes its caller-resident output header. */
test("handback includes its actual output header",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT ? + ? + ? + ? + ? AS bytes");for(const c of cases){const row=q.get(c.payload,c.outputHeader,c.ownerHeader,c.flag,c.aliases) as {bytes:number};expect(row.bytes).toBe(c.expected)}}finally{db.close()}});
