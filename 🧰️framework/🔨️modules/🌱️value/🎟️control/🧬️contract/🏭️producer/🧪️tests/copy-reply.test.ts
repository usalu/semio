import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./copy-reply.schema.json";
import cases from "./copy-reply.json";
/** 📤️ A recorded success transfers only under its original funded authority. */
test("recorded Copy success preserves refused and unfunded replies",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{for(const row of cases){const r=db.query("SELECT ? AND ? AND (NOT ? OR ?) AND ? AND ? > 0 AND ? > 0 AS accepted").get(Number(row.settled),Number(row.success),Number(row.occupied),Number(row.replace),Number(row.matching),row.items,row.work) as {accepted:number};expect(Boolean(r.accepted)).toBe(row.accepted)}}finally{db.close()}});
