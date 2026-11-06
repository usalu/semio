import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./authority.schema.json";
import fixture from "./authority.json";
/** 🪪️ The original producer and Source turn are required to recover a recorded action reply. */
test("work action recovery binds the same producer and source turn",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE authority(issuer INTEGER, producer INTEGER, source INTEGER, reply TEXT)");db.query("INSERT INTO authority VALUES(1,1,1,?)").run(null);for(const row of fixture.cases){db.query("UPDATE authority SET reply=?").run(row.reply);const owner=db.query("SELECT reply FROM authority WHERE issuer=? AND producer=? AND source=?").get(row.issuer,row.producer,row.source) as {reply:string|null}|null;expect(owner!==null).toBe(row.allowed);expect(owner?.reply??"pending").toBe(row.recovered)}}finally{db.close()}});
