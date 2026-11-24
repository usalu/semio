import {createHash} from "node:crypto";
import {existsSync,linkSync,lstatSync,readdirSync,mkdirSync,renameSync,rmdirSync,unlinkSync,writeFileSync} from "node:fs";
import {appendFile,lstat,mkdir,readdir,readFile,unlink} from "node:fs/promises";
import type {Dirent} from "node:fs";
import {dirname,join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {readCasPinsV1,readCasTransientsV1} from "../📌️pins/🟦️.ts";
import type {CasPolicyV1} from "../🧬️schema/🟦️.ts";

export {planCasHolderEvictionV1,protectCasHolderEntriesV1} from "./🧮️plan.ts";
export type {CasHolderPlanBlobV1,CasHolderPlanEntryV1,CasHolderPlanV1} from "./🧮️plan.ts";

const HOLDER=new RegExp(schema.definitions.Holder.properties.holder.pattern,"u"),KIND=new RegExp(schema.definitions.Holder.properties.kind.pattern,"u"),REFERENCE=new RegExp(schema.definitions.Holder.properties.indexes.items.properties.references.items.pattern,"u"),DIGEST=new RegExp(schema.definitions.Digest.pattern,"u");

/** 🫂️ One folder of a holder's entry files and the dotted paths inside each JSON file whose string values are object digests (`*` stands for every element of an array or every value of an object). */
export type CasHolderIndexV1=Readonly<{directory:string;references:readonly string[]}>;
/** 🫂️ A registered reference holder of the host store: a cache root outside the store whose live entries keep objects alive. It ages out when it is not renewed for `holderTtlMs` or when its root vanished.
 * @see ../🧬️schema/🔣️.json */
export type CasHolderV1=Readonly<{version:1;holder:string;kind:string;root:string;renewedAtMs:number;expiresAtMs:number;indexes:readonly CasHolderIndexV1[];evictionLog:string}>;
/** 🫂️ What a holder declares about itself; its id comes from kind and root, the times from the registration. */
export type CasHolderDeclarationV1=Readonly<{kind:string;root:string;indexes:readonly CasHolderIndexV1[];evictionLog:string}>;
/** 🫂️ One entry file of a holder with the object digests it names. */
export type CasHolderEntryV1=Readonly<{holder:string;file:string;mtimeMs:number;digests:readonly string[]}>;
/** 🫂️ The live holders, their entries and every digest the entries keep alive (with the first holder naming it). */
export type CasHeldV1=Readonly<{holders:readonly CasHolderV1[];entries:readonly CasHolderEntryV1[];digests:ReadonlyMap<string,string>}>;

/** 🫂️ Directory of the holder records of the store; one file per holder, rewritten atomically on renewal. */
export const casHolderDirectoryV1=(root:string):string=>join(root,"v1","h");

/** 🫂️ The stable id of the holder of one root: its kind and a hash of the root path. */
export const casHolderIdV1=(kind:string,path:string):string=>`${kind}-${createHash("sha256").update(path).digest("hex").slice(0,16)}`;

const renewed=new Map<string,number>();

/** 🫂️ Registers or renews the holder of a cache root; synchronous for callers that cannot await, and a renewal inside `holderRenewMs` of this process's last write is skipped. Returns the holder id. */
export function registerCasHolderSyncV1(root:string,policy:Pick<CasPolicyV1,"holderTtlMs"|"holderRenewMs">,holder:CasHolderDeclarationV1,nowMs:number=Date.now()):string{
 const id=casHolderIdV1(holder.kind,holder.root),key=root+"\0"+id,last=renewed.get(key);
 if(last!==undefined&&nowMs-last<policy.holderRenewMs)return id;
 if(!HOLDER.test(id)||!KIND.test(holder.kind))throw Error("Valid holder required / Gültiger Halter erforderlich");
 const directory=casHolderDirectoryV1(root);mkdirSync(directory,{recursive:true});
 const temporary=join(directory,`.${id}.${process.pid}.${Math.random().toString(36).slice(2)}`);
 writeFileSync(temporary,JSON.stringify({version:1,holder:id,kind:holder.kind,root:holder.root,renewedAtMs:nowMs,expiresAtMs:nowMs+policy.holderTtlMs,indexes:holder.indexes,evictionLog:holder.evictionLog}));
 renameSync(temporary,join(directory,id+".json"));renewed.set(key,nowMs);
 return id;
}

/** 🫂️ Removes the holder record of a root. */
export async function releaseCasHolderV1(root:string,holder:string):Promise<void>{
 if(!HOLDER.test(holder))throw Error("Valid holder required / Gültiger Halter erforderlich");
 renewed.delete(root+"\0"+holder);await unlink(join(casHolderDirectoryV1(root),holder+".json")).catch(()=>undefined);
}

const valid=(row:Partial<CasHolderV1>,name:string):row is CasHolderV1=>row.version===1&&typeof row.holder==="string"&&row.holder+".json"===name&&HOLDER.test(row.holder)&&typeof row.kind==="string"&&KIND.test(row.kind)&&typeof row.root==="string"&&row.root!==""&&typeof row.evictionLog==="string"&&Number.isSafeInteger(row.expiresAtMs)&&Number.isSafeInteger(row.renewedAtMs)&&Array.isArray(row.indexes)&&row.indexes.every(index=>typeof index?.directory==="string"&&Array.isArray(index.references)&&index.references.length>0&&index.references.every((path:unknown)=>typeof path==="string"&&REFERENCE.test(path)));

/** 🫂️ Reads the holders alive at `nowMs`: record not expired and root still there. Expired or orphaned records are removed. */
export async function readCasHoldersV1(root:string,nowMs:number):Promise<CasHolderV1[]>{
 const directory=casHolderDirectoryV1(root),rows:CasHolderV1[]=[];let names:string[];
 try{names=(await readdir(directory)).sort();}catch{return rows;}
 for(const name of names){
  if(!name.endsWith(".json")||name.startsWith("."))continue;
  const file=join(directory,name);let row:Partial<CasHolderV1>;
  try{row=JSON.parse(await readFile(file,"utf8"));}catch{continue;}
  if(!valid(row,name))continue;
  if(!(row.expiresAtMs>nowMs)||!existsSync(row.root)){await unlink(file).catch(()=>undefined);continue;}
  rows.push(row);
 }
 return rows;
}

function pick(value:unknown,path:readonly string[],found:Set<string>):void{
 if(path.length===0){if(typeof value==="string"&&DIGEST.test(value))found.add(value);return;}
 const [head,...rest]=path;
 if(value===null||typeof value!=="object")return;
 if(head==="*"){for(const item of Array.isArray(value)?value:Object.values(value))pick(item,rest,found);return;}
 if(!Array.isArray(value))pick((value as Record<string,unknown>)[head!],rest,found);
}

async function* jsonFiles(directory:string,signal?:AbortSignal):AsyncGenerator<string>{
 let names:Dirent[];try{names=await readdir(directory,{withFileTypes:true});}catch{return;}
 names.sort((left,right)=>left.name<right.name?-1:left.name>right.name?1:0);
 for(const name of names){
  signal?.throwIfAborted();
  if(name.name.startsWith("."))continue;
  const path=join(directory,name.name);
  if(name.isDirectory())yield*jsonFiles(path,signal);else if(name.isFile()&&name.name.endsWith(".json"))yield path;
 }
}

/** 🫂️ Streams the entry files of one holder, one open file at a time, with the digests they name; unreadable files are skipped. */
export async function* casHolderEntriesV1(holder:CasHolderV1,signal?:AbortSignal):AsyncGenerator<CasHolderEntryV1>{
 for(const index of holder.indexes){
  const paths=index.references.map(path=>path.split("."));
  for await(const file of jsonFiles(index.directory,signal)){
   let value:unknown,mtimeMs:number;
   try{[value,mtimeMs]=[JSON.parse(await readFile(file,"utf8")),(await lstat(file)).mtimeMs];}catch{continue;}
   const found=new Set<string>();for(const path of paths)pick(value,path,found);
   yield{holder:holder.holder,file,mtimeMs,digests:[...found].sort()};
  }
 }
}

/** 🫂️ Reads every live holder and its entries: the objects its live entries reference are the held set. Bounded: entry files are read one at a time. */
export async function readCasHeldV1(root:string,nowMs:number,signal?:AbortSignal):Promise<CasHeldV1>{
 const holders=await readCasHoldersV1(root,nowMs),entries:CasHolderEntryV1[]=[],digests=new Map<string,string>();
 for(const holder of holders)for await(const entry of casHolderEntriesV1(holder,signal)){entries.push(entry);for(const digest of entry.digests)if(!digests.has(digest))digests.set(digest,holder.holder);}
 return Object.freeze({holders,entries,digests});
}

const cached=new Map<string,{atMs:number;held:Promise<CasHeldV1>}>();

/** 🫂️ The held set for callers that ask often (a reclaim that checks many objects): reused for `holderCacheMs`, far below the minimum age, within which any entry published after the read has touched its objects. */
export function readCasHeldCachedV1(root:string,policy:Pick<CasPolicyV1,"holderCacheMs">,nowMs:number):Promise<CasHeldV1>{
 const hit=cached.get(root);
 if(hit&&nowMs-hit.atMs<policy.holderCacheMs&&nowMs>=hit.atMs)return hit.held;
 const held=readCasHeldV1(root,nowMs);cached.set(root,{atMs:nowMs,held});held.catch(()=>cached.delete(root));
 return held;
}

/** 🫂️ Forgets the cached held sets (a new collection or a test starts from the disk). */
export const resetCasHeldCacheV1=():void=>{cached.clear();};

/** 🫂️ Appends the capacity eviction of a holder to its own eviction log, with the reason `capacity`, the evicting process and its role (`guard`, `operator`, `recovery` or `collector`) so a loss can be attributed. Best effort, never throws. */
export async function recordCasHolderEvictionV1(holder:CasHolderV1,counts:Readonly<{entries:number;bytes:number}>,nowMs:number,role:string="collector"):Promise<void>{
 try{
  await mkdir(dirname(holder.evictionLog),{recursive:true});
  await appendFile(holder.evictionLog,JSON.stringify({version:1,atMs:nowMs,pid:process.pid,role,capacity:counts.entries,capacityBytes:counts.bytes,reason:"capacity"})+"\n");
 }catch{}
}

/** 🫂️ What one holder of the host store keeps: a pin holder (the cache server's live set, derived nodes), a registered unit-cache root with its entries, or the leased transient inputs. */
export type CasHolderUsageV1=Readonly<{kind:"pin"|"unit-cache"|"inputs";holder:string;label:string;entries:number;digests:number;bytes:number;expiresAtMs?:number}>;
/** 🫂️ The holders of the store with the bytes each one keeps (an object two holders keep counts for both rows and once in `referencedBytes`); Sequential and bounded: one object is statted at a time. */
export async function casHoldersUsageV1(root:string,policy:Pick<CasPolicyV1,"fanOutHexLength">,nowMs:number,signal?:AbortSignal):Promise<{rows:CasHolderUsageV1[];referencedBytes:number}>{
 const sizes=new Map<string,number>(),size=async(digest:string):Promise<number>=>{
  const known=sizes.get(digest);if(known!==undefined)return known;
  signal?.throwIfAborted();let bytes=0;try{const info=await lstat(join(root,"v1","o",digest.slice(0,policy.fanOutHexLength),digest.slice(policy.fanOutHexLength)));bytes=info.isFile()?info.size:0;}catch{}
  sizes.set(digest,bytes);return bytes;
 };
 const rows:CasHolderUsageV1[]=[],union=new Set<string>(),total=async(digests:Iterable<string>):Promise<{digests:number;bytes:number}>=>{let count=0,bytes=0;for(const digest of digests){const one=await size(digest);if(one>0){count++;bytes+=one;union.add(digest);}}return{digests:count,bytes};};
 const pinned=new Map<string,string[]>();
 for(const [digest,holder] of await readCasPinsV1(root,nowMs))pinned.set(holder,[...pinned.get(holder)??[],digest]);
 for(const [holder,digests] of [...pinned].sort(([left],[right])=>left<right?-1:1))rows.push({kind:"pin",holder,label:holder,entries:0,...await total(digests)});
 const held=await readCasHeldV1(root,nowMs,signal);
 for(const holder of held.holders){
  const mine=held.entries.filter(entry=>entry.holder===holder.holder),digests=new Set(mine.flatMap(entry=>entry.digests));
  rows.push({kind:"unit-cache",holder:holder.holder,label:holder.root,entries:mine.length,...await total(digests),expiresAtMs:holder.expiresAtMs});
 }
 const leases=[...(await readCasTransientsV1(root))].map(([digest])=>digest).sort();
 if(leases.length>0)rows.push({kind:"inputs",holder:"transient-inputs",label:"transient-inputs",entries:0,...await total(leases)});
 let referencedBytes=0;for(const digest of union)referencedBytes+=sizes.get(digest)??0;
 return{rows,referencedBytes};
}

/** 🔗️ Directory of the keeper links of the store: `<holder>/<shard>/<rest>` is a second hard link of an object that a live entry of that holder names. */
export const casKeeperDirectoryV1=(root:string):string=>join(root,"v1","k");
const keeperPath=(root:string,holder:string,hash:string,fan:number):string=>join(casKeeperDirectoryV1(root),holder,hash.slice(0,fan),hash.slice(fan));
const objectOf=(root:string,hash:string,fan:number):string=>join(root,"v1","o",hash.slice(0,fan),hash.slice(fan));

/** 🔗️ Keeps the objects of live entries alive against every collector, whatever its code version: a held object gets a second hard link in the keeper directory, and every generation of the collector and of the reclaimer has always protected a hard-linked object (it may be linked into a workspace). The link is the protection at the one place that unlinks objects, so a process that never heard of holders is harmless. Best effort, synchronous, idempotent; returns the links created.
 * @see ../🧹️collector/🟦️.ts */
export function keepCasObjectsSyncV1(root:string,policy:Pick<CasPolicyV1,"fanOutHexLength">,holder:string,digests:Iterable<string>):number{
 let created=0;
 for(const hash of digests){
  if(!DIGEST.test(hash))continue;const target=keeperPath(root,holder,hash,policy.fanOutHexLength);
  try{linkSync(objectOf(root,hash,policy.fanOutHexLength),target);created++;}
  catch(error){
   const code=(error as NodeJS.ErrnoException).code;
   if(code==="ENOENT"){try{mkdirSync(dirname(target),{recursive:true});linkSync(objectOf(root,hash,policy.fanOutHexLength),target);created++;}catch{}}
  }
 }
 return created;
}

/** 🔗️ The keeper links of one object: how many of its hard links belong to holders (those are not foreign links). */
export function casKeeperCountSyncV1(root:string,policy:Pick<CasPolicyV1,"fanOutHexLength">,hash:string):number{
 let count=0;let holders:string[];try{holders=readdirSync(casKeeperDirectoryV1(root));}catch{return 0;}
 for(const holder of holders)try{if(lstatSync(keeperPath(root,holder,hash,policy.fanOutHexLength)).isFile())count++;}catch{}
 return count;
}

/** 🔗️ Releases every keeper link of one object (its holder no longer names it); returns the links removed. */
export function dropCasKeepersSyncV1(root:string,policy:Pick<CasPolicyV1,"fanOutHexLength">,hash:string):number{
 let dropped=0;let holders:string[];try{holders=readdirSync(casKeeperDirectoryV1(root));}catch{return 0;}
 for(const holder of holders){const path=keeperPath(root,holder,hash,policy.fanOutHexLength);try{unlinkSync(path);dropped++;}catch{}}
 return dropped;
}

/** 🔗️ Aligns the keeper links with the live entries: every held object has its link, every link whose holder no longer names the object (or whose holder is gone) is released. Sequential and bounded by the held set. Returns the links created and released. */
export async function reconcileCasKeepersV1(root:string,policy:Pick<CasPolicyV1,"fanOutHexLength">,held:CasHeldV1,signal?:AbortSignal):Promise<{linked:number;released:number}>{
 const mine=new Map<string,Set<string>>();let linked=0,released=0;
 for(const entry of held.entries){let set=mine.get(entry.holder);if(!set){set=new Set();mine.set(entry.holder,set);}for(const digest of entry.digests)set.add(digest);}
 for(const [holder,digests] of mine){signal?.throwIfAborted();linked+=keepCasObjectsSyncV1(root,policy,holder,digests);}
 const base=casKeeperDirectoryV1(root);let holders:string[];try{holders=await readdir(base);}catch{return{linked,released};}
 for(const holder of holders){
  const keep=mine.get(holder);let shards:string[];try{shards=await readdir(join(base,holder));}catch{continue;}
  for(const shard of shards){
   signal?.throwIfAborted();let names:string[];try{names=await readdir(join(base,holder,shard));}catch{continue;}
   for(const name of names)if(keep===undefined||!keep.has(shard+name)){try{unlinkSync(join(base,holder,shard,name));released++;}catch{}}
   try{rmdirSync(join(base,holder,shard));}catch{}
  }
  if(keep===undefined)try{rmdirSync(join(base,holder));}catch{}
 }
 return{linked,released};
}
