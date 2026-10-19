import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

test("native receiving keeps original custody through the declared close ladder",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,closeOrder:["original-source","receiver","output","frame"]})).toBe(false);
 const db=new Database(":memory:");try{
  db.exec("create table owners(name text primary key,rank integer not null)");for(const [rank,name]of fixture.closeOrder.entries())db.query("insert into owners values(?,?)").run(name,rank);
  const closed=db.query("select name from owners order by rank").all()as{name:string}[];expect(closed.map(row=>row.name)).toEqual(["receiver","output","original-source","frame"]);
  for(let prefix=0;prefix<=closed.length;prefix++){const original=db.query("select count(*) as live from owners where name='original-source' and rank>=?").get(prefix)as{live:number};const receiver=db.query("select count(*) as live from owners where name='receiver' and rank>=?").get(prefix)as{live:number};expect(receiver.live<=original.live).toBe(true);}
 }finally{db.close();}
 console.log("[DEBUG] native receiving strict schema and SQLite closure frontier oracle passed");
});
