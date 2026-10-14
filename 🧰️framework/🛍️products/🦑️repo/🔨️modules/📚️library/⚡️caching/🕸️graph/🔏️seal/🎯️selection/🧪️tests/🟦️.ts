import {test,expect} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,writeFileSync,rmSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {createRequire} from "node:module";
import {spawnSync} from "node:child_process";
import fg from "fast-glob";
import Ajv from "ajv";
import {createSelectedGraphSourceReceiptV1,selectedGraphSourceReceiptHoldsV1,isGlobalGraphIdentityPathV1} from "../🟦️.ts";
import {writeSelectedGraphPublisherReceiptV1,selectedGraphPublisherReceiptHoldsV1,captureSelectedGraphReuseReceiptV1} from "../📤️publisher/🟦️.ts";
import {writeSelectedGraphHandoffV1,selectedGraphHandoffHoldsV1} from "../📥️handoff/🟦️.ts";
import {selectedTaskScopeV1} from "../📥️scope/🟦️.ts";
import {nativeSelectedTaskPlannerV1} from "../📥️scope/🔌️sdk/🟦️.ts";
import {receiveScriptProcessInvocation,scriptInvocationEnvironment} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";

const fixture=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8")),schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8")),tooling=process.env.SEMIO_NX_TOOLING_IDENTITY;
if(!tooling||!/^[a-f0-9]{64}$/u.test(tooling))throw Error("Actual incoming immutable Nx tooling identity required");
const sdkRequire=createRequire(join(process.cwd(),".🧬semio/🦑️repo/⚡️cache/tools/nx",tooling,"package.json")),native=sdkRequire(sdkRequire.resolve("nx/src/native"));
const digest=(paths:string[])=>createHash("sha256").update(paths.map(path=>path+"\0").join("")).digest("hex");

test("installed graph scheduling retains changes without eager work for an idle service",()=>{
 const scheduler=sdkRequire(sdkRequire.resolve("nx/src/daemon/server/project-graph-incremental-recomputation")),{Subject}=sdkRequire(sdkRequire.resolve("rxjs")),validate=new Ajv({strict:true}).compile({...schema,$ref:"#/$defs/GraphDemandV1"});
 for(const row of fixture.demandCases){expect(validate(row.state)).toBe(true);const demand=new Subject(),subscriptions=Array.from({length:row.state.requests+row.state.callbacks+Number(row.state.computing)+Number(row.state.sockets)},()=>demand.subscribe(()=>{}));try{expect(demand.observed,row.id).toBe(row.eager);expect(scheduler.hasSemioGraphDemandV1(row.state),row.id).toBe(row.eager);}finally{for(const subscription of subscriptions)subscription.unsubscribe();demand.complete();}}
});

test("actual idle graph queues native facts until a request and keeps subscribers live",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original ticket artifact directory required");const root=mkdtempSync(join(artifacts,"id-"));
 try{
  mkdirSync(join(root,"leaf"));writeFileSync(join(root,"nx.json"),JSON.stringify({plugins:[]}));writeFileSync(join(root,"package.json"),JSON.stringify({name:"idle-graph-fixture",private:true}));writeFileSync(join(root,"leaf/project.json"),JSON.stringify({name:"leaf",targets:{build:{}}}));writeFileSync(join(root,"leaf/source.ts"),"export const value=1;\n");
  const source=`const {createRequire}=require('node:module'),{writeFileSync}=require('node:fs'),{join}=require('node:path'),sdk=createRequire(process.env.SEMIO_IDLE_SDK),root=process.cwd();
  (async()=>{
   const context=sdk('nx/src/utils/workspace-context');await context.setupWorkspaceContext(root);const graph=sdk('nx/src/daemon/server/project-graph-incremental-recomputation'),native=sdk('nx/src/native'),wait=()=>new Promise(resolve=>setTimeout(resolve,150));
   graph.scheduleProjectGraphRecomputation(['leaf/source.ts'],[],[]);await wait();const idle=graph.currentProjectGraph===undefined,requested=await graph.getCachedSerializedProjectGraphPromise();if(!requested.projectGraph?.nodes.leaf)throw Error('Actual requested graph missing leaf '+String(requested.error));
   const previous=graph.currentProjectGraph,sourceHash=()=>graph.fileMapWithFiles.fileMap.projectFileMap.leaf.find(row=>row.file==='leaf/source.ts').hash,oldHash=sourceHash();writeFileSync(join(root,'leaf/source.ts'),'export const value=2;\\n');graph.scheduleProjectGraphRecomputation([],['leaf/source.ts'],[]);const generation=graph.getRecomputationGeneration();await wait();const pending=graph.currentProjectGraph===previous&&graph.getRecomputationGeneration()===generation&&sourceHash()===oldHash;
   const successor=await graph.getCachedSerializedProjectGraphPromise(),fresh=successor.projectGraph?.nodes.leaf&&sourceHash()===native.hashFile(join(root,'leaf/source.ts'))&&sourceHash()!==oldHash;
   writeFileSync(join(root,'leaf/fresh.ts'),'export const fresh=true;\\n');graph.scheduleProjectGraphRecomputation(['leaf/fresh.ts'],[],[]);await wait();const createDeferred=!graph.fileMapWithFiles.fileMap.projectFileMap.leaf.some(row=>row.file==='leaf/fresh.ts');await graph.getCachedSerializedProjectGraphPromise();const created=graph.fileMapWithFiles.fileMap.projectFileMap.leaf.some(row=>row.file==='leaf/fresh.ts');require('node:fs').unlinkSync(join(root,'leaf/fresh.ts'));graph.scheduleProjectGraphRecomputation([],[],['leaf/fresh.ts']);await wait();const deleteDeferred=graph.fileMapWithFiles.fileMap.projectFileMap.leaf.some(row=>row.file==='leaf/fresh.ts');await graph.getCachedSerializedProjectGraphPromise();const deleted=!graph.fileMapWithFiles.fileMap.projectFileMap.leaf.some(row=>row.file==='leaf/fresh.ts');
   const listener=new Promise(resolve=>graph.registerProjectGraphRecomputationListener(value=>resolve(Boolean(value.nodes.leaf))));writeFileSync(join(root,'leaf/source.ts'),'export const value=3;\\n');graph.scheduleProjectGraphRecomputation([],['leaf/source.ts'],[]);const observed=await Promise.race([listener,new Promise((_,reject)=>setTimeout(()=>reject(Error('Actual graph subscriber timed out')),8000))]);console.log(JSON.stringify({idle,pending,fresh:Boolean(fresh),createDeferred,created,deleteDeferred,deleted,requested:Boolean(requested.projectGraph.nodes.leaf),observed}));process.exit(0);
  })().catch(error=>{console.error(error);process.exit(1)});`;
  await receiveScriptProcessInvocation(process.env,async original=>{const control=scriptCompilerAdmissionControlV1(original,"nx:idle-graph:actual",15000),result=await captureOwnedCommandV1("node",["-e",source],root,{control,env:{...scriptInvocationEnvironment(original,process.env),SEMIO_IDLE_SDK:sdkRequire.resolve("nx/package.json"),NX_WORKSPACE_ROOT_PATH:root,NX_WORKSPACE_DATA_DIRECTORY:join(root,".nx"),NX_DAEMON:"false",NX_VERBOSE_LOGGING:"false",SEMIO_NX_PROFILE_INPUTS:"0",SEMIO_NX_GRAPH_SOURCE_MEMBERSHIP:undefined,SEMIO_NX_SELECTED_GRAPH_RECEIPT:undefined}}),output=result.trim().split("\n").findLast(line=>line.startsWith("{"));expect(JSON.parse(output!)).toEqual({idle:true,pending:true,fresh:true,createDeferred:true,created:true,deleteDeferred:true,deleted:true,requested:true,observed:true});console.log("[DEBUG] Actual installed Nx idle/request/subscriber result "+output);});
 }finally{rmSync(root,{recursive:true,force:true});}
},20_000);

test("repository Cargo caller projection belongs above the reusable workspace owner",async()=>{
 const {cacheInternals,libraryBootstrap}=await import("../../../../../🟨️.mjs");await libraryBootstrap;const contract=fixture.placement,inventory=Object.fromEntries(Object.entries(contract.files).map(([name,paths])=>[name,[...new Set([...(paths as string[]),...fg.sync(contract.productionGlobs[name],{cwd:process.cwd(),onlyFiles:true,followSymbolicLinks:false,ignore:contract.exclude})])]])),projects=Object.fromEntries(Object.entries(contract.projects).map(([name,root])=>[name,{root,targets:{}}])),projectFileMap=Object.fromEntries(Object.entries(inventory).map(([name,files])=>[name,(files as string[]).map(file=>({file}))])),edges=await cacheInternals.createDependenciesImplementation({}, {workspaceRoot:process.cwd(),projects,fileMap:{projectFileMap}}),actual=[...new Set(edges.map((row:any)=>JSON.stringify([row.source,row.target])))].map(row=>JSON.parse(row as string)).sort();expect(actual).toEqual(contract.edges);
 const ts=sdkRequire(sdkRequire.resolve("typescript")),oracle=new Set<string>();for(const [name,files]of Object.entries(inventory))for(const file of files as string[]){let text;try{text=readFileSync(file,"utf8");}catch(error){if((error as {code?:string}).code==="ENOENT")continue;throw error;}for(const imported of ts.preProcessFile(text,true,true).importedFiles){const resolved=ts.resolveModuleName(imported.fileName,resolve(file),{moduleResolution:ts.ModuleResolutionKind.Bundler,allowImportingTsExtensions:true},ts.sys).resolvedModule?.resolvedFileName;if(!resolved)continue;for(const[target,owner]of Object.entries(contract.projects))if(target!==name&&(resolved===resolve(owner as string)||resolved.startsWith(resolve(owner as string)+"/")))oracle.add(JSON.stringify([name,target]));}}expect([...oracle].map(row=>JSON.parse(row)).sort()).toEqual(actual);const graphlib=sdkRequire(sdkRequire.resolve("graphlib")),graph=new graphlib.Graph();for(const[source,target]of actual)graph.setEdge(source,target);expect(graphlib.alg.findCycles(graph)).toEqual([]);
});

test("selected source custody retains all global owners but isolates unrelated source changes",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original ticket artifact directory required");mkdirSync(artifacts,{recursive:true});
 for(const row of fixture.cases){
  const root=mkdtempSync(join(artifacts,"sg-"));
  try{
   for(const [path,content]of Object.entries(fixture.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content as string);}
   spawnSync("git",["init","-q"],{cwd:root});
   const enumerate=()=>fg.sync("**/*",{cwd:root,onlyFiles:true,dot:true,followSymbolicLinks:false,ignore:[".git/**"]}).sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right)));
   const snapshot=()=>{const paths=enumerate(),globals=paths.filter(isGlobalGraphIdentityPathV1),selected=paths.filter(path=>fixture.scope.files.includes(path)||fixture.scope.roots.some((owner:string)=>path===owner||path.startsWith(owner+"/")));const relevant=row.mutation&&(isGlobalGraphIdentityPathV1(row.mutation.path)||fixture.scope.files.includes(row.mutation.path)||fixture.scope.roots.some((owner:string)=>row.mutation.path.startsWith(owner+"/")));return {version:1 as const,epoch:mutated?(row.epoch??"original-service"):"original-service",sequence:mutated&&relevant?1:0,globalIdentity:{paths:globals,sha256:digest(globals)},selectedMembership:{paths:selected,sha256:digest(selected)},changes:[]};};
   let mutated=false;const before=snapshot(),provider={capture:async()=>{if(mutated&&row.overflow)throw Error("Native observer overflow");return snapshot();},hashFile:(path:string)=>native.hashFile(path)};
   const validate=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/JournalWitnessV1"});expect(validate(before)).toBe(true);
   const indexed=spawnSync("git",["ls-files","--others","-z","--exclude-standard"],{cwd:root});expect(indexed.status).toBe(0);expect(indexed.stdout.toString().split("\0").filter(Boolean).filter(isGlobalGraphIdentityPathV1).sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right)))).toEqual(before.globalIdentity.paths);
   const graphFiles=enumerate().map(file=>({file,hash:native.hashFile(join(root,file))}));
   const receipt=await createSelectedGraphSourceReceiptV1(root,fixture.scope,before,graphFiles,provider);
   expect(await selectedGraphSourceReceiptHoldsV1(root,receipt,provider)).toBe(true);
   if(row.mutation){mkdirSync(dirname(join(root,row.mutation.path)),{recursive:true});writeFileSync(join(root,row.mutation.path),row.mutation.content);}mutated=true;
   expect(await selectedGraphSourceReceiptHoldsV1(root,receipt,provider),row.id).toBe(row.accepts);
  }finally{rmSync(root,{recursive:true,force:true});}
 }
},30_000);

test("installed task planning scopes the selected configuration, dependencies and compiler closure",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original ticket artifact directory required");const root=mkdtempSync(join(artifacts,"ss-")),contract=JSON.parse(readFileSync(new URL("./🧫️fixtures/📥️scope/🔣️.json",import.meta.url),"utf8"));
 try{
  writeFileSync(join(root,"nx.json"),JSON.stringify(contract.nx));const planner=nativeSelectedTaskPlannerV1(sdkRequire.resolve("nx/package.json"));
  expect(new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/TaskSelectionV1"})(contract.selection)).toBe(true);
  const scope=await selectedTaskScopeV1(root,contract.graph,contract.selection,planner);expect(scope).toEqual(contract.scope);
  const graphlib=sdkRequire(sdkRequire.resolve("graphlib")),graph=new graphlib.Graph({directed:true});for(const name of Object.keys(contract.graph.nodes))graph.setNode(name);for(const [name,rows]of Object.entries(contract.graph.dependencies))for(const row of rows as any[])graph.setEdge(name,row.target);const reachable=new Set(graphlib.alg.preorder(graph,contract.selection.project));expect(scope!.roots).toEqual([...reachable].sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right))));expect(scope!.tasks.every(task=>task.endsWith(":"+contract.selection.configuration))).toBe(true);
  const entry=join(root,"compiler/script.ts");mkdirSync(dirname(entry),{recursive:true});writeFileSync(entry,"export const compiler=1;\n");const built=await sdkRequire(sdkRequire.resolve("esbuild")).build({entryPoints:[entry],absWorkingDir:root,bundle:true,write:false,metafile:true,platform:"node",logLevel:"silent"});expect(Object.keys(built.metafile.inputs)).toEqual(scope!.files);
  const orchestrator=structuredClone(contract.graph);orchestrator.nodes.leaf.data.root=".";expect(await selectedTaskScopeV1(root,orchestrator,contract.selection,planner)).toBe(null);
  await expect(selectedTaskScopeV1(root,contract.graph,{...contract.selection,configuration:"dev"},planner)).rejects.toThrow("TaskSelectionV1");
 }finally{rmSync(root,{recursive:true,force:true});}
},30_000);

test("Cargo development-only edges cannot become production task prerequisites",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original ticket artifact directory required");const root=mkdtempSync(join(artifacts,"dk-")),contract=JSON.parse(readFileSync(new URL("./🧫️fixtures/🦀️dependencies/🔣️.json",import.meta.url),"utf8"));
 try{
  for(const [path,content]of Object.entries(contract.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content as string);}
  const {cacheInternals,libraryBootstrap}=await import("../../../../../🟨️.mjs");await libraryBootstrap;
  const projects=Object.fromEntries(["value","derive"].map(name=>[name,{root:name,targets:{build:{}}}])),files=Object.fromEntries(["value","derive"].map(name=>[name,[{file:name+"/Cargo.toml"}]])),edges=await cacheInternals.createDependenciesImplementation({}, {workspaceRoot:root,projects,fileMap:{projectFileMap:files}});
  expect(edges.map((row:any)=>[row.source,row.target]).sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right)))).toEqual(contract.production);
  const oracle=spawnSync("cargo",["metadata","--offline","--no-deps","--format-version=1","--manifest-path",join(root,"Cargo.toml")],{cwd:root,encoding:"utf8",timeout:10_000,env:{...process.env,CARGO_TARGET_DIR:join(root,"target")}});expect(oracle.status,oracle.stderr).toBe(0);const metadata=JSON.parse(oracle.stdout),production=metadata.packages.flatMap((owner:any)=>owner.dependencies.filter((dependency:any)=>dependency.path&&dependency.kind!=="dev").map((dependency:any)=>[owner.name,dependency.name])),tests=metadata.packages.flatMap((owner:any)=>owner.dependencies.filter((dependency:any)=>dependency.path&&dependency.kind==="dev").map((dependency:any)=>[owner.name,dependency.name]));expect(production).toEqual(contract.production);expect(tests).toEqual(contract.test);
  const graphlib=sdkRequire(sdkRequire.resolve("graphlib")),graph=new graphlib.Graph();for(const [source,target]of production)graph.setEdge(source,target);expect(graphlib.alg.findCycles(graph)).toEqual([]);
  expect(cacheInternals.nativeDependencies(sdkRequire(sdkRequire.resolve("@iarna/toml")).parse(contract.files["derive/Cargo.toml"]),{}).map((row:any)=>row.kind)).toEqual(["dev-dependencies"]);
 }finally{rmSync(root,{recursive:true,force:true});}
},30_000);

test("selected reuse keeps original graph producer, native hashes and inference authority custody",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original ticket artifact directory required");
 const root=mkdtempSync(join(artifacts,"sp-")),data=join(root,".nx"),header=join(data,"inference/header.json"),identity="a".repeat(64);
 try{
  for(const [path,content]of Object.entries(fixture.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content as string);}
  mkdirSync(dirname(header),{recursive:true});
  const paths=Object.keys(fixture.files).sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right))),globals=paths.filter(isGlobalGraphIdentityPathV1),graphFiles=paths.map(file=>({file,hash:native.hashFile(join(root,file))}));
  let epoch="original-service",sequence=0;const provider={toolingIdentity:identity,environment:{} as Record<string,string|undefined>,scope:async()=>fixture.scope,hashFile:(file:string)=>native.hashFile(file),capture:async(scope:{roots:readonly string[];files:readonly string[]})=>{const current=fg.sync("**/*",{cwd:root,onlyFiles:true,dot:true,followSymbolicLinks:false,ignore:[".nx/**"]}).sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right))),global=current.filter(isGlobalGraphIdentityPathV1),selected=current.filter(path=>scope.files.includes(path)||scope.roots.some(owner=>path===owner||path.startsWith(owner+"/")));return {version:1 as const,epoch,sequence,globalIdentity:{paths:global,sha256:digest(global)},selectedMembership:{paths:selected,sha256:digest(selected)},changes:[]};}};
  const started=Date.now()-1000,headers=JSON.stringify({digest:identity,authorityObservations:[{kind:"exists",path:join(root,"compiler/script.ts"),fact:true}],observations:[{kind:"exists",path:join(root,"leaf/source.rs"),fact:true},{kind:"exists",path:join(root,"other/source.rs"),fact:true}]});
  writeFileSync(header,headers+"\n{}");
  const original={version:1,revision:identity,pid:process.pid,startedAt:started,completedAt:Date.now()-10,inference:[{path:header,...JSON.parse(headers)}],observations:[]};
  expect(new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/InferencePublicationV1"})(original)).toBe(true);
  for(const name of ["emoji-project-json.publication.json","test-cases.publication.json"])writeFileSync(join(data,"inference",name),JSON.stringify(original));
  writeFileSync(join(data,"project-graph.json"),JSON.stringify({nodes:{leaf:{data:{root:"leaf"}}},dependencies:{leaf:[]}}));writeFileSync(join(data,"file-map.json"),JSON.stringify({fileMap:{projectFileMap:{leaf:graphFiles},nonProjectFiles:[]}}));
  rmSync(header);writeFileSync(join(root,"other/source.rs"),"unselected mutation during graph publication");
  expect(await writeSelectedGraphPublisherReceiptV1(root,data,started,provider)).toBe(true);
  const receipt=JSON.parse(readFileSync(join(data,"semio-selected-graph-publisher.json"),"utf8")),validate=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/PublisherReceiptV1"});expect(validate(receipt),JSON.stringify(validate.errors)).toBe(true);expect(receipt.pid).toBe(process.pid);expect(receipt.journal.globalIdentity.paths).toEqual(globals);
  expect(await selectedGraphPublisherReceiptHoldsV1(root,data,provider)).toBe(true);
  for(const row of fixture.environmentCases){provider.environment[row.key]=row.next;expect(await selectedGraphPublisherReceiptHoldsV1(root,data,provider),row.key).toBe(row.accepts);delete provider.environment[row.key];}
  const source=await captureSelectedGraphReuseReceiptV1(root,data,provider);expect(source).not.toBe(null);const handoff=await writeSelectedGraphHandoffV1(data,source!);expect(await selectedGraphHandoffHoldsV1(root,data,handoff.environment,provider)).toBe(true);
  writeFileSync(join(root,"leaf/new-input.rs"),"new input appeared during task execution");expect(await selectedGraphHandoffHoldsV1(root,data,handoff.environment,provider)).toBe(false);rmSync(join(root,"leaf/new-input.rs"));expect(await selectedGraphHandoffHoldsV1(root,data,handoff.environment,provider)).toBe(true);await handoff.close();expect(await selectedGraphHandoffHoldsV1(root,data,handoff.environment,provider)).toBe(false);
  writeFileSync(join(root,"other/source.rs"),"unselected source changed");expect(await selectedGraphPublisherReceiptHoldsV1(root,data,provider)).toBe(true);
  rmSync(join(root,"compiler/script.ts"));expect(await selectedGraphPublisherReceiptHoldsV1(root,data,provider)).toBe(false);writeFileSync(join(root,"compiler/script.ts"),fixture.files["compiler/script.ts"]);
  epoch="replacement-observer";expect(await selectedGraphPublisherReceiptHoldsV1(root,data,provider)).toBe(false);epoch="original-service";
  writeFileSync(join(root,"leaf/new.rs"),"new selected member");expect(await selectedGraphPublisherReceiptHoldsV1(root,data,provider)).toBe(false);rmSync(join(root,"leaf/new.rs"));
  writeFileSync(join(root,"leaf/source.rs"),"changed selected source");expect(await selectedGraphPublisherReceiptHoldsV1(root,data,provider)).toBe(false);writeFileSync(join(root,"leaf/source.rs"),fixture.files["leaf/source.rs"]);
  const manifest=join(root,"fresh/Cargo.toml");mkdirSync(dirname(manifest),{recursive:true});writeFileSync(manifest,"fresh global owner");expect(await selectedGraphPublisherReceiptHoldsV1(root,data,provider)).toBe(false);rmSync(dirname(manifest),{recursive:true});
  writeFileSync(join(data,"inference/test-cases.publication.json"),JSON.stringify({...original,observations:[{kind:"exists",path:join(root,"leaf/source.rs"),fact:false}]}));expect(await selectedGraphPublisherReceiptHoldsV1(root,data,provider)).toBe(false);writeFileSync(join(data,"inference/test-cases.publication.json"),JSON.stringify(original));
  writeFileSync(join(data,"inference/emoji-project-json.publication.json"),JSON.stringify({...original,pid:process.pid+1}));expect(await writeSelectedGraphPublisherReceiptV1(root,data,started,provider)).toBe(false);expect(await selectedGraphPublisherReceiptHoldsV1(root,data,provider)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
},30_000);


test("original inference publication survives racy unselected source without persisting whole graph cache",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original ticket artifact directory required");const {observeInference,observeDependencies,observedFs}=await import("../../../../../🔌️nx-plugin/👁️observation/🟨️.mjs");
 for(const row of fixture.publicationCases){const root=mkdtempSync(join(artifacts,"op-")),source=join(root,"source.rs"),cache=join(root,"inference/owner.json"),receipt=join(root,"inference/owner.receipt.json");
  try{
   writeFileSync(source,"original source bytes");await observeInference({cachePath:cache,key:[row.id]},()=>{const bytes=observedFs.readFileSync(source);if(row.changeDuringComputation)writeFileSync(source,"mutated source bytes");return {digest:createHash("sha256").update(bytes).digest("hex")};});
   await observeDependencies({receiptPath:receipt},()=>observedFs.existsSync(source));
   expect(()=>readFileSync(cache)).toThrow();expect(()=>readFileSync(receipt)).toThrow();const publication=JSON.parse(readFileSync(join(root,"inference/owner.publication.json"),"utf8"));
   const validate=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/InferencePublicationV1"});expect(validate(publication),JSON.stringify(validate.errors)).toBe(true);expect(publication.pid).toBe(process.pid);
   const original=publication.inference[0].observations.find((fact:any)=>fact.kind==="file"&&fact.path===source);expect(original.fact.sha256).toBe(createHash("sha256").update("original source bytes").digest("hex"));expect(publication.observations).toContainEqual({kind:"exists",path:source,fact:true});
  }finally{rmSync(root,{recursive:true,force:true});}
 }
},30_000);


test("installed daemon graph environment isolates actual task controls and retains resolution authority",()=>{
 const policy=JSON.parse(readFileSync(new URL("../../../../../🔌️nx-plugin/🌍️environment/📋️policy/🔣️.json",import.meta.url),"utf8")),contract=JSON.parse(readFileSync(new URL("../../../../../🔌️nx-plugin/🌍️environment/🧬️schema/🔣️.json",import.meta.url),"utf8")),sdk=sdkRequire(sdkRequire.resolve("nx/src/daemon/client/daemon-environment"));
 expect(new Ajv({strict:true}).compile(contract)(policy)).toBe(true);
 for(const name of policy.taskOnly){const original=process.env[name];try{process.env[name]="caller-one";const first=sdk.getDaemonEnv(),hash=sdk.hashDaemonClientEnv();process.env[name]="caller-two";expect(sdk.getDaemonEnv()[name],name).toBe(undefined);expect(first[name],name).toBe(undefined);expect(sdk.hashDaemonClientEnv(),name).toBe(hash);expect(process.env[name]).toBe("caller-two");}finally{if(original===undefined)delete process.env[name];else process.env[name]=original;}}
 for(const name of [...policy.graphDependent,"SEMIO_UNKNOWN_RESOLUTION_FIXTURE"]){if(name==="NX_LOAD_DOT_ENV_FILES")continue;const original=process.env[name];try{process.env[name]="resolver-one";const first=sdk.hashDaemonClientEnv();process.env[name]="resolver-two";expect(sdk.getDaemonEnv()[name],name).toBe("resolver-two");expect(sdk.hashDaemonClientEnv(),name).not.toBe(first);}finally{if(original===undefined)delete process.env[name];else process.env[name]=original;}}
});


test("selected observer cannot omit a declared compiler source or original selected member",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original ticket artifact directory required");const root=mkdtempSync(join(artifacts,"oe-"));
 try{
  for(const [path,content]of Object.entries(fixture.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content as string);}
  for(const row of fixture.omittedSourceCases){const paths=fg.sync("**/*",{cwd:root,onlyFiles:true,dot:true,followSymbolicLinks:false}).sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b))),globals=paths.filter(isGlobalGraphIdentityPathV1),selected=paths.filter(path=>fixture.scope.roots.some((owner:string)=>path.startsWith(owner+"/"))||fixture.scope.files.includes(path)).filter(path=>path!==row.file),witness={version:1 as const,epoch:"original-service",sequence:0,globalIdentity:{paths:globals,sha256:digest(globals)},selectedMembership:{paths:selected,sha256:digest(selected)},changes:[]};
  expect(paths).toContain(row.file);expect(new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/JournalWitnessV1"})(witness)).toBe(true);const provider={capture:async()=>witness,hashFile:(path:string)=>native.hashFile(path)};
  await expect(createSelectedGraphSourceReceiptV1(root,fixture.scope,witness,paths.map(file=>({file,hash:native.hashFile(join(root,file))})),provider)).rejects.toThrow(row.refusal);}
 }finally{rmSync(root,{recursive:true,force:true});}
});


test("actual SDK graph producer keeps original environment and refuses a changed generation",async()=>{
 const producer=sdkRequire(sdkRequire.resolve("nx/src/project-graph/nx-deps-cache")),environment=sdkRequire(sdkRequire.resolve("nx/src/daemon/client/daemon-environment")),original={...process.env},key="SEMIO_UNKNOWN_RESOLUTION_FIXTURE";
 try{
  process.env[key]="original-graph-resolver";const captured=producer.captureSemioGraphPublicationContextV1();process.env[key]="different-graph-resolver";expect(captured.environment[key]).toBe("original-graph-resolver");expect(captured.pid).toBe(process.pid);expect(Object.isFrozen(captured.environment)).toBe(true);expect(captured.startedAt<=Date.now()).toBe(true);
  const next={...environment.getDaemonEnv()};process.env[key]="original-graph-resolver";environment.applyDaemonEnvFromClient(next);expect(await producer.publishSemioGraphCustodyV1(captured)).toBe(false);expect(await producer.publishSemioGraphCustodyV1({...captured,generation:environment.getDaemonClientEnvGeneration(),pid:process.pid+1})).toBe(false);
 }finally{for(const name of Object.keys(process.env))if(!(name in original))delete process.env[name];Object.assign(process.env,original);}
});
