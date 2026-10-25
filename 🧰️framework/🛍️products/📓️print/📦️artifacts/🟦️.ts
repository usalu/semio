import{repositoryCachePartitionRootDataV1}from"../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts";
import {posix,win32,relative,sep} from "node:path";
import {validateJsonSchemaSubset} from "../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📦️ Projects fixed bounded slots without granting cache or filesystem custody. */
export function printArtifactLayoutV1(root:string,request:unknown):Readonly<{store:"tools"|"products";directory:string}>{
 if(validateJsonSchemaSubset(schema,request).length)throw Error("Closed Print artifact layout required");const path=/^[a-z]:[\\/]/iu.test(root)?win32:posix;if(!path.isAbsolute(root)||path.resolve(root)!==root||root.includes("\0"))throw Error("Canonical Print artifact root required");
 const row=request as {kind:"fonts"|"tex"|"tectonic"|"document"|"measurement";mode:"prototype"|"release";id:string;namespace:string;target:string},parts=row.kind==="tectonic"?["tectonic",row.target]:row.kind==="tex"?["tex"]:row.kind==="fonts"?[row.mode,"fonts"]:row.kind==="measurement"?[row.mode,"measurements",row.id]:[row.mode,"documents",row.namespace,row.id],directory=path.join(root,schema.definitions.Protocol.const.namespace,...parts);if(directory.length>schema.definitions.Protocol.const.maximumDirectoryUnits)throw Error("Finite Print artifact path required");return Object.freeze({store:row.kind==="tectonic"||row.kind==="tex"?"tools":"products",directory});
}

/** 🧵️ Joins all accepted bounded workers and preserves the first failure, including thrown undefined. */
export async function runPrintWorkersV1<T>(items:readonly T[],signal:AbortSignal,run:(item:T,signal:AbortSignal,index:number)=>Promise<void>):Promise<void>{
 if(items.length>schema.definitions.Protocol.const.maximumWorkerItems||typeof run!=="function")throw Error("Finite Print worker input required");signal.throwIfAborted();const controller=new AbortController(),abort=()=>controller.abort(signal.reason);signal.addEventListener("abort",abort,{once:true});const entries=[...items].entries();let failed=false,failure:unknown;
 try{await Promise.all(Array.from({length:Math.min(items.length,schema.definitions.Protocol.const.maximumWorkers)},async()=>{try{while(!failed){controller.signal.throwIfAborted();const row=entries.next();if(row.done)return;const[index,item]=row.value;await run(item,controller.signal,index);}}catch(error){if(!failed){failed=true;failure=error;}controller.abort(error);}}));if(failed)throw failure;signal.throwIfAborted();}finally{signal.removeEventListener("abort",abort);}
}

/** 🏷️ Projects a portable scratch identity without conflating it with an artifact receipt owner. */
export function printScratchOwnerV1(name:string):string{if(validateJsonSchemaSubset(schema.definitions.ScratchName,name,schema).length)throw Error("Closed portable Print scratch name required");return schema.definitions.Protocol.const.scratchNamespace+"/"+name;}

/** 🗺️ Projects the exact selected-mode producer slot as build-graph DATA without opening or adopting storage. */
export function printArtifactNxOutputV1(workspace:string,request:unknown):string{const root=repositoryCachePartitionRootDataV1(workspace,"products"),layout=printArtifactLayoutV1(root,request),row=request as {kind:string;mode:string};if(layout.store!=="products"||!["fonts","document","measurement"].includes(row.kind))throw Error("Mode-owned Print product output required");const parts=relative(root,layout.directory).split(sep);if(parts[0]!==schema.definitions.Protocol.const.namespace||parts[1]!==row.mode)throw Error("Exact Print product mode slot required");parts[1]=schema.definitions.NxProjection.const.modeToken;return"{workspaceRoot}/"+[relative(workspace,root).split(sep).join("/"),...parts].join("/");}
