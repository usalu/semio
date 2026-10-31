import {expect,test} from "bun:test";
import {compileJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import {SEMIO_SCHEMA_VENDOR_VOCABULARY_V1,semioSchemaAjvV1} from "@semio-tech/schema-ts/oracles/validator";
import fixture from "./🔣️.json";

test("localized schema descriptions require the declared locale contract",()=>{
 const declaration=(SEMIO_SCHEMA_VENDOR_VOCABULARY_V1.keywords as Record<string,{$ref?:string}>)[fixture.keyword];
 expect(declaration?.$ref).toBeDefined();
 const ajv=semioSchemaAjvV1(),external=ajv.getSchema(declaration!.$ref!);
 expect(external).toBeDefined();
 const owned=compileJsonSchemaSubset(external!.schema);
 for(const row of fixture.cases){
  expect(Boolean(external!(row.value))).toBe(row.valid);
  expect(owned.validate(row.value).length===0).toBe(row.valid);
  const compile=()=>semioSchemaAjvV1().compile({type:"string",[fixture.keyword]:row.value});
  if(row.valid)expect(compile()("value")).toBe(true);
  else expect(compile).toThrow();
 }
});
