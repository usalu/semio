import {mkdir,readdir,readFile,rename,unlink,writeFile} from "node:fs/promises";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import type {CasPolicyV1} from "../🧬️schema/🟦️.ts";

const HOLDER=new RegExp(schema.definitions.PinFile.properties.holder.pattern,"u"),DIGEST=new RegExp(schema.definitions.Digest.pattern,"u");

/** 📌️ Directory of the pin files of the store; one file per holder, rewritten atomically. */
export const casPinDirectoryV1=(root:string):string=>join(root,"v1","p");

const alive=(pid:number):boolean=>{if(pid<=0)return true;try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};

/** 📌️ Pins `digests` for `holder` until `nowMs + ttlMs` (capped by the policy's lease maximum); a pin or lease protects objects from eviction, and a holder that died loses it at once. Re-pinning with the same holder replaces its earlier set (renewal). */
export async function pinCasV1(root:string,policy:Pick<CasPolicyV1,"leaseMaxMs">,holder:string,digests:readonly string[],ttlMs:number,nowMs:number,pid:number=process.pid):Promise<void>{
 if(!HOLDER.test(holder))throw Error("Valid pin holder required / Gültiger Pin-Halter erforderlich");
 for(const digest of digests)if(!DIGEST.test(digest))throw Error("Valid digest required / Gültiger Digest erforderlich");
 const directory=casPinDirectoryV1(root);await mkdir(directory,{recursive:true});
 const body=JSON.stringify({version:1,holder,pid,expiresAtMs:nowMs+Math.min(Math.max(ttlMs,0),policy.leaseMaxMs),digests:[...new Set(digests)].sort()}),temporary=join(directory,`.${holder}.${process.pid}.${Math.random().toString(36).slice(2)}`);
 await writeFile(temporary,body);await rename(temporary,join(directory,holder+".json"));
}

/** 🔓️ Releases every pin of a holder. */
export async function releaseCasPinsV1(root:string,holder:string):Promise<void>{
 if(!HOLDER.test(holder))throw Error("Valid pin holder required / Gültiger Pin-Halter erforderlich");
 await unlink(join(casPinDirectoryV1(root),holder+".json")).catch(()=>undefined);
}

/** 🔎️ Reads the digests that are pinned at `nowMs` by a live holder with an unexpired lease; expired or orphaned pin files are removed. */
export async function readCasPinsV1(root:string,nowMs:number):Promise<Map<string,string>>{
 const pinned=new Map<string,string>(),directory=casPinDirectoryV1(root);let names:string[];
 try{names=await readdir(directory);}catch{return pinned;}
 for(const name of names){
  if(!name.endsWith(".json")||name.startsWith("."))continue;
  const file=join(directory,name);let pin:{holder?:string;pid?:number;expiresAtMs?:number;digests?:unknown};
  try{pin=JSON.parse(await readFile(file,"utf8"));}catch{continue;}
  if(!(Number(pin.expiresAtMs)>nowMs)||!alive(Number(pin.pid))){await unlink(file).catch(()=>undefined);continue;}
  if(Array.isArray(pin.digests))for(const digest of pin.digests)if(typeof digest==="string"&&DIGEST.test(digest)&&!pinned.has(digest))pinned.set(digest,String(pin.holder));
 }
 return pinned;
}
