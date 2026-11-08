import {expect,test} from "bun:test";
import {compileJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import {SEMIO_SCHEMA_VENDOR_VOCABULARY_V1,semioSchemaAjvV1} from "@semio-tech/schema-ts/oracles/validator";
import fixture from "./🔣️.json";
import twin from "./twin.json";

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

test("schema twins declare exact annotation metadata without changing validation",()=>{
 const declaration=(SEMIO_SCHEMA_VENDOR_VOCABULARY_V1.keywords as Record<string,unknown>)[twin.keyword];expect(declaration).toEqual(twin.metaSchema);
 const owned=compileJsonSchemaSubset(twin.metaSchema),external=semioSchemaAjvV1().compile(twin.metaSchema);
 for(const row of twin.cases){expect(owned.validate(row.value).length===0).toBe(row.valid);expect(Boolean(external(row.value))).toBe(row.valid);const compile=()=>semioSchemaAjvV1().compile({type:"string",[twin.keyword]:row.value});if(row.valid){const validate=compile();expect(validate("value")).toBe(true);expect(validate(1)).toBe(false)}else expect(compile).toThrow()}
 expect(()=>semioSchemaAjvV1().compile({type:"string","x-semio-twn":{}})).toThrow();
});
