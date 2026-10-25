import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

/** 🪢️ Independent relational cases conserve an admitted typed result alongside its refused receipt. */
test("typed native construction preserves output and simultaneous original causes",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);
 const db=new Database(":memory:");try {for(const row of fixture.cases){
  const oracle=db.query("SELECT (? AND NOT ?) AS output, (? AND NOT ?) AS receipt").get(Number(row.admitted),Number(row.primary),Number(row.admitted),Number(row.accept)) as {output:number;receipt:number};
  expect(Boolean(oracle.output)).toBe(row.output);expect(Boolean(oracle.receipt)).toBe(row.receipt);
  const original={text:"original 源",receipt:{items:1}},output={original},primary={kind:"invalidValue",original},receipt={kind:"invariantViolated",original};
  const result=row.admitted?(row.primary?{error:primary}:{value:output}):undefined,refused=row.admitted&&!row.accept?receipt:undefined;
  expect(result&&"value"in result?result.value:undefined).toBe(row.output?output:undefined);
  expect(result&&"error"in result?result.error:undefined).toBe(row.admitted&&row.primary?primary:undefined);
  expect(refused).toBe(row.receipt?receipt:undefined);
 }}finally{db.close()}
});
