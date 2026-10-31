/** 🏗️ The portable header contract keeps its original guard without pretending to execute a native owner. */
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import fixture from"../🧫️fixtures/🔣️.json";
import schema from"../../🧬️schema/🏗️initialization.json";
test("portable dense header contract agrees with independent SQLite across the original cut guard",()=>{
 const row=fixture.initialization,validate=new Ajv({strict:true}).compile(schema);expect(validate(row)).toBe(true);expect(validate({...row,borrowMutableLedger:true})).toBe(false);
 const db=new Database(":memory:");try{
  db.run("create table contract(value text not null)");db.query("insert into contract values(?)").run(JSON.stringify(row));
  const relation=db.query("select json_extract(value,'$.maximumPages')=256 as capacity,json_extract(value,'$.livePages')=0 as empty,json_extract(value,'$.turns')=json_extract(value,'$.headerTurns') as header,json_extract(value,'$.turns')<=json_extract(value,'$.maximumTurns') as bounded from contract").get();expect(relation).toEqual({capacity:1,empty:1,header:1,bounded:1});
  const cuts=db.query("with recursive cuts(n) as (select 0 union all select n+1 from cuts where n<(select json_extract(value,'$.maximumTurns') from contract)) select count(*) as cuts,sum(case when n<(select json_extract(value,'$.turns') from contract) then 1 else 0 end) as cold from cuts").get();expect(cuts).toEqual({cuts:row.maximumTurns+1,cold:row.turns});expect(row.maximumTurns).toBe(257);expect(row.policy.copy).toBe(4096);
 }finally{db.close();}
});
