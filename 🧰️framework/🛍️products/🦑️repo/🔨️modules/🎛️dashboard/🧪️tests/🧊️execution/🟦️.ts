import{createIssuedResourceControlV1}from"../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {createRequire} from "node:module";
import {createHash} from "node:crypto";
import {chmodSync,copyFileSync,constants,existsSync,mkdirSync,readFileSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {basename,dirname,join,relative} from "node:path";
import {receiveScriptProcessInvocation,createScriptProcessEnvelope,withScriptProcessEnvelope,type ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {captureOwnedCommandV1} from "@semio-tech/process/owned-execution";
import {scriptCompilerAdmissionControlV1} from "@semio-tech/process/compiler-capacity";
import {stageArtifacts} from "@semio-tech/process/artifact-publication";
import {publishedCargoArtifactDirectoryV1} from "@semio-tech/process/artifacts/native-build/published";
import { withRepositoryCompilerStoreV1, withRepositoryToolingStoreV1, withRepositoryNxStorageV1 } from "@semio-tech/repo-lib/storage";
import { buildGraphPathV1 } from "@semio-tech/repo-lib/build";
import {captureDashboardSources,digestDashboardSources,installDashboard,dashboardInvocation,launchDashboard,mayRunStale,staleDashboard,type DashboardExecutionOptions} from "../../📦️installation/🟦️.ts";
import {dashboardExecutable} from "../../📦️packages/🦀️rust/📜️script.ts";
import installationSchema from "../../📦️installation/🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🧊️execution/🔣️.json" with {type:"json"};

const artifactRoot=()=>{const root=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!root)throw Error("Original dashboard test artifact root required");return root;};
let fixtureSequence=0;
function fixtureRoot():string{for(let attempt=0;attempt<36;attempt++){const root=join(artifactRoot(),"d"+(fixtureSequence++).toString(36));if(existsSync(root))continue;mkdirSync(root);return root;}throw Error("Bounded dashboard fixture namespace exhausted");}
async function cleanupFixtureRoot(root:string):Promise<void>{
 await receiveScriptProcessInvocation(process.env,async original=>{const control=scriptCompilerAdmissionControlV1(original,"dashboard:fixture-cleanup",120000);if(process.platform==="darwin")for(const partition of["r","t","l"]){const mounted=join(root,".cache/b",partition,"m");if(existsSync(mounted)&&statSync(mounted).dev!==statSync(dirname(mounted)).dev)await captureOwnedCommandV1("hdiutil",["detach",mounted],process.cwd(),{control,env:process.env});}rmSync(root,{recursive:true,force:true});});
}
const name=process.platform==="win32"?"semio.exe":"semio";
const mode=()=>{const value=process.env.SEMIO_BUILD_MODE;if(value!=="prototype"&&value!=="release")throw Error("Original dashboard fixture mode required");return value;};
function stalenessTree(){const root=fixtureRoot();for(const[path,content]of Object.entries(fixture.staleness.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content);}return{root,packageRoot:join(root,fixture.staleness.package)};}
async function withPublishedFixture<T>(root:string,packageRoot:string,original:ScriptInvocation,body:(publish:(bytes:string|Buffer)=>Promise<void>,options:DashboardExecutionOptions)=>Promise<T>):Promise<T>{
 const selected=mode(),control=scriptCompilerAdmissionControlV1(original,"dashboard:fixture",120000),manifest=join(packageRoot,"Cargo.toml"),options:DashboardExecutionOptions={workspace:process.cwd(),mode:selected,network:{offline:true},control,invocation:original};
 return withRepositoryCompilerStoreV1(process.cwd(),selected,control,process.env,async (store,control)=>{const source=join(store.captureDirectory,"dashboard-fixture-"+basename(root)),published=publishedCargoArtifactDirectoryV1(store.root,manifest,selected),publish=async(bytes:string|Buffer)=>{writeFileSync(source,bytes);chmodSync(source,0o755);await stageArtifacts(published,relative(process.cwd(),manifest).replaceAll("\\","/"),new Map([[name,source]]),{control,leaseDirectory:join(process.cwd(),".cache/b/l/m/data")});};try{return await withRepositoryToolingStoreV1(process.cwd(),original,control,()=>body(publish,options));}finally{rmSync(source,{force:true});rmSync(published,{recursive:true,force:true});}});
}

test("native launch arguments match shared vectors while Nx keeps compile and installation scheduling",async()=>receiveScriptProcessInvocation(process.env,async original=>{
 for(const row of fixture.cases)expect(dashboardInvocation(row.args)??null).toEqual(row.expected);for(const row of fixture.mayRunStale)expect(mayRunStale(row.args),row.args.join(" ")).toBe(row.expected);
 const control=scriptCompilerAdmissionControlV1(original,"dashboard:nx-scheduling",120000);
 await withRepositoryNxStorageV1(process.cwd(),original,control,async bind=>{const identity=process.env.SEMIO_NX_TOOLING_IDENTITY,sdk=process.env.SEMIO_NX_TOOLING_ROOT;if(!identity||!sdk)throw Error("Original actual Nx SDK required");const namespace=await bind(identity,null,(await import("@semio-tech/repo-lib/build/inference")).nxBuildDependencyImplementationV1()),prior=Object.fromEntries(Object.keys(namespace.environment).map(key=>[key,process.env[key]]));for(const[key,value]of Object.entries(namespace.environment))if(value===undefined)delete process.env[key];else process.env[key]=value;try{const{createTaskGraph}=createRequire(join(sdk,"package.json"))("nx/src/tasks-runner/create-task-graph"),project=JSON.parse(readFileSync(new URL("../../📦️packages/🦀️rust/📋️project.json",import.meta.url),"utf8")),graph={nodes:{[project.name]:{name:project.name,type:"lib",data:{root:"dashboard",targets:project.targets}}},dependencies:{[project.name]:[]}};expect(Object.keys(createTaskGraph(graph,{},[project.name],["run"],undefined,{}).tasks)).toEqual([project.name+":run"]);expect(Object.keys(createTaskGraph(graph,{},[project.name],["install"],undefined,{}).tasks).sort()).toEqual([project.name+":build",project.name+":install"]);}finally{for(const[key,value]of Object.entries(prior))if(value===undefined)delete process.env[key];else process.env[key]=value;}});
 const project=JSON.parse(readFileSync(new URL("../../📦️packages/🦀️rust/📋️project.json",import.meta.url),"utf8"));expect(project.targets.compile.metadata.semio.rustBuildIntent.binary).toBe("semio");const script=readFileSync(new URL("../../📦️packages/🦀️rust/📜️script.ts",import.meta.url),"utf8");expect(script.indexOf("const sources = await captureDashboardSources")).toBeLessThan(script.indexOf("await runDeclaredRustBuildV1(workspace"));expect(script.indexOf("await runDeclaredRustBuildV1(workspace")).toBeLessThan(script.indexOf("return await installDashboard(packageRoot"));
}));

test("dashboard immutable installation preserves old bytes and reuses identical published executable",async()=>{
 const root=fixtureRoot(),packageRoot=join(root,"p");mkdirSync(packageRoot);writeFileSync(join(packageRoot,"Cargo.toml"),'[package]\nname="dashboard-fixture"\nversion="0.0.0"\n');try{await receiveScriptProcessInvocation(process.env,original=>withPublishedFixture(root,packageRoot,original,async(publish,options)=>{const first=Buffer.from("first executable"),second=Buffer.from("second executable");await publish(first);const a=await installDashboard(packageRoot,root,options);expect(basename(dirname(a))).toBe(createHash("sha256").update(first).digest("hex"));expect(await dashboardExecutable(packageRoot,root,options)).toBe(a);await publish(second);expect(await dashboardExecutable(packageRoot,root,options)).toBe(a);const b=await installDashboard(packageRoot,root,options);expect(b).not.toBe(a);expect(readFileSync(a)).toEqual(first);expect(readFileSync(b)).toEqual(second);}));}finally{await cleanupFixtureRoot(root);}
},120000);

test("source digest names the crate closure and changes only for compiled Source vectors",async()=>{
 for(const mutation of fixture.staleness.mutations){const{root,packageRoot}=stalenessTree();try{const before=await captureDashboardSources(packageRoot,root);expect([...before.roots],mutation.name).toEqual(fixture.staleness.roots);for(const[path,content]of Object.entries(mutation.write??{})){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content);}for(const[path,seconds]of Object.entries(mutation.touch??{})){const time=statSync(join(root,path)).mtimeMs/1000+seconds;utimesSync(join(root,path),time,time);}expect(await digestDashboardSources(root,before.roots)!==before.digest,mutation.name).toBe(mutation.stale);}finally{await cleanupFixtureRoot(root);}}
});

test("installation Source staleness excludes publication bytes and records genuine Source changes",async()=>{
 const{root,packageRoot}=stalenessTree();try{await receiveScriptProcessInvocation(process.env,original=>withPublishedFixture(root,packageRoot,original,async(publish,options)=>{expect(await staleDashboard(root,options.mode)).toBe("it is not installed");await publish("first executable");await installDashboard(packageRoot,root,options);expect(await staleDashboard(root,options.mode)).toBeUndefined();await publish("second executable");expect(await staleDashboard(root,options.mode)).toBeUndefined();writeFileSync(join(root,"mod/🎮️registry/🦀️.rs"),"pub fn registry(){2;}\n");expect(await staleDashboard(root,options.mode)).toMatch(/^its sources changed since installation \(\d+ files\)$/);await installDashboard(packageRoot,root,options);expect(await staleDashboard(root,options.mode)).toBeUndefined();writeFileSync(join(root,"mod/🎮️registry/🆕️.rs"),"pub fn added(){}\n");expect(await staleDashboard(root,options.mode)).toMatch(/\d+ -> \d+ files/);}));}finally{await cleanupFixtureRoot(root);}
},120000);

test("installation receipt schema binds selected mode to exact finite tools bytes and permissions",async()=>{
 const root=fixtureRoot(),packageRoot=join(root,"p"),oracle=new Ajv({strict:true}).compile(installationSchema);mkdirSync(packageRoot);writeFileSync(join(packageRoot,"Cargo.toml"),'[package]\nname="dashboard-fixture"\nversion="0.0.0"\n');try{await receiveScriptProcessInvocation(process.env,original=>withPublishedFixture(root,packageRoot,original,async(publish,options)=>{await publish(options.mode);const path=await installDashboard(packageRoot,root,options),record=JSON.parse(readFileSync(join(root,".🧬semio/🦑️repo/⚡️cache/🎛️dashboard",options.mode,"installed.json"),"utf8"));expect(oracle(record)).toBe(true);expect(record.mode).toBe(options.mode);expect(record.path).toContain(`.cache/b/t/m/dashboard-cli/${options.mode}/`);expect(record.bytes).toBe(Buffer.byteLength(options.mode));expect(record.permissions).toBe(0o755);expect(await dashboardExecutable(packageRoot,root,options)).toBe(path);}));}finally{await cleanupFixtureRoot(root);}
},120000);

/** 🛫️ The finite installed dashboard receives the unchanged original process handoff. */
test("installed finite dashboard retains authored capabilities and original deadline through its child lifetime",async()=>{
 const root=fixtureRoot(),workspace=process.cwd(),selected=mode(),authored=fixture.installedRunner.authoring;
 try{await receiveScriptProcessInvocation(process.env,original=>withRepositoryToolingStoreV1(workspace,original,createIssuedResourceControlV1(original,"cache:entry",0),async tools=>{
  const control=scriptCompilerAdmissionControlV1(original,"dashboard:bridge-fixture",120000),source=join(tools.root,"bridge-"+basename(root)),bytes=readFileSync(process.execPath),digest=createHash("sha256").update(bytes).digest("hex"),executable=join(tools.root,"dashboard-cli",selected,digest,name),directory=dirname(executable),existing=existsSync(directory);
  copyFileSync(process.execPath,source,constants.COPYFILE_FICLONE);chmodSync(source,0o755);
  try{
   await stageArtifacts(directory,"dashboard-installation:"+selected,new Map([[name,source]]),{control,leaseDirectory:join(workspace,".cache/b/l/m/data")});
   writeFileSync(join(root,"daemon"),'import {writeFileSync} from "node:fs";writeFileSync("observed.json",JSON.stringify({arguments:process.argv.slice(2),mode:process.env.SEMIO_BUILD_MODE,envelope:JSON.parse(process.env.SEMIO_SCRIPT_PROCESS_INVOCATION)}));console.log("[DEBUG] finite installed dashboard original handoff");');
   const record={version:3,platform:process.platform,mode:selected,path:relative(workspace,executable).replaceAll("\\","/"),sources:{roots:["fixture"],digest:`0:${"a".repeat(40)}`},digest,bytes:bytes.length,permissions:0o755};expect(new Ajv({strict:true}).compile(installationSchema)(record)).toBe(true);
   const file=join(root,".🧬semio/🦑️repo/⚡️cache/🎛️dashboard",selected,"installed.json");mkdirSync(dirname(file),{recursive:true});writeFileSync(file,JSON.stringify(record));
   const capabilities={tool:"bazel",arguments:authored.arguments,native:authored.native},envelope=createScriptProcessEnvelope({...authored.policy,version:1},capabilities,Date.now());
   await new Promise<void>(done=>setTimeout(done,authored.receivingDelayMilliseconds));
   await withScriptProcessEnvelope(envelope,async invocation=>{const launchControl=scriptCompilerAdmissionControlV1(invocation,"dashboard:bridge-launch",30000);expect(await launchDashboard(root,["daemon","status"],{workspace,mode:selected,network:{offline:true},control:launchControl,invocation})).toBe(0);});
   const observed=JSON.parse(readFileSync(join(root,"observed.json"),"utf8"));expect(observed.arguments).toEqual(["status"]);expect(observed.mode).toBe(selected);expect(observed.envelope.policy).toEqual(authored.policy);expect(observed.envelope.capabilities).toEqual(capabilities);expect(observed.envelope.deadlineEpochMilliseconds).toBeLessThanOrEqual(envelope.deadlineEpochMilliseconds!);expect(observed.envelope.deadlineEpochMilliseconds).toBeGreaterThan(envelope.deadlineEpochMilliseconds!-100);expect(()=>statSync(buildGraphPathV1(root))).toThrow();
  }finally{rmSync(source,{force:true});if(!existing)rmSync(directory,{recursive:true,force:true});}
 }));}finally{await cleanupFixtureRoot(root);}
},120000);
