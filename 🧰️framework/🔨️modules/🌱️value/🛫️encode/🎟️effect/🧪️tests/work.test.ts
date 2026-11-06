import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./work.schema.json";
import fixture from "./work.json";
/** ⛽️ Real child work remains independent of the original child failure and quote. */
test("actual attempted work and original failure survive overrun",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE attempt(actual INTEGER, primary_error TEXT)");for(const row of fixture.cases){db.exec("DELETE FROM attempt");db.query("INSERT INTO attempt VALUES(?,?)").run(row.work,row.primary);const result=db.query("SELECT COALESCE(actual,0) AS charged, COALESCE(actual,0)>? AS overrun,primary_error FROM attempt").get(fixture.quote) as {charged:number,overrun:number,primary_error:string|null};expect(result.charged).toBe(row.charged);expect(Boolean(result.overrun)).toBe(row.overrun);expect(result.primary_error).toBe(row.primary)}}finally{db.close()}});
