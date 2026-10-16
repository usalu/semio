import{type ResourceLeaseControl}from"@semio-tech/process/leases";
import {join,isAbsolute} from "node:path";
import {mkdir,mkdtemp,rm} from "node:fs/promises";
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import {requireNativeStoragePathV1} from "@semio-tech/process/artifacts/native-build/storage/platform/contract";
import {requireMaterializedPathV1} from "@semio-tech/process/leases";
import type {ScriptInvocation} from "@semio-tech/process/routing/invocation";
import {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from "../../../../📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🗂️ Owns coordinator output, warm compiler state and isolated live scratch in one fixed product filesystem. */
export function nextStorageLayoutV1(root:string,mode:string):{output:string;state:string;temporary:string}{if(!isAbsolute(root)||validateJsonSchemaSubset(schema,mode).length)throw Error("Closed Next storage intent required");const owner=join(root,schema.definitions.Protocol.const.root,mode);return {output:requireMaterializedPathV1(join(owner,"out")),state:requireMaterializedPathV1(join(owner,"state")),temporary:requireMaterializedPathV1(join(owner,"tmp"))};}

/** 🎟️ Keeps the issued enforcing product image alive through the original Next operation. */
export async function withNextStorageV1<T>(workspace:string,mode:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,body:(store:RepositoryCacheStoreV1,layout:ReturnType<typeof nextStorageLayoutV1>,temporary:string,actualPhase:ResourceLeaseControl)=>Promise<T>):Promise<T>{return withRepositoryCacheStoreV1(workspace,"products",invocation,phaseControl,async(store,actualPhase)=>{const layout=nextStorageLayoutV1(store.root,mode);for(const path of Object.values(layout))requireNativeStoragePathV1(store.root,path);await store.recheck();await mkdir(layout.temporary,{recursive:true});const temporary=await mkdtemp(join(layout.temporary,"n-"));try{return await body(store,layout,temporary,actualPhase);}finally{await rm(temporary,{recursive:true,force:true});}});}
