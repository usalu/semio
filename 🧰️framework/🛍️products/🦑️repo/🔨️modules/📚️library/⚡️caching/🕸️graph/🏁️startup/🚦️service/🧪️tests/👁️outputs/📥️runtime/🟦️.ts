import {prepareRepoCpuProfileArgumentsV1} from "./../../../../../../../🏗️build/⏱️profile/📊️cpu/🟨️.mjs";
import {withRepositoryCompilerRuntimeEntriesV1} from "../../../../../../../🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🟦️.ts";
import {withNxToolingUseV1,nxToolingUseStorageV1} from "../../../../../../🚀️bootstrap/🛠️tools/📥️use/🟦️.ts";
import {withRepositoryToolingStoreV1} from "../../../../../../💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import assert from "node:assert/strict";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {spawn,type ChildProcess} from "node:child_process";
import {createRequire} from "node:module";
import {mkdir,mkdtemp,readFile,readdir,rm,writeFile,realpath} from "node:fs/promises";
import {createWriteStream} from "node:fs";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🔣️.json";
import {createIssuedResourceControlV1,scriptInvocationEnvironment,type ScriptInvocation} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {toolForCommandV1,resolveRegisteredToolV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import {withRepositoryNxStorageV1} from "../../../../../../💾️storage/🔒️enforcement/🔌️nx/🟦️.ts";
import {withNxDaemonServiceBirthV1} from "../../../🟦️.ts";
import {nxBuildDependencyImplementationV1} from "../../../../../../../🟨️.mjs";

type DaemonClientV1={getProjectGraphAndSourceMaps():Promise<{projectGraph:{nodes:Record<string,{data:{metadata:{environment:string}}}>}}>;getAllFileData():Promise<{file:string}[]>;recordOutputsHashBatch(rows:{outputs:string[];hash:string}[]):Promise<unknown>;outputsHashesMatchBatch(rows:{outputs:string[];hash:string}[]):Promise<boolean[]>;requestShutdown():Promise<void>;reset():void};
const validate=new Ajv({strict:false}).compile(schema);
async function watchCount(pid:number):Promise<number|null>{
 if(process.platform!=="linux")return null;let count=0;
 for(const file of await readdir("/proc/"+pid+"/fdinfo"))try{count+=(await readFile("/proc/"+pid+"/fdinfo/"+file,"utf8")).split("\n").filter(line=>line.startsWith("inotify wd:")).length;}catch(error){if((error as {code?:string}).code!=="ENOENT")throw error;}
 return count;
}

/** 📊️ Starts the qualified daemon with only the caller’s closed profiler request. */
function startNxDaemonObservationServiceV1(driver:Readonly<{command:string;prefix:readonly string[];suffix?:readonly string[]}>,entry:string,workspace:string,environment:Readonly<Record<string,string|undefined>>,original:ScriptInvocation):ChildProcess{return spawn(driver.command,[...driver.prefix,...prepareRepoCpuProfileArgumentsV1(environment),entry,...driver.suffix??[]],{cwd:workspace,env:scriptInvocationEnvironment(original,{...environment}),stdio:["ignore","pipe","pipe","ipc"]});}

/** 📨️ Reflects the same issued parent for the actual SDK client and its asynchronous request queue. */
function reflectNxDaemonClientEnvironmentV1(original:ScriptInvocation,environment:Readonly<Record<string,string|undefined>>):void{const forwarded=scriptInvocationEnvironment(original,environment);Object.assign(process.env,forwarded);for(const[key,value]of Object.entries(forwarded))if(value===undefined)delete process.env[key];}

/** 👁️ Proves actual immutable daemon subscriptions and immediate invalidation under the received infrastructure Original. */
export async function proveNxDaemonOutputObservationV1(root:string,artifacts:string,original:ScriptInvocation):Promise<void>{
 const phase=createIssuedResourceControlV1(original,"nx:output-acceptance",110000);
 return withRepositoryCompilerRuntimeEntriesV1(root,original,phase,()=>proveNxDaemonOutputObservationInScopeV1(root,artifacts,original,phase));
}

/** 🛟️ Settles the complete infrastructure operation before its admitted runtime provider retires. */
async function proveNxDaemonOutputObservationInScopeV1(root:string,artifacts:string,original:ScriptInvocation,phase:ResourceLeaseControl):Promise<void>{
 assert.equal(validate(fixture),true,JSON.stringify(validate.errors));const db=new Database(":memory:");
 try{const row=db.query("SELECT ?1 > ?2 AND ?1 > ?3 bounded").get(fixture.unrelatedDirectories,fixture.maximumInitialWatches,fixture.maximumRegisteredWatches) as {bounded:number};assert.equal(row.bounded,1);}finally{db.close();}
 const toolkitRoot=await realpath(join(root,".nx/installation"));if(!artifacts)throw Error("Caller artifacts required");
 const identity=JSON.parse(await readFile(join(toolkitRoot,".semio-nx-tooling.json"),"utf8")).digest;
 const require=createRequire(join(toolkitRoot,"package.json")),sdkWatcher=await readFile(require.resolve("nx/src/daemon/server/watcher"),"utf8");
 assert.ok(sdkWatcher.includes("semioOutputObservationV1"));assert.ok(sdkWatcher.includes("Relative Native output event required"));
 const implementation=nxBuildDependencyImplementationV1();
 await mkdir(artifacts,{recursive:true});const workspace=await realpath(await mkdtemp(join(artifacts,"d"))),resultPath=join(artifacts,"daemon-output-acceptance.json"),logPath=join(artifacts,"daemon-output-service.log");
 const plugin="const fs=require('node:fs'),path=require('node:path');exports.createNodesV2=['project.json',async files=>files.map(file=>[file,{projects:{app:{name:'app',root:'.',metadata:{environment:fs.readFileSync(path.join(__dirname,'.env'),'utf8').trim()},targets:{proof:{executor:'nx:noop'}}}}}])];\n";
 await writeFile(join(workspace,"package.json"),JSON.stringify({name:"output-observation-fixture",version:"1.0.0",private:true}));
 await writeFile(join(workspace,"nx.json"),JSON.stringify({plugins:["./plugin.cjs"]}));await writeFile(join(workspace,"project.json"),JSON.stringify({name:"app",root:".",targets:{proof:{executor:"nx:noop"}}}));
 await writeFile(join(workspace,"plugin.cjs"),plugin);await writeFile(join(workspace,".gitignore"),".env\n.cache/\ndist/\n");await writeFile(join(workspace,".env"),fixture.initialEnvironment);
 await mkdir(join(workspace,fixture.output,"deep"),{recursive:true});await writeFile(join(workspace,fixture.output,"deep/result"),"before");
 for(let index=0;index<fixture.unrelatedDirectories;index++){checkResourceLeaseControl(phase);await mkdir(join(workspace,".cache/unrelated",String(index)),{recursive:true});}
 let child:ChildProcess|undefined,terminal:Promise<number|null>|undefined,client:DaemonClientV1|undefined,trace="",stopped=false,hard:ReturnType<typeof setTimeout>|undefined;const prior={...process.env},facts:Record<string,unknown>={workspace,identity},log=createWriteStream(logPath);
 try{await withRepositoryToolingStoreV1(root,original,phase,(_store,storePhase)=>withNxToolingUseV1(nxToolingUseStorageV1(root),{identity,toolkitRoot,invocation:original},storePhase,(_witness,actualPhase)=>withRepositoryNxStorageV1(root,original,actualPhase,async(bind,nxPhase)=>{
  const namespace=await bind(identity,null,implementation,workspace);
  await withNxDaemonServiceBirthV1(original,nxPhase,namespace.environment.NX_WORKSPACE_DATA_DIRECTORY!,identity,{...process.env,...namespace.environment,NX_WORKSPACE_ROOT_PATH:workspace,SEMIO_WORKSPACE_ROOT_PATH:workspace,SEMIO_NX_TOOLING_ROOT:toolkitRoot,SEMIO_NX_TOOLING_IDENTITY:identity,SEMIO_NX_IMPLEMENTATION_IDENTITY:implementation,NX_ISOLATE_PLUGINS:"false",NX_DAEMON:"true",NX_VERBOSE_LOGGING:"true",NODE_PATH:join(toolkitRoot,"node_modules")},async environment=>{
   reflectNxDaemonClientEnvironmentV1(original,environment);
   let failure:unknown;const receive=(bytes:Buffer)=>{log.write(bytes);trace+=bytes.toString();if(trace.length>262144)trace=trace.slice(-262144);};
   const entry=toolForCommandV1("node");if(!entry)throw Error("Registered Node service driver required");const driver=resolveRegisteredToolV1(entry,root,{env:environment});if(!driver.command)throw Error("Provisioned Node service driver required");
   child=startNxDaemonObservationServiceV1({command:driver.command,prefix:driver.prefix,suffix:driver.suffix},require.resolve("nx/src/daemon/server/start"),workspace,environment,original);
   child.stdout!.on("data",receive);child.stderr!.on("data",receive);terminal=new Promise((accept,reject)=>{child!.once("error",reject);child!.once("close",accept);});void terminal.then(status=>{if(!stopped&&status!==null)failure=Error("Actual daemon exited: "+status+"\n"+trace);},error=>{failure=error;});
   const cancel=()=>{child?.kill("SIGTERM");hard??=setTimeout(()=>child?.kill("SIGKILL"),2000);};phase.signal.addEventListener("abort",cancel,{once:true});const timer=setTimeout(cancel,105000);
   try{
    const cache=require("nx/src/daemon/cache"),started=performance.now();let metadata:{processId:number}|null;
    while(!(metadata=cache.readDaemonProcessJsonCache())){checkResourceLeaseControl(phase);if(failure)throw failure;if(performance.now()-started>45000)throw Error("Actual daemon readiness exhausted\n"+trace);await phase.publish({resource:"nx:output-daemon-readiness",mode:"shared",elapsedMs:performance.now()-started});await phase.yieldContinuation();await new Promise(resolve=>setTimeout(resolve,20));}
    assert.equal(metadata.processId,child.pid);facts.pid=child.pid;client=new(require("nx/src/daemon/client/client").DaemonClient)();
    const first=await client!.getProjectGraphAndSourceMaps();assert.equal(first.projectGraph.nodes.app!.data.metadata.environment,fixture.initialEnvironment);
    const files=await client!.getAllFileData();assert.equal(files.some(row=>row.file===".env"),false);facts.ignoredDotenv=true;
    const initial=await watchCount(child.pid!);facts.initialWatches=initial;if(initial!==null)assert.ok(initial<=fixture.maximumInitialWatches);
    await writeFile(join(workspace,".env"),fixture.changedEnvironment);const next=await client!.getProjectGraphAndSourceMaps();assert.equal(next.projectGraph.nodes.app!.data.metadata.environment,fixture.changedEnvironment);facts.immediateDotenv=true;
    const entries=[{outputs:[fixture.output],hash:"output-observation-proof"}];await client!.recordOutputsHashBatch(entries);assert.deepEqual(await client!.outputsHashesMatchBatch(entries),[true]);
    const registered=await watchCount(child.pid!);facts.registeredWatches=registered;if(registered!==null)assert.ok(registered<=fixture.maximumRegisteredWatches);
    await writeFile(join(workspace,fixture.output,"deep/result"),"after");assert.deepEqual(await client!.outputsHashesMatchBatch(entries),[false]);facts.immediateOutput=true;
    stopped=true;await client!.requestShutdown();assert.equal(await terminal,0);facts.exit=0;
   }finally{clearTimeout(timer);phase.signal.removeEventListener("abort",cancel);}
  });
 })));}finally{
  stopped=true;client?.reset();if(child&&child.exitCode===null&&child.signalCode===null){child.kill("SIGTERM");hard??=setTimeout(()=>child?.kill("SIGKILL"),2000);await terminal;}clearTimeout(hard);
  for(const key of Object.keys(process.env))if(!(key in prior))delete process.env[key];Object.assign(process.env,prior);
  await new Promise<void>((accept,reject)=>{log.once("error",reject);log.end(accept);});await writeFile(resultPath,JSON.stringify(facts,null,2)+"\n");await rm(workspace,{recursive:true,force:true});
 }
}
