/** 🧫️ Registers the complete caller-owned compute witness with independent test oracles. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import * as TOML from "@iarna/toml";
import {verifyComputeConsumer,type ComputeConsumerDescriptor} from "../🟦️.ts";
import {validateJsonSchemaSubset,requireRecord,type UnknownRecord} from "../../../../../🧬️schema/✅️validator/🟦️.ts";
const dependencies=(source:string):string[]=>{const parsed=requireRecord(Bun.TOML.parse(source),"manifest");return Object.keys(parsed.dependencies===undefined?{}:requireRecord(parsed.dependencies,"dependencies"));};
export function registerComputeConsumerTests(descriptor:ComputeConsumerDescriptor,schema:UnknownRecord,read:(path:string)=>string):void{
 test("closed compute consumer descriptor agrees with independent schema admission",()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(descriptor)).toBe(true);expect(validateJsonSchemaSubset(schema,descriptor)).toEqual([]);for(const hostile of [{...descriptor,extra:true},{...descriptor,bindings:[]},{...descriptor,bindings:["ForeignEngine"]}]){expect(validate(hostile)).toBe(false);expect(validateJsonSchemaSubset(schema,hostile).length).toBeGreaterThan(0);}});
 test("actual consumer directly binds the canonical compute family",()=>{
  const source=read(descriptor.source);for(const name of descriptor.bindings)expect(source).toContain("semio_framework_2d::compute::"+name);expect(source).not.toMatch(/\b(?:store|semio_framework_os_kernel)::Engine(?:Handles|Cache|Key|Handle|Fault|Rep)?\b|\bKernelEngineHandle\b/);
  const manifest=read(descriptor.manifest),native=dependencies(manifest),oracle=TOML.parse(manifest);expect(native).toEqual(Object.keys(oracle.dependencies??{}));expect(Object.keys(oracle.dependencies??{}),descriptor.id).toContain("semio-framework-2d");
  expect(verifyComputeConsumer(descriptor,{read,dependencies})).toEqual({directDependency:true,bindings:descriptor.bindings});
 });
 test("a declared but broken owner refuses instead of skipping",()=>{for(const missing of [descriptor.source,descriptor.manifest])expect(()=>verifyComputeConsumer(descriptor,{read:path=>{if(path===missing)throw Error("Missing owned source");return read(path);},dependencies})).toThrow("Missing owned source");});
}
