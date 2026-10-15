import {lstat,mkdir} from "node:fs/promises";
import {posix,win32,resolve} from "node:path";
import {randomUUID} from "node:crypto";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import {checkScriptInvocation,type ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {acquireQueuedResourceLease} from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {withRepositoryCacheStoreV1} from "../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {repositoryLeaseDirectoryV1} from "../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
const protocol=schema.definitions.Protocol.const;
export const NX_COLLECTION_ADMISSION_MAXIMUM_BYTES_V1=protocol.maximumSnapshotBytes;

/** 🧭️ Names only the independently serialized admission directory inside graph4. */
export function nxCollectionAdmissionPlanV1(value:unknown):{directory:string;resource:string;maximumSnapshotBytes:number}{
 if(validateJsonSchemaSubset(schema,value).length)throw Error("Closed collection admission scope required");const row=value as {version:1;platform:"posix"|"windows";workspace:string;storage:string},paths=row.platform==="windows"?win32:posix;
 if(!paths.isAbsolute(row.workspace)||paths.resolve(row.workspace)!==row.workspace||row.workspace.includes("\0")||row.storage!==paths.join(row.workspace,".cache","b","g","m"))throw Error("Original canonical collection graph backing required");const directory=paths.join(row.storage,protocol.namespace);if(directory.length>256||[...directory].length>256)throw Error("Collection admission path capacity exceeded");return {directory,resource:protocol.writerPrefix+row.workspace,maximumSnapshotBytes:protocol.maximumSnapshotBytes};
}
/** 🔒️ Retains real graph funding and independent Original collection writer custody. */
export async function withNxCollectionAdmissionStoreV1<T>(workspace:string,invocation:ScriptInvocation,body:(directory:string)=>Promise<T>):Promise<T>{
 checkScriptInvocation(invocation);workspace=resolve(workspace);if((invocation.capabilities as {workspaceRoot?:unknown}).workspaceRoot!==workspace)throw Error("Original collection admission repository required");return withRepositoryCacheStoreV1(workspace,"graph",invocation,async store=>{
  const plan=nxCollectionAdmissionPlanV1({version:1,platform:process.platform==="win32"?"windows":"posix",workspace,storage:store.root}),held=await acquireQueuedResourceLease({directory:repositoryLeaseDirectoryV1(workspace),resource:plan.resource,mode:"exclusive",owner:randomUUID(),control:store.control,signal:store.control.signal});
  try{await store.recheck();const directory=requireNativeStoragePathV1(store.root,plan.directory);await mkdir(directory,{recursive:true});const actual=await lstat(directory),root=await lstat(store.root);if(!actual.isDirectory()||actual.isSymbolicLink()||actual.dev!==root.dev)throw Error("Original collection admission mounted directory required");checkScriptInvocation(invocation);const value=await body(directory);await store.recheck();checkScriptInvocation(invocation);return value;}finally{held.release();}
 });
}
