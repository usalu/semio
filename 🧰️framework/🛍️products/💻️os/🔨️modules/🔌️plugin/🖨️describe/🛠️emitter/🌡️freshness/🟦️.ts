import {createHash} from "node:crypto";
import {readFile} from "node:fs/promises";
import {isAbsolute,relative,resolve,sep} from "node:path";
import {mapBoundedV1,SOURCE_FRESHNESS_FILE_CONCURRENCY} from "../../../🏗️build/🔍️freshness/🟦️.ts";

export const DESCRIPTOR_EMITTER_BUILD_TARGET_V1="@semio-tech/os-plugin-describe-rs:build";

/** 🌡️ Lists the workspace sources, as posix paths relative to `workspace`, that the emitter's compiler receipt hashed and that are missing or differ now.
 * Dependency sources under `.cache` are versioned and immutable, so only authored files count.
 * @see ../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts */
export async function staleDescriptorEmitterInputsV1(receipt:string,workspace:string,signal?:AbortSignal):Promise<Readonly<{checked:number;stale:readonly string[]}>>{
 const root=resolve(workspace),parsed:unknown=JSON.parse(receipt),row=parsed as {strings?:unknown;inputs?:unknown};
 if(!row||!Array.isArray(row.strings)||!Array.isArray(row.inputs))throw Error("Descriptor emitter compiler receipt without hashed inputs");
 const sources=row.inputs.flatMap((input:unknown)=>{
  const [index,kind,sha256]=input as [unknown,unknown,unknown],path=typeof index==="number"?(row.strings as unknown[])[index]:undefined;
  if(kind!=="file"||typeof path!=="string"||typeof sha256!=="string"||!isAbsolute(path)||!/^[a-f0-9]{64}$/u.test(sha256))return [];
  const local=relative(root,path);
  return local===""||local.startsWith("..")||isAbsolute(local)||local.split(sep)[0]===".cache"?[]:[{path,local:local.split(sep).join("/"),sha256}];
 });
 const fresh=await mapBoundedV1(sources,SOURCE_FRESHNESS_FILE_CONCURRENCY,async source=>{
  try{return createHash("sha256").update(await readFile(source.path,{signal})).digest("hex")===source.sha256;}catch(error){signal?.throwIfAborted();return false;}
 },signal);
 return Object.freeze({checked:sources.length,stale:Object.freeze(sources.filter((_,index)=>!fresh[index]).map(source=>source.local).sort())});
}
