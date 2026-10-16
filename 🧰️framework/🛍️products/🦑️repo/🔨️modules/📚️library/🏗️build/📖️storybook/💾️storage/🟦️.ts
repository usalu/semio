import{withRequestedIssuedResourceControlV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{readNativeStorageReservationPolicyV1}from"../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import{type ResourceLeaseControl}from"../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {createHash} from "node:crypto";
import {join,isAbsolute} from "node:path";
import {mkdir,mkdtemp,rm} from "node:fs/promises";
import {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {acquireResourceLease,requireMaterializedPathV1} from "../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📖️ Resolves compact immutable intent/scope identity inside one declared products root. */
export function storybookStorageLayoutV1(root:string,mode:string,scope:string):{output:string;temporary:string;state:string}{
 if(!isAbsolute(root)||validateJsonSchemaSubset(schema,{version:1,mode,scope}).length)throw Error("Closed Storybook storage intent required");const owner=join(root,schema.definitions.Protocol.const.root,mode,createHash("sha256").update(scope).digest("hex").slice(0,16));return {output:requireMaterializedPathV1(join(owner,"out")),temporary:requireMaterializedPathV1(join(owner,"tmp")),state:requireMaterializedPathV1(join(owner,"state"))};
}
/** 🧮️ Bounds writer lock metadata while isolating ordinary independent Storybook scopes. */
export function storybookWriterResourceV1(scope:string):string{return "repository-storybook:"+(parseInt(createHash("sha256").update(scope).digest("hex").slice(0,2),16)%schema.definitions.Protocol.const.writerShards);}
/** 🎟️ Retains actual finite product storage and original single writer until build settlement. */
export async function withStorybookStorageV1<T>(workspace:string,mode:string,scope:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,body:(store:RepositoryCacheStoreV1,layout:ReturnType<typeof storybookStorageLayoutV1>,temporary:string,actualControl:ResourceLeaseControl)=>Promise<T>):Promise<T>{
 return withRepositoryCacheStoreV1(workspace,"products",invocation,phaseControl,async(store,storePhase)=>{const layout=storybookStorageLayoutV1(store.root,mode,scope),policy=readNativeStorageReservationPolicyV1(store.storage,store.control),lease=await acquireResourceLease({directory:policy.leaseDirectory,resource:storybookWriterResourceV1(scope),mode:"exclusive",signal:storePhase.signal,control:storePhase});let temporary:string|undefined,result:{value:T}|undefined;const failures:unknown[]=[];
  try{result={value:await withRequestedIssuedResourceControlV1(storePhase,0,async actualPhase=>{for(const path of Object.values(layout))requireNativeStoragePathV1(store.root,path);await store.recheck();await mkdir(layout.temporary,{recursive:true});temporary=await mkdtemp(join(layout.temporary,"b-"));return body(store,layout,temporary,actualPhase);})};}catch(error){failures.push(error);}
  try{if(temporary)await rm(temporary,{recursive:true,force:true});}catch(error){failures.push(error);}try{lease.release();}catch(error){failures.push(error);}if(failures.length>1)throw new AggregateError(failures,"Storybook body, temporary cleanup or writer release failed");if(failures.length)throw failures[0];if(!result)throw Error("Original Storybook callback result required");return result.value;
 });
}
