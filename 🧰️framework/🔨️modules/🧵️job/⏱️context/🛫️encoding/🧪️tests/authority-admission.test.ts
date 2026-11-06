import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./authority-admission.schema.json";
import fixture from "./authority-admission.json";
/** 🏛️ A published original authority survives refusal but only an accepted receipt makes it usable. */
test("authority birth and original receipt remain distinct",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE birth(published INTEGER, receipt INTEGER)");for(const row of fixture.cases){db.exec("DELETE FROM birth");if(row.published)db.query("INSERT INTO birth VALUES(1,?)").run(Number(row.receipt));const actual=db.query("SELECT published,receipt FROM birth").get() as {published:number,receipt:number}|null;expect(actual!==null).toBe(row.retained);expect(actual?.receipt===1).toBe(row.usable)}}finally{db.close()}});
