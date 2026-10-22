import {test,expect} from "bun:test";
import {createRequire} from "node:module";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import probeSchema from "../../../../🔁️verification/🧬️schema/🔣️.json" with {type:"json"};
import probeDocument from "../../../../🔁️verification/🔣️probe.json" with {type:"json"};
import {openNxSdkV1} from "../../🔌️sdk/🟦️.ts";
import {nxCacheProbeV1} from "../🟦️.ts";
import type {CacheProbeSpecV1} from "../../../../../🏗️build/⚙️operations/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

const probeSpecification=probeDocument as CacheProbeSpecV1;
const root=join(import.meta.dir,"../../../../../../../../../..").replace(/[\\/]+$/u,"");

test("the neutral cache probe specification is schema-valid and renders the exact Nx workspace",()=>{
 expect(validateJsonSchemaSubset(probeSchema,probeSpecification)).toEqual([]);
 const rendered=nxCacheProbeV1(root).files(probeSpecification);
 expect(JSON.parse(JSON.stringify(rendered))).toEqual(fixture.files);
});

test("the rendered Nx workspace is accepted by the schemas shipped with Nx itself",()=>{
 const directory=openNxSdkV1(join(root,".nx/installation/package.json")).packageDirectory(),read=(name:string)=>JSON.parse(readFileSync(join(directory,"schemas",name),"utf8")),rendered=nxCacheProbeV1(root).files(probeSpecification) as Record<string,any>;
 for(const [file,name] of [["nx.json","nx-schema.json"],["project.json","project-schema.json"]] as const){
  const ajv=new Ajv({strict:false,allErrors:true,validateFormats:false}),validate=ajv.compile({...read(name),$schema:"http://json-schema.org/draft-07/schema#"});
  expect(validate(rendered[file]),file+" "+JSON.stringify(validate.errors)).toBe(true);
 }
});

test("the Nx probe invocation pins the sandboxed runtime and strips ambient task controls",()=>{
 const row=fixture.invocation,tool=nxCacheProbeV1(root),invocation=tool.invocation(probeSpecification,row.task,row.fixture,row.environment);
 expect(invocation.command).toBe(row.command);
 expect(invocation.arguments.slice(1)).toEqual(row.arguments);
 expect(invocation.arguments[0]).toBe(createRequire(join(root,".nx/installation/package.json")).resolve("nx/bin/nx.js"));
 expect(invocation.environment).toEqual(row.expectedEnvironment);
 for(const key of row.absentEnvironment)expect(key in invocation.environment,key).toBe(false);
 expect(tool.version()).toBe(JSON.parse(readFileSync(createRequire(join(root,".nx/installation/package.json")).resolve("nx/package.json"),"utf8")).version);
});
