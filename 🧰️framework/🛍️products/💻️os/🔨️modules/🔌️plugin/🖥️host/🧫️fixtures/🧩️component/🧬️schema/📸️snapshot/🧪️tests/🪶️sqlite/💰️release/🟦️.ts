import {test,expect} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {resolve} from "node:path";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
const snapshot=resolve(import.meta.dir,"../../..");
test("Count complete erased backing has a closed scratch diagnostic and incoming release contract",()=>{
 const path=resolve(snapshot,"🧫️fixtures/🪶️sqlite/💰️release/🔣️.json");
 expect(existsSync(path),"actual Count failed/canceled retained backing contract").toBe(true);
 const fixture=JSON.parse(readFileSync(path,"utf8"));
 const schema=JSON.parse(readFileSync(resolve(snapshot,"🪶️sqlite/🧬️schema/💰️release/🔣️.json"),"utf8"));
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);
 expect(fixture.counts).toEqual([-2147483648,-1,0,1,2147483647]);
 expect(fixture.encodings).toEqual(["binary","text"]);expect(fixture.directions).toEqual(["export","import"]);
 for(const mutation of [(value:any)=>value.requests="netRetainedBytes",(value:any)=>value.incoming="unmeasuredGuess",(value:any)=>value.diagnostic="fixedCredit",(value:any)=>value.caller="resetPerStage",(value:any)=>value.foreign=true]){const wrong=structuredClone(fixture);mutation(wrong);expect(validate(wrong)).toBe(false);}
 const db=new Database(":memory:");
 db.exec("CREATE TABLE release_scope(requested INTEGER NOT NULL,admitted INTEGER NOT NULL,diagnostic INTEGER NOT NULL,incoming INTEGER NOT NULL,released INTEGER NOT NULL,CHECK(requested<=admitted+diagnostic),CHECK(released=requested+incoming));");
 const insert=db.query("INSERT INTO release_scope VALUES(?,?,?,?,?)");
 for(const sample of fixture.census){
  expect(Buffer.byteLength(sample.message,"utf8")).toBe(new TextEncoder().encode(sample.message).length);
  expect(Buffer.byteLength(sample.message,"utf8")).toBe(sample.diagnostic);
  insert.run(sample.requested,sample.admitted,sample.diagnostic,sample.incoming,sample.released);
 }
 expect(db.query("SELECT count(*) AS count FROM release_scope").get()).toEqual({count:fixture.census.length});
 expect(()=>insert.run(47,0,46,128,175)).toThrow();
 expect(()=>insert.run(46,0,46,128,173)).toThrow();db.close();
});
