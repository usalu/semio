import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./terminal-metadata.schema.json";
import cases from "./terminal-metadata.json";
/** 📬️ An independent metadata refusal keeps original data sealed and intact. */
test("terminal failure metadata custody",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? = 10) AND (? = (? AND ? AND ? = 0)) AS valid");for(const c of cases){const r=q.get(c.original,+c.visible,+c.moved,+c.settled,c.metadata) as {valid:number};expect(r.valid).toBe(1)}}finally{db.close()}});
