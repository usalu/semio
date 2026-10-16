import{createIssuedResourceControlV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {requireNativeStorageReservationV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {createRequire} from "node:module";
import {dirname,join,resolve} from "node:path";
import {lstat} from "node:fs/promises";
import type {ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1,compilerAdmissionBudgetV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import {runOwnedCommand} from "../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {withCompilerState} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🗃️state/🟦️.ts";
import {collectArtifactFiles} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🗂️files/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {stageRepositoryArtifacts,withRepositoryArtifactDestinationV1} from "../../../⚡️caching/📦️artifacts/🟦️.ts";
import {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {withStorybookStorageV1} from "../💾️storage/🟦️.ts";
import schema from "../💾️storage/🧬️schema/🔣️.json" with {type:"json"};
import {assertNoOwnedStorybookMdx,assertUiStorybookDiscovery} from "../🟦️.ts";

/** 📖️ Builds and publishes Storybook inside retained dependency and product filesystem owners. */
export async function buildRepositoryStorybookV1(workspace:string,mode:string,invocation:ScriptInvocation):Promise<void>{
 if(process.env.STORYBOOK_OUTPUT_DIR!==undefined)throw Error("Storybook output belongs to the declared finite products store");assertNoOwnedStorybookMdx(workspace);const scope=process.env.STORYBOOK_SCOPE??"",control=scriptCompilerAdmissionControlV1(invocation,"storybook:build",600000);
 await withRepositoryCacheStoreV1(workspace,"dependencies",invocation,control,()=>withStorybookStorageV1(workspace,mode,scope,invocation,control,async(store,layout,temporary)=>{
  const require=createRequire(join(workspace,"package.json")),manifest=require.resolve("storybook/package.json"),installed=require(manifest) as {version:string;bin:string};if(typeof installed.bin!=="string")throw Error("Original Storybook CLI entry required");const cli=join(dirname(manifest),installed.bin);
  await withCompilerState({root:layout.state,owner:"storybook",compiler:`storybook:${installed.version}:bun:${process.versions.bun}`,configuration:scope,mode:mode as "prototype"|"release",control,environment:store.environment,leaseDirectory:join(workspace,".cache/b/l/m/data"),files:[".semio-artifact.json"]},async(directory,compilerEnvironment)=>{
   requireNativeStoragePathV1(store.root,directory);await store.recheck();await runOwnedCommand(process.execPath,[cli,"build","-c",".storybook","--output-dir",temporary],workspace,"storybook:build",compilerAdmissionBudgetV1(control),{signal:control.signal,env:{...compilerEnvironment,SEMIO_VITE_CACHE_DIRECTORY:directory,SEMIO_STORYBOOK_BUILD:"true",STORYBOOK_DISABLE_TELEMETRY:"1"}});
   const files=await collectArtifactFiles(temporary,control.signal);let bytes=0;for(const path of files.values()){requireNativeStoragePathV1(store.root,path);bytes+=(await lstat(path)).size;}if(files.size>schema.definitions.Protocol.const.maximumOutputFiles||bytes>schema.definitions.Protocol.const.maximumOutputBytes)throw Error("Storybook deliverable exceeds finite output boundary");await store.recheck();await withRepositoryArtifactDestinationV1({workspace,invocation,storage:store.storage,control:store.control,phaseControl:control,directory:layout.output},destination=>stageRepositoryArtifacts(destination,`storybook:${mode}:${scope}`,files));console.log(`[DEBUG] Published finite Storybook artifact ${layout.output}`);
  });
 }));assertUiStorybookDiscovery(workspace);
}

/** 🔭️ Holds the prototype development compiler and bounded cache through original server settlement. */
export async function serveRepositoryStorybookV1(workspace:string,scope:string,args:readonly string[],invocation:ScriptInvocation):Promise<void>{
 if(args.some(arg=>!["--quiet","--debug","--ci","--no-manager-cache","--no-version-updates"].includes(arg)))throw Error("Storybook development arguments must preserve owned configuration and storage");const control=scriptCompilerAdmissionControlV1(invocation,"storybook:dev",0);await withRepositoryCacheStoreV1(workspace,"dependencies",invocation,control,()=>withStorybookStorageV1(workspace,"prototype",scope,invocation,control,async(store,layout)=>{
  const require=createRequire(join(workspace,"package.json")),manifest=require.resolve("storybook/package.json"),installed=require(manifest) as {version:string;bin:string};if(typeof installed.bin!=="string")throw Error("Original Storybook CLI entry required");
  await withCompilerState({root:layout.state,owner:"storybook",compiler:`storybook:${installed.version}:bun:${process.versions.bun}`,configuration:scope,mode:"prototype",control,capacity:false,environment:store.environment,leaseDirectory:join(workspace,".cache/b/l/m/data"),files:[".semio-artifact.json"]},async(directory,environment)=>{
   requireNativeStoragePathV1(store.root,directory);await store.recheck();const host=process.env.DEVCONTAINER==="true"?"0.0.0.0":"127.0.0.1",port=process.env.STORYBOOK_PORT??"6010",exact=["1","true"].includes(process.env.STORYBOOK_EXACT_PORT??"");
   await runOwnedCommand(process.execPath,[join(dirname(manifest),installed.bin),"dev","-c",".storybook","-p",port,...(exact?["--exact-port"]:[]),"--host",host,"--no-open",...args],workspace,"storybook:dev",compilerAdmissionBudgetV1(control),{signal:control.signal,env:{...environment,SEMIO_BUILD_MODE:"prototype",SEMIO_VITE_CACHE_DIRECTORY:directory,STORYBOOK_SCOPE:scope,STORYBOOK_DISABLE_TELEMETRY:"1",WATCHPACK_POLLING:process.env.WATCHPACK_POLLING??"true",CHOKIDAR_USEPOLLING:process.env.CHOKIDAR_USEPOLLING??"true"}});
  });
 }));
}

/** 🌐️ Serves only the actual bounded built artifact and releases server custody on cancellation. */
export async function serveRepositoryStorybookArtifactV1(workspace:string,mode:string,scope:string,invocation:ScriptInvocation):Promise<void>{return withStorybookStorageV1(workspace,mode,scope,invocation,createIssuedResourceControlV1(invocation,"cache:entry",0),(store,layout)=>serveStorybookArtifactInStorageV1(store,layout.output,{host:process.env.STORYBOOK_STATIC_HOST??"127.0.0.1",port:Number(process.env.STORYBOOK_PORT??"6010")}));}

/** 📡️ Serves an issued original product artifact through a cancellable bounded HTTP owner. */
export async function serveStorybookArtifactInStorageV1(store:RepositoryCacheStoreV1,output:string,options:Readonly<{host:string;port:number;signal?:AbortSignal;ready?:(port:number)=>Promise<void>}>):Promise<void>{
 requireNativeStorageReservationV1(store.storage,store.control);const control=store.control,signal=options.signal?AbortSignal.any([control.signal,options.signal]):control.signal,documentRoot=requireNativeStoragePathV1(store.root,output),status=await lstat(documentRoot);await store.recheck();if(!status.isDirectory()||status.isSymbolicLink())throw Error("Built Storybook artifact required");
 const server=Bun.serve({hostname:options.host,port:options.port,async fetch(request){try{signal.throwIfAborted();const pathname=decodeURIComponent(new URL(request.url).pathname),path=requireNativeStoragePathV1(documentRoot,resolve(documentRoot,"."+pathname));let file=path;try{if((await lstat(path)).isDirectory())file=join(path,"index.html");}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}requireNativeStoragePathV1(documentRoot,file);const output=Bun.file(file);if(!await output.exists())return new Response("Not Found",{status:404});return new Response(output);}catch{return new Response("Forbidden",{status:403});}}});
 console.log(`[DEBUG] Bounded Storybook artifact listening on http://${options.host}:${server.port}`);const settled=new Promise<void>((accept,reject)=>{const abort=()=>reject(signal.reason);signal.addEventListener("abort",abort,{once:true});if(signal.aborted)abort();});void settled.catch(()=>{});try{const port=server.port;if(port===undefined)throw Error("Original Storybook server port required");await options.ready?.(port);await settled;}finally{await server.stop(true);}
}
