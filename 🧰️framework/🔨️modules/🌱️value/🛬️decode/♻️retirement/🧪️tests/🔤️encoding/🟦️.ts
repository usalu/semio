import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "./🧫️fixtures/📋️schema.json";
test("retirement forwarding preserves original custody and charges partial error and unwind effects",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);
 for(const row of fixture.cases){
  const db=new Database(":memory:");
  db.run("CREATE TABLE custody(ledger TEXT, recipient TEXT, initial_owned INTEGER, capacity INTEGER)");
  db.run("INSERT INTO custody VALUES (?,?,?,?)",fixture.ledger,fixture.recipient,fixture.initialOwned,fixture.hopCapacity);
  db.run("CREATE TABLE effects(bytes INTEGER NOT NULL)");
  let refused=0;
  for(const action of row.actions){
   const remaining=(db.query("SELECT capacity-COALESCE((SELECT SUM(bytes) FROM effects),0) AS n FROM custody").get() as {n:number}).n;
   if(action.kind==="deny"||action.bytes>remaining){refused++;continue;}
   db.run("INSERT INTO effects VALUES (?)",action.bytes);
  }
  const actual=db.query("SELECT initial_owned+COALESCE((SELECT SUM(bytes) FROM effects),0) AS owned,COALESCE((SELECT SUM(bytes) FROM effects),0) AS spent,capacity-COALESCE((SELECT SUM(bytes) FROM effects),0) AS remaining FROM custody").get();
  expect({...actual as object,refused}).toEqual(row.expected);
  expect(db.query("SELECT ledger,recipient FROM custody").get()).toEqual({ledger:fixture.ledger,recipient:fixture.recipient});
  db.close();
 }
 expect(validate({...fixture,hopCapacity:9})).toBe(false);
});
