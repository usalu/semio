import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../🧬️schema/🔣️.json";
import{type ResourceLeaseControl}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {isAbsolute,join} from "node:path";
import {createHash} from "node:crypto";
import budget from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {repositoryCachePartitionRootV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {requireNativeStorageReservationV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";

const hostPaths=schema.definitions.HostPaths.const;
/** 🔤️ Refuses the host paths that a platform's Bazel server cannot open byte-exactly, with the empirical reason authored in the schema. */
export function requireBazelHostPathsV1(platform:string,layout:Readonly<Record<string,unknown>>):void{
 const rule=(hostPaths.platforms as Readonly<Record<string,{pattern:string}>>)[platform];if(!rule)return;
 const pattern=new RegExp(rule.pattern);for(const key of hostPaths.guarded){const value=layout[key];if(typeof value!=="string"||!pattern.test(value))throw Error(`ASCII Bazel host path required on ${platform}: ${key}`);}
}
/** 🛤️ Names every native Bazel cache writer inside its authored finite partition. */
export function bazelCacheLayoutV1(roots:Readonly<{tools:string;dependencies:string;products:string}>){
 if(Object.values(roots).some(root=>!isAbsolute(root)))throw Error("Absolute Bazel storage roots required");
 const layout={version:1 as const,executable:join(roots.tools,"bazel/bin"),install:join(roots.tools,"bazel/install"),modules:join(roots.dependencies,"bazel/modules"),workspace:join(roots.products,"bazel/workspaces"),outputs:join(roots.products,"bazel/outputs"),actions:join(roots.products,"bazel/actions"),server:join(roots.products,"bazel/outputs/server"),temporary:join(roots.products,"tmp")};if(Object.values(layout).some(value=>typeof value==="string"&&value.length>256))throw Error("Bazel storage path exceeds 256 characters");if(validateJsonSchemaSubset(schema.definitions.StorageLayout,layout,schema).length)throw Error("Closed Bazel cache layout required");requireBazelHostPathsV1(process.platform,layout);return Object.freeze(layout);
}
/** 🎟️ Retains authenticated executable, dependency and output backing until all original children settle. */
export interface BazelStorageV1{readonly tools:RepositoryCacheStoreV1;readonly dependencies:RepositoryCacheStoreV1;readonly products:RepositoryCacheStoreV1;readonly paths:ReturnType<typeof bazelCacheLayoutV1>}
/** 🔐️ Refuses fabricated store contexts before the first filesystem observation or writer. */
export async function requireBazelStorageV1(root:string,storage:BazelStorageV1):Promise<void>{for(const id of ["tools","dependencies","products"] as const){const store=storage[id];requireNativeStorageReservationV1(store.storage,store.control);await store.recheck();if(store.root!==repositoryCachePartitionRootV1(root,id))throw Error("Original declared Bazel partition required");}const roots={tools:storage.tools.root,dependencies:storage.dependencies.root,products:storage.products.root},layout=bazelCacheLayoutV1(roots);if(JSON.stringify(layout)!==JSON.stringify(storage.paths))throw Error("Original Bazel partition layout required");for(const[key,path]of Object.entries(layout))if(key!=="version")requireNativeStoragePathV1(key==="executable"||key==="install"?roots.tools:key==="modules"?roots.dependencies:roots.products,path as string);}
/** 🏪️ Keeps all finite stores alive through the complete native Bazel invocation. */
export function withBazelStorageV1<T>(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,body:(storage:BazelStorageV1,actualPhase:ResourceLeaseControl)=>Promise<T>):Promise<T>{return withRepositoryCacheStoreV1(root,"tools",invocation,phaseControl,(tools,toolPhase)=>withRepositoryCacheStoreV1(root,"dependencies",invocation,toolPhase,(dependencies,dependencyPhase)=>withRepositoryCacheStoreV1(root,"products",invocation,dependencyPhase,(products,actualPhase)=>body(Object.freeze({tools,dependencies,products,paths:bazelCacheLayoutV1({tools:tools.root,dependencies:dependencies.root,products:products.root})}),actualPhase))));}

const cachePolicy=schema.definitions.CachePolicy.const;
/** 📏️ Bounds Bazel's disk cache to its authored share of the finite products partition, aligned down to whole MiB. */
export function bazelDiskCacheMaximumBytesV1(productsMaximumLogicalBytes:number=budget.partitions.products.maximumLogicalBytes):number{
 if(!Number.isSafeInteger(productsMaximumLogicalBytes)||productsMaximumLogicalBytes<1048576)throw Error("Finite products budget required");
 return Math.floor(productsMaximumLogicalBytes*cachePolicy.diskCache.productsShareBasisPoints/10000/1048576)*1048576;
}
/** 🖥️ Names the one output base whose server serves every command of a single workspace, because Bazel restarts a server bound to another workspace directory. */
export function bazelServerBaseV1(layout:ReturnType<typeof bazelCacheLayoutV1>,workspace:string):string{
 if(!isAbsolute(workspace))throw Error("Absolute Bazel workspace required");
 const base=join(layout.server,createHash("sha256").update(workspace).digest("hex").slice(0,16));if(base.length>256)throw Error("Bazel storage path exceeds 256 characters");return base;
}
/** 🌐️ Derives the closed startup and command flags for Bazel's disk, repository-content and optional remote caches; remote stays off until its endpoint environment is set. */
export function bazelCacheFlagsV1(layout:ReturnType<typeof bazelCacheLayoutV1>,workspace:string,options:Readonly<{environment?:Readonly<Record<string,string|undefined>>;skipCache?:boolean;skipRemoteCache?:boolean}>={}):Readonly<{startup:readonly string[];command:readonly string[];remote:boolean}>{
 const names=cachePolicy.remote,input=options.environment??{},selected:Record<string,string>={};
 for(const name of [names.endpointEnvironment,names.uploadEnvironment,names.credentialHelperEnvironment]){const value=input[name];if(value!==undefined&&value!=="")selected[name]=value;}
 if(validateJsonSchemaSubset(schema.definitions.CacheEnvironment,selected,schema).length)throw Error("Closed Bazel remote cache environment required");
 const endpoint=selected[names.endpointEnvironment],remote=endpoint!==undefined&&!options.skipRemoteCache,command=[`--disk_cache=${layout.actions}`,`--experimental_disk_cache_gc_max_size=${bazelDiskCacheMaximumBytesV1()}`];
 if(options.skipCache)command.push("--noremote_accept_cached");
 if(remote){command.push(`--remote_cache=${endpoint}`,`--${selected[names.uploadEnvironment]==="true"?"":"no"}remote_upload_local_results`);const helper=selected[names.credentialHelperEnvironment];if(helper!==undefined)command.push(`--credential_helper=${helper}`);}
 const flags={startup:Object.freeze([`--output_base=${bazelServerBaseV1(layout,workspace)}`]),command:Object.freeze(command),remote};
 if(validateJsonSchemaSubset(schema.definitions.CacheFlags,flags,schema).length)throw Error("Closed Bazel cache flags required");return Object.freeze(flags);
}
