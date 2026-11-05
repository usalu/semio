/** 📏️ Independent JSON Schema and relational limits validate request data without claiming native authority. */
import Ajv from "ajv";
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";

test("MCP inference exposes finite requested bounds and refuses private native turn data",async()=>{
 const source=await Bun.file(new URL("../🧫️fixtures/call-request.json",import.meta.url)).json();
 const component=await Bun.file(new URL("../../../🧬️schema/🔣️.json",import.meta.url)).json();
 const ajv=new Ajv({strict:false,allErrors:true});ajv.addSchema(component,"mcp-data");
 const validate=ajv.compile({$ref:"mcp-data#/$defs/InferenceRunInput"}),policy=ajv.compile({$ref:"mcp-data#/$defs/InferenceRetainedPolicy"});
 const request={...source,artifactKind:"test.neutral.document",inferenceSchema:"test.neutral.inference"};
 expect(validate(request)).toBe(true);expect(policy(source.retainedPolicy)).toBe(true);
 for(const key of ["documentGeneration","retainedPolicy","revision","cancellationId","workUnits","maximumElapsedMilliseconds"]){const missing={...request};delete missing[key];expect(validate(missing)).toBe(false);}
 expect(validate({...request,turn:{operation:71,generation:3,epoch:19}})).toBe(false);
 for(const value of [-1,1.5,9007199254740992])expect(validate({...request,documentGeneration:value})).toBe(false);
 const database=new Database(":memory:");
 try{
  database.exec("create table requested(axis text primary key,bound integer check(typeof(bound)='integer' and bound between 0 and 4294967295))");
  for(const[key,bound]of Object.entries(source.retainedPolicy))database.query("insert into requested values(?,?)").run(key,bound as number);
  expect(database.query("select count(*) count from requested").get()).toEqual({count:5});
  expect(database.query("select bound from requested where axis=?").get("maximumCopyBytes")).toEqual({bound:3});
  expect(()=>database.query("insert into requested values(?,?)").run("wide",4294967296)).toThrow();
  expect(()=>database.query("insert into requested values(?,?)").run("fractional",1.5)).toThrow();
 }finally{database.close();}
});
