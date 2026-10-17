import{requireIssuedResourceControlInvocationV1,type ScriptInvocation}from"../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type{ResourceLeaseControl}from"@semio-tech/process/leases";
import{fontAssetPublicationRootV1}from"@semio-tech/framework-canvas-ts/fonts/publication";
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import{withRepositoryCacheStoreV1}from"../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import{withRepositoryArtifactV1,withRepositoryArtifactTaskV1,type RepositoryArtifactV1}from"../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/👓️reading/🟦️.ts";
import{readNativeCallerAuthorityV1}from"../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 🔤️ Retains the actual mode's packed font publication until its accepted consumer settles. */
export function withInfiniteFontArtifactV1<T>(workspace:string,mode:string,invocation:ScriptInvocation,control:ResourceLeaseControl,run:(artifact:RepositoryArtifactV1)=>Promise<T>):Promise<T>{
 requireIssuedResourceControlInvocationV1(control,invocation);if(validateJsonSchemaSubset(schema.definitions.Request,{version:1,mode}).length||readNativeCallerAuthorityV1(invocation.capabilities).original.buildMode!==mode||typeof run!=="function")throw Error("Actual Original font mode and consumer required");const protocol=schema.definitions.Protocol.const;
 return withRepositoryCacheStoreV1(workspace,"products",invocation,control,store=>withRepositoryArtifactV1({workspace,invocation,storage:store.storage,control:store.control,phaseControl:control,request:{version:1,path:fontAssetPublicationRootV1(store.root,mode),owner:protocol.owner,maximumBytes:protocol.maximumAssetBytes+protocol.maximumManifestBytes,maximumFiles:protocol.maximumFiles}},reader=>withRepositoryArtifactTaskV1(reader,invocation,run)));
}
