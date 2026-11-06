import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";

test("original allocation port panic fixture has one non-replayed event",()=>{
 const read=(path:string)=>JSON.parse(readFileSync(new URL(path,import.meta.url),"utf8"));const fixture=read("🧫️fixtures/🪢️port-unwind/🔣️.json"),schema=read("../🧬️schema/🪢️port-unwind/🔣️.json");expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);expect(fixture.partial.actualBytes).toBeLessThan(fixture.partial.requestBytes);const db=new Database(":memory:");try{db.exec("CREATE TABLE events(port TEXT,event TEXT,PRIMARY KEY(port,event))");expect(db.query("SELECT 0 + ? AS committed").get(fixture.partial.actualBytes)).toEqual({committed:fixture.partial.actualBytes});for(const row of fixture.cases){db.run("INSERT INTO events VALUES(?,?)",[row.port,row.event]);expect(row.expectedState).toBe(row.event==="quote"?"quoteStarted":"settlementStarted");expect(row.actual).toBe(row.event==="post"?fixture.bytes:null);expect(db.query("SELECT count(*) n FROM events WHERE port=? AND event=?").get(row.port,row.event)).toEqual({n:1});}}finally{db.close();}
});
