import{nativeArtifactCacheLayoutV1,nativeArtifactCacheProducerLayoutV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/💾️cache/📍️layout/🟦️.ts";
import{resolve,isAbsolute}from"node:path";
import{repositoryCachePartitionRootDataV1}from"../../../../../💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts";
import{compilerStatePolicyV1 as policy}from"../⚙️policy/🟦️.ts";
export interface RustCompilerExecutionV2{root:string;policy:"compiler-context-v2"|"temporary-v1"}
/** 🗺️ Projects deterministic producer paths as DATA; it opens no image or reservation. */
export function rustCompilerStateLayoutV1(parentRoot:string){return nativeArtifactCacheLayoutV1(parentRoot,policy);}
/** 🧭️ Captures the actual deterministic cwd contract before unit/context hashing. */
export function rustCompilerExecutionV2(workspace:string,mode:"prototype"|"release"):RustCompilerExecutionV2{if(!isAbsolute(workspace)||resolve(workspace)!==workspace||!["prototype","release"].includes(mode))throw Error("Actual compiler workspace and mode required");return{root:mode==="prototype"?rustCompilerStateLayoutV1(repositoryCachePartitionRootDataV1(workspace,"rust")).root:workspace,policy:mode==="prototype"?"compiler-context-v2":"temporary-v1"};}

/** 🧮️ Resolves cwd only after the complete root-bound context digest exists, avoiding recursive hash inputs. */
export function rustCompilerExecutionDirectoryV1(execution:RustCompilerExecutionV2,context:string):string{if(!["compiler-context-v2","temporary-v1"].includes(execution.policy)||!isAbsolute(execution.root)||resolve(execution.root)!==execution.root||!/^[a-f0-9]{64}$/u.test(context))throw Error("Canonical compiler execution root and context required");return execution.policy==="compiler-context-v2"?nativeArtifactCacheProducerLayoutV1(execution.root,policy,context).cwd:execution.root;}
