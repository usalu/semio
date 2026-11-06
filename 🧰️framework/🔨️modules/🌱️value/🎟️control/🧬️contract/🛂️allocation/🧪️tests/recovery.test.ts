import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./recovery.schema.json";
import fixture from "./recovery.json";
/** 📬️ Only a producer-recorded original verdict can leave its matching transaction. */
test("allocation recovery transfers the actual recorded reply once",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE reply(owner INTEGER, action INTEGER, verdict TEXT)");for(const row of fixture.cases){db.exec("DELETE FROM reply");if(row.stored!==null)db.query("INSERT INTO reply VALUES(1,1,?)").run(row.stored);expect(db.query("SELECT verdict FROM reply WHERE owner=2 AND action=1").get()).toBeNull();const result=db.query("DELETE FROM reply WHERE owner=1 AND action=1 RETURNING verdict").get() as {verdict:string}|null;expect(result!==null).toBe(row.available);expect(result?.verdict??null).toBe(row.stored);expect(db.query("SELECT count(*) AS n FROM reply").get()?.n).toBe(0)}}finally{db.close()}});
