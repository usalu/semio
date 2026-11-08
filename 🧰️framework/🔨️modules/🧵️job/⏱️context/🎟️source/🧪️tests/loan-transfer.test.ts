import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./loan-transfer.schema.json";
import cases from "./loan-transfer.json";
/** 🪢️ Transfer needs the actual Source, an empty destination, and original Work. */
test("Source loan direct transfer",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? AND NOT ? AND ? AND ? > 0) AS accepted");for(const row of cases){const result=q.get(+row.owner,+row.output,+row.sameSource,row.work) as {accepted:number};expect(Boolean(result.accepted)).toBe(row.transferred)}}finally{db.close()}});
