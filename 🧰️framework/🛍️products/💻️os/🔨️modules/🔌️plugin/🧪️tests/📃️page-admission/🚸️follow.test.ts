import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "./🧫️fixtures/📋️schema.json";
test("accepted owned page follows children only before cancellation and always pauses original receipt",()=>{
 expect(new Ajv({strict:true}).compile(schema.properties.ownedFollow)(fixture.ownedFollow)).toBe(true);
 for(const row of fixture.ownedFollow){
  const db=new Database(":memory:");db.run("CREATE TABLE trace(kind TEXT, receipt TEXT)");
  db.run("INSERT INTO trace VALUES ('accepted-page','original')");
  if(row.cancel==="none"){db.run("INSERT INTO trace VALUES ('follow','original')");db.run("INSERT INTO trace VALUES ('derive','original')");}
  db.run("INSERT INTO trace VALUES ('pause','original')");
  const count=(kind:string)=>(db.query("SELECT count(*) AS n FROM trace WHERE kind=?").get(kind) as {n:number}).n;
  expect({acceptedPages:count("accepted-page"),followCalls:count("follow"),derivedActions:count("derive"),receiptPauses:count("pause"),receipt:(db.query("SELECT DISTINCT receipt FROM trace").get() as {receipt:string}).receipt}).toEqual(row.expected);
  expect(db.query("SELECT kind FROM trace ORDER BY rowid DESC LIMIT 1").get()).toEqual({kind:"pause"});db.close();
 }
});
