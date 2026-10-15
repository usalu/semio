import {withNxGraphAuxiliaryCapacityV1} from "../../../../💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts";
import {createHash,randomUUID} from "node:crypto";
import {lstat,mkdir,readFile,rename,rm,writeFile} from "node:fs/promises";
import {join} from "node:path";
import {validateSelectedGraphValueV1} from "../🟦️.ts";
import {selectedGraphReuseReceiptHoldsV1,type SelectedGraphReuseReceiptV1,type SelectedGraphReusePortsV1} from "../📤️publisher/🟦️.ts";

/** 📥️ The child receives a bounded content-bound pointer, never caller-supplied graph source facts. */
export interface SelectedGraphHandoffV1 {readonly version:1;readonly file:string;readonly sha256:string;}
export const SELECTED_GRAPH_HANDOFF_ENV="SEMIO_NX_SELECTED_GRAPH_RECEIPT";
const hash=(value:Uint8Array|string)=>createHash("sha256").update(value).digest("hex");

/** ✍️ Stages one original caller's narrowed proof until that caller's children finish. */
export async function writeSelectedGraphHandoffV1(root:string,directory:string,source:SelectedGraphReuseReceiptV1,environment:Readonly<Record<string,string|undefined>>,signal?:AbortSignal):Promise<Readonly<{environment:string;close():Promise<void>}>>{
 validateSelectedGraphValueV1("ReuseReceiptV1",source);signal?.throwIfAborted();const file="semio-selected-source-"+process.pid+"-"+randomUUID()+".json",path=join(directory,file),temporary=path+".tmp",bytes=JSON.stringify(source);if(path.length>256||Buffer.byteLength(bytes)>8*1024*1024)throw Error("Selected graph handoff capacity refused");
 await withNxGraphAuxiliaryCapacityV1(root,environment,{version:1,extraBytes:Math.ceil(Buffer.byteLength(bytes)/4096)*4096},async()=>{signal?.throwIfAborted();await mkdir(directory,{recursive:true});try{await writeFile(temporary,bytes,{flag:"wx"});signal?.throwIfAborted();await rename(temporary,path);signal?.throwIfAborted();}finally{await rm(temporary,{force:true});}});
 const pointer:SelectedGraphHandoffV1={version:1,file,sha256:hash(bytes)};validateSelectedGraphValueV1("HandoffV1",pointer);return {environment:JSON.stringify(pointer),close:()=>rm(path,{force:true})};
}

/** 🔐️ Native hashing and cache promotion must still hold the launcher's exact live source handoff. */
export async function selectedGraphHandoffHoldsV1(root:string,directory:string,environment:string|undefined,ports:Pick<SelectedGraphReusePortsV1,"capture"|"toolingIdentity"|"hashFile"|"environment">,signal?:AbortSignal):Promise<boolean>{
 if(environment===undefined)return true;
 try{
  const pointer=JSON.parse(environment) as SelectedGraphHandoffV1;validateSelectedGraphValueV1("HandoffV1",pointer);const path=join(directory,pointer.file);if(path.length>256)return false;
  const before=await lstat(path,{bigint:true});if(!before.isFile()||before.isSymbolicLink()||before.size>8n*1024n*1024n)return false;const bytes=await readFile(path),after=await lstat(path,{bigint:true});if(before.dev!==after.dev||before.ino!==after.ino||before.size!==after.size||before.mtimeNs!==after.mtimeNs||before.ctimeNs!==after.ctimeNs||hash(bytes)!==pointer.sha256)return false;
  return await selectedGraphReuseReceiptHoldsV1(root,directory,JSON.parse(bytes.toString()),ports,signal);
 }catch(error){signal?.throwIfAborted();return false;}
}
