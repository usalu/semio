import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import seedSchema from "../🧬️schema/🌾️struct-seed.json";
import seed from "./🧫️fixtures/🌾️struct-seed.json";

test("native receiving keeps original custody through the declared close ladder",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,closeOrder:["original-source","receiver","output","frame"]})).toBe(false);
 const db=new Database(":memory:");try{
  db.exec("create table owners(name text primary key,rank integer not null)");for(const [rank,name]of fixture.closeOrder.entries())db.query("insert into owners values(?,?)").run(name,rank);
  const closed=db.query("select name from owners order by rank").all()as{name:string}[];expect(closed.map(row=>row.name)).toEqual(["receiver","output","original-source","frame"]);
  for(let prefix=0;prefix<=closed.length;prefix++){const original=db.query("select count(*) as live from owners where name='original-source' and rank>=?").get(prefix)as{live:number};const receiver=db.query("select count(*) as live from owners where name='receiver' and rank>=?").get(prefix)as{live:number};expect(receiver.live<=original.live).toBe(true);}
 }finally{db.close();}
});
test("a derived record receiver seeds one part per vacant field instead of one indivisible copy",()=>{
 expect(new Ajv({strict:true}).compile(seedSchema)(seed)).toBe(true);
 const db=new Database(":memory:");try{
  const rows=db.query("select json_extract(value,'$.parts') as parts,?2+?3*json_extract(value,'$.fields')+?4*json_extract(value,'$.defaults') as planned from json_each(?1,'$.cases')").all(JSON.stringify(seed),seed.fixedParts,seed.partsPerField,seed.partsPerDefault)as{parts:number,planned:number}[];
  expect(rows.length).toBe(seed.cases.length);for(const row of rows){expect(row.planned).toBe(row.parts);expect(row.parts).toBeGreaterThan(1);}
 }finally{db.close();}
});
