import "./🚀️launch/🟦️.ts";
import "./🛠️bootstrap/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {createRequire} from "node:module";
import {createHash} from "node:crypto";
import {chmodSync,copyFileSync,constants,existsSync,mkdirSync,readFileSync,renameSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {basename,dirname,join,relative} from "node:path";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,type ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withBuildTestInvocationV1} from "../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {captureOwnedCommandV1} from "@semio-tech/process/owned-execution";
import {scriptCompilerAdmissionControlV1} from "@semio-tech/process/compiler-capacity";
import {stageArtifacts} from "@semio-tech/process/artifact-publication";
import {publishedCargoArtifactDirectoryV1} from "@semio-tech/process/artifacts/native-build/published";
import { withRepositoryCompilerStoreV1, withRepositoryToolingStoreV1, withRepositoryNxStorageV1 } from "@semio-tech/repo-lib/storage";
import { buildGraphPathV1 } from "@semio-tech/repo-lib/build";
import {captureDashboardSources,digestDashboardSources,dashboardInstalled,installedDashboard,installDashboard,dashboardInvocation,launchDashboard,mayRunStale,staleDashboard,type DashboardExecutionOptions} from "../../../📚️library/⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts";
import {dashboardExecutable} from "../../📦️packages/🦀️rust/📜️script.ts";
import installationSchema from "../../../📚️library/⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🧊️execution/🔣️.json" with {type:"json"};
import workspaceFixture from "../🧫️fixtures/🧊️execution/🧱️workspaces.json" with {type:"json"};

const artifactRoot=()=>{const root=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!root)throw Error("Original dashboard test artifact root required");return root;};
let fixtureSequence=0;
function fixtureRoot():string{for(let attempt=0;attempt<36;attempt++){const root=join(artifactRoot(),"d"+(fixtureSequence++).toString(36));if(existsSync(root))continue;mkdirSync(root);return root;}throw Error("Bounded dashboard fixture namespace exhausted");}
async function cleanupFixtureRoot(root:string):Promise<void>{
 await withBuildTestInvocationV1(async original=>{const control=scriptCompilerAdmissionControlV1(original,"dashboard:fixture-cleanup",120000);if(process.platform==="darwin")for(const partition of["r","t","l"]){const mounted=join(root,".cache/b",partition,"m");if(existsSync(mounted)&&statSync(mounted).dev!==statSync(dirname(mounted)).dev)await captureOwnedCommandV1("hdiutil",["detach",mounted],process.cwd(),{control,env:process.env});}rmSync(root,{recursive:true,force:true});});
}
const name=process.platform==="win32"?"semio.exe":"semio";
const mode=()=>{const value=process.env.SEMIO_BUILD_MODE;if(value!=="prototype"&&value!=="release")throw Error("Original dashboard fixture mode required");return value;};
function stalenessTree(){const root=fixtureRoot();for(const[path,content]of Object.entries(fixture.staleness.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content);}return{root,packageRoot:join(root,fixture.staleness.package)};}
async function withPublishedFixture<T>(root:string,packageRoot:string,original:ScriptInvocation,body:(publish:(bytes:string|Buffer)=>Promise<void>,options:DashboardExecutionOptions)=>Promise<T>):Promise<T>{
 const selected=mode(),control=scriptCompilerAdmissionControlV1(original,"dashboard:fixture",120000),manifest=join(packageRoot,"Cargo.toml"),options:DashboardExecutionOptions={workspace:process.cwd(),mode:selected,network:{offline:true},control,invocation:original};
 return withRepositoryCompilerStoreV1(process.cwd(),selected,control,process.env,async (store,control)=>{const source=join(store.captureDirectory,"dashboard-fixture-"+basename(root)),published=publishedCargoArtifactDirectoryV1(store.root,manifest,selected),publish=async(bytes:string|Buffer)=>{writeFileSync(source,bytes);chmodSync(source,0o755);await stageArtifacts(published,relative(process.cwd(),manifest).replaceAll("\\","/"),new Map([[name,source]]),{control,leaseDirectory:join(process.cwd(),".cache/b/l/m/data")});};try{return await withRepositoryToolingStoreV1(process.cwd(),original,control,()=>body(publish,options));}finally{rmSync(source,{force:true});rmSync(published,{recursive:true,force:true});}});
}

test("native launch arguments match shared vectors while Nx keeps compile and installation scheduling",async()=>withBuildTestInvocationV1(async original=>{
 for(const row of fixture.cases)expect(dashboardInvocation(row.args)??null).toEqual(row.expected);for(const row of fixture.mayRunStale)expect(mayRunStale(row.args),row.args.join(" ")).toBe(row.expected);
 const control=scriptCompilerAdmissionControlV1(original,"dashboard:nx-scheduling",120000);
 await withRepositoryNxStorageV1(process.cwd(),original,control,async bind=>{const identity=process.env.SEMIO_NX_TOOLING_IDENTITY,sdk=process.env.SEMIO_NX_TOOLING_ROOT;if(!identity||!sdk)throw Error("Original actual Nx SDK required");const namespace=await bind(identity,null,(await import("@semio-tech/repo-lib/build/inference")).nxBuildDependencyImplementationV1()),prior=Object.fromEntries(Object.keys(namespace.environment).map(key=>[key,process.env[key]]));for(const[key,value]of Object.entries(namespace.environment))if(value===undefined)delete process.env[key];else process.env[key]=value;try{const{createTaskGraph}=createRequire(join(sdk,"package.json"))("nx/src/tasks-runner/create-task-graph"),project=JSON.parse(readFileSync(new URL("../../📦️packages/🦀️rust/📋️project.json",import.meta.url),"utf8")),graph={nodes:{[project.name]:{name:project.name,type:"lib",data:{root:"dashboard",targets:project.targets}}},dependencies:{[project.name]:[]}};expect(Object.keys(createTaskGraph(graph,{},[project.name],["run"],undefined,{}).tasks)).toEqual([project.name+":run"]);expect(Object.keys(createTaskGraph(graph,{},[project.name],["install"],undefined,{}).tasks).sort()).toEqual([project.name+":build",project.name+":install"]);}finally{for(const[key,value]of Object.entries(prior))if(value===undefined)delete process.env[key];else process.env[key]=value;}});
 const project=JSON.parse(readFileSync(new URL("../../📦️packages/🦀️rust/📋️project.json",import.meta.url),"utf8"));expect(project.targets.compile.metadata.semio.rustBuildIntent.binary).toBe("semio");const script=readFileSync(new URL("../../📦️packages/🦀️rust/📜️script.ts",import.meta.url),"utf8");expect(script.indexOf("const sources = await captureDashboardSources")).toBeLessThan(script.indexOf("await runDeclaredRustBuildV1(workspace"));expect(script.indexOf("await runDeclaredRustBuildV1(workspace")).toBeLessThan(script.indexOf("return await installDashboard(packageRoot"));
}));

test("dashboard immutable installation preserves old bytes and reuses identical published executable",async()=>{
 const root=fixtureRoot(),packageRoot=join(root,"p");mkdirSync(packageRoot);writeFileSync(join(packageRoot,"Cargo.toml"),'[package]\nname="dashboard-fixture"\nversion="0.0.0"\n');try{await withBuildTestInvocationV1(original=>withPublishedFixture(root,packageRoot,original,async(publish,options)=>{const first=Buffer.from("first executable"),second=Buffer.from("second executable");await publish(first);const a=await installDashboard(packageRoot,root,options);expect(basename(dirname(a))).toBe(createHash("sha256").update(first).digest("hex"));expect(await dashboardExecutable(packageRoot,root,options)).toBe(a);await publish(second);expect(await dashboardExecutable(packageRoot,root,options)).toBe(a);const b=await installDashboard(packageRoot,root,options);expect(b).not.toBe(a);expect(readFileSync(a)).toEqual(first);expect(readFileSync(b)).toEqual(second);}));}finally{await cleanupFixtureRoot(root);}
},120000);

test("source digest names the crate closure and changes only for compiled Source vectors",async()=>{
 for(const mutation of fixture.staleness.mutations){const{root,packageRoot}=stalenessTree();try{const before=await captureDashboardSources(packageRoot,root);expect([...before.roots],mutation.name).toEqual(fixture.staleness.roots);for(const[path,content]of Object.entries(mutation.write??{})){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content);}for(const[path,seconds]of Object.entries(mutation.touch??{})){const time=statSync(join(root,path)).mtimeMs/1000+seconds;utimesSync(join(root,path),time,time);}expect(await digestDashboardSources(root,before)!==before.digest,mutation.name).toBe(mutation.stale);}finally{await cleanupFixtureRoot(root);}}
});

test("source digest retains each actual Cargo workspace and table-form inherited dependency",async()=>{
 const {parse}=await import("@iarna/toml"),validate=new Ajv({strict:true}).compile(installationSchema.properties.sources);
 for(const selection of [workspaceFixture,...workspaceFixture.selections])for(const row of selection.mutations){const root=fixtureRoot();try{for(const[path,content]of Object.entries(workspaceFixture.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content);if(path.endsWith("Cargo.toml"))expect(Bun.TOML.parse(content)).toEqual(parse(content));}const before=await captureDashboardSources(join(root,selection.package),root);expect(validate(before)).toBe(true);expect(before.roots).toEqual(selection.roots);expect(before.manifests).toEqual(selection.manifests);const declaration=parse(readFileSync(join(root,selection.package,"Cargo.toml"),"utf8")),declared=(declaration.package as {workspace?:string}).workspace,lock=declared?relative(root,join(root,selection.package,declared,"Cargo.lock")).replaceAll("\\","/"):"Cargo.lock";expect(before.manifests).toContain(lock);const time=statSync(join(root,row.path)).mtimeMs/1000+2;utimesSync(join(root,row.path),time,time);expect(await digestDashboardSources(root,before)!==before.digest,selection.package+": "+row.path).toBe(row.stale);}finally{await cleanupFixtureRoot(root);}}
 console.log("[DEBUG] Dashboard freshness retains exact dependency workspaces and inherited path sources");
});

test("source digest and installed record routing recover invalid metadata before stale-capable controls",async()=>{
 const validate=new Ajv({strict:true}).compile(installationSchema),database=new Database(":memory:");
 try{for(const row of fixture.installationRecordCases){const{root,packageRoot}=stalenessTree();try{
  const selected=mode(),digest="a".repeat(64),record:any={version:3,platform:process.platform,mode:selected,path:`.cache/b/t/m/dashboard-cli/${selected}/${digest}/${name}`,sources:await captureDashboardSources(packageRoot,root),digest,bytes:1,permissions:0o755},file=join(root,".🧬semio/🦑️repo/⚡️cache/🎛️dashboard",selected,"installed.json");
  if(row.mutation==="version")record.version=2;else if(row.mutation==="platform")record.platform=process.platform==="linux"?"darwin":"linux";else if(row.mutation==="mode")record.mode=selected==="prototype"?"release":"prototype";else if(row.mutation==="path")record.path=".cache/b/t/m/other/semio";else if(row.mutation==="manifests")delete record.sources.manifests;else if(row.mutation==="unknown")record.unknown=true;
  if(row.mutation!=="absent"){mkdirSync(dirname(file),{recursive:true});writeFileSync(file,row.mutation==="unreadable"?"{":JSON.stringify(record));}
  const independent=database.query("select ?1 and json_extract(?2,'$.platform')=?3 and json_extract(?2,'$.mode')=?4 and json_extract(?2,'$.path')='.cache/b/t/m/dashboard-cli/'||?4||'/'||json_extract(?2,'$.digest')||'/'||?5 installed").get(Number(row.mutation!=="absent"&&row.mutation!=="unreadable"&&validate(record)),JSON.stringify(record),process.platform,selected,name)as{installed:number};
  expect(Boolean(independent.installed),row.name).toBe(row.installed);expect(dashboardInstalled(root,selected),row.name).toBe(row.installed);expect(await staleDashboard(root,selected),row.name).toBe(row.reason??undefined);if(row.installed)expect(installedDashboard(root,selected,root)).toBe(join(root,record.path));else expect(()=>installedDashboard(root,selected,root),row.name).toThrow();expect(mayRunStale(["tasks"])).toBe(true);
 }finally{await cleanupFixtureRoot(root);}}}finally{database.close();}
 console.log("[DEBUG] Actual metadata-only installation probes reject invalid records so observation/stop controls can rebuild before immutable executable admission");
});

test("source digest detects same-size atomic Source replacement with preserved timestamps",async()=>{
 const database=new Database(":memory:");try{for(const row of workspaceFixture.replacements){const root=fixtureRoot();try{
  for(const[path,content]of Object.entries(workspaceFixture.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content);}
  const path=join(root,row.path),prior=statSync(path),content=readFileSync(path,"utf8"),replacement=content.replace(row.from,row.to),before=await captureDashboardSources(join(root,workspaceFixture.package),root),temporary=path+".swap";expect(replacement).not.toBe(content);expect(Buffer.byteLength(replacement)).toBe(Buffer.byteLength(content));writeFileSync(temporary,replacement);utimesSync(temporary,prior.atime,prior.mtime);renameSync(temporary,path);const after=statSync(path);
  expect([after.size,Math.trunc(after.mtimeMs)],row.path).toEqual([prior.size,Math.trunc(prior.mtimeMs)]);const independent=database.query("select ?1<>?2 and (?3 in (select value from json_each(?4)) or exists(select 1 from json_each(?5) where substr(?3,1,length(value)+1)=value||'/')) stale").get(Buffer.from(content),Buffer.from(replacement),row.path,JSON.stringify(before.manifests),JSON.stringify(before.roots))as{stale:number};expect(Boolean(independent.stale),row.path).toBe(row.stale);expect(await digestDashboardSources(root,before)!==before.digest,row.path).toBe(row.stale);
 }finally{await cleanupFixtureRoot(root);}}}finally{database.close();}
 console.log("[DEBUG] Metadata-only Source signatures detect equal-size timestamp-preserving atomic edits while retaining exact dependency workspace scope");
});

test("installation Source staleness excludes publication bytes and records genuine Source changes",async()=>{
 const{root,packageRoot}=stalenessTree();try{await withBuildTestInvocationV1(original=>withPublishedFixture(root,packageRoot,original,async(publish,options)=>{expect(await staleDashboard(root,options.mode)).toBe("it is not installed");await publish("first executable");await installDashboard(packageRoot,root,options);expect(await staleDashboard(root,options.mode)).toBeUndefined();await publish("second executable");expect(await staleDashboard(root,options.mode)).toBeUndefined();writeFileSync(join(root,"mod/🎮️registry/🦀️.rs"),"pub fn registry(){2;}\n");expect(await staleDashboard(root,options.mode)).toMatch(/^its sources changed since installation \(\d+ files\)$/);await installDashboard(packageRoot,root,options);expect(await staleDashboard(root,options.mode)).toBeUndefined();writeFileSync(join(root,"mod/🎮️registry/🆕️.rs"),"pub fn added(){}\n");expect(await staleDashboard(root,options.mode)).toMatch(/\d+ -> \d+ files/);}));}finally{await cleanupFixtureRoot(root);}
},120000);

test("installation receipt schema binds selected mode to exact finite tools bytes and permissions",async()=>{
 const root=fixtureRoot(),packageRoot=join(root,"p"),oracle=new Ajv({strict:true}).compile(installationSchema);mkdirSync(packageRoot);writeFileSync(join(packageRoot,"Cargo.toml"),'[package]\nname="dashboard-fixture"\nversion="0.0.0"\n');try{await withBuildTestInvocationV1(original=>withPublishedFixture(root,packageRoot,original,async(publish,options)=>{await publish(options.mode);const path=await installDashboard(packageRoot,root,options),record=JSON.parse(readFileSync(join(root,".🧬semio/🦑️repo/⚡️cache/🎛️dashboard",options.mode,"installed.json"),"utf8"));expect(oracle(record)).toBe(true);expect(record.mode).toBe(options.mode);expect(record.path).toContain(`.cache/b/t/m/dashboard-cli/${options.mode}/`);expect(record.bytes).toBe(Buffer.byteLength(options.mode));expect(record.permissions).toBe(0o755);expect(await dashboardExecutable(packageRoot,root,options)).toBe(path);}));}finally{await cleanupFixtureRoot(root);}
},120000);

/** 🛫️ The finite installed dashboard receives the unchanged original process handoff. */
test("installed finite dashboard retains authored capabilities and original deadline through its child lifetime",async()=>{
 const root=fixtureRoot(),workspace=process.cwd(),selected=mode(),authored=fixture.installedRunner.authoring;
 try{await withBuildTestInvocationV1(original=>withRepositoryToolingStoreV1(workspace,original,createIssuedResourceControlV1(original,"cache:entry",0),async tools=>{
  const control=scriptCompilerAdmissionControlV1(original,"dashboard:bridge-fixture",120000),source=join(tools.root,"bridge-"+basename(root)),bytes=readFileSync(process.execPath),digest=createHash("sha256").update(bytes).digest("hex"),executable=join(tools.root,"dashboard-cli",selected,digest,name),directory=dirname(executable),existing=existsSync(directory);
  copyFileSync(process.execPath,source,constants.COPYFILE_FICLONE);chmodSync(source,0o755);
  try{
   await stageArtifacts(directory,"dashboard-installation:"+selected,new Map([[name,source]]),{control,leaseDirectory:join(workspace,".cache/b/l/m/data")});
   writeFileSync(join(root,"daemon"),'import {writeFileSync} from "node:fs";writeFileSync("observed.json",JSON.stringify({arguments:process.argv.slice(2),mode:process.env.SEMIO_BUILD_MODE,envelope:JSON.parse(process.env.SEMIO_SCRIPT_PROCESS_INVOCATION)}));console.log("[DEBUG] finite installed dashboard original handoff");');
   const record={version:3,platform:process.platform,mode:selected,path:relative(workspace,executable).replaceAll("\\","/"),sources:{roots:["fixture"],manifests:["Cargo.toml"],digest:`0:${"a".repeat(40)}`},digest,bytes:bytes.length,permissions:0o755};expect(new Ajv({strict:true}).compile(installationSchema)(record)).toBe(true);
   const file=join(root,".🧬semio/🦑️repo/⚡️cache/🎛️dashboard",selected,"installed.json");mkdirSync(dirname(file),{recursive:true});writeFileSync(file,JSON.stringify(record));
   const capabilities={tool:"bazel",arguments:authored.arguments,native:authored.native},envelope=createScriptProcessEnvelope({...authored.policy,version:1},capabilities,Date.now());
   await new Promise<void>(done=>setTimeout(done,authored.receivingDelayMilliseconds));
   await withScriptProcessEnvelope(envelope,async invocation=>{const launchControl=scriptCompilerAdmissionControlV1(invocation,"dashboard:bridge-launch",30000);expect(await launchDashboard(root,["daemon","status"],{workspace,mode:selected,network:{offline:true},control:launchControl,invocation})).toBe(0);});
   const observed=JSON.parse(readFileSync(join(root,"observed.json"),"utf8"));expect(observed.arguments).toEqual(["status"]);expect(observed.mode).toBe(selected);expect(observed.envelope.policy).toEqual(authored.policy);expect(observed.envelope.capabilities).toEqual(capabilities);expect(observed.envelope.deadlineEpochMilliseconds).toBeLessThanOrEqual(envelope.deadlineEpochMilliseconds!);expect(observed.envelope.deadlineEpochMilliseconds).toBeGreaterThan(envelope.deadlineEpochMilliseconds!-100);expect(()=>statSync(buildGraphPathV1(root))).toThrow();
  }finally{rmSync(source,{force:true});if(!existing)rmSync(directory,{recursive:true,force:true});}
 }));}finally{await cleanupFixtureRoot(root);}
},120000);
