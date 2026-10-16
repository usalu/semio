import{nativeArtifactCacheLayoutV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/💾️cache/📍️layout/🟦️.ts";
import{resolve,isAbsolute}from"node:path";
import{repositoryCachePartitionRootDataV1}from"../../../../../💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts";
import{compilerStatePolicyV1 as policy}from"../⚙️policy/🟦️.ts";
export interface RustCompilerExecutionV2{cwd:string;policy:"compiler-state-v1"|"temporary-v1"}
/** 🗺️ Projects deterministic producer paths as DATA; it opens no image or reservation. */
export function rustCompilerStateLayoutV1(parentRoot:string){return nativeArtifactCacheLayoutV1(parentRoot,policy);}
/** 🧭️ Captures the actual deterministic cwd contract before unit/context hashing. */
export function rustCompilerExecutionV2(workspace:string,mode:"prototype"|"release"):RustCompilerExecutionV2{if(!isAbsolute(workspace)||resolve(workspace)!==workspace||!["prototype","release"].includes(mode))throw Error("Actual compiler workspace and mode required");return{cwd:mode==="prototype"?rustCompilerStateLayoutV1(repositoryCachePartitionRootDataV1(workspace,"rust")).cwd:workspace,policy:mode==="prototype"?"compiler-state-v1":"temporary-v1"};}
