import {privateRepositoryWorkspaceV1,testCheckoutRootV1} from "../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import {withBuildTestInvocationV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {withAdmittedCompilerStoreV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🔌️provider/🟦️.ts";
import {createIssuedResourceControlV1,withScriptInvocationSignalV1,scriptInvocationEnvironment} from "../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {readBuildMode} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import {afterAll,beforeAll,describe,expect,test} from "bun:test";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {AsyncLocalStorage} from "node:async_hooks";
import {join,dirname} from "node:path";
import {preparePrivateDirectoryV1} from "../../📨️protocol/📍️endpoint/🔐️directory/🟨️.mjs";
import {ensureWatcherV1,readWatcherPinV1,watcherManifestV1} from "../../🔭️watch/🏗️provision/🟨️.mjs";
import rawCustodyRows from "./🔣️.json";
import pinnedRows from "./📌️.json";
import custodySchema from "./🔣️schema.json";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import ts from "typescript";
import {cleanup,tempRoot,residentWatcherV1} from "../🟦️.ts";
import {POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";

const SOURCE=join(import.meta.dir,"../../🔭️watch/🦀️rust/🦀️.rs"),PIN=join(import.meta.dir,"../../🔭️watch/🦀️rust/🔣️.json");
const roots:string[]=[];afterAll(()=>roots.forEach(cleanup));
const outsideReceiver=AsyncLocalStorage.snapshot();
let residentRepository:Awaited<ReturnType<typeof privateRepositoryWorkspaceV1>>;
beforeAll(async()=>{
 if(process.env.SEMIO_GENUINE_BIRTH!=="setup-sdk")return;
 await withBuildTestInvocationV1(parent=>outsideReceiver(async()=>{
  residentRepository=await privateRepositoryWorkspaceV1();const fs=await import("node:fs"),checkout=testCheckoutRootV1();
  if(!existsSync(join(residentRepository.root,"🧰️framework")))fs.symlinkSync(join(checkout,"🧰️framework"),join(residentRepository.root,"🧰️framework"),process.platform==="win32"?"junction":"dir");
  fs.copyFileSync(join(checkout,"rust-toolchain.toml"),join(residentRepository.root,"rust-toolchain.toml"));
  const remaining=parent.control.remainingMilliseconds(),budget=Math.floor(Math.min(300000,remaining??300000));if(budget<=0)throw Error("Original compiler preparation budget exhausted");
  await residentRepository.invoke(original=>withScriptInvocationSignalV1(original,parent.control.signal,actual=>withAdmittedCompilerStoreV1(residentRepository.root,readBuildMode(actual.capabilities.buildMode),createIssuedResourceControlV1(actual,"resident:compiler-preparation",0),scriptInvocationEnvironment(actual,process.env),async store=>{await store.storage.recheck();})),{owner:"resident:compiler-preparation",maximumElapsedMilliseconds:budget});
 }));
},300000);
const frame='{"v":1,"t":"version","name":"semio-repo-watch","protocol":1,"os":"test"}';
const script=(body:string)=>"#!/bin/sh\n"+body+"\n";

describe("watcher pin",()=>{
 test("the committed pin names exactly the source on disk (regenerate it whenever the source changes)",()=>{
  const pin=readWatcherPinV1(PIN,SOURCE),bytes=readFileSync(SOURCE);
  expect(pin.sha256).toBe(createHash("sha256").update(bytes).digest("hex"));
  const document=JSON.parse(readFileSync(PIN,"utf8"));
  expect(document.sources).toHaveLength(1);expect(document.sources[0].bytes).toBe(bytes.length);expect(document.sources[0]).toMatchObject({platform:"any",architecture:"any"});
 });
 test("a stale pin, a pin for one platform and a broken pin are refused",()=>{
  const root=tempRoot("pin-");roots.push(root);
  const good=JSON.parse(readFileSync(PIN,"utf8"));
  for(const[name,mutate]of[["stale",(d:any)=>{d.sources[0].sha256="0".repeat(64);}],["bytes",(d:any)=>{d.sources[0].bytes+=1;}],["platform",(d:any)=>{d.sources[0].platform="darwin";}],["version",(d:any)=>{d.version=1;}]] as const){
   const copy=JSON.parse(JSON.stringify(good));mutate(copy);const path=join(root,name+".json");writeFileSync(path,JSON.stringify(copy));
   expect(()=>readWatcherPinV1(path,SOURCE)).toThrow();
  }
 });
 test("the build package has no dependency and its own workspace",()=>{
  const manifest=watcherManifestV1();
  expect(manifest).toContain("[workspace]");expect(manifest).not.toContain("[dependencies]");expect(manifest).toContain('path = "main.rs"');
 });
});

describe("ensure the watcher",()=>{
 const harness=()=>{const root=tempRoot("ensure-");roots.push(root);return{root,outputRoot:join(root,"tools","repo-watch")};};
 test("an installed binary is used without compiling",async()=>{
  const{outputRoot}=harness();let compiled=0;
  const result=await ensureWatcherV1({sourcePath:SOURCE,pinPath:PIN,outputRoot,locate:async()=>"/installed/semio-repo-watch",compile:async()=>{compiled++;return"";}});
  expect(result).toMatchObject({path:"/installed/semio-repo-watch",built:false});expect(compiled).toBe(0);
 });
 test("a missing binary is compiled once into the content addressed directory, probed, and found next time",async()=>{
  const{root,outputRoot}=harness();const phases:string[]=[];
  const compile=async({directory}:{directory:string})=>{
   expect(readFileSync(join(directory,"main.rs"),"utf8")).toBe(readFileSync(SOURCE,"utf8"));expect(existsSync(join(directory,"Cargo.toml"))).toBe(true);
   const out=join(directory,"target","release");mkdirSync(out,{recursive:true});const path=join(out,"semio-repo-watch");writeFileSync(path,script("echo '"+frame+"'"));chmodSync(path,0o755);return path;
  };
  const first=await ensureWatcherV1({sourcePath:SOURCE,pinPath:PIN,outputRoot,writable:()=>true,locate:async()=>undefined,compile,progress:p=>phases.push(p)});
  expect(first.built).toBe(true);expect(first.path).toBe(join(outputRoot,first.sha256.slice(0,16),"semio-repo-watch"));expect(phases).toEqual(["compile","ready"]);
  expect(readdirSync(join(outputRoot,first.sha256.slice(0,16)))).toEqual(["semio-repo-watch"]);
  const second=await ensureWatcherV1({sourcePath:SOURCE,pinPath:PIN,outputRoot,locate:async()=>first.path,compile:async()=>{throw new Error("must not compile");}});
  expect(second).toMatchObject({built:false,path:first.path});void root;
 });
 test("a build that does not answer the protocol is never installed; a failing or cancelled compile leaves nothing",async()=>{
  const{outputRoot}=harness();
  const bad=async({directory}:{directory:string})=>{const path=join(directory,"x");writeFileSync(path,script("echo nonsense"));chmodSync(path,0o755);return path;};
  await expect(ensureWatcherV1({sourcePath:SOURCE,pinPath:PIN,outputRoot,writable:()=>true,locate:async()=>undefined,compile:bad})).rejects.toThrow(/version frame/u);
  const sha=readWatcherPinV1(PIN,SOURCE).sha256.slice(0,16);
  expect(existsSync(join(outputRoot,sha,"semio-repo-watch"))).toBe(false);
  await expect(ensureWatcherV1({sourcePath:SOURCE,pinPath:PIN,outputRoot,writable:()=>true,locate:async()=>undefined,compile:async()=>{throw new Error("cancelled");}})).rejects.toThrow("cancelled");
  expect(readdirSync(join(outputRoot,sha)).filter(name=>!name.startsWith("."))).toEqual([]);
 });
 test("an unwritable output root (an image that is not mounted) is refused before anything is compiled or written",async()=>{
  const{outputRoot}=harness();let compiled=0;
  await expect(ensureWatcherV1({sourcePath:SOURCE,pinPath:PIN,outputRoot,locate:async()=>undefined,writable:()=>false,compile:async()=>{compiled++;return"";}})).rejects.toThrow(/watcher\.output-root-not-mounted/u);
  expect(compiled).toBe(0);expect(existsSync(outputRoot)?readdirSync(outputRoot):[]).toEqual([]);
  const installed=await ensureWatcherV1({sourcePath:SOURCE,pinPath:PIN,outputRoot,locate:async()=>"/installed/semio-repo-watch",writable:()=>false,compile:async()=>{throw new Error("must not compile");}});
  expect(installed.path).toBe("/installed/semio-repo-watch");
 });
 test("the registered pinned watcher answers its actual version frame",async()=>{
  const binary=await residentWatcherV1(),{spawnSync}=await import("node:child_process"),result=spawnSync(binary,["version"],{encoding:"utf8",timeout:5000});
  expect(result.status,result.stderr).toBe(0);expect(JSON.parse(result.stdout)).toMatchObject({name:"semio-repo-watch",protocol:1});
 });
});

describe("watcher provisioning custody",()=>{
 const rows=rawCustodyRows as {name:string;root:boolean;control:boolean;storage:boolean;custody:boolean;lose:boolean;expected:boolean}[],schema=custodySchema;
 expect(new Ajv({strict:true}).validate(schema,rows)).toBe(true);
 const source=readFileSync(join(import.meta.dir,"../../../../⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts"),"utf8"),ast=ts.createSourceFile("tools.ts",source,ts.ScriptTarget.Latest,true);
 let callback:ts.Node|undefined;const visit=(node:ts.Node)=>{if(ts.isCallExpression(node)&&node.expression.getText(ast)==="ensureResidentWatcherV1"){const options=node.arguments[1];if(options&&ts.isObjectLiteralExpression(options))callback=options.properties.find(row=>ts.isPropertyAssignment(row)&&row.name.getText(ast)==="writable");}ts.forEachChild(node,visit);};visit(ast);
 if(!callback||!ts.isPropertyAssignment(callback))throw Error("Actual Nx provisioning custody port required");
 const callbackSource=ts.transpile("("+callback.initializer.getText(ast)+")",{target:ts.ScriptTarget.ESNext}).trim().replace(/;$/u,"");
 for(const row of rows)test(row.name,async()=>{
  const artifact=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifact)throw Error("Caller-owned provisioning artifacts required");mkdirSync(artifact,{recursive:true});const root=mkdtempSync(join(artifact,"wp-"));roots.push(root);const outputRoot=join(root,"watcher");let compiled=false,checks=0;
  const db=new Database(":memory:");try{const oracle=db.query("SELECT ?1 AND ?2 AND ?3 AND ?4 AND NOT ?5 accepted").get(row.root?1:0,row.control?1:0,row.storage?1:0,row.custody?1:0,row.lose?1:0) as {accepted:number};expect(Boolean(oracle.accepted)).toBe(row.expected);
   const guard=(value:boolean,message:string)=>{if(!value)throw Error(message);},store={root:row.root?"owned":"foreign",control:{},storage:{},recheck:async()=>{checks++;guard(!(compiled&&row.lose),"released image");}},writable=new Function("store","control","workspace","requireIssuedResourceControlRefinementV1","checkResourceLeaseControl","requireNativeStorageReservationV1","requireRepositoryCacheStoreReadCustodyV1","repositoryCachePartitionRootV1","return "+callbackSource)(store,{},"workspace",()=>guard(row.control,"expired control"),()=>guard(row.control,"expired control"),()=>guard(row.storage,"unissued reservation"),()=>guard(row.custody,"foreign store"),()=>"owned");
   const work=ensureWatcherV1({sourcePath:SOURCE,pinPath:PIN,outputRoot,writable,locate:async()=>undefined,compile:async({directory}:{directory:string})=>{compiled=true;const file=join(directory,"result");writeFileSync(file,"recording-port-result");return file;},probe:()=>true});
   if(row.expected){const result=await work;expect(result.built).toBe(true);expect(checks).toBeGreaterThanOrEqual(2)}else await expect(work).rejects.toThrow();
   if(!row.expected)expect(existsSync(outputRoot)?readdirSync(outputRoot):[]).toEqual([]);
  }finally{db.close()}
 });
});


test("Nx provisioning retains genuine compiler ownership through tooling publication",async()=>{
 const rows=pinnedRows,check=new Ajv({strict:true}).compile({type:"array",minItems:4,items:{type:"object",additionalProperties:false,required:["name","fail","invalidCompiler","invalidNative","order","environment"],properties:{environment:{type:"object",additionalProperties:{type:"string"}},name:{type:"string"},fail:{type:"boolean"},invalidCompiler:{type:"boolean"},invalidNative:{type:"boolean"},order:{type:"array",items:{enum:["open","check","publish","build","close"]}}}}});expect(check(rows)).toBe(true);
 const text=readFileSync(join(import.meta.dir,"../../../../⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts"),"utf8"),ast=ts.createSourceFile("tools.ts",text,ts.ScriptTarget.Latest,true),node=ast.statements.find(n=>ts.isFunctionDeclaration(n)&&n.name?.text==="provisionNxTools");if(!node)throw Error("Actual public provisioning owner required");
 const body=ts.transpile(node.getText(ast),{target:ts.ScriptTarget.ESNext}).replace("export ","");
 for(const row of rows){const order:string[]=[],environment={RUSTUP_TOOLCHAIN:"source-pinned-test",PATH:"/funded/compiler"},compiler={environment:{...environment,...row.environment},storage:{},captureDirectory:"/funded/compiler/work",toolchain:{host:"host-v1",environment}},native={environment:compiler.environment,recheck:async()=>{order.push("check");if(row.invalidNative)throw failure}},original={},refined={},store={environment:{},invocation:{}},publisher={},failure=Error(row.name);
  const invoke=new Function("withRepositoryCompilerStoreV1","requireRepositoryCompilerStoreV1","repositoryNativeCompilerConfigurationV1","withManagedToolingProvisionV1","retentionSchema","provisionNxToolsOwnedV1",body+";return provisionNxTools;")(
   async(_root:unknown,mode:unknown,actualControl:unknown,actualEnvironment:unknown,run:any)=>{expect(mode).toBe("release");expect(actualControl).toBe(original);expect(actualEnvironment).toBe(store.environment);order.push("open");try{return await run(compiler,refined)}finally{order.push("close")}},
   (actual:unknown,_root:unknown,invocation:unknown,control:unknown)=>{expect(actual).toBe(compiler);expect(invocation).toBe(store.invocation);expect(control).toBe(refined);if(row.invalidCompiler)throw failure;},
   async(storage:unknown,actualEnvironment:unknown,context:unknown)=>{expect(storage).toBe(compiler.storage);expect(actualEnvironment).toBe(compiler.environment);expect(context).toEqual({host:"host-v1",target:"host-v1",configuredLinker:false,configuredHostLinker:false,configuredEnvironment:[]});return native},
   async(_root:unknown,actualStore:unknown,actualControl:unknown,_policy:unknown,run:any)=>{expect(actualStore).toBe(store);expect(actualControl).toBe(refined);order.push("publish");return run(publisher,refined)},
   {definitions:{Protocol:{const:{minimumInstallerAvailableBytes:1}}}},
   async(_root:unknown,actualStore:unknown,actualPublisher:unknown,actualControl:unknown,actualEnvironment:unknown,workRoot:unknown)=>{order.push("build");expect(actualStore).toBe(store);expect(actualPublisher).toBe(publisher);expect(actualControl).toBe(refined);expect(actualEnvironment).toBe(compiler.environment);expect(workRoot).toBe(compiler.captureDirectory);if(row.fail)throw failure;return"ready"});
  if(row.fail||row.invalidCompiler||row.invalidNative)await expect(invoke("workspace",store,original)).rejects.toBe(failure);else expect(await invoke("workspace",store,original)).toBe("ready");expect(order,row.name).toEqual(row.order);expect(new Ajv({strict:true}).validate({const:row.order},order)).toBe(true);
 }
});

test("resident cold publication closes compiler preparation while tooling owns the complete watcher lifetime",async()=>{
 const text=readFileSync(join(import.meta.dir,"../../../../⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts"),"utf8"),ast=ts.createSourceFile("tools.ts",text,ts.ScriptTarget.Latest,true),node=ast.statements.find(n=>ts.isFunctionDeclaration(n)&&n.name?.text==="withRepositoryResidentWatcherV1");
 if(!node)throw Error("Actual resident cold provisioning scope required");
 const body=ts.transpile(node.getText(ast),{target:ts.ScriptTarget.ESNext}).replace("export ",""),rows=(await import("./🏡️.json")).default;
 expect(new Ajv({strict:true}).validate({type:"array",items:{type:"object",additionalProperties:false,required:["name","fail"],properties:{name:{type:"string"},fail:{enum:["none","compile","body","compiler","native"]}}}},rows)).toBe(true);
 for(const row of rows){
  let tooling=false,compilerOpen=false;const order:string[]=[],original={},control={},compilerControl={},store={environment:{},invocation:original,recheck:async()=>{expect(tooling).toBe(true);}},compiler={storage:{},environment:{CC:"/managed/cc",RUSTFLAGS:"owned"},captureDirectory:"/compiler/captures",toolchain:{host:"host"}},failure=Error(row.name),native={environment:compiler.environment,recheck:async()=>{if(row.fail==="native")throw failure;}};
  const invoke=new Function("withNxToolingHeadroomRecoveryV1","withRepositoryToolingStoreV1","withRepositoryCompilerStoreV1","requireRepositoryCompilerStoreV1","repositoryNativeCompilerConfigurationV1","withManagedToolingProvisionV1","retentionSchema","provisionResidentWatcherV1","isToolingHeadroomRefusalV1","checkResourceLeaseControl",body+";return withRepositoryResidentWatcherV1;")(
   async(root:unknown,actual:unknown,phase:unknown,run:any)=>{expect(actual).toBe(original);expect(phase).toBe(control);return run(control);},
   async(root:unknown,actual:unknown,phase:unknown,run:any)=>{expect(actual).toBe(original);expect(phase).toBe(control);tooling=true;order.push("tools-open");try{return await run(store,control);}finally{tooling=false;order.push("tools-close");}},
   async(root:unknown,mode:unknown,phase:unknown,environment:unknown,run:any)=>{expect(phase).toBe(control);expect(environment).toBe(store.environment);compilerOpen=true;order.push("compiler-open");try{return await run(compiler,compilerControl);}finally{compilerOpen=false;order.push("compiler-close");}},
   (actual:unknown,root:unknown,invocation:unknown,phase:unknown)=>{expect(actual).toBe(compiler);expect(invocation).toBe(original);expect(phase).toBe(compilerControl);if(row.fail==="compiler")throw failure;},
   async(storage:unknown,environment:unknown)=>{expect(storage).toBe(compiler.storage);expect(environment).toBe(compiler.environment);return native;},
   async(root:unknown,actual:unknown,phase:unknown,policy:unknown,run:any)=>{expect(actual).toBe(store);expect(phase).toBe(compilerControl);return run({},compilerControl);},
   {definitions:{Protocol:{const:{minimumInstallerAvailableBytes:1}}}},
   async(root:unknown,actual:unknown,publisher:unknown,phase:unknown,environment:unknown,directory:unknown)=>{expect(tooling&&compilerOpen).toBe(true);expect(actual).toBe(store);expect(phase).toBe(compilerControl);expect(environment).toBe(compiler.environment);expect(directory).toBe(compiler.captureDirectory);order.push("publish");if(row.fail==="compile")throw failure;return "/owned/watcher";},
   ()=>false,()=>{});
  const work=invoke("workspace",original,control,async(path:string,phase:unknown)=>{expect(tooling&&!compilerOpen).toBe(true);expect(path).toBe("/owned/watcher");expect(phase).toBe(control);order.push("body");await Promise.resolve();expect(tooling&&!compilerOpen).toBe(true);order.push("settled");if(row.fail==="body")throw failure;return "done";});
  if(row.fail==="none")expect(await work).toBe("done");else await expect(work).rejects.toBe(failure);
  expect(tooling||compilerOpen).toBe(false);const db=new Database(":memory:");try{expect((db.query("SELECT ?1 < ?2 AND ?2 < ?3 ordered").get(order.indexOf("tools-open"),order.indexOf("compiler-open"),order.indexOf("compiler-close")) as {ordered:number}).ordered).toBe(1);}finally{db.close();}
  expect(order.at(-1)).toBe("tools-close");if(row.fail==="none"||row.fail==="body")expect(order.indexOf("compiler-close")).toBeLessThan(order.indexOf("body"));
 }
 const runtime=readFileSync(join(import.meta.dir,"../../../⌨️command/⚙️runtime/🟦️.ts"),"utf8");
 expect(runtime.includes("SEMIO_REPO_WATCH_BINARY")).toBe(false);expect(runtime).toContain("withRepositoryResidentWatcherV1(root,parent,phaseControl,async(watcherBinary,residentControl)=>");
});

test("resident foreground cancellation settles the service and worker pools before releasing its owner",async()=>{
 const text=readFileSync(join(import.meta.dir,"../../🏗️composition/🟨️.mjs"),"utf8"),ast=ts.createSourceFile("composition.js",text,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),node=ast.statements.find(n=>ts.isFunctionDeclaration(n)&&n.name?.text==="createForegroundRunnerV1");if(!node)throw Error("Actual foreground lifetime required");
 for(const fail of [false,true]){const cleanupFailure=Error("pool close");
 const controller=new AbortController(),reason=Error("resident cancelled"),order:string[]=[];let settled:(reason:string)=>void=()=>{};
 const service={closed:new Promise<string>(resolve=>{settled=resolve;}),start:async()=>{order.push("start");queueMicrotask(()=>controller.abort(reason));},stop:async()=>{order.push("stop");settled("cancelled");}};
 const create=new Function("existsSync","residentImplementationV1","writeWarmPreloadV1","createWarmPoolV1","createBuildGraphPortV1","createExecutorPoolV1","createDelegatedExecutionPortV1","createResidentServiceV1","RESIDENT_EXCLUDES_V1","RESIDENT_FENCE_DIRECTORY_V1","POLICY_V1","preparePrivateDirectoryV1","dirname",ts.transpile(node.getText(ast),{target:ts.ScriptTarget.ESNext}).replace("export ","")+";return createForegroundRunnerV1;")(
 ()=>true,()=>({implementation:"fixture",implementationPaths:[]}),()=>"/owned/preload",()=>({top:()=>{},close:async()=>{order.push("pool-close");if(fail)throw cleanupFailure;}}),()=>({}),()=>({}),()=>({}),()=>service,{rooted:[],names:[]},"fence",{idleShutdownMilliseconds:1},preparePrivateDirectoryV1,dirname);
 const run=create({signal:controller.signal,loadGraph:async()=>({}),script:"entry",salt:()=>"",warmSize:0,executorSize:0});
 const runtimeRoot=tempRoot("composition-");roots.push(runtimeRoot);
  const pending=run({runtime:{directory:join(runtimeRoot,"r")},root:runtimeRoot,environment:{}});if(fail){let error:unknown;try{await pending;}catch(caught){error=caught;}expect(error).toBeInstanceOf(AggregateError);expect((error as AggregateError).errors).toEqual([reason,cleanupFailure]);}else await expect(pending).rejects.toBe(reason);expect(order).toEqual(["start","stop","pool-close"]);
 }
});

test.skipIf(process.env.SEMIO_GENUINE_BIRTH!=="setup-sdk")("genuine fresh resident owner compiles the pinned watcher and retains Source delivery through physical settlement",async()=>{
 const {createIssuedResourceControlV1}=await import("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),{withRepositoryResidentWatcherV1}=await import("../../../../⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts"),{createRepositoryWorkspaceWatcherV1}=await import("../../../../⚡️caching/🕸️graph/🏁️startup/🚦️service/👁️workspace/🟦️.ts");
 const artifact=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifact)throw Error("Actual resident proof output required");
 const repository=residentRepository;
 expect(existsSync(join(repository.root,".cache/b/t/m/repo-watch"))).toBe(false);
 await withBuildTestInvocationV1(parent=>outsideReceiver(()=>repository.invoke(original=>withScriptInvocationSignalV1(original,parent.control.signal,actual=>withRepositoryResidentWatcherV1(repository.root,actual,createIssuedResourceControlV1(actual,"resident:watcher-proof",0),async(binary,control)=>{
  const root=mkdtempSync(join(artifact,"rw-"));roots.push(root);const received:string[]=[];const watcher=await createRepositoryWorkspaceWatcherV1(root,{roots:[".nx"],names:[]},join(root,".nx","r","f"),binary,control);
  try{watcher.watch((error,events)=>{if(error)throw error;received.push(...events.map(event=>event.path));});writeFileSync(join(root,"visible.json"),"owned");writeFileSync(join(root,".p0123abcd"),"private");await watcher.forceFlushPending();expect(received).toContain("visible.json");expect(received).not.toContain(".p0123abcd");expect(existsSync(binary)).toBe(true);}finally{await watcher.stop();}
  expect(existsSync(binary)).toBe(true);

  expect(binary.startsWith(join(repository.root,".cache","b","t","m","repo-watch"))).toBe(true);
  const{createFilterV1}=await import("../../🗃️state/🟨️.mjs"),{scanListingV1}=await import("../../🔌️ports/📋️listing/🟨️.mjs"),{createWatchSupervisorV1}=await import("../../🔭️watch/🟨️.mjs"),{SOURCE_OPACITY_PROTOCOL_V1,isOpaqueSourcePathV1}=await import("../../../../⚡️caching/🔍️discovery/📂️source/🟦️.ts");
  for(const directory of [".p89abcdef/p",".dabcdef01/p",".p0123abc/p",".prettier",".package-cache"])mkdirSync(join(root,directory),{recursive:true});
  const exclude={rooted:[".nx"],names:[]},filter=createFilterV1({...exclude,opaqueBasenames:SOURCE_OPACITY_PROTOCOL_V1}),native=await scanListingV1({root,binary,exclude,filter,opaqueBasenames:SOURCE_OPACITY_PROTOCOL_V1}),portable=await scanListingV1({root,exclude,filter,opaqueBasenames:SOURCE_OPACITY_PROTOCOL_V1});
  expect(native.directories.map(row=>row[0]).sort()).toEqual(portable.directories.map(row=>row[0]).sort());expect(native.members.map(row=>row[0]).sort()).toEqual(portable.members.map(row=>row[0]).sort());expect(native.directories.some(row=>isOpaqueSourcePathV1(row[0]))).toBe(false);expect(native.members.some(row=>isOpaqueSourcePathV1(row[0]))).toBe(false);
  mkdirSync(join(root,".nx","r","f"),{recursive:true});const events:string[]=[],losses:string[]=[],supervisor=createWatchSupervisorV1({root,binary,policy:POLICY_V1,filter,...exclude,opaqueBasenames:SOURCE_OPACITY_PROTOCOL_V1,backend:"native",privateFence:true,failureMode:"terminal",fenceDirectory:".nx/r/f",instance:"resident-owned-proof",onBatch:async rows=>{events.push(...rows.map(row=>row[2]));},onLost:(_scope,reason)=>{losses.push(reason);}});
  try{await supervisor.start();writeFileSync(join(root,".d76543210"),"private");writeFileSync(join(root,".prettierrc.json"),"visible");writeFileSync(join(root,".p0123abc","p","new"),"visible");expect((await supervisor.fence()).ok).toBe(true);await supervisor.drained();expect(events).toContain(".prettierrc.json");expect(events).toContain(".p0123abc/p/new");expect(events.some(isOpaqueSourcePathV1)).toBe(false);expect(losses).toEqual([]);}finally{await supervisor.stop();}
 })),{owner:"resident-cold-proof",maximumElapsedMilliseconds:Math.floor(Math.min(180000,parent.control.remainingMilliseconds()??180000))})));
},300000);

test("repository resident composition keeps the canonical Source boundary and strict private readiness",async()=>{
 const text=readFileSync(join(import.meta.dir,"../../🏗️composition/🟨️.mjs"),"utf8"),ast=ts.createSourceFile("composition.js",text,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),node=ast.statements.find(n=>ts.isFunctionDeclaration(n)&&n.name?.text==="createForegroundRunnerV1");if(!node)throw Error("Actual composition owner required");
 const {SOURCE_OPACITY_PROTOCOL_V1}=await import("../../../../⚡️caching/🔍️discovery/📂️source/🟦️.ts"),sourcePolicy={roots:[".metadata"],names:["generated"],opaqueBasenames:SOURCE_OPACITY_PROTOCOL_V1},failure=Error("recorded startup"),captures:any[]=[];
 const create=new Function("existsSync","residentImplementationV1","writeWarmPreloadV1","createWarmPoolV1","createBuildGraphPortV1","createExecutorPoolV1","createDelegatedExecutionPortV1","createResidentServiceV1","RESIDENT_EXCLUDES_V1","RESIDENT_FENCE_DIRECTORY_V1","POLICY_V1","createFilterV1","preparePrivateDirectoryV1","dirname",ts.transpile(node.getText(ast),{target:ts.ScriptTarget.ESNext}).replace("export ","")+";return createForegroundRunnerV1;")(
 ()=>true,()=>({implementation:"fixture",implementationPaths:[]}),()=>"/owned/preload",()=>({close:async()=>{}}),()=>({}),()=>({}),()=>({}),(options:unknown)=>{captures.push(options);return{start:async()=>{throw failure;},stop:async()=>{}};},{rooted:["other"],names:["other"]},".metadata/fence",{idleShutdownMilliseconds:1},()=>({excluded:()=>true}),preparePrivateDirectoryV1,dirname);
 const run=create({sourcePolicy,loadGraph:async()=>({}),script:"entry",watcherBinary:"/owned/native",salt:()=>"",warmSize:0,executorSize:0});
 const runtimeRoot=tempRoot("composition-");roots.push(runtimeRoot);
  await expect(run({runtime:{directory:join(runtimeRoot,"r")},root:runtimeRoot,environment:{}})).rejects.toBe(failure);
 expect(captures[0].exclude).toEqual({rooted:sourcePolicy.roots,names:sourcePolicy.names});expect(captures[0].opaqueBasenames).toBe(SOURCE_OPACITY_PROTOCOL_V1);expect(captures[0].backend).toBe("native");expect(captures[0].privateFence).toBe(true);expect(captures[0].failureMode).toBe("terminal");
 const runtime=readFileSync(join(import.meta.dir,"../../../⌨️command/⚙️runtime/🟦️.ts"),"utf8");expect(runtime).toContain("repositoryFilesystemJournalPolicyV1(root)");expect(runtime).toContain("sourcePolicy,signal:residentControl.signal");
});
