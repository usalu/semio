import type {ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {join,resolve,dirname} from "node:path";
import {requireNativeStoragePathV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import policy from "./🔣️policy.json" with {type:"json"};
import {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {repositoryLeaseDirectoryV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {withRepositoryCompilerStoreV1,type RepositoryCompilerStoreV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts";
import {repositoryCargoNetworkV1,type CargoDependencyNetworkV1} from "../../../../🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import {cargoDirectories,type CargoDirectories} from "../../../../⚡️caching/🦀️cargo/🗂️directories/🟦️.ts";
import {repositoryCargoPreparationStorageV1} from "../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import type {CargoPreparationStorageV1} from "../../../../🗂️workspaces/🦀️cargo/🛠️preparation/📦️storage/🟦️.ts";
import {withResourceLeases} from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {readBuildMode} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import {scriptInvocationEnvironment,type ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🧾️ Confines default and explicitly requested reports to the original diagnostics backing. */
export function runtimeVerificationReportV1(root:string,requested?:string):string{if(validateJsonSchemaSubset(schema,policy).length)throw Error("Closed runtime verification storage policy required");return requireNativeStoragePathV1(root,requested??join(root,policy.reportDirectory,policy.reportName));}
/** 🗂️ Names each actual metadata observation beside its guarded report using the closed protocol. */
export function runtimeVerificationMetadataV1(root:string,report:string,index:number):string{if(!Number.isSafeInteger(index)||index<0)throw Error("Original runtime metadata index required");return requireNativeStoragePathV1(root,join(dirname(runtimeVerificationReportV1(root,report)),policy.metadataPrefix+index+".json"));}

/** 🏪️ Retains both authentic finite stores and one report writer through actual runtime witness acquisition. */
export interface RuntimeVerificationStorageV1{readonly report:string;readonly phaseControl:ResourceLeaseControl;readonly compiler:RepositoryCompilerStoreV1;readonly diagnostics:RepositoryCacheStoreV1;readonly environment:Readonly<Record<string,string|undefined>>;readonly preparation:CargoPreparationStorageV1;readonly directories:CargoDirectories;readonly network:CargoDependencyNetworkV1}
/** 🛡️ Opens enforcing backings before admitting any report, metadata, temporary or registry write. */
export async function withRuntimeVerificationStorageV1<T>(root:string,args:readonly string[],invocation:ScriptInvocation,body:(scope:RuntimeVerificationStorageV1)=>Promise<T>):Promise<T>{
 const mode=readBuildMode((invocation.capabilities as {buildMode?:unknown}).buildMode),network=repositoryCargoNetworkV1(invocation,root);
 return withRepositoryCacheStoreV1(root,"diagnostics",invocation,createIssuedResourceControlV1(invocation,"cache:store-entry",0),diagnostics=>withResourceLeases({directory:repositoryLeaseDirectoryV1(root),control:diagnostics.control,signal:diagnostics.control.signal,resources:[{resource:"runtime-graph:reports",mode:"exclusive"}]},()=>withRepositoryCompilerStoreV1(root,mode,diagnostics.control,scriptInvocationEnvironment(invocation,{...diagnostics.environment,CARGO_NET_OFFLINE:String(network.offline)}),async (compiler,control)=>{
  const index=args.indexOf("--report");if(index>=0&&!args[index+1])throw Error("--report needs a path");const report=runtimeVerificationReportV1(diagnostics.root,index<0?undefined:resolve(root,args[index+1]!)),environment=compiler.environment,preparation={...repositoryCargoPreparationStorageV1(root),directory:requireNativeStoragePathV1(compiler.root,join(compiler.buildDirectory,"preparation"))},directories=cargoDirectories(root,environment);console.log("[DEBUG] Authentic runtime verifier fixed storage "+JSON.stringify({report,diagnostics:diagnostics.root,compiler:compiler.root,target:directories.target,build:directories.build,home:environment.CARGO_HOME}));
  await compiler.storage.recheck();await diagnostics.recheck();const result=await body(Object.freeze({report,phaseControl:control,compiler,diagnostics,environment,preparation,directories,network}));await compiler.storage.recheck();await diagnostics.recheck();return result;
 })));
}
