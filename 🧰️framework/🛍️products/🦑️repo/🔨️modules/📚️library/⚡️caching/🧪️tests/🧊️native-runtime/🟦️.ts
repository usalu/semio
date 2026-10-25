import {readNativeRuntimeManifestV1} from "@semio-tech/framework-renderer-wgpu/native-entrypoint/modules/contract";
import {captureOwnedCommandResultV1} from "@semio-tech/process/execution";
import {withCapturedCargoArtifactsTaskV1} from "@semio-tech/process/native-build";
import {preparePlaygroundFixtureTargetsV1} from "../🎮️playground-preparation/🟦️.ts";
import {scriptInvocationEnvironment,createRequestedIssuedResourceControlV1,requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "@semio-tech/process/leases";
import {buildRepositoryCargoArtifacts} from "../../📦️artifacts/🏗️native-build/🟦️.ts";
import {withRepositoryExecutablePinV1} from "../../📦️artifacts/📌️executable/🟦️.ts";
import {repositoryCargoNetworkV1} from "../../../🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {withInstalledNxFixtureV1} from "../🔒️nx/🟦️.ts";
import {nativeRunnerEnvironment} from "@semio-tech/framework-renderer-wgpu/native-entrypoint/script";
import assert from "node:assert/strict";
import {existsSync,mkdirSync,readFileSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {createServer} from "node:http";
import {createRequire} from "node:module";
import {dirname,join} from "node:path";

/** 🧬️ Projects the shared language-neutral manifest corpus without granting any runtime custody. */
function nativeRuntimeManifestCasesV1(fixture:Readonly<{variant:string;manifestCases:readonly Readonly<{name:string;profile:string;modules:number;accepted:boolean;duplicate?:boolean;pluginId?:string;variant?:string;wasmPath?:string;descriptorPath?:string;extra?:boolean}>[]}>){
 return fixture.manifestCases.map(row=>({name:row.name,accepted:row.accepted,value:{version:1,variant:row.variant??fixture.variant,profile:row.profile,...(row.extra?{extra:true}:{}),modules:Array.from({length:row.modules},(_value,index)=>{const pluginId=row.pluginId??("model-"+(row.duplicate?0:index));return{pluginId,wasmSha256:"1".repeat(64),wasmPath:row.wasmPath??pluginId+"/📦️component.wasm",descriptorPath:row.descriptorPath??pluginId+"/🔣️.json"};})}}));
}

/** 📥️ Retains the admitted compiler capture and product pin through all bounded reader processes. */
async function withNativeRuntimeReaderV1(workspace:string,root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,body:(run:(args:string[])=>ReturnType<typeof captureOwnedCommandResultV1>)=>Promise<void>):Promise<void>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);const control=createRequestedIssuedResourceControlV1(phaseControl,90000),mode=readNativeCallerAuthorityV1(invocation.capabilities).original.buildMode,environment=scriptInvocationEnvironment(invocation,{...process.env,SEMIO_BUILD_MODE:mode});let failed=false,primary:unknown;
 try{await buildRepositoryCargoArtifacts(join(workspace,"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📦️modules/🧪️tests/📥️reader/📦️packages/🦀️rust/Cargo.toml"),["-p","semio-framework-os-native-runtime-reader","--bin","semio-native-runtime-reader"],workspace,{invocation,control,producer:"artifact",network:repositoryCargoNetworkV1(invocation,workspace),environment,consume:capture=>withCapturedCargoArtifactsTaskV1(capture,async view=>{
  const binary=view.files.get("semio-native-runtime-reader"+(process.platform==="win32"?".exe":""));assert(binary&&view.files.size===1,"Exact authored reader capture required");await view.recheck();return withRepositoryExecutablePinV1({workspace,invocation,phaseControl:view.control,source:binary},async(executable,actualPhase)=>{try{await body(args=>captureOwnedCommandResultV1(executable,args,root,{control:actualPhase,env:scriptInvocationEnvironment(invocation,nativeRunnerEnvironment(environment)),output:{maximumBytes:65536,maximumLines:256}}));}catch(error){failed=true;primary=error;throw error;}});
 })});}catch(error){if(failed)throw primary;throw error;}
}

/** 🛑️ Stops and drains only the owned Native fixture child before releasing any enclosing lifetime. */
async function withNativeRuntimeChildDrainV1(controller:AbortController,running:Promise<void>,body:()=>Promise<void>):Promise<Readonly<{failed:boolean;error:unknown}>>{
 const settled=running.then(()=>({failed:false,error:undefined}),(error:unknown)=>({failed:true,error}));let failed=false,primary:unknown,child:Readonly<{failed:boolean;error:unknown}>;try{await body();}catch(error){failed=true;primary=error;}finally{controller.abort(Error("native cancelled"));child=await settled;}if(failed)throw primary;return child;
}

/** 🧊️ Qualifies genuine model publication, canonical Nx restoration and retained Native consumer lifetime. */
export async function testNativeRuntime(workspace:string,_output:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);const require=createRequire(import.meta.url),fixture=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🧊️native-runtime/🔣️.json"),"utf8")),mode=readNativeCallerAuthorityV1(invocation.capabilities).original.buildMode,profiles=fixture.profiles.filter((profile:string)=>profile===mode);assert.equal(profiles.length,1);
  const native = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint");
  const execution = await import(join(native, "📜️script.ts"));
  const { resolveNxInvocation: resolveInvocation } = await import("../../🚀️bootstrap/📜️script.ts");
  for (const profile of fixture.profiles) for (const operation of ["run", "smoke"]) {
    const target = `@semio-tech/framework-os-dev:${operation}-${fixture.variant}-native-${profile}`, resolved = resolveInvocation(["run", target], {});
    assert.deepEqual(resolved.args, ["run", target], "The generated per-playground native target is the only entry; the registry resolves playground:<variant> renderer=wgpu-native to it");
    assert.deepEqual(resolved.env, { SEMIO_PLUGIN: fixture.variant, SEMIO_RENDERER: "wgpu", SEMIO_BUILD_MODE: profile === "release" ? "release" : "prototype" });
  }
  for (const retired of [["--", fixture.variant, "--release"], ["--", fixture.variant, "--smoke"], []]) assert.deepEqual(resolveInvocation(["run", "@semio-tech/framework-renderer-wgpu:native", ...retired], {}).args, ["run", "@semio-tech/framework-renderer-wgpu:native", ...retired], "The hard-coded native alias no longer picks a variant");
  assert.deepEqual(resolveInvocation(["run", `@semio-tech/framework-os-dev:run-${fixture.variant}-native-release`], {}).env, { SEMIO_PLUGIN: fixture.variant, SEMIO_RENDERER: "wgpu", SEMIO_BUILD_MODE: "release" });
  const script = readFileSync(join(native, "📜️script.ts"), "utf8"), old = readFileSync(join(native, "../📦️packages/🟦️typescript/📜️script.ts"), "utf8");
  assert.ok(!/runCmdStatus|new NativeRunScript|\["repo", "run"/.test(script));
  assert.ok(!/class NativeRunScript|ensureAssetServer/.test(old));
  const capture = await require("esbuild").build({ entryPoints: [join(native, "📦️modules/📜️script.ts")], bundle: true, write: false, metafile: true, platform: "node", format: "esm", packages: "external", logLevel: "silent" });
  assert.ok(!Object.keys(capture.metafile.inputs).some(path => path.includes("🏗️compiler") || path.includes("styling/🏗️builder") || path.includes("📚️library/🎮️playground/🖥️asset-server/") || path.includes("📚️library/🏃️process/🍞️bun/")), "Native publication must not load application tooling or asset servers");
  const plugin=join(workspace,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"),declaration={pluginId:fixture.pluginId,cratePath:fixture.cratePath};
  const {buildComponentClosureV1}=await import(join(plugin,"🏗️build/🧩️driver-stages/🟦️.ts"));
  await withInstalledNxFixtureV1(workspace,{kind:"products"},invocation,phaseControl,async(root,namespaceEnvironment,productOutput,actualPhase,toolkitRoot)=>{
  const put=(path:string,value:string)=>{if(join(root,path).length>256)throw Error("Bounded Native fixture path required");mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),value);};
  const sourceRoot=join(root,"preparation-source"),receivedTargets=await preparePlaygroundFixtureTargetsV1(workspace,sourceRoot,invocation,actualPhase);assert(receivedTargets[`prepare-${fixture.variant}-native-${mode}`]);
  await buildComponentClosureV1(workspace,new Set([declaration.pluginId]),{facet:"model",invocation,network:repositoryCargoNetworkV1(invocation,workspace),control:actualPhase});checkResourceLeaseControl(actualPhase);
  put("package.json",'{"name":"native-runtime-fixture","private":true,"type":"module"}');put("nx.json",JSON.stringify({useDaemonProcess:false}));put("source/input","first");put(".gitignore","node_modules\n.nx\nout\n.counts\n*.pid\n");
  put("project.json",JSON.stringify({name:"fixture",targets:{prepare:{executor:"nx:run-commands",cache:true,inputs:["{workspaceRoot}/source/input","{workspaceRoot}/📜️script.ts"],outputs:["{workspaceRoot}/out/runtime"],options:{command:"repo ./📜️script.ts publish"}},consume:{executor:"nx:run-commands",cache:false,outputs:[],dependsOn:["prepare"],options:{command:"repo ./📜️script.ts run"}}}}));
  put("📜️script.ts",`import assert from "node:assert/strict";
import {appendFileSync,copyFileSync,mkdirSync,readFileSync,writeFileSync,rmSync} from "node:fs";
import {spawn} from "node:child_process";
import {createHash} from "node:crypto";
import {createServer} from "node:http";
import {join,dirname} from "node:path";
const operation=process.argv[2],root=process.cwd(),workspace=${JSON.stringify(workspace)},fixture=${JSON.stringify(fixture)},declaration=${JSON.stringify(declaration)},mirror=join(root,"out/runtime");
if(operation==="consumer"||operation==="waiting"){
 const response=await fetch(process.env.SEMIO_ASSET_BASE_URL+"/asset");assert.equal(await response.text(),fixture.asset);assert.equal(process.env.S_USER,undefined);assert.equal(process.env.NPM_TOKEN,undefined);assert.equal(process.env.SEMIO_DIRECT_CHILD_BENIGN,"preserved");
 if(operation==="waiting"){const child=spawn(process.execPath,["-e","setInterval(()=>{},1000)"],{stdio:"ignore"});writeFileSync(join(root,"child.pid"),JSON.stringify([process.pid,child.pid]));setInterval(()=>{},1000);}
 else{const manifest=JSON.parse(readFileSync(join(mirror,"🔣️runtime.json"),"utf8"));for(const module of manifest.modules){const bytes=readFileSync(join(mirror,module.wasmPath));assert.equal(bytes.subarray(0,8).toString("hex"),fixture.componentHex);assert.equal(createHash("sha256").update(bytes).digest("hex"),module.wasmSha256);assert.deepEqual(bytes,readFileSync(join(process.env.SEMIO_NATIVE_RUNTIME,module.wasmPath)));}console.log("[TRACE] Native consumer fetched its live asset and retained product");}
}else{
 const{receiveScriptProcessInvocation,createIssuedResourceControlV1,createRequestedIssuedResourceControlV1}=await import(${JSON.stringify(join(workspace,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"))});
 const{publishNativeRuntime,withNativeRuntimeV1}=await import(${JSON.stringify(join(native,"📦️modules/🟦️.ts"))});const{runNativeSession}=await import(${JSON.stringify(join(native,"📜️script.ts"))});
 await receiveScriptProcessInvocation(process.env,async original=>{const phase=createIssuedResourceControlV1(original,"test:native-runtime",0);
  if(operation==="refuse"){await publishNativeRuntime(workspace,original,fixture.variant,[{...declaration,pluginId:"wrong-model"}],phase);throw Error("Hostile model declaration was accepted");}
  else if(operation==="cancelled"){const stop=new AbortController();stop.abort(Error("fixture publication cancelled"));await publishNativeRuntime(workspace,original,fixture.variant,[declaration],createRequestedIssuedResourceControlV1(phase,0,stop.signal));throw Error("Cancelled publication was accepted");}
  else if(operation==="publish"){await publishNativeRuntime(workspace,original,fixture.variant,[declaration],phase);await withNativeRuntimeV1(workspace,original,fixture.variant,phase,async(directory,manifest,control)=>{rmSync(mirror,{recursive:true,force:true});mkdirSync(mirror,{recursive:true});for(const file of["🔣️runtime.json",...manifest.modules.flatMap(module=>[module.wasmPath,module.descriptorPath,module.pluginId+"/🛂️.descriptor.semio",module.pluginId+"/🧾️receipt.json"])]){const target=join(mirror,file);mkdirSync(dirname(target),{recursive:true});copyFileSync(join(directory,file),target);}await control.yieldContinuation();});appendFileSync(join(root,".counts"),"publication\\n");}
  else{await withNativeRuntimeV1(workspace,original,fixture.variant,phase,async(directory,manifest,control)=>{assert.deepEqual(JSON.parse(readFileSync(join(mirror,"🔣️runtime.json"),"utf8")),manifest);let server;await runNativeSession("node",[join(root,"📜️script.ts"),"consumer"],{...process.env,SEMIO_NATIVE_RUNTIME:directory,S_USER:"poison",NPM_TOKEN:"poison"},root,original,control,()=>{server=createServer((_request,response)=>response.end(fixture.asset));server.listen(0,"127.0.0.1");return server;});assert.equal(server.listening,false);});}
 });
}
`);
  symlinkSync(join(workspace,"node_modules"),join(root,"node_modules"),process.platform==="win32"?"junction":"dir");
  const env=scriptInvocationEnvironment(invocation,{...process.env,...namespaceEnvironment,SEMIO_REPO_ROOT:workspace,SEMIO_WORKSPACE_ROOT:workspace,SEMIO_WORKSPACE_ROOT_PATH:workspace,NX_DAEMON:"false"}),cli=join(toolkitRoot,"node_modules/nx/dist/bin/nx.js"),mirror=join(productOutput,"runtime"),manifestPath=join(mirror,"🔣️runtime.json");
  const run=async()=>{const result=await captureOwnedCommandResultV1("node",[cli,"run","fixture:consume","--outputStyle=static"],root,{env,control:createRequestedIssuedResourceControlV1(actualPhase,60000),output:{maximumBytes:16777216,maximumLines:65536}});assert.equal(result.status,0,result.stdout+result.stderr);assert.equal(result.signal,null);assert(result.stdout.includes("Native consumer fetched its live asset and retained product"));};
  const runtimeSchema=JSON.parse(readFileSync(join(native,"📦️modules/🧬️schema/🔣️.json"),"utf8"));runtimeSchema.properties.modules.uniqueItems=true;const validateManifest=new(require("ajv"))().compile(runtimeSchema),count=()=>readFileSync(join(root,".counts"),"utf8").trim().split("\n").length;
  for(const cycle of fixture.cycles){if(cycle.remove)rmSync(mirror,{recursive:true});await run();assert.equal(count(),cycle.publications);const manifest=JSON.parse(readFileSync(manifestPath,"utf8"));assert(validateManifest(manifest),JSON.stringify(validateManifest.errors));assert.equal(manifest.profile,mode);assert.deepEqual(require("lodash").map(manifest.modules,"pluginId"),[declaration.pluginId]);}
  for(const operation of["refuse","cancelled"]){const result=await captureOwnedCommandResultV1("bun",[join(root,"📜️script.ts"),operation],root,{env,control:createRequestedIssuedResourceControlV1(actualPhase,60000),output:{maximumBytes:65536,maximumLines:256}});assert.equal(result.status,1,result.stdout+result.stderr);assert.equal(result.signal,null);assert.match(result.stderr,operation==="refuse"?/Actual model bytes, descriptor and receipt must agree/:/cancelled/);}
  await withNativeRuntimeReaderV1(workspace,root,invocation,actualPhase,async read=>{
   const manifest=JSON.parse(readFileSync(manifestPath,"utf8")),rows=await read([manifestPath,fixture.variant]);assert.equal(rows.status,0,rows.stderr);assert.deepEqual(JSON.parse(rows.stdout),manifest.modules.map(({pluginId,wasmSha256}:{pluginId:string;wasmSha256:string})=>({pluginId,wasmSha256})));assert.equal((await read([manifestPath,"wrong-variant"])).status,2);
   const path=join(root,"manifest.json"),validateNative=async(value:unknown,accepted:boolean)=>{put("manifest.json",typeof value==="string"?value:JSON.stringify(value));assert.equal((await read([path,fixture.variant,"validate"])).status,accepted?0:2);};
   for(const entry of fixture.paths){const value={...manifest,modules:[{...manifest.modules[0],wasmPath:entry.value}]};await validateNative(value,entry.valid);}
   for(const value of[{...manifest,version:2},{...manifest,extra:true},{...manifest,modules:[...manifest.modules,...manifest.modules]}]){assert.equal(validateManifest(value),false);await validateNative(value,false);}
   for(const row of nativeRuntimeManifestCasesV1(fixture)){let accepted=true;try{readNativeRuntimeManifestV1(row.value,fixture.variant,row.value.profile==="release"?"release":"prototype");}catch{accepted=false;}assert.equal(accepted,row.accepted,row.name);await validateNative(row.value,row.accepted);}
   const text=JSON.stringify(manifest),bytes=Buffer.byteLength(text);await validateNative(text+" ".repeat(fixture.manifestLimitBytes-bytes),true);await validateNative(text+" ".repeat(fixture.manifestLimitBytes-bytes+1),false);await validateNative(text+" ".repeat(fixture.manifestLimitBytes-bytes)+"null",false);
  });
  put("source/unrelated.test.ts","throw Error('unrelated')");await run();assert.equal(count(),1);put("source/input","changed");await run();assert.equal(count(),2);
  const controller=new AbortController();let server:ReturnType<typeof createServer>|undefined,port=0;
  const running=execution.runNativeSession("node",[join(root,"📜️script.ts"),"waiting"],env,root,invocation,createRequestedIssuedResourceControlV1(actualPhase,0,controller.signal),()=>{server=createServer((_request,response)=>response.end(fixture.asset));server.once("listening",()=>{port=(server!.address()as{port:number}).port;});server.listen(0,"127.0.0.1");return server;});
  const stopped=await withNativeRuntimeChildDrainV1(controller,running,async()=>{const deadline=Date.now()+15000;while(!existsSync(join(root,"child.pid"))&&Date.now()<deadline){checkResourceLeaseControl(actualPhase);await new Promise(accept=>setTimeout(accept,25));}assert(existsSync(join(root,"child.pid")),"Native child must become ready while its parent serves HTTP");});assert(stopped.failed);assert.match(String(stopped.error),/cancelled/);assert.equal(server!.listening,false);
  const pids:number[]=JSON.parse(readFileSync(join(root,"child.pid"),"utf8")),alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch(error:any){if(error.code==="ESRCH")return false;throw error;}},deadline=Date.now()+5000;while(pids.some(alive)&&Date.now()<deadline){checkResourceLeaseControl(actualPhase);await new Promise(accept=>setTimeout(accept,25));}assert.deepEqual(pids.filter(alive),[],"Cancelled Native process tree must leave the OS process table");await assert.rejects(()=>fetch(`http://127.0.0.1:${port}/asset`));
  });
}
