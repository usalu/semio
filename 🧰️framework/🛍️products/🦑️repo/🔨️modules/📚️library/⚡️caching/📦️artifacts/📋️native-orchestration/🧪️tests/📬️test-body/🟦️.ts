import { advanceScriptInvocation } from "../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts";
import { readNativeOwnerCapabilities } from "../../📥️invocation/🟦️.ts";
import { scriptInvocationBudget } from "../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts";
import {consumeOwnerArgumentsV1,declaredNativeOwnerCommandV1} from "../../../../../🔌️nx-plugin/📤️arguments/🟨️.mjs";
import {test,expect} from "bun:test";

import {readFileSync} from "node:fs";
import {resolve,dirname,join,relative,isAbsolute} from "node:path";
import ts from "typescript";
import {transformSync} from "esbuild";
import Ajv from "ajv";
import {parse as parseJsonc} from "jsonc-parser";
import {repositoryCargoPreparationStorageV1,parseCargoPreparationStorageV1} from "../../../../../🗂️workspaces/🦀️cargo/🛠️preparation/📦️storage/🟦️.ts";
import {nativeOwnerTestManifestRequestV1} from "../../🗺️owner-test-manifests/🟦️.ts";
const owner=resolve(import.meta.dir,"../.."),fixture=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/📬️test-body/🔣️.json"),"utf8"));
const source=ts.createSourceFile("native.ts",readFileSync(resolve(owner,"🟦️.ts"),"utf8"),ts.ScriptTarget.Latest,true);
const native=source.statements.find((node:any)=>node.name?.text==="NativeScript")!.getText(source).replace(/^export /,"");
const library=resolve(owner,"../../.."),bodySource=ts.createSourceFile("library.ts",readFileSync(resolve(library,"🟦️.ts"),"utf8"),ts.ScriptTarget.Latest,true);
const consume=bodySource.statements.find((node:any)=>node.name?.text==="runRepositoryCargoTests")!.getText(bodySource).replace(/^export /,"");
const storageAdmission=new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(library,"🗂️workspaces/🦀️cargo/🛠️preparation/📦️storage/🧬️schema/🔣️.json"),"utf8")));
test("typed owner transport prepares only at actual current Cargo consumption",async()=>{
 const oracle=Bun.spawnSync(["node","--eval",`const cases=${JSON.stringify(fixture.cases)};console.log(JSON.stringify(cases.map(row=>row.route==='owner-command'?(row.consume?['owned','prepare:2','cargo:2']:['owned']):['refused'])))`]);expect(oracle.exitCode).toBe(0);expect(JSON.parse(new TextDecoder().decode(oracle.stdout))).toEqual(fixture.cases.map((row:any)=>row.expected));
 for(const compile of [(text:string)=>new Bun.Transpiler({loader:"ts"}).transformSync(text),(text:string)=>ts.transpileModule(text,{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.ESNext}}).outputText,(text:string)=>transformSync(text,{loader:"ts",target:"es2022"}).code]){
  for(const row of fixture.cases){
   const records:string[]=[], progress:string[]=[], started=performance.now(), signal=new AbortController();let version=1;
   const invocation={policy:{version:1,owner:"original-native-body",maximumElapsedMilliseconds:1000},control:{signal:signal.signal,remainingMilliseconds:()=>1000-(performance.now()-started),publish:async(value:any)=>{progress.push(value.stage);},yieldContinuation:()=>new Promise<void>(done=>setImmediate(done))},capabilities:{version:1,repositoryRoot:"/repo",artifactDirectory:"/repo/artifacts",command:{kind:"native-owner-command",manifest:"owner/Cargo.toml",workingDirectory:"owner",program:row.command,arguments:row.args},transport:{maximumBytes:67108864,maximumLines:65536},child:{maximumElapsedMilliseconds:1000},network:{offline:true}}};
   const prepare=()=>records.push(`prepare:${version}`),policy=()=>({manifestPath:"/repo/owner/Cargo.toml",version});
   const consumeBody=new Function("getWorkspaceRoot","repositoryCargoPreparationStorageV1","cargoRepositoryPackageSelections","cargoWorkspaceForManifest","resolve","relative","prepareCargoWorkspaceInvocation","repositoryCargoTestPolicyV1","runCargoTestsV1","TEST_LEVELS","packageTestBudgetMs",compile(consume)+";return runRepositoryCargoTests;")(()=>"/repo",repositoryCargoPreparationStorageV1,()=>[{manifest:"owner/Cargo.toml",name:"owner"}],()=>({directory:"owner"}),resolve,()=>"owner/Cargo.toml",prepare,policy,async(_request:any,current:any)=>records.push(`cargo:${current.version}`),["quick"],()=>100);
   const owned=async(_command:string,_args:string[],_cwd:string,_label:string,_timeout:number,options:any)=>{expect(options.output).toBe(invocation.capabilities.transport);const limits=JSON.parse(readFileSync(resolve(library,"../../../../🔨️modules/🏃️process/🎛️owned-execution/🧬️schema/📬️output.json"),"utf8"));expect(new Ajv({strict:true}).compile(limits)(options.output)).toBe(true);records.push("owned");version=2;if(row.consume)await consumeBody(["owner"],"/repo/owner",invocation.control,[],{});};
   const Native=new Function("BundleScript","scriptInvocationBudget","advanceScriptInvocation","readNativeOwnerCapabilities","nativeOwnerTestManifestRequestV1","resolve","dirname","readFileSync","Bun","prepareCargoWorkspaceInvocation","repositoryVitestPolicyV1","repositoryProcessOwnerContextV1","repositoryCargoArtifactBuildPolicyV1","repositoryCargoTestPolicyV1","runOwnedCommand","process","consumeOwnerArgumentsV1",compile(native)+";return NativeScript;")(class{repoRoot="/repo";invocation=invocation},scriptInvocationBudget,advanceScriptInvocation,readNativeOwnerCapabilities,nativeOwnerTestManifestRequestV1,resolve,dirname,()=>"[package]\nname=\"owner\"",{TOML:Bun.TOML},prepare,()=>({}),()=>({}),()=>({}),policy,owned,{env:{CARGO_TARGET_DIR:"/repo/artifacts"},execPath:"bun",stderr:{write(){}}},consumeOwnerArgumentsV1);
   try{await new Native().run([row.route,"--manifest","owner/Cargo.toml","--cwd","owner","--",row.command,...row.args]);}catch{records.push("refused");}
   expect(records,row.id).toEqual(row.expected);
  }
 }
 console.log("[DEBUG] actual native wrapper and actual Cargo consumer executed through Bun/TypeScript; independent Node argv oracle admits generic transport, refuses the retired special route and requires fresh current Cargo-body preparation");
});

test("canonical Nx inference delegates every owned command without eager or special test preparation",()=>{
 const declarationsSchema=JSON.parse(readFileSync(join(owner,"🧬️schema/📬️target-declarations/🔣️.json"),"utf8"));expect(new Ajv({strict:true}).compile(declarationsSchema)(fixture.targets)).toBe(true);
 const syntax=ts.createSourceFile("nx.mjs",readFileSync(resolve(library,"🟨️.mjs"),"utf8"),ts.ScriptTarget.Latest,true);
 const declaration=syntax.statements.find((node:any)=>node.name?.text==="projectWithDefaults")!.getText(syntax);
 const manifest={package:{name:"owner"},workspace:{}},closure=["/repo/🏃️process/🧪️testing/🦀️cargo/🟦️.ts"];
 const bindings={declaredNativeOwnerCommandV1,existsSync:()=>true,join,resolve,dirname,relative,isAbsolute,readToml:()=>manifest,readFileSync:()=>"",SCRIPT_BASENAME:"📜️script.ts",DEFAULT_EXECUTOR:"nx:run-commands",LIBRARY_ROOT:"/repo/library",POLICY:{targetDefaults:{},nxSerialTargets:[]},withWasmTooling:(targets:any)=>targets,rootCommandTargets:()=>({}),cargoTargets:()=>({}),componentTargets:()=>({}),printDocumentTargets:()=>({}),generatorContractInputs:()=>({}),generatorOutputOwners:()=>[],genericCommandFallbackInputs:()=>[],withLeveledTestTargets:(targets:any)=>targets,targetPolicy:(_name:string,target:any)=>target,targetWithDefaults:(target:any,root:string)=>({...target,options:{cwd:root,...target.options}}),nativeTargetCommandInputs:()=>closure,targetScriptClosure:()=>closure,nxPath:(path:string)=>path.replaceAll("\\","/"),nativeLockInputs:()=>[],genericTargetCommandInputs:()=>[],generatorOutputCouplingInputs:()=>[],projectInputs:()=>({})};
 for(const compile of [(text:string)=>text,(text:string)=>transformSync(text,{loader:"js",target:"es2022"}).code]){
  const infer=new Function(...Object.keys(bindings),declaration+";return projectWithDefaults;")(...Object.values(bindings));
  for(const row of fixture.targets){
   const result=infer({name:"owner",targets:{test:row.target}},"owner","/repo/owner","/repo"),command=result.targets.test.options.command;
   expect(command,row.id).toContain(" native owner-command --manifest ");
   expect(command,row.id).not.toContain("repository-test-body");
   const expected=row.target.options.command.replace(/^bun\s+("[^"\n]+"|'[^'\n]+'|[^\s]+)/u,(_:string,path:string)=>`bun ${JSON.stringify(resolve("/repo/owner",path.replace(/^["']|["']$/g,"")))}`);
   expect(command.endsWith(` -- ${expected}`),row.id).toBe(true);
   expect(result.targets.test.options.nativeOwnerCommand,row.id).toEqual(row.declaredCommand);
   expect(result.targets.test.options.cwd).toBe(".");
   expect(result.targets.test.inputs).toContain("nativeTestSources");
  }
 }
 const schema=JSON.parse(readFileSync(resolve(owner,"🧬️schema/🗺️owner-test-manifests/🔣️.json"),"utf8")),admit=new Ajv({strict:true}).compile(schema);
 for(const row of fixture.cases.filter((row:any)=>row.route==="owner-command")){
  const request=nativeOwnerTestManifestRequestV1(["--manifest","owner/Cargo.toml","--cwd","owner","--",row.command,...row.args]);
  expect(Boolean(admit(request)),row.id).toBe(true);
 }
 const repository=resolve(library,"../../../../..");
 for(const root of fixture.testOwners){
  const project=JSON.parse(readFileSync(resolve(repository,root,"📋️project.json"),"utf8"));
  expect(project.targets.test.metadata?.semio?.nativePreparation,project.name).toBeUndefined();
 }
 console.log("[DEBUG] actual complete Nx target inference and independent esbuild agree; generic/test/build argv preserved; Ajv admits current owner requests;5 redundant metadata declarations absent");
});

test("canonical selected native callers preserve compiler argv and keep full setup explicit",()=>{
 const repository=resolve(library,"../../../../..");
 const oracle=Bun.spawnSync(["node","--eval",`const fs=require('node:fs'),path=require('node:path');const root=process.argv[1],rows=JSON.parse(process.argv[2]);console.log(JSON.stringify(rows.map(row=>{const target=JSON.parse(fs.readFileSync(path.join(root,row.project),'utf8')).targets[row.target];return{...row,command:target.options.command,dependencies:target.dependsOn}})))`,repository,JSON.stringify(fixture.callerTargets)]);
 expect(oracle.exitCode).toBe(0);expect(JSON.parse(new TextDecoder().decode(oracle.stdout))).toEqual(fixture.callerTargets);
 for(const row of fixture.callerTargets){
  const target=JSON.parse(readFileSync(resolve(repository,row.project),"utf8")).targets[row.target];
  expect(target.options.command,row.target).toBe(row.command);
  expect(target.dependsOn,row.target).toEqual(row.dependencies);
  expect(target.dependsOn,row.target).not.toContain("workspace:deps-cargo");
 }
 const workspace=JSON.parse(readFileSync(resolve(repository,"📋️project.json"),"utf8"));
 expect(workspace.targets["deps-cargo"].options.command).toMatch(/ sync cargo$/u);
 expect(workspace.targets["deps-cargo"].cache).toBe(false);
 const target=JSON.parse(readFileSync(resolve(library,"🗂️workspaces/🦀️cargo/📋️project.json"),"utf8")).targets.synchronize;
 expect(target.options.command).toBe("bun ./📜️script.ts synchronize");expect(target.options.forwardAllArgs).toBe(true);expect(target.cache).toBe(false);
 for(const path of [".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){
  const errors:any[]=[],launch=parseJsonc(readFileSync(resolve(repository,path),"utf8"),errors);expect(errors).toEqual([]);
  const selected=launch.configurations.filter((row:any)=>row.command?.includes("@semio-tech/cargo-workspaces:synchronize"));
  expect(selected.length,path).toBe(1);expect(selected[0].command).toContain('-- --manifest "${input:cargoSynchronizationManifest}"');
  expect(launch.inputs.filter((row:any)=>row.id==="cargoSynchronizationManifest").length,path).toBe(1);
 }
 const router=readFileSync(resolve(library,"🗂️workspaces/🦀️cargo/📜️script.ts"),"utf8");expect(router).toContain('.register("synchronize",SynchronizeScript)');
 console.log("[DEBUG]4 real native caller vectors retain compiler argv/tool prerequisites; Node JSON oracle agrees; explicit global setup and mandatory-manifest Nx/live/seed synchronization route admitted");
});


for(const selected of fixture.consumingAdapters)test("actual Cargo adapters forward selected argv and explicit preparation environment "+selected.id,async()=>{
 const {EventEmitter}=await import("node:events"),{PassThrough}=await import("node:stream");
 const adapters={artifact:["⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts","buildRepositoryCargoArtifacts"],streaming:["🏃️process/🟦️.ts","cargoStreamingStatus"],command:["🏃️process/🎛️owned-execution/🟦️.ts","runRepositoryCommand"]};
 for(const compile of [(text:string)=>new Bun.Transpiler({loader:"ts"}).transformSync(text),(text:string)=>transformSync(text,{loader:"ts",target:"es2022"}).code])for(const row of [selected]){
  const [path,name]=adapters[row.owner as keyof typeof adapters],syntax=ts.createSourceFile(path,readFileSync(resolve(library,path),"utf8"),ts.ScriptTarget.Latest,true),body=syntax.statements.find((node:any)=>node.name?.text===name)!.getText(syntax).replace(/^export /,"");
  const records:any[]=[],prepare=(storage:unknown,root:string,args:string[],cwd:string,env:any)=>{expect(storageAdmission(storage),JSON.stringify(storageAdmission.errors)).toBe(true);expect(parseCargoPreparationStorageV1(storage)).toEqual(repositoryCargoPreparationStorageV1(root));records.push({root,args,cwd,env:env??null});};
  const child=()=>{const value:any=new EventEmitter();value.stdout=new PassThrough();value.stderr=new PassThrough();value.exitCode=null;value.signalCode=null;queueMicrotask(()=>{value.stdout.end();value.stderr.end();value.exitCode=0;value.emit("close",0,null)});return value;};
  const bindings={getWorkspaceRoot:()=>"/repo",repositoryCargoPreparationStorageV1,resolve,dirname,join,prepareCargoWorkspaceInvocation:prepare,repositoryCargoArtifactBuildPolicyV1:()=>({}),buildCargoArtifacts:async()=>{},runOwnedCommand:async()=>{},buildBudgetMs:()=>1000,observesCargoInvocation:()=>false,spawn:child,terminateOwnedProcessTree:()=>{},budgetTimeoutHint:()=>"",process};
  const run=new Function(...Object.keys(bindings),compile(body)+`;return ${name};`)(...Object.values(bindings));
  if(row.owner==="artifact")await run("owner/Cargo.toml",row.args.slice(3),"/repo",{command:row.args[0],environment:row.env});else if(row.owner==="streaming")await run(row.args,"/repo/owner",row.env,1000);else await run("cargo",row.args,"/repo/owner","selected",1000,{env:row.env});
  expect(records.length,row.id).toBe(1);expect(records[0].args,row.id).toEqual(row.args);expect(records[0].env,row.id).toEqual(row.env);expect(records[0].root).toBe("/repo");expect(records[0].cwd).toBe(row.owner==="artifact"?"/repo":"/repo/owner");
 }
 console.log("[DEBUG] actual artifact/streaming/owned-command adapters execute under Bun and independent esbuild with exact package selectors/environment; one preparation call per consumption");
});

test("actual fresh Hub Cargo consumers asynchronously prepare controlled current inputs before capture",async()=>{
 const fs=await import("node:fs"),{homedir}=await import("node:os");
 const repository=resolve(library,"../../../../.."),fresh=join(repository,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe"),path=join(fresh,"🏭️fresh-component/🟦️.ts"),syntax=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),body=syntax.statements.find((node:any)=>node.name?.text==="freshRun")!.getText(syntax).replace(/^export /,"");
 const {selectedCargoArguments}=await import(join(library,"🗂️workspaces/🦀️cargo/📤️arguments/🟦️.ts")),{cargoCommandPreparationV1}=await import(join(library,"🗂️workspaces/🦀️cargo/🏃️command/🟦️.ts")),{CargoCliOperationOwner,CargoCommandFailure,runCargoCliCommand}=await import(join(library,"🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🟦️.ts"));
 const {parseFreshProcessPolicyV1}=await import(join(fresh,"🏭️fresh-component/🎛️control/🟦️.ts")),{freshCheckpoint}=await import(join(fresh,"🧾️source-epoch/🟦️.ts")),{isGeneratedPath}=await import(join(library,"⚡️caching/🟦️.ts")),{acquireCargoBuildLeaseV1}=await import(join(repository,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts")),{writeCompletedCargoInvocationProvenanceV1}=await import(join(repository,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts")),{CurrentPhysicalOwnerV1}=await import(join(repository,"🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️current/🟦️.ts"));
 const policyFixture=JSON.parse(readFileSync(join(fresh,"🏭️fresh-component/🎛️control/🧫️fixtures/🔣️.json"),"utf8")),output=process.env.SEMIO_TEST_ARTIFACT_DIR!;expect(isAbsolute(output)).toBe(true);fs.mkdirSync(output,{recursive:true});
 const oracle=Bun.spawnSync(["node","--eval",`console.log(JSON.stringify(JSON.parse(process.argv[1]).map(row=>row.command==='cargo'?(row.refused?['plan','prepare']:['plan','prepare','lease','capture','release']):['capture'])))`,JSON.stringify(fixture.freshCargoConsumers)]);expect(oracle.exitCode).toBe(0);expect(JSON.parse(new TextDecoder().decode(oracle.stdout))).toEqual(fixture.freshCargoConsumers.map((row:any)=>row.expected));
 for(const compile of [(text:string)=>new Bun.Transpiler({loader:"ts"}).transformSync(text),(text:string)=>transformSync(text,{loader:"ts",target:"es2022"}).code])for(const row of fixture.freshCargoConsumers){
  const root=fs.mkdtempSync(join(output,"f-")),records:string[]=[],stages:string[]=[],build=join(root,"b"),target=join(root,"t"),home=join(root,"h");for(const directory of[build,target,home])fs.mkdirSync(directory);
  fs.writeFileSync(join(root,"Cargo.toml"),'[workspace]\nresolver="2"\nmembers=["owner","emitter"]\n[workspace.metadata.semio.repository]\nschema-version=1\nowner-manifests=[]\nmember-manifests=["owner/Cargo.toml","emitter/Cargo.toml"]\nexclude-patterns=[]\n');
  for(const [directory,name]of[["owner","selected"],["emitter","emitter"]]){fs.mkdirSync(join(root,directory));fs.writeFileSync(join(root,directory,"Cargo.toml"),'[package]\nname='+JSON.stringify(name)+'\nversion="0.0.0"\nedition="2021"\n[lib]\npath="lib.rs"\n');fs.writeFileSync(join(root,directory,"lib.rs"),"pub fn value()->u8{1}\n");}
  const storage=parseCargoPreparationStorageV1({version:1,directory:output});expect(storageAdmission(storage),JSON.stringify(storageAdmission.errors)).toBe(true);const policy=parseFreshProcessPolicyV1({...policyFixture.policy,storage}),args=row.args.map((arg:string)=>arg==="/repo/owner/Cargo.toml"?join(root,row.id==="describe-emitter"?"emitter":"owner","Cargo.toml"):arg),selected=row.command==="cargo"&&!args.includes("--manifest-path")?[args[0],"--manifest-path",join(root,"owner/Cargo.toml"),...args.slice(1)]:args,argv=row.command==="cargo"?[...selected,"--message-format=json"]:args,env={CARGO_HOME:home,CARGO_TARGET_DIR:target,CARGO_BUILD_BUILD_DIR:build},expectedEnv={...env,SEMIO_COMPILER_RESOURCE_ROOT:join(build,"semio-compiler-resources")};
  const signal=new AbortController(),started=performance.now(),control={process:policy,diagnosticsRoot:root,cancelled:()=>signal.signal.aborted,remainingMs:()=>Math.floor(1000-(performance.now()-started)),checkpoint:(stage:string)=>{stages.push(stage);}};
  let plan:any;
  const bindings={freshCheckpoint,parseFreshProcessPolicyV1,CargoCliOperationOwner,CargoCommandFailure,runCargoCliCommand,selectedCargoArguments,cargoCommandPreparationV1:async(request:any,current:string[],operation:any)=>{records.push("plan");expect(request).toEqual({root,cwd:root,environment:env,storage});expect(current).toEqual(argv);plan=await cargoCommandPreparationV1(request,current,operation);expect(plan.cwd).toBe(root);expect(plan.args.slice(1)).toEqual(["synchronize","--manifest","Cargo.toml","--package",row.id==="describe-emitter"?"emitter":"selected"]);return plan;},isAbsolute,isGeneratedPath,lstatSync:fs.lstatSync,mkdtempSync:fs.mkdtempSync,join,tmpdir:()=>output,setInterval,clearInterval,AbortController,acquireCargoBuildLeaseV1:async(options:any)=>{expect(options.directory).toBe(output);expect(options.buildDirectory).toBe(build);records.push("lease");const lease=await acquireCargoBuildLeaseV1(options);return{release:()=>{lease.release();records.push("release");}};},cargoDirectories:()=>({build}),mkdirSync:fs.mkdirSync,captureOwnedProcess:async(command:string,current:string[],options:any)=>{expect(options.budgetMs).toBeGreaterThan(0);expect(options.budgetMs).toBeLessThanOrEqual(1000);expect(options.cancelled()).toBe(false);if(command===plan?.command){records.push("prepare");expect(current).toEqual(plan.args);expect(options.cwd).toBe(root);expect(options.env).toEqual({...expectedEnv,NX_WORKSPACE_ROOT:root,SEMIO_CARGO_PREPARATION_STORAGE:JSON.stringify(storage)});return{status:row.refused?1:0,signal:null,stdout:"",stderr:row.refused?"current selected preparation refused":""};}records.push("capture");expect(command).toBe(row.command);expect(current).toEqual(argv);expect(options.cwd).toBe(root);expect(options.env).toEqual(row.command==="cargo"?expectedEnv:env);return{status:0,signal:null,stdout:"",stderr:""};},Date,performance,resolve,homedir,CurrentPhysicalOwnerV1,writeCompletedCargoInvocationProvenanceV1,Map,writeFileSync:fs.writeFileSync,rmSync:fs.rmSync,console};
  const run=new Function(...Object.keys(bindings),compile(body)+";return freshRun;")(...Object.values(bindings));
  try{const call=()=>run(row.command,args,root,env,control,row.id,0,1);if(row.refused)await expect(call()).rejects.toThrow("current selected preparation refused");else{const receipt=await call();if(row.command==="cargo"){const retained=JSON.parse(readFileSync(receipt,"utf8"));expect(retained.args).toEqual(argv);expect(retained.manifest).toBe(resolve(root,selected[selected.indexOf("--manifest-path")+1]));expect(stages).toContain("compiler-provenance");}}expect(records,row.id).toEqual(row.expected);expect(stages).toContain(row.id);}finally{fs.rmSync(root,{recursive:true,force:true});}
 }
 console.log("[DEBUG] original Fresh consumer rows execute actual finite selector/preparation/lease/physical writer ports with mandatory original storage/policy; original Node argv oracle and independent esbuild agree");
});


test("current Cargo relay physical authority retains cancellation through publication",async()=>{
 const {EventEmitter}=await import("node:events"),{PassThrough}=await import("node:stream"),fs=await import("node:fs"),{createInterface}=await import("node:readline"),{randomUUID,createHash}=await import("node:crypto"),{homedir}=await import("node:os");
 const repository=resolve(library,"../../../../.."),general=join(repository,"🧰️framework/🔨️modules"),{CurrentPhysicalOwnerV1}=await import(join(general,"📁️filesystem/🧾️observation/📁️current/🟦️.ts")),writer=await import(join(general,"🏃️process/📦️artifacts/🏗️native-build/🟦️.ts"));
 const syntax=ts.createSourceFile("relay.ts",readFileSync(join(library,"🏃️process/🟦️.ts"),"utf8"),ts.ScriptTarget.Latest,true),source=syntax.statements.find((node:any)=>node.name?.text==="cargoStreamingStatus")!.getText(syntax).replace(/^export /,"");
 const edge='await import("../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts")';expect(source.split(edge).length).toBe(2);
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR!;expect(isAbsolute(output)).toBe(true);fs.mkdirSync(output,{recursive:true});
 for(const compile of [(text:string)=>new Bun.Transpiler({loader:"ts"}).transformSync(text),(text:string)=>transformSync(text,{loader:"ts",target:"es2022"}).code])for(const row of fixture.provenanceRelay.cases){
  const root=fs.mkdtempSync(join(output,"r-")),manifest=join(root,"Cargo.toml"),original=join(root,"lib.rs"),artifact=join(root,"libselected.rmeta"),depInfo=join(root,"libselected.d"),receiptRoot=join(root,"receipts"),build=join(root,"build");
  fs.mkdirSync(receiptRoot);fs.mkdirSync(build);fs.writeFileSync(manifest,'[package]\nname="selected"\nversion="0.1.0"\n');fs.writeFileSync(original,fixture.provenanceRelay.source);fs.writeFileSync(artifact,Buffer.from([0,255,1,128]));fs.writeFileSync(depInfo,`${artifact}: ${original}\n`);
  const message={reason:"compiler-artifact",manifest_path:manifest,package_id:"selected",target:{kind:["lib"],name:"selected",src_path:original},filenames:[artifact]},events:any[]=[],signals:any=new EventEmitter();let child:any,cancelled=false;
  const currentProcess=Object.assign(signals,{platform:process.platform,stdout:new PassThrough(),stderr:new PassThrough()}),env={SEMIO_TEST_ARTIFACT_DIR:receiptRoot,CARGO_HOME:root,CARGO_BUILD_BUILD_DIR:build,NX_WORKSPACE_ROOT:root};
  const cargoSyntax=ts.createSourceFile("cargo.ts",readFileSync(join(library,"🗂️workspaces/🦀️cargo/🟦️.ts"),"utf8"),ts.ScriptTarget.Latest,true),manifestBody=cargoSyntax.statements.find((node:any)=>node.name?.text==="cargoInvocationManifestV1")!.getText(cargoSyntax).replace(/^export /,"");
  const cargoInvocationManifestV1=new Function("resolve","join",compile(manifestBody)+";return cargoInvocationManifestV1;")(resolve,join);
  const bindings={cargoInvocationManifestV1,repositoryCargoPreparationStorageV1:(value:string)=>{expect(value).toBe(root);return parseCargoPreparationStorageV1({version:1,directory:output});},getWorkspaceRoot:()=>root,prepareCargoWorkspaceInvocation:(_storage:any,ownerRoot:string,args:string[],cwd:string,currentEnv:any)=>{expect(ownerRoot).toBe(root);expect(args).toEqual(fixture.provenanceRelay.args);expect(cwd).toBe(root);expect(currentEnv).toBe(env);events.push("prepare")},observesCargoInvocation:()=>true,cargoDirectories:()=>({build}),join,resolve,homedir,randomUUID,createInterface,CurrentPhysicalOwnerV1,provenanceOwner:async()=>writer,process:currentProcess,budgetTimeoutHint:()=>"",terminateOwnedProcessTree:()=>{throw Error("Completed neutral child must not be terminated")},console:{error(){},log(label:string){const progress=JSON.parse(label.slice("[cargo-provenance] ".length));events.push(progress);if(row.cancelDuringRead&&!cancelled){cancelled=true;expect(signals.listenerCount("SIGINT")).toBe(1);signals.emit("SIGINT","SIGINT")}}},spawn:(command:string,args:string[],options:any)=>{expect(command).toBe("cargo");expect(args).toEqual(fixture.provenanceRelay.args);expect(options.cwd).toBe(root);expect(options.env).toEqual({...env,SEMIO_COMPILER_RESOURCE_ROOT:join(build,"semio-compiler-resources")});child=new EventEmitter();child.stdout=new PassThrough();child.stderr=new PassThrough();child.exitCode=null;child.signalCode=null;queueMicrotask(()=>{child.stdout.end(JSON.stringify(message)+"\n");child.stderr.end();child.exitCode=row.status;child.emit("close",row.status,null)});return child;}};
  const execute=new Function(...Object.keys(bindings),compile(source.replace(edge,"await provenanceOwner()"))+";return cargoStreamingStatus;")(...Object.values(bindings));
  try{
   if(row.published)expect(await execute(fixture.provenanceRelay.args,root,env,0),row.id).toBe(row.status);else await expect(execute(fixture.provenanceRelay.args,root,env,0)).rejects.toThrow();
   const receipts=fs.readdirSync(receiptRoot);expect(receipts.length,row.id).toBe(row.published?1:0);expect(signals.listenerCount("SIGINT")).toBe(0);expect(signals.listenerCount("SIGTERM")).toBe(0);expect(events[0]).toBe("prepare");expect(events.some(event=>event.phase==="read")).toBe(true);
   if(row.published){const receipt=JSON.parse(fs.readFileSync(join(receiptRoot,receipts[0]),"utf8")),unit=receipt.units[0],input=unit.inputs.find((entry:any)=>entry.path===original),bytes=fs.readFileSync(original),digest=createHash("sha256").update(bytes).digest("hex");expect(input.sha256).toBe(digest);expect(input.sha256).toBe(Buffer.from(await crypto.subtle.digest("SHA-256",bytes)).toString("hex"));expect(unit.artifacts[0].sha256).toBe(createHash("sha256").update(fs.readFileSync(artifact)).digest("hex"));expect(receipt.status).toBe(row.status);expect(receipt.args).toEqual(fixture.provenanceRelay.args);expect(receipt.cancelled).toBe(false);}
  }finally{fs.rmSync(root,{recursive:true,force:true});}
 }
 for(const path of [".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){const errors:any[]=[],launch=parseJsonc(readFileSync(join(repository,path),"utf8"),errors);expect(errors).toEqual([]);const entries=launch.configurations.filter((entry:any)=>entry.name==="🧫️fixtures-testing-only-provenance-physical🧪️");expect(entries.length).toBe(1);expect(entries[0].command).toContain("ticket-fixture-verification:publication-provenance-physical-source");expect(entries[0].presentation).toEqual({group:"4_gate",order:900.29872});expect(entries[0].env.SEMIO_TEST_ARTIFACT_DIR.endsWith("/🗑️generated/p")).toBe(true);}
 console.log("[DEBUG] current selected Cargo relay and real physical writer retain original-byte/digest/dep-info authority, compiler refusal status, progress and cancellation through publication; independent esbuild and Node/WebCrypto agree; neutral transport starts no compiler");
});
