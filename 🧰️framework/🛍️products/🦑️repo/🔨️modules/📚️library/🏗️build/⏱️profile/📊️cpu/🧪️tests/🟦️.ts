import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {repoCpuProfileArgumentsV1,prepareRepoCpuTestProfileArgumentsV1} from "../🟨️.mjs";
import {fileURLToPath} from "node:url";
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🔣️.json";

/** 📊️ Compares the closed cross-platform profile request and fixed driver flags to Ajv and SQLite. */
test("CPU profiling accepts only declared requests and explicit bounded artifact paths",()=>{
 const ajv=new Ajv({strict:false}),caseValidator=ajv.compile(schema.definitions.Case),requestValidator=ajv.compile(schema.definitions.Request),db=new Database(":memory:");
 try{
  for(const row of fixture.cases){
   expect(caseValidator(row)).toBe(true);
   if(row.expected===null){expect(()=>repoCpuProfileArgumentsV1(row.environment,{platform:row.platform,pid:123,sequence:1})).toThrow();continue;}
   const actual=repoCpuProfileArgumentsV1(row.environment,{platform:row.platform,pid:123,sequence:1});expect(actual).toEqual(row.expected);
   const wire=row.environment.SEMIO_REPO_CPU_PROFILE;if(!wire)continue;
   expect(requestValidator(JSON.parse(wire))).toBe(true);
   const oracle=db.query("SELECT '--cpu-prof' enabled,'--cpu-prof-interval=10000' interval,'--cpu-prof-dir='||?1 directory,'--cpu-prof-name=repo-'||?2||'-'||?3||'.cpuprofile' name").get(JSON.parse(wire).directory,123,1) as {enabled:string;interval:string;directory:string;name:string};
   expect(actual).toEqual([oracle.enabled,oracle.interval,oracle.directory,oracle.name]);
  }
 }finally{db.close();}
});

/** 🚦️ Both real captured launchers must prepend the owned flags before their immutable loader. */
test("live and sealed launchers retain the loader and parent operation behind CPU flags",()=>{
 for(const path of ["../../../📥️source/🔏️closure/🚪️command/🟦️.ts","../../../🏛️sealed/📥️execution/🟦️.ts"]){
  const source=readFileSync(new URL(path,import.meta.url),"utf8");
  expect(source).toContain('...prepareRepoCpuProfileArgumentsV1(environment),"--eval"');
  expect(source).toContain("scriptResourceProcessEnvironmentV1(control,environment)");
  expect(source).toContain("signal:control.signal");
  expect(source).toContain("auxiliaryInput:{version:1,text}");
 }
});

/** 🔏️ Exercises the actual launcher functions through owned ports without replacing their Original or loader data. */
test("both launcher ports retain exact custody and add only the closed profiler prefix",async()=>{
 const ts=await import("typescript"),path=await import("node:path"),url=await import("node:url");
 const fixture={text:"fixture",sha256:"a".repeat(64)},original=Object.freeze({identity:"original"}),control={signal:new AbortController().signal};
 for(const mode of ["live","sealed"])for(const enabled of [false,true]){
  const environment:Record<string,string>={SEMIO_LANGUAGE:"en",...(enabled?{SEMIO_REPO_CPU_PROFILE:'{"version":1,"directory":"/tmp/generated"}'}:{})},calls:{command:string;args:readonly string[];options:Record<string,unknown>;budget:number}[]=[];
  const profile=(env:Readonly<Record<string,string|undefined>>)=>{expect(env).toBe(environment);return repoCpuProfileArgumentsV1(env,{platform:"linux",pid:123,sequence:1});};
  const ports={
   requireIssuedScriptInvocationV1:(value:unknown)=>expect(value).toBe(original),
   requireIssuedResourceControlInvocationV1:(phase:unknown,value:unknown)=>{expect(phase).toBe(control);expect(value).toBe(original);},
   withIssuedResourceOperationV1:async(phase:unknown,run:()=>Promise<unknown>)=>{expect(phase).toBe(control);return run();},
   captureCompilerBuildSourceClosureV1:async(_root:unknown,_entry:unknown,value:unknown,phase:unknown,env:unknown)=>{expect(value).toBe(original);expect(phase).toBe(control);expect(env).toBe(environment);return fixture;},
   buildSourceClosureTextV1:()=>fixture,
   prepareRepoCpuProfileArgumentsV1:profile,
   buildSourceClosureStreamLoaderV1:"loader",
   schema:{definitions:{Policy:{const:{maximumSnapshotBytes:1048576}}}},
   closureSchema:{definitions:{Policy:{const:{maximumSnapshotBytes:1048576}}}},
   runOwnedCommand:async(command:string,args:readonly string[],_cwd:string,_label:string,budget:number,options:Record<string,unknown>)=>{calls.push({command,args,options,budget});},
   scriptInvocationBudget:(value:unknown,fallback:number)=>{expect(value).toBe(original);expect(fallback).toBe(0);return 12345;},
   scriptResourceProcessEnvironmentV1:(phase:unknown,env:unknown)=>{expect(phase).toBe(control);expect(env).toBe(environment);return {FORWARDED:"unchanged"};},
   writeOwnedCommandOutputV1:()=>{},
   implementationModeV1:()=>"sealed",
   sealedEntryOfV1:()=>({}),
   readSealedPointerV1:()=>({digest:fixture.sha256}),
   readSealedSnapshotV1:()=>fixture.text,
   readSealedLoaderV1:()=>"loader",
   sealedPolicyV1:{maximumCommandLineCharacters:32767},
   profileMarkV1:()=>{},
   relative:path.relative,
   fileURLToPath:url.fileURLToPath,
  };
  const sourcePath=new URL(mode==="live"?"../../../📥️source/🔏️closure/🚪️command/🟦️.ts":"../../../🏛️sealed/📥️execution/🟦️.ts",import.meta.url),source=readFileSync(sourcePath,"utf8"),name=mode==="live"?"executeCapturedBuildSourceV1":"executeImplementationOnceV1",ast=ts.createSourceFile("launcher.ts",source,ts.ScriptTarget.Latest,true);
  const declaration=ast.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text===name);
  if(!declaration)throw Error("Actual launcher function required");
  const code=ts.transpileModule(declaration.getText(ast).replace(/^export /u,"").replaceAll("import.meta.url",JSON.stringify(sourcePath.href)),{compilerOptions:{target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext}}).outputText;
  const run=new Function(...Object.keys(ports),code+";return "+name+";")(...Object.values(ports));
  await run("/repo","entry.ts",["run","target"],original,control,environment);
  expect(calls.length).toBe(1);const call=calls[0]!;
  expect(call.command).toBe(process.execPath);expect(call.budget).toBe(12345);
  expect(call.args).toEqual([...repoCpuProfileArgumentsV1(environment,{platform:"linux",pid:123,sequence:1}),"--eval","loader","--",fixture.sha256,"run","target"]);
  expect(call.options.control).toBe(control);expect(call.options.signal).toBe(control.signal);
  expect(call.options.auxiliaryInput).toEqual({version:1,text:fixture.text});
  expect(call.options.env).toEqual({FORWARDED:"unchanged",BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"});
 }
});

/** 🌱️ Retains the actual Original, deadline and guarded test argv when enabling the owned profiler. */
test("genuine file launch retains the owned sampler within its guarded test",async()=>{
 const ts=await import("typescript"),routing=await import("../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts"),receiver=await import("../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts"),source=readFileSync(new URL("../../../../🧪️testing/🌱️genuine-birth/🟦️.ts",import.meta.url),"utf8"),ast=ts.createSourceFile("genuine.ts",source,ts.ScriptTarget.Latest,true);
 let launch:import("typescript").ArrowFunction|undefined;const visit=(node:import("typescript").Node):void=>{if(ts.isVariableDeclaration(node)&&node.name.getText()==="launch"&&node.initializer&&ts.isArrowFunction(node.initializer))launch=node.initializer;ts.forEachChild(node,visit);};visit(ast);if(!launch)throw Error("Actual genuine launch required");
 const code=ts.transpileModule("const launch="+launch.getText(ast)+";return launch;",{compilerOptions:{target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.None}}).outputText;
 await receiver.withBuildTestInvocationV1(async original=>{for(const row of fixture.cases.filter(row=>row.name==="off"||row.name==="linux")){
  const environment=row.environment,request={pattern:"^selected$"},flags=repoCpuProfileArgumentsV1(environment,{platform:"linux",pid:123,sequence:1}),calls:{command:string;args:readonly string[];budget:number;options:any}[]=[],before=original.control.remainingMilliseconds();
  const run=new Function("advanceScriptInvocation","prepareRepoCpuTestProfileArgumentsV1","guardArgumentsV1","declared","local","request","environment","root","label","runOwnedCommand","scriptInvocationEnvironment","birth",code)(async(value:unknown)=>expect(value).toBe(original),(env:unknown)=>{expect(env).toBe(environment);return flags.length?["--preload","owned-profiler"]:[];},(command:string,args:readonly string[],env:unknown)=>{expect(command).toBe(process.execPath);expect(env).toBe(environment);return["guarded",...args];},{childTimeoutMilliseconds:120000},"suite.ts",request,environment,process.cwd(),"build:genuine-test",async(command:string,args:readonly string[],_root:string,_label:string,budget:number,options:any)=>{calls.push({command,args,budget,options});},routing.scriptInvocationEnvironment,{id:"fixture"});
  await run(original);expect(calls).toHaveLength(1);const call=calls[0]!;expect(call.command).toBe(process.execPath);expect(call.args).toEqual(["guarded","test",...(row.expected!.length?["--preload","owned-profiler"]:[]),"--timeout","120000","./suite.ts","--test-name-pattern","^selected$"]);expect(call.budget).toBeGreaterThan(0);expect(call.budget).toBeLessThanOrEqual(before!);expect(call.options.signal).toBe(original.control.signal);expect(call.options.env[routing.SCRIPT_PROCESS_INVOCATION_ENV]).toBe(routing.scriptInvocationEnvironment(original,{})[routing.SCRIPT_PROCESS_INVOCATION_ENV]);
 }});
});

/** 🍞️ Keeps the test adapter behind its closed request and validates language-neutral sampling DATA. */
test("Bun test profiles use the fixed sampler and a schema-owned artifact",()=>{
 for(const row of fixture.cases.filter(row=>row.platform===process.platform)){if(row.expected===null){expect(()=>prepareRepoCpuTestProfileArgumentsV1(row.environment)).toThrow();continue;}expect(prepareRepoCpuTestProfileArgumentsV1(row.environment)).toEqual(row.expected.length?[schema.definitions.Protocol.const.testPreloadFlag,fileURLToPath(new URL("../🍞️bun/📜️script.mjs",import.meta.url))]:[]);}
 const definition={...schema.definitions.BunSamplingArtifact,definitions:schema.definitions};expect(new Ajv({strict:false}).validate(definition,fixture.bunArtifact)).toBe(true);expect(validateJsonSchemaSubset(definition,fixture.bunArtifact)).toEqual([]);const db=new Database(":memory:");try{expect(db.query("select count(*) traces,sum(json_array_length(value,'$.frames')) frames from json_each(?1,'$.stackTraces.traces')").get(JSON.stringify(fixture.bunArtifact))).toEqual({traces:1,frames:1});}finally{db.close();}
});
