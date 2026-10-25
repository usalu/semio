import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {ceil,divide} from "mathjs";
test("original output construction has exact neutral chunk-slot bounds",()=>{
 const fixture=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 const schema={type:"object",additionalProperties:false,required:["schema","chunkBytes","maximumTotalBytes","cases"],properties:{schema:{const:"semio.plugin.original-output-construction.v1"},chunkBytes:{const:4096},maximumTotalBytes:{const:33554432},cases:{type:"array",minItems:9,items:{type:"object",additionalProperties:false,required:["maximum","accepted","slots"],properties:{maximum:{type:"integer",minimum:0},accepted:{type:"boolean"},slots:{type:["integer","null"]}}}}}};
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 for(const row of fixture.cases){
  expect(row.maximum>0&&row.maximum<=fixture.maximumTotalBytes).toBe(row.accepted);
  if(row.accepted){
   const slots=Number((BigInt(row.maximum)+BigInt(fixture.chunkBytes)-1n)/BigInt(fixture.chunkBytes));
   expect(slots).toBe(row.slots);
   expect(ceil(divide(row.maximum,fixture.chunkBytes) as number)).toBe(row.slots);
  }
 }
 const rust=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");
 expect(rust.includes("pub fn original_birth_demands")).toBe(true);
 expect(rust.includes("pub fn from_original_construction")).toBe(true);
 expect(rust.includes("chunks.capacity()")).toBe(true);
});
