import {appendFile,lstat,mkdir,open,readdir,rename,unlink,writeFile} from "node:fs/promises";
import {readFile} from "node:fs/promises";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import type {CasPolicyV1} from "../🧬️schema/🟦️.ts";
import {processIdentityForPidV1,requireProcessIdentityV1,processHolderAliveV1} from "../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";

const HOLDER=new RegExp(schema.definitions.PinFile.properties.holder.pattern,"u"),DIGEST=new RegExp(schema.definitions.Digest.pattern,"u");

/** 📌️ Directory of the pin files of the store; one file per holder, rewritten atomically. */
export const casPinDirectoryV1=(root:string):string=>join(root,"v1","p");

/** 📌️ Pins `digests` for `holder` until `nowMs + ttlMs` (capped by the policy's lease maximum); a pin or lease protects objects from eviction, and a holder that died loses it at once. Re-pinning with the same holder replaces its earlier set (renewal). */
export async function pinCasV1(root:string,policy:Pick<CasPolicyV1,"leaseMaxMs">,holder:string,digests:readonly string[],ttlMs:number,nowMs:number,pid:number=process.pid):Promise<void>{
 if(!HOLDER.test(holder))throw Error("Valid pin holder required / Gültiger Pin-Halter erforderlich");
 for(const digest of digests)if(!DIGEST.test(digest))throw Error("Valid digest required / Gültiger Digest erforderlich");
 const directory=casPinDirectoryV1(root);await mkdir(directory,{recursive:true});
 const identity=processIdentityForPidV1(pid),body=JSON.stringify({version:1,holder,pid:identity.pid,startMs:identity.startMs,expiresAtMs:nowMs+Math.min(Math.max(ttlMs,0),policy.leaseMaxMs),digests:[...new Set(digests)].sort()}),temporary=join(directory,`.${holder}.${process.pid}.${Math.random().toString(36).slice(2)}`);
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
  const file=join(directory,name);let pin:{holder?:string;pid?:number;startMs?:number;expiresAtMs?:number;digests?:unknown};
  try{pin=JSON.parse(await readFile(file,"utf8"));}catch{continue;}
  const identity=requireProcessIdentityV1(pin,"cas pin "+name);
  if(!(Number(pin.expiresAtMs)>nowMs)||!processHolderAliveV1(identity)){await unlink(file).catch(()=>undefined);continue;}
  if(Array.isArray(pin.digests))for(const digest of pin.digests)if(typeof digest==="string"&&DIGEST.test(digest)&&!pinned.has(digest))pinned.set(digest,String(pin.holder));
 }
 return pinned;
}

type HeldCasPinsV1={digests:Set<string>;lastWrite:Promise<void>;queued:boolean;leaseMaxMs:number};
const holds=new Map<string,HeldCasPinsV1>();

function publishHeldCasPinsV1(root:string,holder:string,hold:HeldCasPinsV1,leaseMaxMs:number):Promise<void>{
 hold.leaseMaxMs=leaseMaxMs;
 if(hold.queued)return hold.lastWrite;
 hold.queued=true;
 hold.lastWrite=hold.lastWrite.then(()=>{
  hold.queued=false;
  const {digests,leaseMaxMs}=hold;
  return pinCasV1(root,{leaseMaxMs},holder,[...digests],leaseMaxMs,Date.now()).catch(()=>undefined);
 });
 return hold.lastWrite;
}

/** 🪢️ Holds digests for the life of this process, dropping the least recently named above `maximum`. Updates and renewals coalesce behind the current write and publish the latest set in order. A failed write keeps the prior pin. */
export async function holdCasPinsV1(root:string,policy:Pick<CasPolicyV1,"leaseMaxMs">,holder:string,digests:Iterable<string>,maximum:number=65536):Promise<void>{
 const key=root+"\0"+holder;
 let hold=holds.get(key);
 if(!hold){
  const created:HeldCasPinsV1={digests:new Set(),lastWrite:Promise.resolve(),queued:false,leaseMaxMs:policy.leaseMaxMs};
  const renew=setInterval(()=>{void publishHeldCasPinsV1(root,holder,created,policy.leaseMaxMs);},Math.max(1,Math.floor(policy.leaseMaxMs/3)));
  renew.unref?.();holds.set(key,hold=created);
 }
 const set=hold.digests;
 for(const digest of digests){set.delete(digest);set.add(digest);}
 while(set.size>maximum)set.delete(set.values().next().value!);
 await publishHeldCasPinsV1(root,holder,hold,policy.leaseMaxMs);
}

/** ⏳️ The request header with which a client marks a blob as an input of one action (value `input`): the cache server leases it instead of treating it like a published output. */
export const CAS_LEASE_HEADER_V1="x-semio-lease";

/** ⏳️ Reads the lease header: `input` marks an input of an anonymous action, `input:<action>` of the named one (`action` is what lets two actions that share a blob end their leases independently); anything else is not a lease. */
export function parseCasLeaseHeaderV1(value:string|null):Readonly<{action:string}>|undefined{
 if(value==="input")return Object.freeze({action:""});
 const action=value?.startsWith("input:")?value.slice(6):undefined;
 return action!==undefined&&HOLDER.test(action)?Object.freeze({action}):undefined;
}

/** ⏳️ Directory of the transient lease segments of the store: one append-only file per writer process, a line `{d,u}` per digest and lease expiry. */
export const casTransientDirectoryV1=(root:string):string=>join(root,"v1","x");

/** ⏳️ Leases blobs that are inputs of one action (not outputs): they are protected until `nowMs + ttlMs` (the action runs, plus a grace) and are the first the collector takes after that, unless something read them later. A lease names the `action` that holds it (blobs shared by two actions are protected until both ended); a new lease of the same action replaces its earlier one (renewal). Never pins: outputs are pinned, inputs only lease. */
export async function leaseCasTransientV1(root:string,policy:Pick<CasPolicyV1,"transientLeaseMs">&Partial<Pick<CasPolicyV1,"transientSegmentBytes">>,writer:string,digests:readonly string[],nowMs:number,ttlMs:number=policy.transientLeaseMs,action:string=""):Promise<void>{
 await appendCasTransientsV1(root,policy,writer,digests,nowMs,ttlMs,action,false);
}

/** 🔓️ Ends the leases `action` holds on inputs whose action ended: they now run only `transientGraceMs` longer (a retry of the action leases again) unless another action still holds the blob, after which the blobs are the first the collector takes. Never deletes a blob and never touches a pin. */
export async function releaseCasTransientV1(root:string,policy:Pick<CasPolicyV1,"transientLeaseMs"|"transientGraceMs">&Partial<Pick<CasPolicyV1,"transientSegmentBytes">>,writer:string,digests:readonly string[],nowMs:number,action:string=""):Promise<void>{
 await appendCasTransientsV1(root,policy,writer,digests,nowMs,policy.transientGraceMs,action,true);
}

async function appendCasTransientsV1(root:string,policy:Pick<CasPolicyV1,"transientLeaseMs">&Partial<Pick<CasPolicyV1,"transientSegmentBytes">>,writer:string,digests:readonly string[],nowMs:number,ttlMs:number,action:string,release:boolean):Promise<void>{
 if(!HOLDER.test(writer))throw Error("Valid lease writer required / Gültiger Lease-Halter erforderlich");
 if(action!==""&&!HOLDER.test(action))throw Error("Valid lease action required / Gültige Lease-Aktion erforderlich");
 for(const digest of digests)if(!DIGEST.test(digest))throw Error("Valid digest required / Gültiger Digest erforderlich");
 if(digests.length===0)return;
 const directory=casTransientDirectoryV1(root),until=nowMs+Math.min(Math.max(ttlMs,0),policy.transientLeaseMs*4);
 await mkdir(directory,{recursive:true});
 const text=[...new Set(digests)].map(d=>JSON.stringify({d,u:until,a:nowMs,...action===""?{}:{h:action},...release?{x:1}:{}})+"\n").join("");
 await appendFile(join(directory,await segmentOfWriterV1(directory,writer,Buffer.byteLength(text),policy.transientSegmentBytes??CAS_TRANSIENT_SEGMENT_BYTES_V1)),text);
}

/** 🔄️ The size from which a writer starts a new lease segment, so that a long-lived writer never grows one file without bound and closed segments can be compacted (declared as `Policy.transientSegmentBytes`). */
export const CAS_TRANSIENT_SEGMENT_BYTES_V1=8388608;
const writerSegments=new Map<string,{name:string;bytes:number}>();

async function segmentOfWriterV1(directory:string,writer:string,adding:number,limit:number):Promise<string>{
 const key=directory+"\0"+writer;let segment=writerSegments.get(key);
 if(segment===undefined){const info=await lstat(join(directory,writer+".jsonl")).catch(()=>undefined);segment={name:writer+".jsonl",bytes:info?.size??0};}
 if(segment.bytes>=limit)segment={name:`${writer}.${Date.now().toString(36)}.jsonl`,bytes:0};
 segment={name:segment.name,bytes:segment.bytes+adding};writerSegments.set(key,segment);return segment.name;
}

type SegmentCursorV1={ino:number;offset:number};
type LeaseRowV1={at:number;until:number};
type LeaseStateV1={files:Map<string,SegmentCursorV1>;rows:Map<string,Map<string,LeaseRowV1>>};
const leaseStates=new Map<string,LeaseStateV1>();

function foldLeaseLineV1(rows:LeaseStateV1["rows"],line:string):void{
 if(line==="")return;
 try{
  const row=JSON.parse(line) as {d?:unknown;u?:unknown;a?:unknown;h?:unknown};
  if(typeof row.d!=="string"||!DIGEST.test(row.d)||typeof row.u!=="number")return;
  const at=typeof row.a==="number"?row.a:0,action=typeof row.h==="string"?row.h:"";let actions=rows.get(row.d);
  if(actions===undefined)rows.set(row.d,actions=new Map());
  const held=actions.get(action);
  if(held===undefined||at>held.at)actions.set(action,{at,until:row.u});else if(at===held.at)held.until=Math.max(held.until,row.u);
 }catch{}
}

/** ⏳️ Reads the leases: per digest and holding action the latest line (by its time; a release is such a line with its short grace), then the latest expiry over the actions, over every segment; unreadable lines are skipped. Segments are append-only, so a segment already read is read again only from the byte after its last complete line (a rewritten, shrunk or replaced segment starts the root over): a collection that asks again at every shard costs the new bytes, not the whole history. */
export async function readCasTransientsV1(root:string,requesterLeaseMs:number=REQUESTER_LEASE_MS_V1):Promise<Map<string,number>>{
 const directory=casTransientDirectoryV1(root);let names:string[];
 try{names=(await readdir(directory)).filter(name=>name.endsWith(".jsonl")).sort();}catch{leaseStates.delete(root);return new Map();}
 let state=leaseStates.get(root);const infos=new Map<string,{ino:number;size:number}>();
 for(const name of names)try{const info=await lstat(join(directory,name));infos.set(name,{ino:info.ino,size:info.size});}catch{}
 if(state!==undefined)for(const [name,cursor] of state.files){const info=infos.get(name);if(info===undefined||info.ino!==cursor.ino||info.size<cursor.offset){state=undefined;break;}}
 if(state===undefined){state={files:new Map(),rows:new Map()};}
 leaseStates.set(root,state);
 for(const [name,info] of infos){
  const cursor=state.files.get(name)??{ino:info.ino,offset:0};
  if(info.size<=cursor.offset){state.files.set(name,cursor);continue;}
  let handle;try{handle=await open(join(directory,name),"r");}catch{continue;}
  try{
   const buffer=Buffer.allocUnsafe(info.size-cursor.offset),{bytesRead}=await handle.read(buffer,0,buffer.length,cursor.offset),end=buffer.subarray(0,bytesRead).lastIndexOf(10);
   if(end>=0){for(const line of buffer.subarray(0,end).toString("utf8").split("\n"))foldLeaseLineV1(state.rows,line);cursor.offset+=end+1;}
   state.files.set(name,cursor);
  }catch{}finally{await handle.close().catch(()=>undefined);}
 }
 const leases=new Map<string,number>();
 for(const [digest,actions] of state.rows){let until=0;for(const [action,row] of actions){const bound=casLeaseUntilV1(action,row,actions,requesterLeaseMs);if(bound>until)until=bound;}if(until>0)leases.set(digest,until);}
 return leases;
}

/** ⏳️ The expiry one lease line grants: the scheduler's operation lease (`op-<action>`) as written, renewed while the operation lives; a requester's lease (`<action>` or anonymous) at most `requesterLeaseMs` after it was taken, and never past the operation lease of the same action once the scheduler holds that operation, so a client that asks for a long fixed lease cannot keep inputs beyond the upload, the operation and the grace.
 * @see ../../🌐️remote-execution/🗓️scheduler/🔐️input-leases/🟦️.ts */
export function casLeaseUntilV1(action:string,row:Readonly<{at:number;until:number}>,actions:ReadonlyMap<string,Readonly<{at:number;until:number}>>,requesterLeaseMs:number):number{
 if(action.startsWith(CAS_OPERATION_LEASE_PREFIX_V1))return row.until;
 const bounded=Math.min(row.until,row.at+requesterLeaseMs),operation=action===""?undefined:actions.get(CAS_OPERATION_LEASE_PREFIX_V1+action);
 return operation!==undefined&&operation.at>=row.at?Math.min(bounded,operation.until):bounded;
}

/** ⏳️ The prefix of the lease action under which the scheduler holds the inputs of one operation for its lifetime. */
export const CAS_OPERATION_LEASE_PREFIX_V1="op-";
const REQUESTER_LEASE_MS_V1=schema.definitions.Policy.const.requesterLeaseMs;

/** 🧹️ Drops what no longer matters from segments nobody appended to for two leases: lines whose lease ended or whose blob is gone (`exists` false); the objects of an ended lease are plain candidates of the collector from then on. A segment that ends up empty is removed. */
export async function compactCasTransientsV1(root:string,policy:Pick<CasPolicyV1,"transientLeaseMs">,nowMs:number,exists:(digest:string)=>Promise<boolean>):Promise<number>{
 const directory=casTransientDirectoryV1(root);let names:string[],removed=0;
 try{names=await readdir(directory);}catch{return 0;}
 for(const name of names){
  if(!name.endsWith(".jsonl"))continue;
  const path=join(directory,name);let info;try{info=await lstat(path);}catch{continue;}
  if(nowMs-info.mtimeMs<policy.transientLeaseMs*2)continue;
  let text:string;try{text=await readFile(path,"utf8");}catch{continue;}
  const kept:string[]=[];
  for(const line of text.split("\n")){
   if(line==="")continue;
   try{const row=JSON.parse(line) as {d:string;u?:number};if(!(typeof row.u==="number"&&row.u<=nowMs)&&await exists(row.d))kept.push(line);else removed++;}catch{removed++;}
  }
  if(kept.length===0){await unlink(path).catch(()=>undefined);continue;}
  if(kept.length===text.split("\n").filter(Boolean).length)continue;
  const temporary=join(directory,`.${name}.${process.pid}`);
  try{await writeFile(temporary,kept.join("\n")+"\n");await rename(temporary,path);}catch{await unlink(temporary).catch(()=>undefined);}
 }
 return removed;
}

/** ⏳️ A cheap stamp of the lease segments (names, sizes, modification times) that changes whenever a lease was appended, so a long collection reads the segments again only then. */
export async function stampCasTransientsV1(root:string):Promise<string>{
 const directory=casTransientDirectoryV1(root);let names:string[];
 try{names=(await readdir(directory)).sort();}catch{return "";}
 const parts:string[]=[];
 for(const name of names)try{const info=await lstat(join(directory,name));parts.push(`${name}:${info.size}:${info.mtimeMs}`);}catch{}
 return parts.join("|");
}
