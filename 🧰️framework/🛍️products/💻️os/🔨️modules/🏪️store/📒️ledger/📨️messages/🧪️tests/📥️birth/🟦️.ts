import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import law from "./🔣️.json";
import schema from "./🧬️schema.json";
test("original message retirement keeps denied custody and admits one actual child",()=>{
 expect(new Ajv().compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");
 try{
  db.exec("create table custody(rank integer,event text,original integer,cursor integer)");
  law.states.forEach((row,index)=>db.query("insert into custody values(?,?,?,?)").run(index,row.event,row.original,row.cursor));
  expect(db.query("select count(*) as n from custody where event like 'refuse-%' and original=1 and cursor=0").get()).toEqual({n:4});
  expect(db.query("select max(cursor) as n from custody").get()).toEqual({n:law.maximumConcurrentChildFrames});
  expect(db.query("select original+cursor as n from custody where event='admit'").get()).toEqual({n:1});
  expect(db.query("select original+cursor as n from custody where event='close'").get()).toEqual({n:0});
  expect(law.policy.maximumCapacityBytes).toBe(4096);
 }finally{db.close()}
});
