import{test,expect}from"bun:test";
import Ajv from"ajv";
import{validateJsonSchemaSubset}from"../../🟦️.ts";
import corpus from"../🧫️fixtures/🔣️.json";
import projectionSchema from"./🧬️schema/🔣️.json";

test("first-party tuple schemas match portable cases and Ajv",()=>{
 const ids=new Set(projectionSchema.properties.cases.items.properties.id.enum),cases=corpus.cases.filter(row=>ids.has(row.id));
 expect(new Ajv({strict:true}).validate(projectionSchema,{version:1,cases})).toBe(true);
 expect(cases.map(row=>row.id).sort()).toEqual([...ids].sort());
 for(const row of cases){
  const documents:unknown[]=("documents"in row.input?row.input.documents:undefined)??[],oracle=new Ajv({strict:false,schemas:documents as object[]}).compile(row.input.schema)(row.value),actual=validateJsonSchemaSubset(row.input.schema,row.value,row.input.schema,documents).length===0;
  expect(oracle,row.id+" Ajv").toBe(row.valid);
  expect(actual,row.id+" owned").toBe(oracle);
 }
},60000);
