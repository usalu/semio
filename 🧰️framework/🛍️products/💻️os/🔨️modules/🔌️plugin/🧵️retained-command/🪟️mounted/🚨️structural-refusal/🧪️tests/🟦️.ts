import {test,expect} from "bun:test";
import Ajv from "ajv";
import corpus from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

/** 🚨️ Independent classifier: the same verdict the Rust mounted operation reaches, written without any of its code. */
function structuralRefusal(row:{refusal:{kind:string;message:string}|null;closeFault:{code:string;message:string}|null}):string|null{
  if(row.closeFault)return `${corpus.closeFaultPrefix}${row.closeFault.code}: ${row.closeFault.message}`;
  if(row.refusal&&corpus.structuralRefusalKinds.includes(row.refusal.kind))return `${corpus.refusedOwnerPrefix}${row.refusal.message}`;
  return null;
}

test("the structural refusal corpus is schema-valid",()=>{
  const validate=new Ajv({strict:false}).compile(schema);
  expect(validate(corpus)).toBe(true);
});

test("every case reaches the verdict the retained mounted operation reaches",()=>{
  for(const row of corpus.cases)expect(structuralRefusal(row),row.name).toBe(row.expected);
});
