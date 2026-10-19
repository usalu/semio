import {join,resolve} from "node:path";
import {requireMaterializedPathV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import policy from "../../🔣️policy.json" with {type:"json"};
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import {repoCacheDirectory} from "../../🟦️.ts";

/** 🗜️ Resolves the backing envelope and lease directory that fixes the sole queue catalog. */
export interface RepositoryStrictStorageV1 {readonly version:1;readonly directory:string;readonly leaseDirectory:string;readonly capacityBytes:number;readonly filesystemBytes:number;readonly maximumWorkspaceBytes:number;readonly maximumArtifactBytes:number;readonly sourceBoundary:Readonly<{version:1;maxSourceBytes:number;maxSourceFiles:number;maxActiveLines:number}>}
/** 📏️ Admits the authored fixed backing, preserving headroom independently from cache eviction. */
export function repositoryStrictStorageV1(workspace:string):RepositoryStrictStorageV1{
 const authored=policy.storage.strictStorage,boundary=policy.storage.sourceBoundary,errors=[...validateJsonSchemaSubset(schema.properties.storage.properties.strictStorage,authored),...validateJsonSchemaSubset(schema.properties.storage.properties.sourceBoundary,boundary)];if(errors.length)throw Error("Invalid repository strict storage: "+errors.join("; "));
 if(authored.filesystemBytes%1048576!==0||authored.capacityBytes<authored.filesystemBytes+1048576||authored.maximumWorkspaceBytes>authored.filesystemBytes||authored.maximumArtifactBytes>authored.filesystemBytes)throw Error("Repository strict storage envelope exceeds physical capacity");
 const root=resolve(workspace),directory=requireMaterializedPathV1(repoCacheDirectory(root,...authored.directory.split("/"))),leaseDirectory=requireMaterializedPathV1(join(root,".cache/b/l/m/data"));return Object.freeze({...authored,version:1,directory,leaseDirectory,sourceBoundary:Object.freeze({...boundary,version:1 as const})});
}
