import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

test("workspace source-view settlement preserves owner generation and ordered effects against SQLite",()=>{
 const validate=new Ajv({strict:true}).compile(schema);
 expect(validate(fixture)).toBe(true);
 expect(validate({...fixture,extra:true})).toBe(false);
 const db=new Database(":memory:");
 try{
  for(const row of fixture.cases){
   const oracle=db.query("select case when ?1='disconnect' then 'disconnect' when not ?2 or ?3<>?4 or (?5 and not ?6) or (?1='load' and (?7<>?8 or ?9)) then 'ignore' else 'adopt' end outcome").get(row.kind,Number(row.windowPresent),row.operation,row.currentOperation,Number(row.cancelled),Number(row.authoritative),row.expectedGeneration,row.currentGeneration,Number(row.dirty))as{outcome:string};
   const model=row.kind==="disconnect"?"disconnect":!row.windowPresent||row.operation!==row.currentOperation||(row.cancelled&&!row.authoritative)||(row.kind==="load"&&(row.expectedGeneration!==row.currentGeneration||row.dirty))?"ignore":"adopt";
   expect(model,row.name).toBe(row.expected);expect(oracle.outcome,row.name).toBe(row.expected);
  }
  const order=db.query("select group_concat(value,'→') value from json_each(?)").get(JSON.stringify(fixture.effectOrder))as{value:string};
  expect(order.value).toBe("worker-settled→model-adopted→view-remounted→pending-started");
 }finally{db.close();}
});
