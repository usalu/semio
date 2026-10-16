import{type ResourceLeaseControl}from"../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {isAbsolute,join} from "node:path";
import {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {repositoryCachePartitionRootV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {requireNativeStorageReservationV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";

/** 🛤️ Names every native Bazel cache writer inside its authored finite partition. */
export function bazelCacheLayoutV1(roots:Readonly<{tools:string;dependencies:string;products:string}>){
 if(Object.values(roots).some(root=>!isAbsolute(root)))throw Error("Absolute Bazel storage roots required");
 const layout={version:1 as const,executable:join(roots.tools,"bazel/bin"),install:join(roots.tools,"bazel/install"),modules:join(roots.dependencies,"bazel/modules"),workspace:join(roots.products,"bazel/workspaces"),outputs:join(roots.products,"bazel/outputs"),actions:join(roots.products,"bazel/actions"),temporary:join(roots.products,"tmp")};if(Object.values(layout).some(value=>typeof value==="string"&&value.length>256))throw Error("Bazel storage path exceeds 256 characters");return Object.freeze(layout);
}
/** 🎟️ Retains authenticated executable, dependency and output backing until all original children settle. */
export interface BazelStorageV1{readonly tools:RepositoryCacheStoreV1;readonly dependencies:RepositoryCacheStoreV1;readonly products:RepositoryCacheStoreV1;readonly paths:ReturnType<typeof bazelCacheLayoutV1>}
/** 🔐️ Refuses fabricated store contexts before the first filesystem observation or writer. */
export async function requireBazelStorageV1(root:string,storage:BazelStorageV1):Promise<void>{for(const id of ["tools","dependencies","products"] as const){const store=storage[id];requireNativeStorageReservationV1(store.storage,store.control);await store.recheck();if(store.root!==repositoryCachePartitionRootV1(root,id))throw Error("Original declared Bazel partition required");}const roots={tools:storage.tools.root,dependencies:storage.dependencies.root,products:storage.products.root},layout=bazelCacheLayoutV1(roots);if(JSON.stringify(layout)!==JSON.stringify(storage.paths))throw Error("Original Bazel partition layout required");for(const[key,path]of Object.entries(layout))if(key!=="version")requireNativeStoragePathV1(key==="executable"||key==="install"?roots.tools:key==="modules"?roots.dependencies:roots.products,path as string);}
/** 🏪️ Keeps all finite stores alive through the complete native Bazel invocation. */
export function withBazelStorageV1<T>(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,body:(storage:BazelStorageV1,actualPhase:ResourceLeaseControl)=>Promise<T>):Promise<T>{return withRepositoryCacheStoreV1(root,"tools",invocation,phaseControl,(tools,toolPhase)=>withRepositoryCacheStoreV1(root,"dependencies",invocation,toolPhase,(dependencies,dependencyPhase)=>withRepositoryCacheStoreV1(root,"products",invocation,dependencyPhase,(products,actualPhase)=>body(Object.freeze({tools,dependencies,products,paths:bazelCacheLayoutV1({tools:tools.root,dependencies:dependencies.root,products:products.root})}),actualPhase))));}
