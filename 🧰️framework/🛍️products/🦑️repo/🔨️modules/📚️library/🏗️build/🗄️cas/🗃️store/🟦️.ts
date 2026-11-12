import {createHash,randomUUID} from "node:crypto";
import {constants} from "node:fs";
import {chmod,copyFile,link,lstat,mkdir,open,readFile,rename,rm,unlink} from "node:fs/promises";
import {homedir} from "node:os";
import {dirname,join} from "node:path";
import {appendCasAccessV1} from "../📒️ledger/🟦️.ts";
import {createCasGuardV1,type CasGuardV1} from "../🚦️guard/🟦️.ts";
import {pinCasV1,releaseCasPinsV1} from "../📌️pins/🟦️.ts";
import {CAS_POLICY_V1,requireCasPolicyV1,type CasPolicyV1} from "../🧬️schema/🟦️.ts";

/** 🧾️ A content address: lowercase sha256 hex and the byte length. */
export type CasDigestV1=Readonly<{hash:string;size:number}>;
/** 🔗️ How `link` materialised an object at its destination. */
export type CasLinkModeV1="reflink"|"hardlink"|"copy";
/** 📥️ The accepted sources of `put`. */
export type CasSourceV1=Uint8Array|AsyncIterable<Uint8Array>|Iterable<Uint8Array>;
/** 🗄️ The host-wide content-addressed store: `put`, `get`, `has`, `link`, `touch`, pins and adoption of existing files. */
export type CasStoreV1=Readonly<{
 root:string;policy:CasPolicyV1;
 put(source:CasSourceV1,signal?:AbortSignal):Promise<CasDigestV1>;
 putChecked(source:CasSourceV1,expect:CasExpectationV1,signal?:AbortSignal):Promise<CasPutOutcomeV1>;
 get(hash:string):Promise<Readonly<{path:string;size:number}>|undefined>;
 read(hash:string):Promise<Uint8Array|undefined>;
 has(hashes:readonly string[]):Promise<boolean[]>;
 link(hash:string,destination:string,mode?:CasLinkModeV1|"auto"):Promise<CasLinkModeV1|undefined>;
 touch(hash:string):Promise<void>;
 adopt(path:string,signal?:AbortSignal):Promise<CasDigestV1>;
 pin(holder:string,hashes:readonly string[],ttlMs?:number):Promise<void>;
 release(holder:string):Promise<void>;
 objectPath(hash:string):string;
}>;
/** 🛂️ What a checked put demands of the stream before the object is published: its digest, its exact length, a length bound. */
export type CasExpectationV1=Readonly<{hash?:string;size?:number;maxBytes?:number}>;
/** 🛂️ How a checked put ended; nothing is published unless it is `created` or `exists`. */
export type CasPutOutcomeV1=Readonly<{outcome:"created"|"exists"|"too-large"|"length-mismatch"|"digest-mismatch"|"aborted";digest?:CasDigestV1}>;
/** 🧰️ Ports of a store: clock (tests), process id and writer identity, and the admission guard (by default the guard of the root, which keeps the cap and the partition watermarks before a write adds to the disk). */
export type CasStoreOptionsV1=Readonly<{root?:string;policy?:CasPolicyV1;now?():number;pid?:number;writer?:string;guard?:CasGuardV1}>;

const HASH=/^[0-9a-f]{64}$/u;
const sha=(hash:string):void=>{if(!HASH.test(hash))throw Error("Valid sha256 digest required / Gültiger sha256-Digest erforderlich");};

/** 🏠️ The host store root: `SEMIO_CAS_ROOT`, else `%LOCALAPPDATA%\semio\cas` on Windows and `~/.cache/semio/cas` elsewhere; one root serves every workspace of the user. */
export function casRootV1(environment:Readonly<Record<string,string|undefined>>=process.env,platform:NodeJS.Platform=process.platform,home:string=homedir()):string{
 const explicit=environment.SEMIO_CAS_ROOT;if(explicit)return explicit;
 if(platform==="win32")return join(environment.LOCALAPPDATA??join(home,"AppData","Local"),"semio","cas");
 return join(home,".cache","semio","cas");
}

/** 📍️ The path of an object below a root: `o/<prefix>/<rest>`. */
export const casObjectPathV1=(root:string,hash:string,fanOut:number=CAS_POLICY_V1.fanOutHexLength):string=>join(root,"v1","o",hash.slice(0,fanOut),hash.slice(fanOut));

const allocated=(info:{size:number;blocks?:number}):number=>info.size;

/** 🗄️ Opens a store; no state is held beyond the throttle of access events, so any number of processes and workspaces share one root. */
export function createCasStoreV1(options:CasStoreOptionsV1={}):CasStoreV1{
 const root=options.root??casRootV1(),policy=requireCasPolicyV1(options.policy??CAS_POLICY_V1),now=options.now??Date.now,pid=options.pid??process.pid,writer=options.writer??`${pid}-${randomUUID().slice(0,8)}`;
 const guard=options.guard??createCasGuardV1({root,policy,now});
 const fan=policy.fanOutHexLength,objectPath=(hash:string):string=>casObjectPathV1(root,hash,fan),shard=(hash:string):string=>hash.slice(0,fan),touched=new Map<string,number>();
 const touch=async(hash:string):Promise<void>=>{
  const at=now(),last=touched.get(hash);
  if(last!==undefined&&at-last<policy.touchGranularityMs)return;
  if(touched.size>=4096)touched.clear();
  touched.set(hash,at);
  try{await appendCasAccessV1(root,shard(hash),writer,hash,at);}catch(error){if(!["ENOSPC","EDQUOT"].includes((error as NodeJS.ErrnoException).code??""))throw error;}
 };
 const exists=(path:string):Promise<boolean>=>lstat(path).then(()=>true,()=>false);
 const place=async(temporary:string,hash:string):Promise<void>=>{
  const target=objectPath(hash);await mkdir(dirname(target),{recursive:true});
  if(await exists(target)){await unlink(temporary).catch(()=>undefined);return;}
  try{await rename(temporary,target);}catch(error){if(await exists(target)){await unlink(temporary).catch(()=>undefined);return;}throw error;}
  await chmod(target,0o444).catch(()=>undefined);
 };

 const ingest=async(source:CasSourceV1,expect:CasExpectationV1,signal?:AbortSignal):Promise<CasPutOutcomeV1>=>{
  await guard.admit(expect.size??0,signal);
  const directory=join(root,"v1","t");await mkdir(directory,{recursive:true});
  const temporary=join(directory,`${writer}-${randomUUID().slice(0,12)}`),hash=createHash("sha256");let size=0,outcome:CasPutOutcomeV1["outcome"]|undefined;
  const handle=await open(temporary,"wx",0o600);
  try{
   const chunks:AsyncIterable<Uint8Array>|Iterable<Uint8Array>=source instanceof Uint8Array?[source]:source;
   try{
    for await(const chunk of chunks){
     signal?.throwIfAborted();size+=chunk.byteLength;
     if(expect.maxBytes!==undefined&&size>expect.maxBytes){outcome="too-large";break;}
     hash.update(chunk);await handle.write(chunk);
    }
   }catch(error){if((error as NodeJS.ErrnoException).syscall!==undefined||signal?.aborted)throw error;outcome="aborted";}
   await handle.close();
  }catch(error){await handle.close().catch(()=>undefined);await unlink(temporary).catch(()=>undefined);guard.relieve(error);throw error;}
  const digest=hash.digest("hex");
  if(!outcome&&expect.size!==undefined&&size!==expect.size)outcome="length-mismatch";
  if(!outcome&&expect.hash!==undefined&&digest!==expect.hash)outcome="digest-mismatch";
  if(outcome){await unlink(temporary).catch(()=>undefined);return{outcome};}
  const target=objectPath(digest),existed=await exists(target);
  await place(temporary,digest);await touch(digest);if(!existed)guard.account(size);
  return{outcome:existed?"exists":"created",digest:Object.freeze({hash:digest,size})};
 };
 const store:CasStoreV1={
  root,policy,objectPath,
  async put(source,signal){
   const result=await ingest(source,{},signal);
   if(!result.digest)throw Error("Cache store write failed / Cache-Speicher-Schreiben fehlgeschlagen: "+result.outcome);
   return result.digest;
  },
  putChecked:(source,expect,signal)=>ingest(source,expect,signal),
  async get(hash){
   sha(hash);
   try{const info=await lstat(objectPath(hash));await touch(hash);return{path:objectPath(hash),size:allocated(info)};}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}
  },
  async read(hash){
   sha(hash);
   try{const bytes=await readFile(objectPath(hash));await touch(hash);return bytes;}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}
  },
  async has(hashes){for(const hash of hashes)sha(hash);return Promise.all(hashes.map(hash=>exists(objectPath(hash))));},
  async link(hash,destination,mode="auto"){
   sha(hash);const source=objectPath(hash);if(!await exists(source))return undefined;
   await mkdir(dirname(destination),{recursive:true});await rm(destination,{force:true});
   const order:CasLinkModeV1[]=mode==="auto"?["reflink","copy"]:[mode];
   for(const attempt of order){
    try{
     if(attempt==="reflink")await copyFile(source,destination,constants.COPYFILE_FICLONE_FORCE);
     else if(attempt==="hardlink")await link(source,destination);
     else await copyFile(source,destination);
     await touch(hash);return attempt;
    }catch(error){await rm(destination,{force:true}).catch(()=>undefined);if(attempt===order.at(-1))throw error;}
   }
   return undefined;
  },
  touch,
  async adopt(path,signal){
   const handle=await open(path,"r"),hash=createHash("sha256");let size=0;
   try{const buffer=Buffer.allocUnsafe(1<<20);for(;;){signal?.throwIfAborted();const{bytesRead}=await handle.read(buffer,0,buffer.length,null);if(!bytesRead)break;hash.update(buffer.subarray(0,bytesRead));size+=bytesRead;}}finally{await handle.close();}
   const digest=hash.digest("hex"),target=objectPath(digest);
   if(!await exists(target)){
    await guard.admit(size,signal);
    const directory=join(root,"v1","t");await mkdir(directory,{recursive:true});
    const temporary=join(directory,`${writer}-${randomUUID().slice(0,12)}`);
    try{await copyFile(path,temporary,constants.COPYFILE_FICLONE);}catch(error){await unlink(temporary).catch(()=>undefined);guard.relieve(error);throw error;}
    await place(temporary,digest);guard.account(size);
   }
   await touch(digest);return Object.freeze({hash:digest,size});
  },
  pin:(holder,hashes,ttlMs=policy.leaseMaxMs)=>pinCasV1(root,policy,holder,hashes,ttlMs,now(),pid),
  release:holder=>releaseCasPinsV1(root,holder),
 };
 return Object.freeze(store);
}
