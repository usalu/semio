import{profilePhaseV1}from"../../../../⏱️profile/🟨️.mjs";
import{buildSourceClosureNodeStdinLoaderV1}from"../../🟢️node/🟨️.mjs";
import closureSchema from"../../🧬️schema/🔣️.json"with{type:"json"};
import{nodeNativeChildEnvironmentV1}from"../../../../../../../../../🔨️modules/⚙️runtime/🟢️node/🧲️native/🟦️.ts";
import{captureCompilerBuildSourceClosureV1}from"../../🏗️compiler/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import{relative,resolve,isAbsolute}from"node:path";
import{createHash}from"node:crypto";
import{captureBuildSourceClosureV1,buildSourceClosureTextV1,buildSourceClosureStdinLoaderV1}from"../../🟦️.ts";
import{runtimeEntrySourcePathV1,readCapturedRuntimeEntryCommandV1,withCapturedRuntimeEntryProviderV1}from"../../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🚪️entry/🟦️.ts";
import{nativeDirectoryRuntimeEntryV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/📜️script.ts";
import{requireIssuedResourceControlInvocationV1,requireIssuedResourceControlV1,withIssuedResourceOperationV1,type ScriptInvocation}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type{ResourceLeaseControl}from"../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
/** 🧊️ Admits the declared Native helper before any Native store work and retains its exact closure through cleanup. */
export function withRepositoryRuntimeEntriesV1<T>(workspace:string,original:ScriptInvocation,control:ResourceLeaseControl,body:()=>Promise<T>):Promise<T>{return withRepositoryRuntimeEntryScopeV1("admitted",workspace,original,control,body);}

/** 🏗️ Admits the actual Native helper Source at its initial compiler owner before store effects. */
export function withRepositoryCompilerRuntimeEntriesV1<T>(workspace:string,original:ScriptInvocation,control:ResourceLeaseControl,body:()=>Promise<T>):Promise<T>{return withRepositoryRuntimeEntryScopeV1("compiler",workspace,original,control,body);}

async function withRepositoryRuntimeEntryScopeV1<T>(role:"admitted"|"compiler",workspace:string,original:ScriptInvocation,control:ResourceLeaseControl,body:()=>Promise<T>):Promise<T>{if(!schema.definitions.Role.enum.includes(role))throw Error("Declared runtime admission role required");requireIssuedResourceControlInvocationV1(control,original);return withIssuedResourceOperationV1(control,async()=>{
 const root=resolve(workspace),source=runtimeEntrySourcePathV1(nativeDirectoryRuntimeEntryV1()),entry=relative(root,source).replaceAll("\\","/");if(isAbsolute(entry)||entry.startsWith("../")||!entry)throw Error("Declared repository runtime entry required");
 const snapshot=await profilePhaseV1("nx.runtime-entry.capture",()=>role==="compiler"?captureCompilerBuildSourceClosureV1(root,entry,original,control):captureBuildSourceClosureV1(root,entry,original),role),publication=profilePhaseV1("nx.runtime-entry.publication",()=>buildSourceClosureTextV1(snapshot),role),loader="bun"in process.versions?buildSourceClosureStdinLoaderV1:buildSourceClosureNodeStdinLoaderV1(closureSchema.definitions.Policy.const),command=profilePhaseV1("nx.runtime-entry.command",()=>readCapturedRuntimeEntryCommandV1({version:1,program:process.execPath,arguments:["--eval",loader,"--",publication.sha256],input:publication.text,implementation:publication.sha256,loader:createHash("sha256").update(loader).digest("hex"),environment:{BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",...("bun"in process.versions?{}:nodeNativeChildEnvironmentV1())}}),role);
 requireIssuedResourceControlInvocationV1(control,original);let live=true;try{return await withCapturedRuntimeEntryProviderV1({capture:async declaration=>{if(!live||runtimeEntrySourcePathV1(declaration)!==source)throw Error("Original admitted runtime entry required");requireIssuedResourceControlV1(control);return command;}},body);}finally{live=false;}
});}
