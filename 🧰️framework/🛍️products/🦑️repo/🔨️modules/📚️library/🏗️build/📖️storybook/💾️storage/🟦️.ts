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
export async function withStorybookStorageV1<T>(workspace:string,mode:string,scope:string,invocation:ScriptInvocation,body:(store:RepositoryCacheStoreV1,layout:ReturnType<typeof storybookStorageLayoutV1>,temporary:string)=>Promise<T>):Promise<T>{
 return withRepositoryCacheStoreV1(workspace,"products",invocation,async store=>{const layout=storybookStorageLayoutV1(store.root,mode,scope),lease=await acquireResourceLease({directory:join(workspace,".cache/b/l/m/data"),resource:storybookWriterResourceV1(scope),mode:"exclusive",signal:store.control.signal,control:store.control});let temporary:string|undefined;
  try{for(const path of Object.values(layout))requireNativeStoragePathV1(store.root,path);await store.recheck();await mkdir(layout.temporary,{recursive:true});temporary=await mkdtemp(join(layout.temporary,"b-"));return await body(store,layout,temporary);}finally{if(temporary)await rm(temporary,{recursive:true,force:true});lease.release();}
 });
}
