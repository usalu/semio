import {test,expect} from "bun:test";
import _ from "lodash";
import {createRepoProfileV1} from "../../../../../🏗️build/⏱️profile/🟨️.mjs";
import type {CargoCompilerPlanningPortV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import {observeCargoPlanningPortV1} from "../🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1} from "../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

const withControl=(body:(control:Parameters<CargoCompilerPlanningPortV1["compiler"]>[2])=>Promise<void>)=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"test:cargo-observation-failure",maximumElapsedMilliseconds:30000},{workspaceRoot:process.cwd()},Date.now()),original=>body(createIssuedResourceControlV1(original,"test:cargo-observation-failure",0)));
const inertPort=(cfg:CargoCompilerPlanningPortV1["cfg"]):CargoCompilerPlanningPortV1=>({cfg,async compiler(){return{program:"compiler",digest:"a".repeat(64),hostTriple:"host",cargoProgram:"cargo"};},async sourceInputs(){return[];},async capture(){return{};}});

test("actual delegated planning operations preserve values, receiver and control while reporting bounded aggregate",()=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"test:cargo-plan-observation",maximumElapsedMilliseconds:30000},{workspaceRoot:process.cwd()},Date.now()),async original=>{
 expect(fixture.version).toBe(1);expect(fixture.operations.map(row=>row.method)).toEqual(["compiler","cfg","sourceInputs","capture"]);const control=createIssuedResourceControlV1(original,"test:cargo-plan-observation",0),environment=Object.freeze({}),returns={compiler:{program:"compiler",digest:"a".repeat(64),hostTriple:"host",cargoProgram:"cargo"},cfg:["host"],sourceInputs:[{path:"source",digest:"b".repeat(64),bytes:4}],capture:{version:1}},seen:{method:string;milliseconds:number}[]=[];let clock=0;
 const record=(method:string,receiver:unknown,actual:unknown)=>{expect(receiver).toBe(port);expect(actual).toBe(control);const row=fixture.operations.find(row=>row.method===method)!;clock+=row.milliseconds;seen.push({method,milliseconds:row.milliseconds});};
 const port:CargoCompilerPlanningPortV1={async compiler(_environment,_cwd,actual){record("compiler",this,actual);return returns.compiler;},async cfg(_program,_target,_environment,_cwd,actual){record("cfg",this,actual);return returns.cfg;},async sourceInputs(_manifest,actual){record("sourceInputs",this,actual);return returns.sourceInputs;},async capture(_program,_args,_cwd,_environment,actual){record("capture",this,actual);return returns.capture;}};
 const profile=createRepoProfileV1({mode:"json",now:()=>clock,write:()=>{}}),observed=observeCargoPlanningPortV1(port,profile,()=>clock);
 for(const row of fixture.operations)for(let n=0;n<row.calls;n++){if(row.method==="compiler")expect(await observed.port.compiler(environment,"root",control)).toBe(returns.compiler);else if(row.method==="cfg")expect(await observed.port.cfg("rustc",null,environment,"root",control)).toBe(returns.cfg);else if(row.method==="sourceInputs")expect(await observed.port.sourceInputs("manifest",control)).toBe(returns.sourceInputs);else expect(await observed.port.capture("cargo",["metadata"],"root",environment,control,1024)).toBe(returns.capture);}
 await observed.finish();const rows=profile.summary().rows,aggregate=rows.find(row=>row.phase==="cargo.planning.aggregate");expect(aggregate).toBeDefined();const detail=JSON.parse(aggregate!.detail!),oracle=_.mapValues(_.groupBy(seen,"method"),rows=>({calls:rows.length,cumulativeOperationMilliseconds:_.sumBy(rows,"milliseconds")}));expect(detail.operations).toEqual(oracle);expect(rows.filter(row=>row.phase==="cargo.planning.capture")).toHaveLength(32);
}));

test("disabled observation preserves the actual port and Promise identity",()=>withControl(async control=>{
 const promise=Promise.resolve(["cfg"]),port=inertPort(()=>promise),profile=createRepoProfileV1({mode:"off",write:()=>{}}),observed=observeCargoPlanningPortV1(port,profile);
 expect(observed.port).toBe(port);expect(observed.port.cfg("rustc",null,{},"root",control)).toBe(promise);await observed.finish();expect(profile.summary().rows).toHaveLength(0);
}));

test("rejected operations drain and preserve their exact original error",()=>withControl(async control=>{
 const reason=Error("actual planning refusal"),profile=createRepoProfileV1({mode:"json",write:()=>{}}),observed=observeCargoPlanningPortV1(inertPort(async()=>{throw reason;}),profile),work=observed.port.cfg("rustc",null,{},"root",control);
 await expect(work).rejects.toBe(reason);await observed.finish();await observed.finish();const rows=profile.summary().rows.filter(row=>row.phase==="cargo.planning.aggregate");expect(rows).toHaveLength(1);expect(JSON.parse(rows[0]!.detail!).failedOperations).toBe(1);
}));

test("overlapping operation durations remain cumulative rather than reported as wall time",()=>withControl(async control=>{
 let clock=0,release!:(value:unknown)=>void;const pending=new Promise(resolve=>{release=resolve;}),port={...inertPort(async()=>[]),capture:()=>pending},profile=createRepoProfileV1({mode:"json",now:()=>clock,write:()=>{}}),observed=observeCargoPlanningPortV1(port,profile,()=>clock),first=observed.port.capture("cargo",[],"root",{},control,1024),second=observed.port.capture("cargo",[],"root",{},control,1024);
 clock=10;release({});await Promise.all([first,second]);await observed.finish();const summary=profile.summary(),detail=JSON.parse(summary.rows.find(row=>row.phase==="cargo.planning.aggregate")!.detail!);expect(summary.total).toBe(10);expect(detail.operations.capture).toEqual({calls:2,cumulativeOperationMilliseconds:20});
}));

import {readFileSync} from "node:fs";
import ts from "typescript";
import {transformSync} from "esbuild";
import {evaluateExtractedFunctionV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧩️extracted-function/🟦️.ts";
import storeFixture from "./📦️store.json" with {type:"json"};
test("compiler-store spans preserve recording-port operations, exact controls, results and failure cleanup",async()=>{
 const file=new URL("../../../../💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts",import.meta.url),source=ts.createSourceFile("compiler.ts",readFileSync(file,"utf8"),ts.ScriptTarget.Latest,true),node=source.statements.find(value=>ts.isFunctionDeclaration(value)&&value.name?.text==="withRepositoryCompilerStoreOwnedV1")!,code=transformSync(node.getText(source),{loader:"ts",format:"cjs"}).code;
 expect(storeFixture.version).toBe(1);
 for(const row of storeFixture.cases){
  const operations:string[]=[],phases:string[]=[],control={},original={},environment=Object.freeze({PATH:"qualified"}),policy={},nativeEnvironment=Object.freeze({PATH:"native-qualified"}),storage={async recheck(){operations.push("recheck");}},result=Object.freeze({result:1}),failure=Error("exact recorded consumer failure"),owners=new WeakMap();let exposed:unknown;
  const profile=createRepoProfileV1({mode:"json",write(){}}),run=evaluateExtractedFunctionV1(code,"withRepositoryCompilerStoreOwnedV1",{
   profilePhaseV1:(name:string,body:()=>unknown)=>{phases.push(name);return profile.phase(name,body);},requireIssuedResourceControlInvocationV1:(actual:unknown,owner:unknown)=>{expect(actual).toBe(control);expect(owner).toBe(original);},readBuildMode:(mode:unknown)=>mode,repositoryStrictStorageV1:()=>policy,
   withRequestedIssuedResourceControlV1:(actual:unknown,_requested:unknown,body:(actual:unknown)=>unknown)=>{expect(actual).toBe(control);return body(control);},
   openRepositoryCachePartitionV1:async(_host:unknown,name:unknown,actual:unknown)=>{expect(actual).toBe(control);expect(name).toBe("rust");operations.push("image");return{root:"root",async close(){operations.push("close");}};},
   repositoryCompilerReservationV1:async(_opened:unknown,_policy:unknown,_mode:unknown,actual:unknown)=>{expect(actual).toBe(control);operations.push("reservation");return storage;},repositoryCompilerLayoutV1:()=>({buildDirectory:"build",captureDirectory:"capture",temporaryDirectory:"temp",homeDirectory:"home",targetDirectory:"target"}),requireNativeStoragePathV1:(_root:unknown,value:unknown)=>{operations.push("path:"+value);},mkdirSync:(value:unknown)=>{operations.push("mkdir:"+value);},
   repositoryCompilerLaunchEnvironmentV1:async(actualStorage:unknown,actual:unknown,env:unknown)=>{expect(actualStorage).toBe(storage);expect(actual).toBe(control);expect(env).toBe(environment);operations.push("environment");return environment;},
   withRepositoryNativeCompilerV1:(actualStorage:unknown,actual:unknown,env:unknown,body:(env:unknown)=>unknown)=>{expect(actualStorage).toBe(storage);expect(actual).toBe(control);expect(env).toBe(environment);operations.push("native");return body(nativeEnvironment);},
   compilerStoreOwners:owners,withCompilerWorkspaceLifetimeV1:(actualStorage:unknown,actual:unknown,body:(actual:unknown)=>unknown)=>{expect(actualStorage).toBe(storage);expect(actual).toBe(control);operations.push("workspace");return body(control);},unitCacheRootV1:()=>"units",UNIT_CACHE_SCRIPT_V1:"script"
  });
  let caught:unknown;try{const actual=await run("host","workspace","prototype",control,environment,async(store:unknown,actual:unknown)=>{expect(actual).toBe(control);exposed=store;operations.push("body");if(row.fail)throw failure;return result;},original);expect(actual).toBe(result);}catch(error){caught=error;}
  expect(caught).toBe(row.fail?failure:undefined);expect(operations).toEqual(storeFixture.operations);expect(_.countBy(operations)).toEqual(_.countBy(storeFixture.operations));expect(phases).toEqual(storeFixture.phases);expect(owners.get(exposed as object).live).toBe(false);
 }
});

import {join} from "node:path";
import {createHash} from "node:crypto";
import rustupFixture from "./🦀️rustup.json" with {type:"json"};
test("Rustup preparation spans preserve actual warm-path recording delegation and exact failure identity",async()=>{
 const source=ts.createSourceFile("rustup.ts",readFileSync(new URL("../../../../🚀️bootstrap/🛠️tools/🦀️rustup/📥️installation/🟦️.ts",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true),node=source.statements.find(value=>ts.isFunctionDeclaration(value)&&value.name?.text==="withRepositoryRustToolchainOwnedV1")!,code=transformSync(node.getText(source),{loader:"ts",format:"cjs"}).code;
 expect(rustupFixture.version).toBe(1);
 for(const row of rustupFixture.cases){
  const phases:string[]=[],operations:string[]=[],control={},original={},failure=Error("exact Rustup recording failure"),result=Object.freeze({value:1}),request={version:1,channel:"nightly",profile:"minimal",components:[],targets:[]},bootstrap={digest:"b".repeat(64),host:"host",environment:Object.freeze({RUSTUP_HOME:"tools/home",CARGO_HOME:"tools/cargo",PATH:"bootstrap-qualified"})},tools=Object.freeze({PATH:"cargo-tools-qualified",SEMIO_TOOL_CARGO_CLIPPY:"qualified-clippy"}),manifest=Buffer.from(JSON.stringify({version:1,files:[{path:"bin/rustc",kind:"file",digest:"c".repeat(64),bytes:4}]})),receipt={request:createHash("sha256").update(JSON.stringify({version:1,request,host:bootstrap.host,bootstrap:bootstrap.digest})).digest("hex"),host:"host",channel:"nightly",bootstrap:bootstrap.digest,allocation:"allocation",seal:"s".repeat(64),entries:1,manifest:createHash("sha256").update(manifest).digest("hex"),directory:{device:"1",inode:"2"},bytes:4},storage={allocation:{owner:"repository:tooling-store",id:"allocation"},async recheck(){operations.push("storage");}},store={control,invocation:original,root:"tools",storage,environment:{}},readers=new WeakMap(),readerControls=new WeakMap(),programDigests=new WeakMap(),compilers=new Map();let exposed:object|undefined;
  const check=(actual:unknown)=>expect(actual).toBe(control),run=evaluateExtractedFunctionV1(code,"withRepositoryRustToolchainOwnedV1",{
   profilePhaseV1:(name:string,body:()=>unknown)=>{phases.push(name);return createRepoProfileV1({mode:"off"}).phase(name,body);},join,createHash,
   requireIssuedResourceControlInvocationV1:(actual:unknown,owner:unknown)=>{check(actual);expect(owner).toBe(original);},requireNativeStorageReservationV1:(actual:unknown,actualControl:unknown)=>{expect(actual).toBe(storage);check(actualControl);},repositoryCachePartitionRootV1:()=>"tools",readNativeCallerAuthorityV1:()=>({network:{offline:false}}),
   sourceRequest:async(_workspace:unknown,actual:unknown,actualStore:unknown)=>{check(actual);expect(actualStore).toBe(store);operations.push("source");return{request,sourceDigest:"d".repeat(64)};},
   provisionRustupBootstrapV1:async(_workspace:unknown,actualStore:unknown,actual:unknown)=>{check(actual);expect(actualStore).toBe(store);operations.push("bootstrap");if(row.fail==="bootstrap")throw failure;return bootstrap;},
   cargoToolsEnvironmentV1:async(_workspace:unknown,actualStore:unknown,actual:unknown,environment:unknown)=>{check(actual);expect(actualStore).toBe(store);expect(environment).toBe(bootstrap.environment);operations.push("cargo-tools");return tools;},
   requireNativeStoragePathV1:(_root:unknown,value:unknown)=>value,protocol:{maximumReceiptBytes:4096,maximumManifestBytes:4096},schema:{definitions:{GenerationReceipt:{}}},validateJsonSchemaSubset:()=>[],
   readClosed:async(_path:unknown,_maximum:unknown,actual:unknown,root:unknown)=>{check(actual);expect(root).toBe("tools");operations.push("receipt");return receipt;},
   withRustupDirectoryIdentityV1:(_workspace:unknown,actualStore:unknown,actual:unknown,_directory:unknown,identity:unknown,body:()=>unknown)=>{check(actual);expect(actualStore).toBe(store);expect(identity).toBe(receipt.directory);operations.push("identity");return body();},
   readPhysicalCargoSourceV1:async(_path:unknown,_policy:unknown,actual:unknown)=>{check(actual);operations.push("manifest");return manifest;},rustupToolchainSealV1:()=>receipt.seal,
   repositoryLeaseDirectoryV1:()=>"leases",withResourceLeases:(options:{control:unknown;resources:{mode:string}[]},body:()=>unknown)=>{check(options.control);expect(options.resources[0]!.mode).toBe("shared");operations.push("home-sh");return body();},
   withRustupGenerationCustodyV1:(_workspace:unknown,actualStore:unknown,actual:unknown,_directory:unknown,mode:unknown,body:(scope:unknown,actual:unknown)=>unknown)=>{check(actual);expect(actualStore).toBe(store);expect(mode).toBe("shared");operations.push("generation-sh");return body({},control);},
   withRequestedIssuedResourceControlV1:(actual:unknown,_requested:unknown,body:(actual:unknown)=>unknown)=>{check(actual);return body(control);},readers,readerControls,programDigests,compilers
  });
  let caught:unknown;try{expect(await run("workspace",store,control,async(generation:{environment:Record<string,string>},actual:unknown)=>{check(actual);exposed=generation;operations.push("body");expect(generation.environment.SEMIO_TOOL_CARGO_CLIPPY).toBe(tools.SEMIO_TOOL_CARGO_CLIPPY);expect(generation.environment.PATH).toEndWith(tools.PATH);if(row.fail==="body")throw failure;return result;})).toBe(result);}catch(error){caught=error;}
  expect(caught).toBe(row.fail?failure:undefined);expect(operations).toEqual(row.fail==="bootstrap"?rustupFixture.operations.slice(0,3):rustupFixture.operations);expect(_.countBy(operations)).toEqual(_.countBy(row.fail==="bootstrap"?rustupFixture.operations.slice(0,3):rustupFixture.operations));expect(phases).toEqual(row.fail==="bootstrap"?rustupFixture.phases.slice(0,3):rustupFixture.phases);if(exposed){expect(readers.has(exposed)).toBe(false);expect(readerControls.has(exposed)).toBe(false);expect(programDigests.has(exposed)).toBe(false);}expect(compilers.size).toBe(0);
 }
});
