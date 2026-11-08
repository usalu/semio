import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./transfer.schema.json";
import cases from "./transfer.json";
/** 📤️ The original receiving result moves only under its actual funded slot. */
test("paid native turn result transfer",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? AND ? AND ?>=1 AND ?) AS transfer");for(const c of cases){const r=q.get(+c.ready,+c.empty,c.work,+c.copy) as {transfer:number};expect(Boolean(r.transfer)).toBe(c.transfer)}}finally{db.close()}});
