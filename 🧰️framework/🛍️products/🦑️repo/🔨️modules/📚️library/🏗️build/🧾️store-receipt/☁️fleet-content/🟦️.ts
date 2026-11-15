import {remoteCacheFetchV1} from "../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {createHash,randomBytes} from "node:crypto";
import {createReadStream} from "node:fs";
import {lstat,mkdir,open,readdir,readFile,realpath,rename,rm,symlink} from "node:fs/promises";
import {dirname,isAbsolute,join,relative,resolve,sep} from "node:path";
import {httpRemoteExecutionContentV1,type RemoteExecutionContentV1} from "../../🌐️remote-execution/🗄️content/🟦️.ts";
import {parseStoreReceiptV1,verifyStoreReceiptV1,type StoreReceiptEntryV1,type StoreReceiptV1} from "../🟨️.mjs";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** ☁️ The authored limits of publishing and restoring receipt content.
 * @see ./🧬️schema/🔣️.json */
export const FLEET_CONTENT_POLICY_V1=schema.definitions.Policy.const;
const policy=FLEET_CONTENT_POLICY_V1,SHA=/^[0-9a-f]{64}$/u;

/** 📤️ What a publication did: blobs the receipt names, blobs the store lacked and uploaded, and their bytes. */
export type FleetContentPublishV1=Readonly<{state:"published"|"skipped"|"failed";blobs:number;uploaded:number;bytes:number;reason?:string}>;
/** 📥️ What a restore did: entries it brought back, files and bytes written. */
export type FleetContentRestoreV1=Readonly<{state:"restored"|"partial"|"unavailable"|"nothing";entries:number;files:number;bytes:number;reason?:string}>;
/** 🌐️ The content store a receipt travels through: the blobs and the instance they live in. */
export type FleetContentStoreV1=Readonly<{content:RemoteExecutionContentV1;instance:string}>;

const bytesOf=(text:string)=>Buffer.from(text,"utf8");
const order=(left:string,right:string)=>Buffer.compare(bytesOf(left),bytesOf(right));

async function digestOf(path:string):Promise<{hash:string;size:number}>{
 const hash=createHash("sha256");let size=0;
 for await(const chunk of createReadStream(path)){hash.update(chunk);size+=(chunk as Buffer).length;}
 return {hash:hash.digest("hex"),size};
}

async function walk(root:string,prefix:string,rows:Array<{path:string;bytes:number;hash:string}>):Promise<void>{
 for(const name of(await readdir(join(root,prefix))).sort()){
  const path=prefix?`${prefix}/${name}`:name,facts=await lstat(join(root,path));
  if(facts.isDirectory())await walk(root,path,rows);
  else if(facts.isFile()){const found=await digestOf(join(root,path));rows.push({path,bytes:found.size,hash:found.hash});}
  else throw Error("Store content holds regular files and directories only: "+path);
 }
}

/** 📜️ The tree manifest of a directory: the exact text whose SHA-256 is the directory digest of a receipt entry, with the files it names. */
export async function storeTreeManifestV1(directory:string):Promise<{text:string;rows:ReadonlyArray<{path:string;bytes:number;hash:string}>}>{
 const rows:Array<{path:string;bytes:number;hash:string}>=[];await walk(directory,"",rows);rows.sort((left,right)=>order(left.path,right.path));
 return {text:rows.map(row=>`${row.path}\n${row.bytes}\n${row.hash}\n`).join(""),rows};
}

/** 🔬️ Reads a manifest text back into its rows; refuses anything the digest basis cannot hold (a missing field, a size or digest that is not canonical, an unsafe path). */
export function parseStoreTreeManifestV1(text:string):ReadonlyArray<{path:string;bytes:number;hash:string}>{
 if(text==="")return [];
 if(!text.endsWith("\n"))throw Error("Manifest must end with a newline");
 const fields=text.slice(0,-1).split("\n");
 if(fields.length%3!==0)throw Error("Manifest rows have three fields");
 if(fields.length/3>policy.maximumFiles)throw Error("Manifest names too many files");
 const rows:Array<{path:string;bytes:number;hash:string}>=[],seen=new Set<string>();
 for(let at=0;at<fields.length;at+=3){
  const path=fields[at]!,size=fields[at+1]!,hash=fields[at+2]!;
  if(path===""||path.startsWith("/")||path.includes("\\")||path.split("/").some(part=>part===""||part==="."||part===".."))throw Error("Manifest path unsafe: "+path);
  if(!/^(0|[1-9][0-9]{0,15})$/u.test(size)||!SHA.test(hash)||seen.has(path))throw Error("Manifest row malformed: "+path);
  seen.add(path);rows.push({path,bytes:Number(size),hash});
 }
 return rows;
}

async function pool<T>(items:readonly T[],limit:number,work:(item:T)=>Promise<void>):Promise<void>{
 let next=0,failure:unknown;
 await Promise.all(Array.from({length:Math.min(limit,items.length)},async()=>{for(;;){if(failure!==undefined)return;const at=next++;if(at>=items.length)return;try{await work(items[at]!);}catch(error){failure??=error;return;}}}));
 if(failure!==undefined)throw failure;
}

/** 📤️ Uploads the store content a receipt names: every file by its digest and every directory's manifest under its directory digest, each only when the store lacks it. Content that no longer matches its receipt entry is not uploaded (the receipt is stale and a restore would be refused). Never throws: a store that cannot be reached yields `failed` and the task keeps working without sharing its payload. */
export async function publishStoreReceiptContentV1(options:Readonly<{workspaceRoot:string;receipt:string;store:FleetContentStoreV1;signal?:AbortSignal}>):Promise<FleetContentPublishV1>{
 const signal=options.signal?AbortSignal.any([options.signal,AbortSignal.timeout(policy.publishTimeoutSeconds*1000)]):AbortSignal.timeout(policy.publishTimeoutSeconds*1000);
 let parsed:StoreReceiptV1;
 try{parsed=parseStoreReceiptV1(await readFile(options.receipt,"utf8"));}catch(error){return {state:"skipped",blobs:0,uploaded:0,bytes:0,reason:"receipt unreadable"};}
 try{
  const blobs=new Map<string,{size:number;source:{kind:"file";path:string}|{kind:"text";text:string}}>();
  for(const entry of parsed.entries){
   const absolute=resolve(options.workspaceRoot,entry.path),inside=relative(options.workspaceRoot,absolute);
   if(!inside||inside.startsWith("..")||isAbsolute(inside))return {state:"skipped",blobs:0,uploaded:0,bytes:0,reason:"entry outside workspace"};
   const facts=await lstat(absolute);
   if(entry.kind==="file"){
    if(!facts.isFile())return {state:"skipped",blobs:0,uploaded:0,bytes:0,reason:"entry changed"};
    const found=await digestOf(absolute);
    if(found.hash!==entry.sha256||found.size!==entry.bytes)return {state:"skipped",blobs:0,uploaded:0,bytes:0,reason:"entry changed"};
    blobs.set(found.hash,{size:found.size,source:{kind:"file",path:absolute}});
   }else{
    if(!facts.isDirectory())return {state:"skipped",blobs:0,uploaded:0,bytes:0,reason:"entry changed"};
    const manifest=await storeTreeManifestV1(absolute),text=bytesOf(manifest.text);
    if(createHash("sha256").update(text).digest("hex")!==entry.sha256||text.length>policy.maximumManifestBytes||manifest.rows.length!==entry.files)return {state:"skipped",blobs:0,uploaded:0,bytes:0,reason:"entry changed"};
    if(manifest.text!=="")blobs.set(entry.sha256,{size:text.length,source:{kind:"text",text:manifest.text}});
    for(const row of manifest.rows)blobs.set(row.hash,{size:row.bytes,source:{kind:"file",path:join(absolute,row.path)}});
   }
  }
  const digests=[...blobs].map(([hash,row])=>({hash,size:row.size})),missing=await options.store.content.missing(options.store.instance,digests,signal);
  let bytes=0;
  await pool(missing,policy.concurrency,async digest=>{
   const row=blobs.get(digest.hash)!;
   if(row.source.kind==="text")await options.store.content.write(options.store.instance,digest,bytesOf(row.source.text),signal);
   else await options.store.content.write(options.store.instance,digest,createReadStream(row.source.path) as AsyncIterable<Uint8Array>,signal);
   bytes+=digest.size;
  });
  return {state:"published",blobs:digests.length,uploaded:missing.length,bytes};
 }catch(error){return {state:"failed",blobs:0,uploaded:0,bytes:0,reason:String((error as Error)?.message??error).slice(0,200)};}
}

async function place(content:FleetContentStoreV1,digest:{hash:string;size:number},target:string,signal:AbortSignal):Promise<number>{
 const stream=await content.content.stream(content.instance,digest,signal);
 if(stream===undefined)throw Error("Blob absent: "+digest.hash);
 await mkdir(dirname(target),{recursive:true});
 const handle=await open(target,"w",0o644),hash=createHash("sha256");let size=0;
 try{for await(const chunk of stream){hash.update(chunk);size+=chunk.length;if(size>digest.size)throw Error("Blob larger than declared: "+digest.hash);await handle.write(chunk);}}finally{await handle.close();}
 if(size!==digest.size||hash.digest("hex")!==digest.hash){await rm(target,{force:true});throw Error("Blob differs from its digest: "+digest.hash);}
 return size;
}

async function restoreEntryLocked(workspaceRoot:string,entry:StoreReceiptEntryV1,store:FleetContentStoreV1,signal:AbortSignal):Promise<{files:number;bytes:number}>{
 const destination=resolve(workspaceRoot,entry.path),suffix=policy.restoreDirectorySuffix+randomBytes(4).toString("hex"),staging=destination+suffix;
 try{
  let files=0,bytes=0;
  if(entry.kind==="file"){
   bytes=await place(store,{hash:entry.sha256,size:entry.bytes},staging,signal);files=1;
  }else{
   await mkdir(staging,{recursive:true});
   if(entry.files>0){
    const text=await store.content.read(store.instance,{hash:entry.sha256,size:1},signal);
    if(text===undefined)throw Error("Manifest absent: "+entry.sha256);
    if(text.length>policy.maximumManifestBytes||createHash("sha256").update(text).digest("hex")!==entry.sha256)throw Error("Manifest differs from its digest");
    const rows=parseStoreTreeManifestV1(new TextDecoder().decode(text));
    if(rows.length!==entry.files||rows.reduce((sum,row)=>sum+row.bytes,0)!==entry.bytes)throw Error("Manifest disagrees with the receipt entry");
    await pool(rows,policy.concurrency,async row=>{bytes+=await place(store,{hash:row.hash,size:row.bytes},join(staging,row.path),signal);files++;});
   }
  }
  await rm(destination,{recursive:true,force:true});
  try{await rename(staging,destination);}catch(error){
   const again=await verifyStoreReceiptV1(workspaceRoot,{version:1,project:"restore",target:"restore",entries:[entry],links:[]},"stat");
   if(again.state!=="valid")throw error;
   await rm(staging,{recursive:true,force:true});
  }
  return {files,bytes};
 }catch(error){await rm(staging,{recursive:true,force:true}).catch(()=>{});throw error;}
}

async function restoreEntry(workspaceRoot:string,entry:StoreReceiptEntryV1,store:FleetContentStoreV1,signal:AbortSignal):Promise<{files:number;bytes:number}>{
 const lock=resolve(workspaceRoot,entry.path)+policy.restoreDirectorySuffix+"lock";
 await mkdir(dirname(lock),{recursive:true});
 for(;;){
  try{await mkdir(lock);break;}catch(error){
   if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;
   const facts=await lstat(lock).catch(()=>undefined);
   if(facts&&Date.now()-facts.mtimeMs>policy.restoreTimeoutSeconds*1000){await rm(lock,{recursive:true,force:true});continue;}
   if(signal.aborted)throw Error("Restore aborted while another restore holds "+entry.path);
   await new Promise(done=>setTimeout(done,20));
  }
 }
 try{
  const current=await verifyStoreReceiptV1(workspaceRoot,{version:1,project:"restore",target:"restore",entries:[entry],links:[]},"digest");
  if(current.state==="valid")return {files:0,bytes:0};
  return await restoreEntryLocked(workspaceRoot,entry,store,signal);
 }finally{await rm(lock,{recursive:true,force:true});}
}

/** 🔗️ How one receipt link is written: a relative POSIX link elsewhere; on Windows a directory junction to the absolute target (a symbolic link needs a privilege most accounts lack) or a file link, so a restore never fails on a missing privilege for the common directory case. */
export function storeReceiptLinkPlanV1(path:string,target:string,directory:boolean,platform:NodeJS.Platform=process.platform):Readonly<{text:string;type:"dir"|"file"|"junction"}>{
 if(platform==="win32")return directory?{text:target,type:"junction"}:{text:relative(dirname(path),target),type:"file"};
 return {text:relative(dirname(path),target).split(sep).join("/"),type:directory?"dir":"file"};
}

async function restoreLink(workspaceRoot:string,link:{path:string;target:string}):Promise<void>{
 const path=resolve(workspaceRoot,link.path),target=resolve(workspaceRoot,link.target);
 try{if((await lstat(path)).isSymbolicLink()&&await realpath(path)===await realpath(target))return;}catch{}
 const facts=await lstat(target);
 await rm(path,{recursive:true,force:true});await mkdir(dirname(path),{recursive:true});
 const plan=storeReceiptLinkPlanV1(path,target,facts.isDirectory());
 await symlink(plan.text,path,plan.type);
}

/** 📥️ Brings back the store content a receipt names but the workspace lacks or holds changed: each missing entry is rebuilt aside from the fleet store, every byte checked against its digest, then swapped in; links are bound again. Entries that are already valid are left alone. An entry that cannot be restored (blob absent, store unreachable, digest mismatch) leaves the workspace unchanged for that entry and yields `partial` or `unavailable`: the caller then re-executes the task as it does today. */
export async function restoreStoreReceiptContentV1(options:Readonly<{workspaceRoot:string;receipt:StoreReceiptV1;store:FleetContentStoreV1;signal?:AbortSignal}>):Promise<FleetContentRestoreV1>{
 const signal=options.signal?AbortSignal.any([options.signal,AbortSignal.timeout(policy.restoreTimeoutSeconds*1000)]):AbortSignal.timeout(policy.restoreTimeoutSeconds*1000);
 let entries=0,files=0,bytes=0,failed:string|undefined;
 for(const entry of options.receipt.entries){
  const current=await verifyStoreReceiptV1(options.workspaceRoot,{...options.receipt,entries:[entry],links:[]},"digest");
  if(current.state==="valid")continue;
  try{const done=await restoreEntry(options.workspaceRoot,entry,options.store,signal);entries++;files+=done.files;bytes+=done.bytes;}
  catch(error){failed??=String((error as Error)?.message??error).slice(0,200);}
 }
 if(failed===undefined)for(const link of options.receipt.links)try{await restoreLink(options.workspaceRoot,link);}catch(error){failed??=String((error as Error)?.message??error).slice(0,200);}
 if(failed!==undefined)return {state:entries>0?"partial":"unavailable",entries,files,bytes,reason:failed};
 return {state:entries>0?"restored":"nothing",entries,files,bytes};
}

/** 🌐️ Opens the fleet content store of an Nx cache environment (the front's address or the cache server itself, with the token Nx was given); `undefined` when Nx has no remote cache. */
export function fleetContentFromEnvironmentV1(environment:Readonly<Record<string,string|undefined>>):FleetContentStoreV1|undefined{
 const server=environment.NX_SELF_HOSTED_REMOTE_CACHE_SERVER;if(server===undefined||server==="")return undefined;
 let parsed:URL;try{parsed=new URL(server);}catch{return undefined;}
 const segments=parsed.pathname.split("/").filter(Boolean);if(segments.length!==1)return undefined;
 return {content:httpRemoteExecutionContentV1({endpoint:parsed.origin,token:environment.NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN,fetch:remoteCacheFetchV1(environment),timeoutSeconds:policy.requestTimeoutSeconds,retries:2}),instance:segments[0]!};
}

/** 🗣️ Fills one text of the catalogue in one language. */
export function fleetContentTextV1(language:"en"|"de",key:keyof typeof schema.definitions.Messages.const,variables:Readonly<Record<string,string|number>>):string{
 return schema.definitions.Messages.const[key][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(variables[name]??""));
}
