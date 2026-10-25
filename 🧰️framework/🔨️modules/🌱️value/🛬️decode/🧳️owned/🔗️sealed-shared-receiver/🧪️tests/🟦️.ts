import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
test("original shared receive neutral text matches independent SQLite JSON decoding",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);
 expect(validate({...fixture,maximumCopyBytes:4097})).toBe(false);
 const db=new Database(":memory:");try{for(const law of fixture.cases){
  const original=law.original;
  const encoded=JSON.stringify(original);
  const independent=db.query("SELECT json_extract(?, '$') AS text").get(encoded) as {text:string};
  expect(independent.text).toBe(law.expected);
  expect(JSON.parse(encoded)).toBe(law.expected);
  expect(law.original).toBe(original);
 }}finally{db.close();}
});
