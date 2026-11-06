import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./partial-reply.schema.json";
import fixture from "./partial-reply.json";
/** 📨️ Performed work remains charged while the original Source reply is unresolved. */
test("partial work and Source reply stay independent",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE action(delta INTEGER, entered INTEGER, reply TEXT, validation TEXT)");for(const row of fixture.cases){db.exec("DELETE FROM action");db.query("INSERT INTO action VALUES(?,?,?,?)").run(row.staged,Number(row.entered),row.recorded,row.validation);const result=db.query("SELECT delta, CASE WHEN validation=? THEN ? WHEN reply != ? THEN reply WHEN entered=1 THEN ? ELSE ? END AS next FROM action").get("refused","context","none","pending","first");expect(result.delta).toBe(row.staged);expect(result.next).toBe(row.expected)}}finally{db.close()}});
