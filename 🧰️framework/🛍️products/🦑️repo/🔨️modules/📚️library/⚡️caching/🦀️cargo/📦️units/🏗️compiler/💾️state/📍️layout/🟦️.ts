import{join,resolve,isAbsolute}from"node:path";
import{repositoryCachePartitionRootDataV1}from"../../../../../💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts";
import native from"../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json";
import{compilerStatePolicyV1 as policy}from"../⚙️policy/🟦️.ts";
export interface RustCompilerExecutionV2{cwd:string;policy:"compiler-state-v1"|"temporary-v1"}
/** 🗺️ Projects deterministic producer paths as DATA; it opens no image or reservation. */
export function rustCompilerStateLayoutV1(parentRoot:string){if(!isAbsolute(parentRoot)||resolve(parentRoot)!==parentRoot)throw Error("Canonical compiler state parent required");const directory=join(parentRoot,policy.namespace,policy.volumeDirectory),root=join(directory,native.definitions.ImageProtocol.const.mountDirectory),cwd=join(root,policy.workDirectory);if(cwd.length>256)throw Error("Finite compiler state paths required");return Object.freeze({directory,root,cwd,slots:join(root,policy.slotDirectory),catalog:join(root,policy.catalogFile),charge:join(parentRoot,policy.chargeFile),intent:join(parentRoot,policy.intentFile)});}
/** 🧭️ Captures the actual deterministic cwd contract before unit/context hashing. */
export function rustCompilerExecutionV2(workspace:string,mode:"prototype"|"release"):RustCompilerExecutionV2{if(!isAbsolute(workspace)||resolve(workspace)!==workspace||!["prototype","release"].includes(mode))throw Error("Actual compiler workspace and mode required");return{cwd:mode==="prototype"?rustCompilerStateLayoutV1(repositoryCachePartitionRootDataV1(workspace,"rust")).cwd:workspace,policy:mode==="prototype"?"compiler-state-v1":"temporary-v1"};}
