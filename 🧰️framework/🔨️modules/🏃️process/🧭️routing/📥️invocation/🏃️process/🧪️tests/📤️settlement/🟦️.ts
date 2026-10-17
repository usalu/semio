import {expect,test} from "bun:test";
import {mkdir,mkdtemp,rm,writeFile} from "node:fs/promises";
import {join,resolve} from "node:path";
import {semioSchemaAjvV1} from "@semio-tech/schema-ts/oracles/validator";
import {captureOwnedCommandResultV1} from "../../../../../🎛️owned-execution/🟦️.ts";
import {createIssuedResourceControlV1,receiveScriptProcessInvocation,scriptInvocationEnvironment,withIssuedResourceOperationV1} from "../../🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const root=resolve(import.meta.dir,"../../../../../../../..");

test("neutral canonical settlement checks the actual phase after detached children and preserves primary failures",async()=>{
 expect(semioSchemaAjvV1().compile(schema)(fixture)).toBe(true);
 await receiveScriptProcessInvocation(process.env,async invocation=>{
  const control=createIssuedResourceControlV1(invocation,"test:settlement-observer",60000),generated=join(root,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated");await mkdir(generated,{recursive:true});const area=await mkdtemp(join(generated,"settlement-"));
  try{const path=join(area,"📜️script.ts");await writeFile(path,`import{observeSettlementCaseV1}from${JSON.stringify(join(import.meta.dir,"🔎️observation/🟦️.ts"))};await observeSettlementCaseV1(JSON.parse(process.argv[2]));`,{flag:"wx"});
   for(const row of fixture.cases){const result=await withIssuedResourceOperationV1(control,()=>captureOwnedCommandResultV1(process.execPath,[path,JSON.stringify(row)],root,{control,env:scriptInvocationEnvironment(invocation,{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",NODE_DISABLE_COMPILE_CACHE:"1"}),output:{maximumBytes:16384,maximumLines:128}}));expect(result.status,result.stderr).toBe(0);const observed=JSON.parse(result.stdout.trim()) as {failed:boolean;positive:boolean;closed:boolean;preserved:boolean;message:string;order:{name:string}[]};expect(observed.positive).toBe(true);expect(observed.closed).toBe(true);expect(observed.order).toEqual([{name:"callback"},{name:"child"},{name:"observed"}]);expect(observed.failed,JSON.stringify(row)).toBe(true);expect(observed.preserved).toBe(true);if(row.failure==="deadline")expect(observed.message).toContain("deadline");if(row.failure==="cancellation")expect(observed.message).toBe("accepted cancellation");}
  }finally{await rm(area,{recursive:true,force:true});}
 });
},10_000);
