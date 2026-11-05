import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "./🧫️fixtures/📋️schema.json";
test("bounded edit work records admitted attempts including failed actions and resets each call",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 for(const row of fixture.cases)for(const call of row.calls){
  const db=new Database(":memory:");db.run("CREATE TABLE attempts(sequence INTEGER PRIMARY KEY, action TEXT NOT NULL, failed INTEGER NOT NULL)");
  let outcome="ok";
  for(const action of call.actions){
   expect(fixture.phases[row.format]).toContain(action.action);
   if(!action.admitted){outcome="denied";break;}
   db.run("INSERT INTO attempts(action,failed) VALUES (?,?)",action.action,Number(action.fails));
   if(action.fails){outcome="failure";break;}
  }
  const attempted=(db.query("SELECT action FROM attempts ORDER BY sequence").all() as {action:string}[]).map(row=>row.action);
  const work=(db.query("SELECT count(*) AS work FROM attempts").get() as {work:number}).work;
  expect({work,attempted,outcome}).toEqual(call.expected);expect(work).not.toBe(call.priorWork);db.close();
 }
});
