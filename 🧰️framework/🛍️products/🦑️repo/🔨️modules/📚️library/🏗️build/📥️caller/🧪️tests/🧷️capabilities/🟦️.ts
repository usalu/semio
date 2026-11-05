import {expect,test} from "bun:test";
import Ajv from "ajv";
import {applyPatch,type Operation} from "fast-json-patch";
import {createNativeBuildCallerCapabilitiesV1} from "../../🟦️.ts";
import callerSchema from "../../../🧬️schema/📥️caller.json";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema.json";

test("actual native caller constructors retain closed capabilities for dependency and Cargo commands",()=>{
 expect(new Ajv({strict:true}).compile(fixtureSchema)(fixture)).toBe(true);
 const ajv=new Ajv({allErrors:true}).compile(callerSchema);
 for(const mode of fixture.modes)for(const args of fixture.commands){
  const original=createNativeBuildCallerCapabilitiesV1(process.cwd(),args,"nx",{SEMIO_BUILD_MODE:mode});
  expect(original.arguments).toEqual(["repo",...args]);expect(original.selection).toEqual(original.arguments);
  expect(original.launches).toEqual([original.arguments]);expect(original.buildMode).toBe(mode);
  for(const row of fixture.rows){
   const value=applyPatch(structuredClone(original),row.patch as Operation[],true).newDocument;
   expect(ajv(value),row.id).toBe(row.valid);
   expect(validateJsonSchemaSubset(callerSchema,value).length===0,row.id).toBe(row.valid);
  }
 }
});

test("native caller refusal diagnoses schema paths without caller values",()=>{
 for(const row of fixture.diagnostics){
  let message="";try{createNativeBuildCallerCapabilitiesV1(process.cwd(),["run","workspace:components"],row.tool,row.environment);}catch(error){message=String(error);}
  expect(message,row.id).toContain("Build native caller capabilities refused");expect(message,row.id).toContain(row.expectedPath);expect(message,row.id).not.toContain(row.forbidden);expect(message.length).toBeLessThan(2200);
 }
});
