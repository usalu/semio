import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Command} from "commander";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {cargoPlanningOutputBytesV1,captureCargoCompilationGraphV1,projectCargoCompilerPlanningArgumentsV1,type CargoCompilerPlanningPortV1} from "../🟦️.ts";
import {registerCargoLockEnsurerV1} from "../../../../../🦀️cargo-workspace/🟦️.ts";
import schema from "../🧬️schema/📋️projection.json" with {type:"json"};
import policy from "../🔣️policy.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
test("Cargo run compiler projection preserves exact selectors and excludes runtime arguments",()=>{
 expect(new Ajv({strict:true}).validate(schema,policy)).toBe(true);
 for(const row of fixture.cases){const original=[...row.arguments],observed=projectCargoCompilerPlanningArgumentsV1(original,fixture.facts);expect(observed,row.name).toEqual(row.expected);expect(original).toEqual(row.arguments);if(original[0]==="run"){const index=original.indexOf("--"),head=index<0?original:original.slice(0,index),oracle=new Command().exitOverride().allowUnknownOption().allowExcessArguments().option("--bin <name>").option("--example <name>").parse(head.slice(1),{from:"user"}).opts<{bin?:string;example?:string}>();if(oracle.bin)expect(observed).toContain(oracle.bin);if(oracle.example)expect(observed.some(arg=>arg.includes(oracle.example!))).toBe(true);expect(observed).not.toContain("--runtime");}}
 const ambiguous={...fixture.facts,packages:fixture.facts.packages.map(row=>({...row,default_run:null}))};expect(()=>projectCargoCompilerPlanningArgumentsV1(["run"],ambiguous)).toThrow();
});

test("Cargo planning verifies the lockfile before locked metadata and repairs one stale failure",async()=>{
 const calls:boolean[]=[];registerCargoLockEnsurerV1(async request=>{calls.push(request.force);return"refreshed";});
 let metadata=0;const stale="error: cannot update the lock file /x/Cargo.lock because --locked was passed to prevent this";
 const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>{},yieldContinuation:async()=>{}},root="/x";
 const plan=(args:string[],capture:(args:readonly string[])=>unknown)=>captureCargoCompilationGraphV1({workspaceRoot:root,cwd:root,program:"cargo",prefix:[],args,environment:{},maximumOutputBytes:1024},control,{capture:async(_program:string,argv:readonly string[])=>capture(argv)} as unknown as CargoCompilerPlanningPortV1).then(()=>null,error=>error);
 const failure=await plan(["build","--manifest-path","/x/Cargo.toml"],argv=>{if(argv[0]==="metadata"){metadata++;throw new Error(metadata===1?stale:"sentinel after repair");}return{};});
 expect(String(failure)).toContain("sentinel after repair");expect(calls).toEqual([false,true]);expect(metadata).toBe(2);
 calls.length=0;
 const other=await plan(["build"],argv=>{if(argv[0]==="metadata")throw new Error("error: no matching package named `x` found");return{};});
 expect(String(other)).toContain("no matching package");expect(calls).toEqual([false]);
 registerCargoLockEnsurerV1(async()=>"current");
});

test("Cargo planning passes --unit-graph exactly once whether or not the caller already selected it",async()=>{
 registerCargoLockEnsurerV1(async()=>"current");
 const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>{},yieldContinuation:async()=>{}};
 for(const args of [["build","--manifest-path","/x/Cargo.toml"],["build","--manifest-path","/x/Cargo.toml","-Z","unstable-options","--unit-graph"]]){
  const seen:string[][]=[];
  await captureCargoCompilationGraphV1({workspaceRoot:"/x",cwd:"/x",program:"cargo",prefix:[],args,environment:{},maximumOutputBytes:1024},control,{capture:async(_program:string,argv:readonly string[])=>{seen.push([...argv]);return{};}} as unknown as CargoCompilerPlanningPortV1).catch(()=>null);
  const graph=seen.find(argv=>argv.includes("--unit-graph"));expect(graph?.filter(arg=>arg==="--unit-graph").length).toBe(1);
 }
});

test("Cargo test and release build planning capture the whole resolved workspace metadata under the shared ceiling",()=>{
 expect(cargoPlanningOutputBytesV1).toBeGreaterThan(16777216);
 const modules=join(import.meta.dirname,"../../../../../");
 for(const path of ["🧪️testing/🦀️cargo/🟦️.ts","🧪️testing/🦀️cargo/🎯️exact/🟦️.ts","📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts"]){
  const source=readFileSync(join(modules,path),"utf8");
  expect(source,path).toContain("maximumOutputBytes:cargoPlanningOutputBytesV1");
  expect(source,path).not.toContain("maximumOutputBytes:16777216");
 }
});
