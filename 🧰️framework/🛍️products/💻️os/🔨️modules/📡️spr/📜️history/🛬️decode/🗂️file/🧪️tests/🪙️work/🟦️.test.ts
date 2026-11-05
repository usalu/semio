import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "./🧫️fixtures/📋️schema.json";
test("History work forwards selected and delegated actions independently of physical items",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 for(const row of fixture.cases){
  const db=new Database(":memory:");
  db.run("CREATE TABLE actions(sequence INTEGER PRIMARY KEY, kind TEXT, units INTEGER, physical INTEGER, failed INTEGER)");
  let outcome=["ready","closed"].includes(row.gate)?"probe":row.gate==="admitted"?"ok":"denied";
  if(row.gate==="admitted"&&!row.callbackEntered)outcome="denied";
  if(row.gate==="admitted"&&row.callbackEntered)for(const action of row.actions){
   if(action.kind==="semantic")expect(action.units).toBe(1);
   db.run("INSERT INTO actions(kind,units,physical,failed) VALUES (?,?,?,?)",action.kind,action.units,action.physicalItems,Number(action.fails));
   if(action.fails){outcome="failure";break;}
  }
  const totals=db.query("SELECT coalesce(sum(units),0) AS work,coalesce(sum(physical),0) AS physicalItems FROM actions").get() as {work:number,physicalItems:number};
  expect({...totals,outcome}).toEqual(row.expected);
  expect(totals.work).not.toBe(row.priorWork);
  expect(totals.physicalItems).not.toBe(row.priorPhysicalItems);
  if(row.gate!=="admitted")expect(row.actions).toEqual([]);
  db.close();
 }
});

test("Fold teardown and output custody advance on separate paid turns",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);for(const row of fixture.foldTransitions){const db=new Database(":memory:");db.run("CREATE TABLE trace(turn INTEGER PRIMARY KEY,units INTEGER,owner TEXT)");for(const action of row.actions)db.run("INSERT INTO trace VALUES (?,?,?)",action.turn,action.units,action.afterOwner);const result=db.query("SELECT turn,units AS work,owner FROM trace ORDER BY turn").all();expect(result).toEqual(row.expected);expect(row.expected[0].owner).toBe(row.initialOwner);if(row.id.includes("cancel"))expect(row.expected[1].owner).toBe("result");db.close();}});
