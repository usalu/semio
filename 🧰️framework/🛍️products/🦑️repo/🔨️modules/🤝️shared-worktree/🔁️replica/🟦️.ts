import {createHash,randomBytes} from "node:crypto";
import {createReadStream,watch,type Stats} from "node:fs";
import {chmod,copyFile,lstat,mkdir,open,opendir,readdir,readFile,readlink,rename,rm,rmdir,stat,symlink,truncate,unlink,writeFile} from "node:fs/promises";
import {basename,dirname,join,sep} from "node:path";
import {compileJsonSchemaSubset,type CompiledJsonSchemaSubset} from "../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {appendEventsV1,blobPathV1,ensureStoreDirectoryV1,foldEventsV1,markStoreSeenV1,formatHlcV1,listOriginsV1,mergeClockV1,parseHlcV1,readEventSpansV1,reclaimStagingV1,readHeadV1,readNewestSnapshotV1,requireOriginIdV1,streamMembersV1,storePathsV1,tickClockV1,winsV1,writeBlobV1,writeSnapshotV1,type FleetClockV1,type FleetCursorV1,type FleetEventV1,type FleetHeadV1,type FleetManifestEntryV1} from "../📒️journal/🟦️.ts";
import {compileIgnoreV1,hashFileV1,linkRefusalV1,refusePathV1,scanSharedV1,sharedV1,type FleetIgnoreV1,type FleetScanEntryV1} from "../🚧️boundary/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🎚️ The explicit inputs of every replica operation: both directories, the identity of this machine and the tunables that default to the topology's `replica` block. */
export type ReplicaOptionsV1=Readonly<{worktree:string;storeRoot:string;originId:string;groupId:string;pollMilliseconds?:number;scanSeconds?:number;debounceMilliseconds?:number;retentionDays?:number;concurrency?:number;persistMilliseconds?:number;lock?:boolean;now?:()=>number}>;

/** 📡️ One NDJSON progress line, shaped by `ProgressEvent` of the schema. */
export type ReplicaEventV1=Readonly<{version:1;event:"started"|"stopped"|"scan"|"progress"|"published"|"applied"|"overwritten"|"kept"|"refused"|"skipped"|"blocked"|"unstable"|"store-unreachable"|"store-reachable"|"ignore-reloaded"|"clock-refused"|"clock-skew"|"blob-missing"|"rejoin-required"|"relayed"|"snapshot"|"compacted"|"error";stage?:string;path?:string;origin?:string;originId?:string;groupId?:string;hlc?:string;reason?:string;message?:string;lost?:string;count?:number;done?:number;total?:number;bytes?:number;seconds?:number}>;

/** 🚰️ Receives progress events; it must not throw. */
export type ReplicaSinkV1=(event:ReplicaEventV1)=>void;

type Say=(event:Omit<ReplicaEventV1,"version">)=>void;
type Settings=Readonly<{worktree:string;storeRoot:string;originId:string;groupId:string;pollMilliseconds:number;scanSeconds:number;debounceMilliseconds:number;retentionDays:number;concurrency:number;persistMilliseconds:number;now:()=>number}>;
type StateEntry=FleetManifestEntryV1&Readonly<{mtimeMs:number;seenMs?:number}>;
type Machine=Readonly<{version:1;originId:string;groupId:string;storeRoot:string}>;
type Drift=Readonly<{path:string;kind:"missing-local"|"extra-local"|"content"|"target"|"state";expected?:string;actual?:string}>;

/** 🔁️ An opened replica: the loaded local state, the folded store view and the single-daemon lock; the loops mutate it in place and `closeReplicaV1` persists and unlocks. */
export type ReplicaV1={
 readonly settings:Settings;
 readonly paths:LocalPathsV1;
 readonly book:ReplicaBookV1;
 readonly files:CompactTableV1;
 readonly queue:ReplicaQueueV1;
 readonly manifest:CompactTableV1;
 readonly cursors:Record<string,FleetCursorV1>;
 readonly folds:ReplicaFoldsV1;
 readonly noted:Set<string>;
 readonly changed:Set<number>;
 readonly held:Map<string,number>;
 readonly say:Say;
 readonly release:()=>Promise<void>;
 clock:FleetClockV1;
 ignore:FleetIgnoreV1;
 ignoreHash:string;
 dirty:boolean;
 persistedAt:number;
 homeWritten:boolean;
 generation:number;
 logBytes:number;
};

/** 🗄️ The local state files below `<worktree>/.cache/f/`. */
export type LocalPathsV1=Readonly<{directory:string;machine:string;state:string;log:string;cursor:string;clock:string;lost:string;tmp:string;lock:string;gitignore:string}>;

/** 🚶️ What one walk of the tree saw: the entries that differ from the recorded state (the only ones a publish needs), a bit per known path that was seen, the number of shared entries and the refused paths. */
export type WalkV1=Readonly<{changed:ReplicaFoundV1;seen:Readonly<{has(id:number):boolean}>;universe:number;scanned:number;refused:readonly Readonly<{path:string;reason:string}>[]}>;

/** 🧾️ What one publish round did: events written, files whose observation was refreshed without an event, paths that were unstable and should be retried. */
export type PublishResultV1=Readonly<{scanned:number;published:number;deleted:number;touched:number;bytes:number;retry:readonly string[]}>;

/** 📬️ What one follow round did: events read, paths materialised, local edits that were newer than the remote event and were published instead, paths still waiting. */
export type FollowResultV1=Readonly<{events:number;applied:number;dropped:number;kept:number;queued:number}>;

/** 🧳️ What a join materialised and what is still queued. */
export type JoinResultV1=Readonly<{files:number;adopted:number;queued:number}>;

/** 🗃️ The store folded to one manifest: the newest snapshot plus every journal event after it. */
export type StoreFoldV1=Readonly<{manifest:CompactTableV1;cursors:Record<string,FleetCursorV1>;hlc:string}>;

/** 🧭️ One disagreement between the local tree, the local state and the folded store. */
export type DriftV1=Drift;

/** 📋️ The report of `verifyV1`, shaped by `VerifyReport` of the schema. */
export type VerifyReportV1=Readonly<{version:1;ok:boolean;checked:number;total:number;truncated:boolean;drift:readonly Drift[];refusedTotal:number;refused:readonly Readonly<{path:string;reason:string}>[]}>;

/** 📊️ The report of `statusV1`, shaped by `Status` of the schema. */
export type StatusReportV1=Readonly<{version:1;originId:string;groupId:string;storeRoot:string;reachable:boolean;daemon:Readonly<{running:boolean;pid:number|null}>;files:number;queue:Readonly<{incoming:number;outgoing:number|null}>;lost:number;refused:number|null;origins:readonly Readonly<{origin:string;behindEvents:number;behindBytes:number;lastHlc:string|null;ageSeconds:number|null;rejoinRequired:boolean}>[]}>;

/** 🎞️ A written snapshot. */
export type SnapshotResultV1=Readonly<{file:string;hlc:string;entries:number;created:boolean}>;

/** 🧹️ What a compaction removed. */
export type CompactResultV1=Readonly<{snapshot:SnapshotResultV1|undefined;segments:number;blobs:number;snapshots:number;staging:number;bytes:number}>;

/** 🚚️ What one relay round copied. */
export type RelayResultV1=Readonly<{origins:number;blobs:number;segments:number;bytes:number}>;

/** 🛰️ The inputs of the relay loop: the own store, the read-only peer stores by group id and the poll interval. */
export type RelayOptionsV1=Readonly<{ownStore:string;peers:Readonly<Record<string,string>>;pollMilliseconds?:number;concurrency?:number}>;

const windows=process.platform==="win32",originPattern=/^[a-z0-9][a-z0-9-]{0,39}$/u,segmentPattern=/^[0-9]{12}\.jsonl$/u,hexPattern=/^[0-9a-f]{64}$/u,chunkSize=256,driftLimit=1000,futureMilliseconds=60_000,racyMilliseconds=2000,staleStagingMilliseconds=6*3_600_000,collectBatch=2048,logLimitBytes=16*1024*1024,writeChunk=1<<20,checkpointChanges=50_000,paceEntries=2000,heapRounds=20,relayConcurrency=8,stateBytesPerEntry=320;
const code=(error:unknown):string|undefined=>(error as {code?:string}).code;
const gone=(error:unknown):boolean=>code(error)==="ENOENT"||code(error)==="ENOTDIR";
const sharing=(error:unknown):boolean=>code(error)==="EBUSY"||code(error)==="EPERM"||code(error)==="EACCES";
const messageOf=(error:unknown):string=>error instanceof Error?error.message:String(error);
const digest=(text:string):string=>createHash("sha256").update(text).digest("hex");
const token=():string=>randomBytes(8).toString("hex");
const sleep=(milliseconds:number):Promise<void>=>new Promise(done=>{setTimeout(done,milliseconds);});
const absolute=(root:string,path:string):string=>join(root,...path.split("/"));
const listed=async(root:string,path:string):Promise<boolean>=>(await readdir(dirname(absolute(root,path))).catch(()=>[] as string[])).some(name=>name.normalize("NFC")===basename(path));
const segmentName=(segment:number):string=>`${String(segment).padStart(12,"0")}.jsonl`;
const plain=(entry:FleetManifestEntryV1):FleetManifestEntryV1=>entry.operation==="put"?{hlc:entry.hlc,operation:"put",sha256:entry.sha256!,bytes:entry.bytes!,executable:entry.executable!}:entry.operation==="link"?{hlc:entry.hlc,operation:"link",target:entry.target!}:{hlc:entry.hlc,operation:"delete"};
const sealed=(path:string):boolean=>path===".cache/f"||path.startsWith(".cache/f/");
const guarded=(ignore:FleetIgnoreV1):FleetIgnoreV1=>({digest:ignore.digest,ignores:(path,isDirectory)=>sealed(path)||ignore.ignores(path,isDirectory),child:(path,isDirectory)=>sealed(path)||(ignore.child??ignore.ignores)(path,isDirectory)});
const racy=(entry:StateEntry):boolean=>entry.seenMs===undefined||entry.seenMs-entry.mtimeMs<racyMilliseconds;
const unsafePath=(path:string):boolean=>path===""||path.split("/").some(part=>part===""||part==="."||part==="..")||path.includes("\\")||path.startsWith("/");

function widen<T extends Uint8Array|Uint16Array|Uint32Array|Float64Array|Int32Array>(array:T,length:number):T{
 const next=new (array.constructor as new(length:number)=>T)(length);
 next.set(array);
 if(next.byteLength>=settleBytes)queueMicrotask(settleHeap);
 return next;
}

const settleBytes=2<<20;
const hexValue=(code:number):number=>code>=48&&code<=57?code-48:code>=97&&code<=102?code-87:-1;

const hashText=(text:string):number=>{
 let hash=2166136261;
 for(let index=0;index<text.length;index++){hash^=text.charCodeAt(index);hash=Math.imul(hash,16777619);}
 return hash>>>0;
};

const encoder=new TextEncoder(),pageBytes=1<<20;

/** 📇️ The path table of one replica: every path ever seen gets one small integer id, and all tables of the replica store their columns by that id. The paths themselves live as UTF-8 in 1 MiB pages behind an open-addressing hash table of ids, so a path costs its bytes and a few table words, not a JavaScript string, and growing never copies the text. */
export class ReplicaBookV1{
 readonly origins:string[]=[];
 readonly originIds=new Map<string,number>();
 private lastOrigin=-1;
 count=0;
 private pages:Buffer[]=[];
 private used=pageBytes;
 private starts=new Float64Array(1025);
 private lengths=new Uint16Array(1024);
 private hashes=new Uint32Array(1024);
 private folded=new Uint32Array(1024);
 private slots=new Int32Array(2048);
 private scratch=new Uint8Array(1024);
 private encode(path:string):number{
  if(this.scratch.length<path.length*3)this.scratch=new Uint8Array(path.length*3);
  return encoder.encodeInto(path,this.scratch).written;
 }
 private hashOf(length:number):number{
  let hash=2166136261;
  for(let index=0;index<length;index++){hash^=this.scratch[index]!;hash=Math.imul(hash,16777619);}
  return hash>>>0;
 }
 private matches(id:number,length:number):boolean{
  if(this.lengths[id]!==length)return false;
  const start=this.starts[id]!,page=this.pages[Math.floor(start/pageBytes)]!,from=start%pageBytes;
  for(let index=0;index<length;index++)if(page[from+index]!==this.scratch[index])return false;
  return true;
 }
 private locate(hash:number,length:number):number{
  const mask=this.slots.length-1;
  let slot=hash&mask;
  for(;;){
   const held=this.slots[slot]!;
   if(held===0)return slot;
   const id=held-1;
   if(this.hashes[id]===hash&&this.matches(id,length))return slot;
   slot=(slot+1)&mask;
  }
 }
 find(path:string):number|undefined{
  const length=this.encode(path),held=this.slots[this.locate(this.hashOf(length),length)]!;
  return held===0?undefined:held-1;
 }
 intern(path:string):number{
  const length=this.encode(path),hash=this.hashOf(length),slot=this.locate(hash,length),held=this.slots[slot]!;
  if(held!==0)return held-1;
  if(length>=pageBytes||!path.isWellFormed())throw Error("Closed replica path required");
  const id=this.count;
  if(this.used+length>pageBytes){this.pages.push(Buffer.alloc(pageBytes));this.used=0;}
  const page=this.pages[this.pages.length-1]!;
  page.set(this.scratch.subarray(0,length),this.used);
  if(id+2>this.starts.length){
   const size=Math.ceil(this.starts.length*1.5);
   this.starts=widen(this.starts,size);this.lengths=widen(this.lengths,size-1);this.hashes=widen(this.hashes,size-1);this.folded=widen(this.folded,size-1);
  }
  this.starts[id]=(this.pages.length-1)*pageBytes+this.used;this.lengths[id]=length;this.hashes[id]=hash;this.folded[id]=hashText(path.toLowerCase());
  this.used+=length;this.count=id+1;
  this.slots[slot]=id+1;
  if(this.count*2>this.slots.length)this.rehash();
  return id;
 }
 private rehash():void{
  const slots=new Int32Array(this.slots.length*2),mask=slots.length-1;
  for(let id=0;id<this.count;id++){
   let slot=this.hashes[id]!&mask;
   while(slots[slot]!==0)slot=(slot+1)&mask;
   slots[slot]=id+1;
  }
  this.slots=slots;
 }
 reserve(count:number):void{
  if(count+2>this.starts.length){
   this.starts=widen(this.starts,count+2);this.lengths=widen(this.lengths,count+1);this.hashes=widen(this.hashes,count+1);this.folded=widen(this.folded,count+1);
  }
  let size=this.slots.length;
  while(size<count*2+2)size*=2;
  if(size>this.slots.length){this.slots=new Int32Array(size);for(let id=0;id<this.count;id++){let slot=this.hashes[id]!&(size-1);while(this.slots[slot]!==0)slot=(slot+1)&(size-1);this.slots[slot]=id+1;}}
 }
 foldHash(id:number):number{return this.folded[id]!;}
 name(id:number):string{
  const start=this.starts[id]!,from=start%pageBytes;
  return this.pages[Math.floor(start/pageBytes)]!.toString("utf8",from,from+this.lengths[id]!);
 }
 originOf(hlc:string):number{
  const last=this.lastOrigin;
  if(last>=0){const name=this.origins[last]!;if(hlc.length-21===name.length&&hlc.startsWith(name,21))return last;}
  return this.lastOrigin=this.origin(hlc.slice(21));
 }
 origin(name:string):number{
  let id=this.originIds.get(name);
  if(id===undefined){
   if(this.origins.length>=65536)throw Error("Closed replica origin count required");
   id=this.origins.length;this.originIds.set(name,id);this.origins.push(name);
  }
  return id;
 }
}

/** 🧊️ A `Map<path, entry>` stored as typed columns instead of one object and two strings per path: hlc as milliseconds, counter and origin index, sha256 as 32 raw bytes, sizes and times as numbers. `get` builds a plain entry on demand; the `*At(id)` readers never allocate. */
export class CompactTableV1{
 flags=new Uint8Array(0);
 ms=new Float64Array(0);
 counter=new Uint32Array(0);
 origin=new Uint16Array(0);
 bytes=new Float64Array(0);
 sha=new Uint8Array(0);
 mtime=new Float64Array(0);
 seen=new Float64Array(0);
 readonly targets=new Map<number,string>();
 size=0;
 constructor(readonly book:ReplicaBookV1,readonly local:boolean){}
 reserve(count:number):void{this.grow(count,count);}
 private grow(need:number,exact?:number):void{
  if(need<=this.flags.length)return;
  const length=exact??Math.max(need,Math.ceil(this.flags.length*1.5),1024);
  this.flags=widen(this.flags,length);this.ms=widen(this.ms,length);this.counter=widen(this.counter,length);this.origin=widen(this.origin,length);this.bytes=widen(this.bytes,length);this.sha=widen(this.sha,length*32);
  if(this.local){this.mtime=widen(this.mtime,length);this.seen=widen(this.seen,length);}
 }
 live(id:number):boolean{return id<this.flags.length&&(this.flags[id]!&1)===1;}
 operationAt(id:number):"put"|"delete"|"link"|undefined{
  if(!this.live(id))return undefined;
  const code=(this.flags[id]!>>1)&3;
  return code===1?"put":code===2?"delete":"link";
 }
 executableAt(id:number):boolean{return (this.flags[id]!&8)!==0;}
 racyAt(id:number):boolean{return (this.flags[id]!&16)===0||this.seen[id]!-this.mtime[id]!<racyMilliseconds;}
 hlcAt(id:number):string{return `${String(this.ms[id]).padStart(13,"0")}-${String(this.counter[id]).padStart(6,"0")}-${this.book.origins[this.origin[id]!]}`;}
 sameHlcAs(other:CompactTableV1,id:number):boolean{return this.ms[id]===other.ms[id]&&this.counter[id]===other.counter[id]&&this.origin[id]===other.origin[id];}
 shaAt(id:number):string{return Buffer.from(this.sha.buffer,this.sha.byteOffset+id*32,32).toString("hex");}
 shaIs(id:number,hex:string):boolean{return Buffer.from(this.sha.buffer,this.sha.byteOffset+id*32,32).equals(Buffer.from(hex,"hex"));}
 entryAt(id:number):StateEntry{
  const flag=this.flags[id]!,code=(flag>>1)&3,hlc=this.hlcAt(id);
  const entry:Record<string,unknown>=code===1?{hlc,operation:"put",sha256:this.shaAt(id),bytes:this.bytes[id],executable:(flag&8)!==0}:code===3?{hlc,operation:"link",target:this.targets.get(id)}:{hlc,operation:"delete"};
  if(this.local){entry.mtimeMs=this.mtime[id];if((flag&16)!==0)entry.seenMs=this.seen[id];}
  return entry as StateEntry;
 }
 write(id:number,entry:FleetManifestEntryV1&Partial<Readonly<{mtimeMs:number;seenMs:number}>>):void{
  this.grow(id+1);
  const hlc=entry.hlc;
  let ms=0,counter=0,valid=hlc.length>=22&&hlc.charCodeAt(13)===45&&hlc.charCodeAt(20)===45;
  for(let index=0;valid&&index<13;index++){const digit=hlc.charCodeAt(index)-48;if(digit<0||digit>9)valid=false;else ms=ms*10+digit;}
  for(let index=14;valid&&index<20;index++){const digit=hlc.charCodeAt(index)-48;if(digit<0||digit>9)valid=false;else counter=counter*10+digit;}
  if(!valid)throw Error(`Closed replica hlc required: ${hlc}`);
  let flag=1;
  if(entry.operation==="put"){
   const hex=entry.sha256;
   if(typeof hex!=="string"||hex.length!==64||typeof entry.bytes!=="number")throw Error("Closed replica entry required: put");
   const base=id*32;
   for(let index=0;index<32;index++){
    const high=hexValue(hex.charCodeAt(index*2)),low=hexValue(hex.charCodeAt(index*2+1));
    if(high<0||low<0)throw Error("Closed replica entry required: put");
    this.sha[base+index]=high*16+low;
   }
   this.bytes[id]=entry.bytes;this.targets.delete(id);
   flag|=2|(entry.executable===true?8:0);
  }else if(entry.operation==="link"){
   if(typeof entry.target!=="string"||entry.target==="")throw Error("Closed replica entry required: link");
   this.targets.set(id,entry.target);this.bytes[id]=0;flag|=6;
  }else{this.targets.delete(id);this.bytes[id]=0;flag|=4;}
  if(this.local){
   this.mtime[id]=entry.mtimeMs??0;
   if(entry.seenMs!==undefined){flag|=16;this.seen[id]=entry.seenMs;}
  }
  if(!this.live(id))this.size++;
  this.flags[id]=flag;this.ms[id]=ms;this.counter[id]=counter;this.origin[id]=this.book.originOf(hlc);
 }
 copyRow(from:CompactTableV1,id:number):void{
  this.grow(id+1);
  if(!this.live(id))this.size++;
  this.flags[id]=(from.flags[id]!&~16)|(this.local&&from.local?from.flags[id]!&16:0);
  this.ms[id]=from.ms[id]!;this.counter[id]=from.counter[id]!;this.origin[id]=from.origin[id]!;this.bytes[id]=from.bytes[id]!;
  for(let index=id*32,end=index+32;index<end;index++)this.sha[index]=from.sha[index]!;
  const target=from.targets.get(id);
  if(target===undefined)this.targets.delete(id);else this.targets.set(id,target);
  if(this.local&&from.local){this.mtime[id]=from.mtime[id]!;this.seen[id]=from.seen[id]!;}
 }
 clearId(id:number):boolean{
  if(!this.live(id))return false;
  this.flags[id]=0;this.targets.delete(id);this.size--;
  return true;
 }
 clear():void{this.flags.fill(0);this.targets.clear();this.size=0;}
 idOf(path:string):number|undefined{return this.book.find(path);}
 has(path:string):boolean{const id=this.book.find(path);return id!==undefined&&this.live(id);}
 get(path:string):StateEntry|undefined{const id=this.book.find(path);return id===undefined||!this.live(id)?undefined:this.entryAt(id);}
 set(path:string,entry:FleetManifestEntryV1&Partial<Readonly<{mtimeMs:number;seenMs:number}>>):this{this.write(this.book.intern(path),entry);return this;}
 delete(path:string):boolean{const id=this.book.find(path);return id===undefined?false:this.clearId(id);}
 ids():Int32Array{
  const ids=new Int32Array(this.size);
  let count=0;
  for(let id=0;id<this.flags.length&&count<ids.length;id++)if((this.flags[id]!&1)===1)ids[count++]=id;
  return ids;
 }
 *keys():Generator<string>{for(const id of this.ids())yield this.book.name(id);}
 *values():Generator<StateEntry>{for(const id of this.ids())yield this.entryAt(id);}
 *entries():Generator<[string,StateEntry]>{for(const id of this.ids())yield [this.book.name(id),this.entryAt(id)];}
 [Symbol.iterator]():Generator<[string,StateEntry]>{return this.entries();}
 count(operation:"put"|"delete"|"link"):number{let count=0;for(const id of this.ids())if(this.operationAt(id)===operation)count++;return count;}
}

/** 📥️ The store winners that the tree does not hold yet; the entry of a queued path is its row in the manifest table, so the queue itself costs one byte per path. */
export class ReplicaQueueV1{
 flags=new Uint8Array(0);
 size=0;
 constructor(readonly book:ReplicaBookV1,readonly manifest:CompactTableV1){}
 reserve(count:number):void{if(count>this.flags.length)this.flags=widen(this.flags,count);}
 hasId(id:number):boolean{return id<this.flags.length&&this.flags[id]===1;}
 has(path:string):boolean{const id=this.book.find(path);return id!==undefined&&this.hasId(id);}
 get(path:string):StateEntry|undefined{return this.has(path)?this.manifest.get(path):undefined;}
 addId(id:number):boolean{
  if(id>=this.flags.length)this.flags=widen(this.flags,Math.max(id+1,Math.ceil(this.flags.length*1.5),1024));
  if(this.flags[id]===1)return false;
  this.flags[id]=1;this.size++;return true;
 }
 removeId(id:number):boolean{
  if(!this.hasId(id))return false;
  this.flags[id]=0;this.size--;return true;
 }
 snapshot():Int32Array{
  const ids=new Int32Array(this.size);
  let count=0;
  for(let id=0;id<this.flags.length&&count<ids.length;id++)if(this.flags[id]===1)ids[count++]=id;
  return ids;
 }
 *keys():Generator<string>{for(const id of this.snapshot())yield this.book.name(id);}
 *entries():Generator<[string,StateEntry]>{for(const id of this.snapshot())yield [this.book.name(id),this.manifest.entryAt(id)];}
 [Symbol.iterator]():Generator<[string,StateEntry]>{return this.entries();}
}

/** 🔠️ Finds the live path that equals a given one when case is ignored, without keeping a lower-cased copy of every path: a table of ids keyed by the 32-bit hash of the folded name that the path table stored when it met the path, whose candidates are confirmed by folding their real name only when the hashes agree. */
export class ReplicaFoldsV1{
 private slots=new Int32Array(2048);
 private used=0;
 private readonly dirs=new Int32Array(1<<20);
 constructor(readonly book:ReplicaBookV1){}
 private count(id:number,step:number):void{
  const name=this.book.name(id),mask=this.dirs.length-1;
  let hash=2166136261;
  for(let index=0;index<name.length;index++){
   const code=name.charCodeAt(index);
   if(code===47)this.dirs[(hash>>>0)&mask]+=step;
   hash=Math.imul(hash^code,16777619);
  }
 }
 below(directory:string):number{
  let hash=2166136261;
  for(let index=0;index<directory.length;index++)hash=Math.imul(hash^directory.charCodeAt(index),16777619);
  return this.dirs[(hash>>>0)&(this.dirs.length-1)]!;
 }
 reserve(count:number):void{
  let size=this.slots.length;
  while(size<count*2+2)size*=2;
  if(size>this.slots.length)this.slots=new Int32Array(size);
 }
 find(path:string):string|undefined{
  const fold=path.toLowerCase(),hash=hashText(fold),mask=this.slots.length-1;
  for(let slot=hash&mask;;slot=(slot+1)&mask){
   const held=this.slots[slot]!;
   if(held===0)return undefined;
   if(held>0&&this.book.foldHash(held-1)===hash&&this.book.name(held-1).toLowerCase()===fold)return this.book.name(held-1);
  }
 }
 add(id:number):void{
  const hash=this.book.foldHash(id),mask=this.slots.length-1;
  let fold:string|undefined,reusable=-1,slot=hash&mask;
  for(;;slot=(slot+1)&mask){
   const held=this.slots[slot]!;
   if(held===0)break;
   if(held<0){if(reusable<0)reusable=slot;continue;}
   if(this.book.foldHash(held-1)!==hash)continue;
   if(held-1===id){return;}
   if((fold??=this.book.name(id).toLowerCase())===this.book.name(held-1).toLowerCase()){this.slots[slot]=id+1;this.count(id,1);return;}
  }
  if(reusable>=0){this.slots[reusable]=id+1;this.count(id,1);return;}
  if((this.used+1)*2>this.slots.length){this.grow();this.add(id);return;}
  this.used++;this.slots[slot]=id+1;this.count(id,1);
 }
 remove(id:number):void{
  const hash=this.book.foldHash(id),mask=this.slots.length-1;
  for(let slot=hash&mask;;slot=(slot+1)&mask){
   const held=this.slots[slot]!;
   if(held===0)return;
   if(held===id+1){this.slots[slot]=-1;this.count(id,-1);return;}
  }
 }
 private grow():void{
  const old=this.slots;
  this.slots=new Int32Array(old.length*2);this.used=0;
  const mask=this.slots.length-1;
  for(const held of old){
   if(held<=0)continue;
   let slot=this.book.foldHash(held-1)&mask;
   while(this.slots[slot]!==0)slot=(slot+1)&mask;
   this.slots[slot]=held;this.used++;
  }
 }
}

/** 🗒️ A list of scan entries stored as columns of ids, sizes and times in the path table instead of one object and one string per entry. */
export class ReplicaFoundV1{
 ids=new Int32Array(1024);
 bytes=new Float64Array(1024);
 mtime=new Float64Array(1024);
 flags=new Uint8Array(1024);
 readonly targets=new Map<number,string>();
 length=0;
 constructor(readonly book:ReplicaBookV1){}
 push(entry:FleetScanEntryV1):void{
  const index=this.length++;
  if(index>=this.ids.length){
   const size=Math.ceil(this.ids.length*1.5);
   this.ids=widen(this.ids,size);this.bytes=widen(this.bytes,size);this.mtime=widen(this.mtime,size);this.flags=widen(this.flags,size);
  }
  this.ids[index]=this.book.intern(entry.path);this.bytes[index]=entry.bytes;this.mtime[index]=entry.mtimeMs;
  this.flags[index]=(entry.kind==="link"?1:0)|(entry.executable?2:0);
  if(entry.target!==undefined)this.targets.set(index,entry.target);
 }
 at(index:number):FleetScanEntryV1{
  const flag=this.flags[index]!,base={path:this.book.name(this.ids[index]!),bytes:this.bytes[index]!,mtimeMs:this.mtime[index]!,executable:(flag&2)!==0};
  return (flag&1)!==0?{...base,kind:"link",target:this.targets.get(index)}:{...base,kind:"file"};
 }
}

class Bits{
 bytes=new Uint8Array(0);
 mark(id:number):void{
  if(id>=this.bytes.length)this.bytes=widen(this.bytes,Math.max(id+1,this.bytes.length*2,1024));
  this.bytes[id]=1;
 }
 has(id:number):boolean{return id<this.bytes.length&&this.bytes[id]===1;}
}

const settleHeap=():void=>{(globalThis as {Bun?:{gc?:(force:boolean)=>void}}).Bun?.gc?.(true);};
const pacer=(every:number):(()=>void)=>{let count=0;return ()=>{if(++count%every===0)settleHeap();};};

/** ⚖️ Decides a conflict between an unpublished local edit and a remote event for the same path: the local edit wins only when its modification time is later than the wall time inside the remote hlc; a local delete has no modification time and always loses. */
export function conflictWinnerV1(localMtimeMs:number|undefined,remoteHlc:string):"local"|"remote"{
 return localMtimeMs!==undefined&&localMtimeMs>parseHlcV1(remoteHlc).ms?"local":"remote";
}

const replicaValidators=new Map<string,CompiledJsonSchemaSubset>();

/** 🛡️ Validates one document against a named definition of the replica schema and names the first problem. */
export function requireReplicaShapeV1(definition:"Machine"|"State"|"StateLogLine"|"Cursor"|"Clock"|"Lock"|"Home"|"ProgressEvent"|"Status"|"VerifyReport",value:unknown,message:string):void{
 let validator=replicaValidators.get(definition);
 if(validator===undefined){validator=compileJsonSchemaSubset({$ref:`#/definitions/${definition}`},schema);replicaValidators.set(definition,validator);}
 const errors=validator.validate(value);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
}

/** 🗄️ Resolves the machine-local state files of a worktree. */
export function localPathsV1(worktree:string):LocalPathsV1{
 const directory=join(worktree,".cache","f");
 return {directory,machine:join(directory,"machine.json"),state:join(directory,"state.json"),log:join(directory,"state.log"),cursor:join(directory,"cursor.json"),clock:join(directory,"clock.json"),lost:join(directory,"lost"),tmp:join(directory,"tmp"),lock:join(directory,"daemon.lock"),gitignore:join(worktree,".gitignore")};
}

/** 📶️ Tells whether the store directory answers; an unmounted export leaves an empty mount point and counts as unreachable. */
export async function storeReachableV1(storeRoot:string):Promise<boolean>{
 try{const present=(await stat(storePathsV1(storeRoot).root)).isDirectory();if(present)markStoreSeenV1(storeRoot);return present;}catch{return false;}
}

const indexes=(length:number):Int32Array=>Int32Array.from({length},(_,index)=>index);

async function pool<T>(items:ArrayLike<T>,limit:number,work:(item:T,index:number)=>Promise<void>,signal?:AbortSignal):Promise<void>{
 let next=0;
 await Promise.all(Array.from({length:Math.min(Math.max(1,limit),items.length)},async()=>{
  while(next<items.length){signal?.throwIfAborted();const index=next++;await work(items[index]!,index);}
 }));
}

async function readJson(file:string):Promise<unknown>{
 try{return JSON.parse(await readFile(file,"utf8")) as unknown;}catch(error){if(gone(error))return undefined;throw error;}
}

async function writeJson(paths:LocalPathsV1,file:string,value:unknown):Promise<void>{
 await mkdir(paths.tmp,{recursive:true});
 const temporary=join(paths.tmp,token());
 try{await writeFile(temporary,JSON.stringify(value));await rename(temporary,file);}
 catch(error){await rm(temporary,{force:true});throw error;}
}

async function readIgnoreText(paths:LocalPathsV1):Promise<string>{
 try{return await readFile(paths.gitignore,"utf8");}catch(error){if(gone(error))return "";throw error;}
}

/** 🔒️ Raised when another live daemon already holds `daemon.lock` of the worktree. */
export class ReplicaBusyV1 extends Error{
 readonly pid:number;
 constructor(pid:number,file:string){super(`Another replica daemon (pid ${pid}) holds ${file}`);this.name="ReplicaBusyV1";this.pid=pid;}
}

const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return code(error)==="EPERM";}};

async function readLock(file:string):Promise<Readonly<{pid:number;stale:boolean}>|undefined>{
 let text:string;
 try{text=await readFile(file,"utf8");}catch(error){if(gone(error))return undefined;throw error;}
 try{
  const value=JSON.parse(text) as {pid?:unknown;startedMs?:unknown};
  if(typeof value.pid==="number"&&Number.isSafeInteger(value.pid)&&value.pid>0)return {pid:value.pid,stale:value.pid===process.pid?typeof value.startedMs!=="number"||value.startedMs<Date.now()-process.uptime()*1000-1000:!alive(value.pid)};
 }catch{}
 const info=await stat(file).catch(()=>undefined);
 return {pid:0,stale:info===undefined||Date.now()-info.mtimeMs>5000};
}

async function acquireLock(paths:LocalPathsV1,originId:string):Promise<()=>Promise<void>>{
 await mkdir(paths.directory,{recursive:true});
 const body=JSON.stringify({version:1,pid:process.pid,startedMs:Date.now(),originId});
 for(let attempt=0;attempt<6;attempt++){
  try{
   const handle=await open(paths.lock,"wx");
   try{await handle.writeFile(body);await handle.sync();}finally{await handle.close();}
   return async()=>{const held=await readLock(paths.lock);if(held?.pid===process.pid)await rm(paths.lock,{force:true});};
  }catch(error){
   if(code(error)!=="EEXIST")throw error;
   const held=await readLock(paths.lock);
   if(held===undefined)continue;
   if(held.stale){
    const aside=`${paths.lock}.stale-${token()}`;
    try{await rename(paths.lock,aside);await rm(aside,{force:true});}catch{}
    continue;
   }
   if(held.pid===0){await sleep(50);continue;}
   throw new ReplicaBusyV1(held.pid,paths.lock);
  }
 }
 throw new ReplicaBusyV1(0,paths.lock);
}

type Tables=Readonly<{book:ReplicaBookV1;files:CompactTableV1;manifest:CompactTableV1;queue:ReplicaQueueV1}>;
type Local=Readonly<{machine:Machine|undefined;cursors:Record<string,FleetCursorV1>;clock:FleetClockV1;generation:number;log:Readonly<{size:number;good:number}>}>;

function tablesV1():Tables{
 const book=new ReplicaBookV1(),manifest=new CompactTableV1(book,false);
 return {book,files:new CompactTableV1(book,true),manifest,queue:new ReplicaQueueV1(book,manifest)};
}

function storeEntry(tables:Tables,kind:"files"|"queue",path:string,entry:StateEntry|null,keep:boolean):void{
 if(kind==="files"){
  if(entry===null)tables.files.delete(path);else tables.files.set(path,entry);
  return;
 }
 const id=tables.book.intern(path);
 if(entry===null){tables.queue.removeId(id);return;}
 if(keep)tables.manifest.write(id,entry);
 tables.queue.addId(id);
}

async function replayLog(paths:LocalPathsV1,generation:number,tables:Tables,keep:boolean):Promise<Readonly<{size:number;good:number}>>{
 let data:Buffer;
 try{data=await readFile(paths.log);}catch(error){if(gone(error))return {size:0,good:0};throw error;}
 const good=data.lastIndexOf(10)+1,lines=data.toString("utf8",0,good).split("\n");
 lines.pop();
 if(lines.length===0)return {size:data.length,good:0};
 const parse=(line:string,number:number):Record<string,unknown>=>{
  try{return JSON.parse(line) as Record<string,unknown>;}catch{throw Error(`Closed replica state log required: line ${number}`);}
 };
 if(parse(lines[0]!,1).generation!==generation)return {size:data.length,good:0};
 for(let index=1;index<lines.length;index++){
  const value=parse(lines[index]!,index+1);
  if((value.kind!=="files"&&value.kind!=="queue")||typeof value.path!=="string"||value.path===""||typeof value.entry!=="object")throw Error(`Closed replica state log required: line ${index+1}`);
  storeEntry(tables,value.kind,value.path,value.entry as StateEntry|null,keep);
 }
 return {size:data.length,good};
}

async function loadState(paths:LocalPathsV1,tables:Tables,keep:boolean):Promise<number>{
 const expected=Math.ceil(((await stat(paths.state).catch(()=>undefined))?.size??0)/stateBytesPerEntry);
 if(expected>0){tables.book.reserve(expected);tables.files.reserve(expected);if(keep)tables.manifest.reserve(expected);tables.queue.reserve(expected);}
 let version:unknown,generation:unknown,sawFiles=false,sawQueue=false;
 const pace=pacer(paceEntries);
 try{
  await streamMembersV1(paths.state,new Set(["files","queue"]),(outer,inner,text)=>{
   if(inner===undefined){
    const value=JSON.parse(text) as unknown;
    if(outer==="version")version=value;
    else if(outer==="generation")generation=value;
    else if(outer==="files"||outer==="queue")throw Error("Closed replica state required: state.json");
    return;
   }
   pace();
   if(outer==="files")storeEntry(tables,"files",inner,JSON.parse(text) as StateEntry,keep);
   else if(outer==="queue")storeEntry(tables,"queue",inner,JSON.parse(text) as StateEntry,keep);
  },outer=>{if(outer==="files")sawFiles=true;else sawQueue=true;});
 }catch(error){
  if(gone(error))return 0;
  if((error as Error).message.startsWith("Closed JSON object required"))throw Error("Closed replica state required: state.json");
  throw error;
 }
 if(version!==1||typeof generation!=="number"||!sawFiles||!sawQueue)throw Error("Closed replica state required: state.json");
 return generation;
}

async function loadLocal(paths:LocalPathsV1,tables:Tables,keep:boolean=false):Promise<Local>{
 const machine=await readJson(paths.machine);
 if(machine!==undefined)requireReplicaShapeV1("Machine",machine,"Closed replica machine required");
 const generation=await loadState(paths,tables,keep),log=await replayLog(paths,generation,tables,keep);
 const cursor=await readJson(paths.cursor);
 if(cursor!==undefined)requireReplicaShapeV1("Cursor",cursor,"Closed replica cursor required");
 const clock=await readJson(paths.clock) as {ms:number;counter:number}|undefined;
 if(clock!==undefined)requireReplicaShapeV1("Clock",clock,"Closed replica clock required");
 if(keep){
  for(const id of tables.manifest.ids())if(!tables.queue.hasId(id)&&!tables.files.live(id))tables.manifest.clearId(id);
  for(const id of tables.files.ids())if(!tables.queue.hasId(id))tables.manifest.copyRow(tables.files,id);
 }
 return {machine:machine as Machine|undefined,cursors:{...(cursor as {cursors:Record<string,FleetCursorV1>}|undefined)?.cursors},clock:{ms:clock?.ms??0,counter:clock?.counter??0},generation,log};
}

function settle(options:ReplicaOptionsV1):Settings{
 requireOriginIdV1(options.originId);
 if(!originPattern.test(options.groupId))throw Error(`Closed replica group id required: ${options.groupId}`);
 return {worktree:options.worktree,storeRoot:options.storeRoot,originId:options.originId,groupId:options.groupId,pollMilliseconds:options.pollMilliseconds??250,scanSeconds:options.scanSeconds??15,debounceMilliseconds:options.debounceMilliseconds??150,retentionDays:options.retentionDays??14,concurrency:options.concurrency??8,persistMilliseconds:options.persistMilliseconds??5000,now:options.now??Date.now};
}

const filesKey=(id:number):number=>id*2,queueKey=(id:number):number=>id*2+1;

function setFile(replica:ReplicaV1,path:string,entry:StateEntry):void{
 const id=replica.book.intern(path);
 replica.files.write(id,entry);
 if(entry.operation==="delete")replica.folds.remove(id);else replica.folds.add(id);
 replica.changed.add(filesKey(id));replica.dirty=true;
}

function forget(replica:ReplicaV1,path:string):void{
 const id=replica.book.find(path);
 if(id===undefined||!replica.files.live(id))return;
 replica.folds.remove(id);replica.files.clearId(id);
 replica.changed.add(filesKey(id));replica.dirty=true;
}

function enqueueId(replica:ReplicaV1,id:number):void{
 replica.queue.addId(id);replica.changed.add(queueKey(id));replica.dirty=true;
}

function dequeue(replica:ReplicaV1,path:string):void{
 const id=replica.book.find(path);
 if(id===undefined)return;
 replica.queue.removeId(id);replica.changed.add(queueKey(id));replica.dirty=true;
}

function requeue(replica:ReplicaV1):void{
 const {files,manifest,queue}=replica;
 for(const id of manifest.ids())if(!queue.hasId(id)&&(!files.live(id)||!files.sameHlcAs(manifest,id)))enqueueId(replica,id);
}

function noteOnce(replica:ReplicaV1,key:string,event:Omit<ReplicaEventV1,"version">):void{
 if(replica.noted.has(key))return;
 replica.noted.add(key);replica.say(event);
}

/** 🧵️ After a crash the own journal can be ahead of the persisted state (events appended, state not yet written): the events after the persisted own cursor are replayed into the state with an unknown modification time, so that the next scan rehashes those files and publishes a delete for each one that left the disk meanwhile. */
async function replayOwnTail(replica:ReplicaV1):Promise<void>{
 const {storeRoot,originId}=replica.settings,cursor=replica.cursors[originId];
 let read:Awaited<ReturnType<typeof readEventSpansV1>>;
 try{
  const head=await readHeadV1(storeRoot,originId);
  if(head===undefined||cursor!==undefined&&cursor.segment===head.segment&&cursor.bytes===head.bytes)return;
  read=await readEventSpansV1(storeRoot,originId,cursor);
 }catch{return;}
 for(const event of read.events){
  const known=replica.files.get(event.path);
  if(known!==undefined&&known.hlc>=event.hlc)continue;
  const entry:StateEntry=event.operation==="delete"?{hlc:event.hlc,operation:"delete",mtimeMs:0}:event.operation==="link"?{hlc:event.hlc,operation:"link",target:event.target!,mtimeMs:0}:{hlc:event.hlc,operation:"put",sha256:event.sha256!,bytes:event.bytes!,executable:event.executable!,mtimeMs:0};
  setFile(replica,event.path,entry);replica.manifest.set(event.path,plain(entry));
 }
 replica.cursors[originId]=read.cursor;replica.dirty=true;
}

/** 🚪️ Opens a replica: takes the single-daemon lock (stale locks of dead pids are taken over), clears torn staging files, streams machine, state, cursors and clock into the compact tables, recovers the clock from the own head and queues every store winner the tree does not hold yet. */
export async function openReplicaV1(options:ReplicaOptionsV1,sink:ReplicaSinkV1=()=>{}):Promise<ReplicaV1>{
 const settings=settle(options),paths=localPathsV1(settings.worktree);
 await mkdir(paths.tmp,{recursive:true});
 const release=options.lock===false?async()=>{}:await acquireLock(paths,settings.originId);
 try{
  await rm(paths.tmp,{recursive:true,force:true});await mkdir(paths.tmp,{recursive:true});
  if(options.lock!==false)await reclaimStagingV1(settings.storeRoot,{originId:settings.originId});
  const tables=tablesV1(),local=await loadLocal(paths,tables,true);
  if(local.log.size!==local.log.good)await truncate(paths.log,local.log.good);
  if(local.machine!==undefined&&local.machine.originId!==settings.originId)throw Error(`This worktree belongs to origin ${local.machine.originId}, not ${settings.originId}`);
  const machine:Machine={version:1,originId:settings.originId,groupId:settings.groupId,storeRoot:settings.storeRoot};
  await writeJson(paths,paths.machine,machine);
  let clock=local.clock;
  try{const head=await readHeadV1(settings.storeRoot,settings.originId);if(head)clock=mergeClockV1(clock,head.hlc,Number.MAX_SAFE_INTEGER-futureMilliseconds);}catch{}
  const text=await readIgnoreText(paths),folds=new ReplicaFoldsV1(tables.book);
  folds.reserve(tables.files.size);
  for(const id of tables.files.ids())if(tables.files.operationAt(id)!=="delete")folds.add(id);
  const replica:ReplicaV1={settings,paths,book:tables.book,files:tables.files,queue:tables.queue,manifest:tables.manifest,cursors:local.cursors,folds,noted:new Set(),changed:new Set(),held:new Map(),say:event=>sink({version:1,...event}),release,clock,ignore:guarded(compileIgnoreV1(text)),ignoreHash:digest(text),dirty:false,persistedAt:settings.now(),homeWritten:false,generation:local.generation,logBytes:local.log.good};
  requeue(replica);
  await replayOwnTail(replica);
  settleHeap();
  return replica;
 }catch(error){await release();throw error;}
}

async function checkpoint(replica:ReplicaV1):Promise<void>{
 const generation=replica.generation+1,{book,files,queue,manifest,paths}=replica;
 await mkdir(paths.tmp,{recursive:true});
 const temporary=join(paths.tmp,token()),handle=await open(temporary,"w");
 try{
  try{
   let text=`{"version":1,"generation":${generation},"files":{`,first=true;
   for(const id of files.ids()){
    text+=`${first?"":","}${JSON.stringify(book.name(id))}:${JSON.stringify(files.entryAt(id))}`;first=false;
    if(text.length>=writeChunk){await handle.write(text);text="";}
   }
   text+='},"queue":{';first=true;
   for(const id of queue.snapshot()){
    text+=`${first?"":","}${JSON.stringify(book.name(id))}:${JSON.stringify(manifest.entryAt(id))}`;first=false;
    if(text.length>=writeChunk){await handle.write(text);text="";}
   }
   await handle.write(`${text}}}`);
  }finally{await handle.close();}
  await rename(temporary,paths.state);
 }catch(error){await rm(temporary,{force:true});throw error;}
 replica.generation=generation;
 await writeFile(paths.log,"");
 replica.logBytes=0;
}

async function appendLog(replica:ReplicaV1,keys:readonly number[]):Promise<void>{
 const {book,files,queue,manifest}=replica,handle=await open(replica.paths.log,"a");
 try{
  let text=replica.logBytes===0?`${JSON.stringify({generation:replica.generation})}\n`:"";
  const flush=async():Promise<void>=>{await handle.write(text);replica.logBytes+=Buffer.byteLength(text);text="";};
  for(const key of keys){
   const id=Math.floor(key/2),isFiles=key%2===0;
   const entry=isFiles?(files.live(id)?files.entryAt(id):null):(queue.hasId(id)?manifest.entryAt(id):null);
   text+=`${JSON.stringify({kind:isFiles?"files":"queue",path:book.name(id),entry})}\n`;
   if(text.length>=writeChunk)await flush();
  }
  if(text!=="")await flush();
 }finally{await handle.close();}
}

async function persist(replica:ReplicaV1,force:boolean,final:boolean=false):Promise<void>{
 const roll=final&&(replica.logBytes>0||replica.changed.size>0)||replica.logBytes>logLimitBytes||replica.changed.size>checkpointChanges;
 if(!replica.dirty&&!roll)return;
 const now=replica.settings.now();
 if(!force&&now-replica.persistedAt<replica.settings.persistMilliseconds)return;
 const keys=[...replica.changed];
 replica.dirty=false;replica.persistedAt=now;replica.changed.clear();
 try{
  await writeJson(replica.paths,replica.paths.clock,{version:1,ms:replica.clock.ms,counter:replica.clock.counter});
  if(roll)await checkpoint(replica);
  else if(keys.length>0){await appendLog(replica,keys);if(replica.logBytes>logLimitBytes)await checkpoint(replica);}
  await writeJson(replica.paths,replica.paths.cursor,{version:1,cursors:replica.cursors});
 }catch(error){
  replica.dirty=true;replica.logBytes=Number.MAX_SAFE_INTEGER;
  for(const key of keys)replica.changed.add(key);
  throw error;
 }
 if(roll)settleHeap();
}

/** 🔚️ Persists the clock, a full state checkpoint (folding `state.log` into `state.json` and emptying the log) and the cursors, then releases the daemon lock. */
export async function closeReplicaV1(replica:ReplicaV1):Promise<void>{
 try{await persist(replica,true,true);}finally{await replica.release();}
}

const ignoreStamps=new WeakMap<ReplicaV1,{signature:string;at:number}>(),ignoreRevalidateMilliseconds=2000;

async function refreshIgnore(replica:ReplicaV1):Promise<boolean>{
 const info=await stat(replica.paths.gitignore).catch(()=>undefined),signature=info===undefined?"":`${info.ino}:${info.size}:${info.mtimeMs}`,known=ignoreStamps.get(replica),now=Date.now();
 if(known!==undefined&&known.signature===signature&&now-known.at<ignoreRevalidateMilliseconds)return false;
 ignoreStamps.set(replica,{signature,at:now});
 const text=await readIgnoreText(replica.paths),hash=digest(text);
 if(hash===replica.ignoreHash)return false;
 if(text===""&&replica.files.get(".gitignore")?.operation==="put")return false;
 replica.ignore=guarded(compileIgnoreV1(text));replica.ignoreHash=hash;
 replica.say({event:"ignore-reloaded",count:text===""?0:text.split("\n").length});
 requeue(replica);
 return true;
}

function mint(replica:ReplicaV1):string{
 replica.clock=tickClockV1(replica.clock,replica.settings.now());replica.dirty=true;
 return formatHlcV1(replica.clock,replica.settings.originId);
}

async function ensureHome(replica:ReplicaV1):Promise<void>{
 if(replica.homeWritten)return;
 const {origins,tmp}=storePathsV1(replica.settings.storeRoot),directory=join(origins,replica.settings.originId),file=join(directory,"home.json");
 if(await stat(file).then(()=>true,()=>false)){replica.homeWritten=true;return;}
 await ensureStoreDirectoryV1(replica.settings.storeRoot,directory);await ensureStoreDirectoryV1(replica.settings.storeRoot,tmp);
 const temporary=join(tmp,`${replica.settings.originId}-${process.pid}-${token()}`);
 await writeFile(temporary,JSON.stringify({version:1,group:replica.settings.groupId}));
 await rename(temporary,file);replica.homeWritten=true;
}

type Job=Readonly<{entry:FleetScanEntryV1;current:StateEntry|undefined}>;
type Prepared=Readonly<{job:Job;sha256?:string;bytes?:number;executable?:boolean;touch:boolean}>;

async function prepare(replica:ReplicaV1,job:Job):Promise<Prepared|undefined>{
 const {entry,current}=job;
 if(entry.kind==="link")return {job,touch:false};
 const target=absolute(replica.settings.worktree,entry.path);
 try{
  const hash=await hashFileV1(target);
  if(!hash.stable){noteOnce(replica,`unstable:${entry.path}:${entry.mtimeMs}`,{event:"unstable",path:entry.path});return undefined;}
  const executable=windows?(current?.executable??false):entry.executable;
  if(current?.operation==="put"&&current.sha256===hash.sha256&&current.executable===executable)return {job,sha256:hash.sha256,bytes:hash.bytes,executable,touch:true};
  await writeBlobV1(replica.settings.storeRoot,replica.settings.originId,hash.sha256,target);
  return {job,sha256:hash.sha256,bytes:hash.bytes,executable,touch:false};
 }catch(error){
  if(gone(error)||code(error)==="EISDIR")return undefined;
  if(sharing(error)){noteOnce(replica,`blocked:${entry.path}:${code(error)}`,{event:"blocked",path:entry.path,reason:code(error)});return undefined;}
  if(messageOf(error).startsWith("Fleet blob source changed")){noteOnce(replica,`unstable:${entry.path}:${entry.mtimeMs}`,{event:"unstable",path:entry.path});return undefined;}
  throw error;
 }
}

async function observe(replica:ReplicaV1,paths:readonly string[]):Promise<Readonly<{entries:FleetScanEntryV1[];missing:string[]}>>{
 const entries:FleetScanEntryV1[]=[],missing:string[]=[],root=replica.settings.worktree,queue=new Set(paths),expanded=new Set<string>();
 for(const marked of queue){
  const expand=marked.endsWith("/"),path=expand?marked.slice(0,-1):marked;
  if(unsafePath(path))continue;
  const target=absolute(root,path);
  let info:Stats;
  try{info=await lstat(target);}catch(error){
   if(code(error)==="ENAMETOOLONG"){if(!expand)noteOnce(replica,`skipped:${path}`,{event:"skipped",path,reason:"too-long"});continue;}
   if(!gone(error))throw error;
   if(expand)continue;
   missing.push(path);
   if(!replica.files.has(path)){const prefix=`${path}/`;for(const id of replica.files.ids()){const other=replica.book.name(id);if(replica.files.operationAt(id)!=="delete"&&other.startsWith(prefix))missing.push(other);}}
   continue;
  }
  if(info.isDirectory()){
   if(sharedV1(path,true,replica.ignore)){expanded.add(path);for(const child of await readdir(target)){const next=`${path}/${child.normalize("NFC")}`;queue.add(next);queue.add(`${next}/`);}}
   continue;
  }
  if(expand)continue;
  if(!sharedV1(path,false,replica.ignore)){
   const reason=path.split("/").includes(".git")||replica.ignore.ignores(path,false)?undefined:refusePathV1(path);
   if(reason!==undefined)noteOnce(replica,`refused:${path}:${reason}`,{event:"refused",path,reason});
   continue;
  }
  const other=replica.folds.find(path);
  if(other!==undefined&&other!==path){
   if(await listed(root,other))continue;
   missing.push(other);
  }
  if(info.isSymbolicLink()){
   const link=await readlink(target);
   if(linkRefusalV1(path,link)===undefined)entries.push({path,kind:"link",bytes:info.size,mtimeMs:info.mtimeMs,executable:false,target:link});
  }else if(info.isFile())entries.push({path,kind:"file",bytes:info.size,mtimeMs:info.mtimeMs,executable:!windows&&(info.mode&0o111)!==0});
 }
 const kept=new Map<string,number>();
 if(expanded.size>0)for(const entry of entries){
  const id=replica.book.find(entry.path);
  if(id===undefined||!replica.files.live(id)||replica.files.operationAt(id)==="delete")continue;
  for(let cut=entry.path.lastIndexOf("/");cut>0;cut=entry.path.lastIndexOf("/",cut-1)){const parent=entry.path.slice(0,cut);if(expanded.has(parent))kept.set(parent,(kept.get(parent)??0)+1);}
 }
 if([...expanded].some(directory=>replica.folds.below(directory)>(kept.get(directory)??0))){
  let seen=0;
  for(const id of replica.files.ids()){
   if(++seen%20000===0)await new Promise<void>(done=>{setImmediate(done);});
   if(replica.files.operationAt(id)==="delete"||replica.queue.hasId(id))continue;
   const name=replica.book.name(id);
   if(queue.has(name))continue;
   for(let cut=name.lastIndexOf("/");cut>0;cut=name.lastIndexOf("/",cut-1))if(expanded.has(name.slice(0,cut))){missing.push(name);break;}
  }
 }
 return {entries,missing};
}

const unchangedSince=(replica:ReplicaV1,id:number,entry:FleetScanEntryV1):boolean=>{
 const {files,queue}=replica;
 if(queue.hasId(id))return true;
 const operation=files.operationAt(id);
 if(entry.kind==="link")return operation==="link"&&files.targets.get(id)===entry.target;
 return operation==="put"&&files.bytes[id]===entry.bytes&&files.mtime[id]===entry.mtimeMs&&!files.racyAt(id)&&(windows||files.executableAt(id)===entry.executable);
};

/** 🚶️ Walks the shared tree once and keeps only what a publish needs: the entries that differ from the recorded state, and one bit per known path that was seen. The tree is never held as a list, so a walk of 270k files costs memory in proportion to what changed. */
export async function walkLocalV1(replica:ReplicaV1,signal?:AbortSignal):Promise<WalkV1>{
 const seen=new Bits(),changed=new ReplicaFoundV1(replica.book),universe=replica.book.count;
 let scanned=0;
 const pace=pacer(paceEntries),{refused}=await scanSharedV1(replica.settings.worktree,replica.ignore,signal,entry=>{
  scanned++;pace();
  const id=replica.book.find(entry.path);
  if(id!==undefined){seen.mark(id);if(unchangedSince(replica,id,entry))return;}
  changed.push(entry);
 });
 return {changed,seen,universe,scanned,refused};
}

/** 📤️ One publish round: walks the shared tree (or only the given touched paths), compares by size and mtime and then by hash, writes blobs before events, appends batched events with `base` = the previous sha256, and updates state and clock; deletions are paths in state that are gone from disk. A `walked` result from `walkLocalV1` replaces the walk. */
export async function publishOnceV1(replica:ReplicaV1,signal?:AbortSignal,paths?:readonly string[],walked?:WalkV1):Promise<PublishResultV1>{
 const {worktree,concurrency,storeRoot,originId}=replica.settings;
 await refreshIgnore(replica);
 let found:ReplicaFoundV1,candidates:string[],scanned:number;
 if(paths===undefined){
  const walk=walked??await walkLocalV1(replica,signal);
  for(const refusal of walk.refused)noteOnce(replica,`refused:${refusal.path}:${refusal.reason}`,{event:"refused",path:refusal.path,reason:refusal.reason});
  found=walk.changed;scanned=walk.scanned;candidates=[];
  for(const id of replica.files.ids())if(id<walk.universe&&!walk.seen.has(id)&&replica.files.operationAt(id)!=="delete"&&!replica.queue.hasId(id))candidates.push(replica.book.name(id));
  replica.say({event:"scan",count:scanned,total:candidates.length});
 }else{
  const seen=await observe(replica,paths);
  found=new ReplicaFoundV1(replica.book);for(const entry of seen.entries)found.push(entry);scanned=found.length;candidates=[...new Set(seen.missing)].filter(path=>{const state=replica.files.get(path);return state!==undefined&&state.operation!=="delete"&&!replica.queue.has(path);});
 }
 const deletions:string[]=[];
 await pool(candidates,concurrency,async path=>{
  try{
   const info=await lstat(absolute(worktree,path));
   if(info.isDirectory()||!await listed(worktree,path))deletions.push(path);else forget(replica,path);
  }catch(error){if(gone(error))deletions.push(path);else throw error;}
 },signal);
 deletions.sort();
 let touched=0,published=0,bytes=0,deleted=0,cursor=0,rounds=0;
 const retry:string[]=[];
 while(cursor<found.length){
  signal?.throwIfAborted();
  const slice:Job[]=[];
  while(cursor<found.length&&slice.length<chunkSize){
   const entry=found.at(cursor++);
   if(replica.queue.has(entry.path))continue;
   const current=replica.files.get(entry.path);
   if(entry.kind==="link"){if(current?.operation==="link"&&current.target===entry.target)continue;}
   else if(current?.operation==="put"&&current.bytes===entry.bytes&&current.mtimeMs===entry.mtimeMs&&!racy(current)&&(windows||current.executable===entry.executable))continue;
   slice.push({entry,current});
  }
  if(slice.length===0)continue;
  const prepared:(Prepared|undefined)[]=new Array(slice.length).fill(undefined);
  const seenMs=replica.settings.now();
  await pool(slice,concurrency,async (job,index)=>{prepared[index]=await prepare(replica,job);},signal);
  const events:FleetEventV1[]=[],updates:[string,StateEntry,boolean][]=[];
  prepared.forEach((item,index)=>{
   const job=slice[index]!,{entry,current}=job;
   if(item===undefined){retry.push(entry.path);return;}
   if(item.touch){updates.push([entry.path,{...plain(current!),mtimeMs:entry.mtimeMs,bytes:item.bytes!,seenMs},false]);touched++;return;}
   const hlc=mint(replica),base=current?.operation==="put"?current.sha256!:null;
   if(entry.kind==="link"){
    events.push({version:1,hlc,operation:"link",path:entry.path,target:entry.target!,base});
    updates.push([entry.path,{hlc,operation:"link",target:entry.target!,mtimeMs:entry.mtimeMs},true]);
   }else{
    events.push({version:1,hlc,operation:"put",path:entry.path,sha256:item.sha256!,bytes:item.bytes!,executable:item.executable!,base});
    updates.push([entry.path,{hlc,operation:"put",sha256:item.sha256!,bytes:item.bytes!,executable:item.executable!,mtimeMs:entry.mtimeMs,seenMs},true]);
    bytes+=item.bytes!;
   }
  });
  if(events.length>0){
   try{
    await ensureHome(replica);
    const head=await appendEventsV1(storeRoot,originId,events);
    replica.cursors[originId]={segment:head.segment,bytes:head.bytes};
    published+=events.length;
   }catch(error){
    if(!sharing(error))throw error;
    for(const [path,,fresh] of updates)if(fresh)retry.push(path);
    updates.splice(0,updates.length,...updates.filter(update=>!update[2]));
    noteOnce(replica,`blocked:journal:${code(error)}`,{event:"blocked",reason:code(error)});
   }
  }
  for(const [path,state,fresh] of updates){setFile(replica,path,state);if(fresh)replica.manifest.set(path,plain(state));}
  await persist(replica,false);
  if(++rounds%8===0)settleHeap();
  replica.say({event:"progress",stage:"publish",done:cursor,total:found.length});
 }
 for(let start=0;start<deletions.length;start+=chunkSize*2){
  signal?.throwIfAborted();
  const slice=deletions.slice(start,start+chunkSize*2),events:FleetEventV1[]=[];
  for(const path of slice){
   const current=replica.files.get(path)!,hlc=mint(replica);
   events.push({version:1,hlc,operation:"delete",path,base:current.operation==="put"?current.sha256!:null});
  }
  let head:FleetHeadV1;
  try{await ensureHome(replica);head=await appendEventsV1(storeRoot,originId,events);}
  catch(error){
   if(!sharing(error))throw error;
   retry.push(...slice);noteOnce(replica,`blocked:journal:${code(error)}`,{event:"blocked",reason:code(error)});
   continue;
  }
  replica.cursors[originId]={segment:head.segment,bytes:head.bytes};
  for(const event of events){setFile(replica,event.path,{hlc:event.hlc,operation:"delete",mtimeMs:0});replica.manifest.set(event.path,{hlc:event.hlc,operation:"delete"});}
  deleted+=events.length;
  for(const dir of new Set(events.map(event=>dirname(absolute(worktree,event.path)))))await prune(replica,dir);
 }
 if(published>0)replica.say({event:"published",count:published,bytes});
 if(deleted>0)replica.say({event:"published",stage:"delete",count:deleted});
 await persist(replica,false);
 return {scanned,published,deleted,touched,bytes,retry};
}

const BUSY=Symbol("busy");

async function retrying<T>(job:()=>Promise<T>):Promise<T|typeof BUSY>{
 for(let attempt=0;;attempt++){
  try{return await job();}
  catch(error){
   if(!sharing(error))throw error;
   if(attempt>=5)return BUSY;
   await sleep(20*2**attempt);
  }
 }
}

async function differs(target:string,here:Stats,known:StateEntry|undefined):Promise<boolean>{
 if(known===undefined||known.operation==="delete")return true;
 if(known.operation==="link")return !here.isSymbolicLink()||await readlink(target)!==known.target;
 if(!here.isFile()||here.size!==known.bytes)return true;
 if(here.mtimeMs===known.mtimeMs&&!racy(known))return false;
 return (await hashFileV1(target)).sha256!==known.sha256;
}

async function keepNewer(replica:ReplicaV1,path:string,entry:FleetManifestEntryV1,here:Stats):Promise<Outcome>{
 setFile(replica,path,entry.operation==="delete"?{hlc:entry.hlc,operation:"delete",mtimeMs:0}:{...plain(entry),mtimeMs:0});
 replica.say({event:"kept",path,hlc:entry.hlc,reason:"newer-than-remote",seconds:Math.max(0,Math.ceil((here.mtimeMs-parseHlcV1(entry.hlc).ms)/1000))});
 return "kept";
}

async function keepLost(replica:ReplicaV1,path:string,target:string,here:Stats,hlc:string):Promise<void>{
 const copy=join(replica.paths.lost,hlc,...path.split("/"));
 await mkdir(dirname(copy),{recursive:true});
 if(here.isSymbolicLink())await writeFile(copy,await readlink(target));else await copyFile(target,copy);
 replica.say({event:"overwritten",path,hlc,lost:`lost/${hlc}/${path}`});
}

async function prune(replica:ReplicaV1,directory:string):Promise<void>{
 const root=dirname(join(replica.settings.worktree,"x"));
 let current=directory;
 while(current.startsWith(root+sep)){
  try{await rmdir(current);}catch{return;}
  current=dirname(current);
 }
}

function refusalOf(replica:ReplicaV1,path:string,entry:FleetManifestEntryV1):string|undefined{
 if(unsafePath(path))return "unsafe-path";
 const refused=refusePathV1(path);
 if(refused!==undefined)return refused;
 if(!sharedV1(path,false,replica.ignore))return "ignored";
 if(entry.operation==="link"){const unsafe=linkRefusalV1(path,entry.target!);if(unsafe!==undefined)return unsafe;}
 const other=replica.folds.find(path);
 if(other!==undefined&&other!==path&&entry.operation!=="delete")return "case-collision";
 return undefined;
}

type Outcome="applied"|"queued"|"dropped"|"kept";

async function materialise(replica:ReplicaV1,path:string,entry:FleetManifestEntryV1):Promise<Outcome>{
 const {worktree,storeRoot}=replica.settings,target=absolute(worktree,path),known=replica.files.get(path),seenMs=replica.settings.now();
 const refusal=refusalOf(replica,path,entry);
 if(refusal!==undefined){noteOnce(replica,`refused:${path}:${refusal}`,{event:"refused",path,reason:refusal,hlc:entry.hlc});return refusal==="case-collision"?"queued":"dropped";}
 let here:Stats|undefined;
 try{here=await lstat(target);}catch(error){if(!gone(error))throw error;}
 if(entry.operation==="delete"){
  if(here!==undefined&&!here.isDirectory()){
   const lose=await differs(target,here,known);
   if(lose&&conflictWinnerV1(here.mtimeMs,entry.hlc)==="local")return keepNewer(replica,path,entry,here);
   if(lose)await keepLost(replica,path,target,here,entry.hlc);
   if(await retrying(()=>unlink(target))===BUSY){noteOnce(replica,`blocked:${path}`,{event:"blocked",path,reason:"sharing-violation"});return "queued";}
   await prune(replica,dirname(target));
  }
  setFile(replica,path,{hlc:entry.hlc,operation:"delete",mtimeMs:0});
  return "applied";
 }
 if(here?.isDirectory()){
  try{await rmdir(target);here=undefined;}
  catch{noteOnce(replica,`blocked:${path}`,{event:"blocked",path,reason:"directory-in-the-way"});return "queued";}
 }
 let same=false,lose=false;
 if(here!==undefined){
  same=entry.operation==="put"?here.isFile()&&here.size===entry.bytes&&(await hashFileV1(target)).sha256===entry.sha256:here.isSymbolicLink()&&await readlink(target)===entry.target;
  if(!same)lose=await differs(target,here,known);
 }
 if(lose&&conflictWinnerV1(here!.mtimeMs,entry.hlc)==="local")return keepNewer(replica,path,entry,here!);
 if(same){
  if(entry.operation==="put"&&!windows&&((here!.mode&0o111)!==0)!==entry.executable)await chmod(target,entry.executable?0o755:0o644);
  const info=await lstat(target);
  setFile(replica,path,{...plain(entry),mtimeMs:info.mtimeMs,seenMs});
  return "applied";
 }
 if(entry.operation==="link"){
  try{
   if(lose)await keepLost(replica,path,target,here!,entry.hlc);
   await mkdir(dirname(target),{recursive:true});
   if(here!==undefined)await rm(target,{force:true});
   await symlink(entry.target!,target);
  }catch(error){
   if(code(error)==="EPERM"||code(error)==="EACCES"||code(error)==="ENOSYS"||code(error)==="UNKNOWN"){noteOnce(replica,`skipped:${path}`,{event:"skipped",path,reason:"symlink-unsupported"});return "dropped";}
   throw error;
  }
  setFile(replica,path,{...plain(entry),mtimeMs:(await lstat(target)).mtimeMs});
  return "applied";
 }
 const temporary=join(replica.paths.tmp,token());
 await mkdir(replica.paths.tmp,{recursive:true});
 try{await copyFile(blobPathV1(storeRoot,entry.sha256!),temporary);}
 catch(error){
  if(!gone(error))throw error;
  if(!await storeReachableV1(storeRoot))return "queued";
  noteOnce(replica,`blob-missing:${path}:${entry.sha256}`,{event:"blob-missing",path,hlc:entry.hlc,reason:entry.sha256});
  return "queued";
 }
 let stamp=0;
 try{
  const check=await hashFileV1(temporary);
  if(check.sha256!==entry.sha256){
   noteOnce(replica,`corrupt:${path}:${entry.sha256}`,{event:"blob-missing",path,hlc:entry.hlc,reason:"blob-corrupt"});
   await rm(temporary,{force:true});return "queued";
  }
  if(!windows)await chmod(temporary,entry.executable?0o755:0o644);
  stamp=(await lstat(temporary)).mtimeMs;
  if(lose)await keepLost(replica,path,target,here!,entry.hlc);
  for(let attempt=0;;attempt++){
   try{await mkdir(dirname(target),{recursive:true});}
   catch(error){
    if(attempt<3&&(code(error)==="ENOTDIR"||code(error)==="EEXIST"||code(error)==="EINVAL")){await sleep(5*2**attempt);continue;}
    if(code(error)==="ENOTDIR"||code(error)==="EEXIST"){await rm(temporary,{force:true});noteOnce(replica,`blocked:${path}`,{event:"blocked",path,reason:"parent-is-a-file"});return "queued";}
    throw error;
   }
   try{
    if(await retrying(()=>rename(temporary,target))===BUSY){await rm(temporary,{force:true});noteOnce(replica,`blocked:${path}`,{event:"blocked",path,reason:"sharing-violation"});return "queued";}
    break;
   }catch(error){if((gone(error)||code(error)==="EINVAL")&&attempt<3){await sleep(5*2**attempt);continue;}throw error;}
  }
 }catch(error){await rm(temporary,{force:true});throw error;}
 setFile(replica,path,{...plain(entry),mtimeMs:stamp,seenMs});
 return "applied";
}

type Applied=Readonly<{applied:number;dropped:number;kept:readonly string[]}>;

const cooling=new WeakMap<ReplicaV1,Map<string,Readonly<{until:number;attempts:number}>>>();

async function applyQueue(replica:ReplicaV1,signal:AbortSignal|undefined,stage?:string):Promise<Applied>{
 const items=replica.queue.snapshot(),kept:string[]=[];
 let applied=0,dropped=0,done=0;
 await pool(items,replica.settings.concurrency,async id=>{
  const path=replica.book.name(id),entry=replica.manifest.entryAt(id),cool=cooling.get(replica)??new Map<string,Readonly<{until:number;attempts:number}>>(),wait=cool.get(path);
  cooling.set(replica,cool);
  if(wait!==undefined&&replica.settings.now()<wait.until)return;
  let outcome:Outcome;
  try{outcome=await materialise(replica,path,entry);}
  catch(error){
   if(signal?.aborted)throw error;
   if(sharing(error))noteOnce(replica,`blocked:${path}`,{event:"blocked",path,reason:code(error)});
   else if(code(error)==="ENAMETOOLONG"){noteOnce(replica,`skipped:${path}`,{event:"skipped",path,reason:"too-long"});dequeue(replica,path);dropped++;return;}
   else noteOnce(replica,`error:${path}:${messageOf(error)}`,{event:"error",path,message:messageOf(error)});
   outcome="queued";
  }
  if(outcome==="queued"&&replica.noted.has(`blocked:${path}`)){const attempts=(wait?.attempts??0)+1;cool.set(path,{attempts,until:replica.settings.now()+Math.min(30_000,250*2**attempts)});}
  else cool.delete(path);
  if(outcome==="applied"){applied++;replica.noted.delete(`blocked:${path}`);dequeue(replica,path);}
  else if(outcome==="dropped"){dropped++;dequeue(replica,path);}
  else if(outcome==="kept"){kept.push(path);dequeue(replica,path);}
  if(stage!==undefined&&++done%200===0)replica.say({event:"progress",stage,done,total:items.length});
 },signal);
 if(stage!==undefined)replica.say({event:"progress",stage,done:items.length,total:items.length});
 if(applied>0)replica.say({event:"applied",count:applied});
 return {applied,dropped,kept};
}

function ingest(replica:ReplicaV1,origin:string,events:readonly FleetEventV1[]):void{
 const wall=replica.settings.now(),accepted:FleetEventV1[]=[];
 for(const event of events){
  try{replica.clock=mergeClockV1(replica.clock,event.hlc,wall);accepted.push(event);}
  catch{replica.say({event:"clock-refused",origin,hlc:event.hlc,path:event.path});}
 }
 for(const event of foldEventsV1(replica.manifest,accepted)){
  const id=replica.book.intern(event.path);
  if(!replica.files.live(id)||!replica.files.sameHlcAs(replica.manifest,id))enqueueId(replica,id);
 }
 replica.dirty=true;
}

async function recoverFromSnapshot(replica:ReplicaV1,origin:string):Promise<boolean>{
 const staged=new CompactTableV1(replica.book,false),snapshot=await readNewestSnapshotV1(replica.settings.storeRoot,origin,()=>{staged.clear();return staged;}),boundary=snapshot?.cursors[origin],cursor=replica.cursors[origin];
 if(snapshot===undefined||boundary===undefined)return false;
 if(cursor!==undefined&&(cursor.segment>boundary.segment||cursor.segment===boundary.segment&&cursor.bytes>=boundary.bytes))return false;
 try{replica.clock=mergeClockV1(replica.clock,snapshot.hlc,replica.settings.now());}catch{replica.say({event:"clock-refused",origin,hlc:snapshot.hlc});}
 for(const id of staged.ids()){
  const entry=staged.entryAt(id);
  if(!winsV1(entry.hlc,replica.manifest.live(id)?replica.manifest.hlcAt(id):undefined))continue;
  replica.manifest.write(id,entry);
  if(!replica.files.live(id)||!replica.files.sameHlcAs(replica.manifest,id))enqueueId(replica,id);
 }
 replica.cursors[origin]=boundary;replica.dirty=true;
 replica.say({event:"progress",stage:"rejoin",origin,done:staged.size,total:staged.size});
 return true;
}

type Spans=Awaited<ReturnType<typeof readEventSpansV1>>;
type Admitted=Readonly<{events:readonly FleetEventV1[];cursor:FleetCursorV1|undefined;late:FleetEventV1|undefined}>;

function admit(read:Spans,after:FleetCursorV1|undefined,wall:number):Admitted{
 const stop=read.events.findIndex(event=>parseHlcV1(event.hlc).ms>wall+futureMilliseconds);
 if(stop<0)return {events:read.events,cursor:read.cursor,late:undefined};
 return {events:read.events.slice(0,stop),cursor:stop===0?after:read.ends[stop-1],late:read.events[stop]};
}

function skew(late:FleetEventV1,origin:string,wall:number):Omit<ReplicaEventV1,"version">{
 return {event:"clock-skew",origin,hlc:late.hlc,path:late.path,seconds:Math.ceil((parseHlcV1(late.hlc).ms-wall)/1000)};
}

/** 📥️ One follow round: reads the new events of every other origin from its cursor (byte offset), merges the clock, folds, and materialises each winner through `.cache/f/tmp` and a verified rename; overwritten unpublished local edits are kept in `.cache/f/lost/<hlc>/<path>`; state is updated so the observer never echoes a change. An origin whose next event is more than 60 s ahead of the wall clock is consumed only up to that event (the cursor stays in front of it, `clock-skew` is reported once per outage, the local clock is not merged with it) and is read again once the wall clock has caught up. */
export async function followOnceV1(replica:ReplicaV1,signal?:AbortSignal):Promise<FollowResultV1>{
 const {storeRoot,originId,concurrency}=replica.settings;
 await refreshIgnore(replica);
 const origins=(await listOriginsV1(storeRoot)).filter(origin=>origin!==originId),heads=new Map<string,FleetHeadV1|undefined>();
 await pool(origins,concurrency,async origin=>{
  try{heads.set(origin,await readHeadV1(storeRoot,origin));}
  catch(error){
   if(signal?.aborted)throw error;
   noteOnce(replica,`error:${origin}:${messageOf(error)}`,{event:"error",origin,message:messageOf(error)});
  }
 },signal);
 let events=0;
 for(const origin of origins){
  signal?.throwIfAborted();
  const head=heads.get(origin),cursor=replica.cursors[origin];
  if(head===undefined||cursor!==undefined&&cursor.segment===head.segment&&cursor.bytes===head.bytes)continue;
  const wall=replica.settings.now(),until=replica.held.get(origin);
  if(until!==undefined&&wall<until)continue;
  let read:Spans|undefined;
  for(let attempt=0;attempt<2&&read===undefined;attempt++){
   try{read=await readEventSpansV1(storeRoot,origin,replica.cursors[origin]);}
   catch(error){
    if(signal?.aborted)throw error;
    if(gone(error)){
     if(attempt===0&&await recoverFromSnapshot(replica,origin))continue;
     noteOnce(replica,`rejoin:${origin}`,{event:"rejoin-required",origin});
    }else noteOnce(replica,`error:${origin}:${messageOf(error)}`,{event:"error",origin,message:messageOf(error)});
    break;
   }
  }
  if(read===undefined)continue;
  replica.noted.delete(`rejoin:${origin}`);
  const taken=admit(read,replica.cursors[origin],wall);
  if(taken.late===undefined){replica.held.delete(origin);replica.noted.delete(`skew:${origin}`);}
  else{replica.held.set(origin,parseHlcV1(taken.late.hlc).ms-futureMilliseconds);noteOnce(replica,`skew:${origin}`,skew(taken.late,origin,wall));}
  ingest(replica,origin,taken.events);
  if(taken.cursor!==undefined)replica.cursors[origin]=taken.cursor;
  events+=taken.events.length;
 }
 const {applied,dropped,kept}=await applyQueue(replica,signal);
 if(kept.length>0)await publishOnceV1(replica,signal,kept);
 await persist(replica,false);
 return {events,applied,dropped,kept:kept.length,queued:replica.queue.size};
}

/** 🎲️ The wait until the next safety-net scan in milliseconds: `scanSeconds` spread uniformly over 75 % to 125 % (`random` in [0,1)), so that machines started together (a fleet-wide restart, a power cut) do not scan their trees in the same second for ever. */
export function scanDelayV1(scanSeconds:number,random:number=Math.random()):number{
 return Math.round(scanSeconds*1000*(0.75+0.5*random));
}

/** 🏃️ The long-running daemon: recursive native watch (when the platform has it) with a 150 ms debounce re-checks only touched paths, a full scan every `scanSeconds` is the safety net and walks the tree in the background (its result is published by the loop, so edits and follow never wait for the walk), follow runs every `pollMilliseconds`; an unreachable store is reported once per outage while local work continues; `.gitignore` changes reload the matcher; cancelling the signal persists state and releases the lock. */
export async function runV1(options:ReplicaOptionsV1,signal:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<void>{
 const replica=await openReplicaV1(options,sink),settings=replica.settings,touched=new Set<string>();
 let rounds=0,walk:Promise<void>|undefined,walked:WalkV1|undefined,wake:(()=>void)|undefined,lastTouch=0,full=true,nextScan=0,outage=false,recovering=true,watcher:ReturnType<typeof watch>|undefined;
 const pause=(milliseconds:number)=>new Promise<void>(done=>{
  const finish=()=>{clearTimeout(timer);signal.removeEventListener("abort",finish);wake=undefined;done();};
  const timer=setTimeout(finish,milliseconds);
  signal.addEventListener("abort",finish,{once:true});wake=finish;
 });
 const down=()=>{if(!outage){outage=true;replica.say({event:"store-unreachable",path:settings.storeRoot});}};
 replica.say({event:"started",originId:settings.originId,groupId:settings.groupId});
 try{
  watcher=watch(settings.worktree,{recursive:true,persistent:false},(kind,name)=>{
   if(name===null||name===undefined)return;
   const path=String(name).split(sep).join("/").normalize("NFC");
   if(path===".git"||path.startsWith(".git/")||path===".cache/f"||path.startsWith(".cache/f/")||replica.ignore.ignores(path,false))return;
   if(path===".gitignore"||touched.size>5000){full=true;touched.clear();}
   else{touched.add(path);if(kind==="rename")touched.add(`${path}/`);}
   lastTouch=settings.now();wake?.();
  });
  watcher.on("error",()=>{watcher?.close();watcher=undefined;});
 }catch{watcher=undefined;}
 try{
  while(!signal.aborted){
   try{
    if(!await storeReachableV1(settings.storeRoot))down();
    else{
     if(outage){outage=false;replica.say({event:"store-reachable"});full=true;}
     const now=settings.now();
     const retry:string[]=[];
     if(recovering){recovering=false;await followOnceV1(replica,signal);}
     if(full){full=false;walked=undefined;touched.clear();nextScan=settings.now()+scanDelayV1(settings.scanSeconds);retry.push(...(await publishOnceV1(replica,signal)).retry);}
     else{
      if(walked!==undefined){const scan=walked;walked=undefined;retry.push(...(await publishOnceV1(replica,signal,undefined,scan)).retry);}
      else if(walk===undefined&&now>=nextScan){
       nextScan=settings.now()+scanDelayV1(settings.scanSeconds);
       const digest=replica.ignoreHash;
       walk=walkLocalV1(replica,signal).then(scan=>{walk=undefined;if(replica.ignoreHash===digest)walked=scan;else full=true;settleHeap();},error=>{walk=undefined;if(!signal.aborted)replica.say({event:"error",message:messageOf(error)});});
      }
      if(touched.size>0&&settings.now()-lastTouch>=settings.debounceMilliseconds){
       const batch=[...touched];touched.clear();
       try{retry.push(...(await publishOnceV1(replica,signal,batch)).retry);}catch(error){for(const path of batch)touched.add(path);throw error;}
      }
     }
     if(retry.length>0){for(const path of retry)touched.add(path);lastTouch=settings.now();}
     await followOnceV1(replica,signal);
    }
   }catch(error){
    if(signal.aborted)break;
    if(!await storeReachableV1(settings.storeRoot))down();
    else replica.say({event:"error",message:messageOf(error)});
   }
   if(++rounds%heapRounds===0)settleHeap();
   await pause(touched.size>0?Math.min(settings.pollMilliseconds,settings.debounceMilliseconds):settings.pollMilliseconds);
  }
 }finally{
  watcher?.close();
  await closeReplicaV1(replica);
  replica.say({event:"stopped"});
 }
}

/** 🌱️ Seeds the store from the first machine: creates the store directories and publishes the whole shared tree with batched events, reporting done/total. */
export async function seedV1(options:ReplicaOptionsV1,signal?:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<PublishResultV1>{
 const store=storePathsV1(options.storeRoot);
 for(const directory of [store.root,store.origins,store.blobs,store.tmp,store.snapshots])await mkdir(directory,{recursive:true});
 const replica=await openReplicaV1(options,sink);
 try{return await publishOnceV1(replica,signal);}finally{await closeReplicaV1(replica);}
}

async function foldInto(storeRoot:string,manifest:CompactTableV1,signal:AbortSignal|undefined,sink:ReplicaSinkV1,now:()=>number):Promise<Readonly<{cursors:Record<string,FleetCursorV1>;hlc:string}>>{
 const snapshot=await readNewestSnapshotV1(storeRoot,undefined,()=>{manifest.clear();return manifest;}),cursors:Record<string,FleetCursorV1>={...snapshot?.cursors},origins=await listOriginsV1(storeRoot);
 let hlc=snapshot?.hlc??"";
 for(const [index,origin] of origins.entries()){
  signal?.throwIfAborted();
  if(await readHeadV1(storeRoot,origin)===undefined)continue;
  let recovered=false;
  for(;;){
   let read:Spans;
   try{read=await readEventSpansV1(storeRoot,origin,cursors[origin],1);}
   catch(error){
    if(!gone(error))throw error;
    const staged=new CompactTableV1(manifest.book,false),own=recovered?undefined:await readNewestSnapshotV1(storeRoot,origin,()=>{staged.clear();return staged;}),boundary=own?.cursors[origin],at=cursors[origin];
    recovered=true;
    if(own!==undefined&&boundary!==undefined&&(at===undefined||at.segment<boundary.segment||at.segment===boundary.segment&&at.bytes<boundary.bytes)){
     for(const id of staged.ids()){
      const entry=staged.entryAt(id);
      if(winsV1(entry.hlc,manifest.live(id)?manifest.hlcAt(id):undefined))manifest.write(id,entry);
     }
     cursors[origin]=boundary;
     if(own.hlc>hlc)hlc=own.hlc;
     continue;
    }
    sink({version:1,event:"rejoin-required",origin});
    break;
   }
   const wall=now(),taken=admit(read,cursors[origin],wall),before=cursors[origin];
   if(taken.late!==undefined)sink({version:1,...skew(taken.late,origin,wall)});
   foldEventsV1(manifest,taken.events);
   for(const event of taken.events)if(event.hlc>hlc)hlc=event.hlc;
   if(taken.cursor!==undefined)cursors[origin]=taken.cursor;
   const after=cursors[origin];
   if(taken.late!==undefined||after===undefined||before!==undefined&&before.segment===after.segment&&before.bytes===after.bytes)break;
  }
  sink({version:1,event:"progress",stage:"fold",done:index+1,total:origins.length});
 }
 return {cursors,hlc};
}

/** 🧮️ Folds the store to one manifest: the newest snapshot (streamed), then every event after its cursors, read one segment at a time; an origin is folded only up to its first event more than 60 s ahead of the wall clock (reported as `clock-skew`), and its cursor stays in front of that event, so snapshots and compaction never pass it. The manifest is a compact table that behaves like a `Map`. */
export async function foldStoreV1(storeRoot:string,signal?:AbortSignal,sink:ReplicaSinkV1=()=>{},now:()=>number=Date.now):Promise<StoreFoldV1>{
 const manifest=new CompactTableV1(new ReplicaBookV1(),false),{cursors,hlc}=await foldInto(storeRoot,manifest,signal,sink,now);
 return {manifest,cursors,hlc};
}

/** 🧳️ Joins a machine: folds the newest snapshot plus the journal, refuses a non-empty worktree whose shared files do not already match by hash, adopts matching files, then materialises the rest while reporting done/total; a half-finished join resumes from the persisted queue. */
export async function joinV1(options:ReplicaOptionsV1,signal?:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<JoinResultV1>{
 const settings=settle(options);
 if(!await storeReachableV1(settings.storeRoot))throw Error(`Replica store not found: ${storePathsV1(settings.storeRoot).root}`);
 const replica=await openReplicaV1(options,sink);
 try{
  let adopted=0;
  if(replica.files.size===0&&replica.queue.size===0){
   const folded=await foldInto(settings.storeRoot,replica.manifest,signal,sink,settings.now),{manifest,files,book}=replica;
   Object.assign(replica.cursors,folded.cursors);
   if(folded.hlc!==""){try{replica.clock=mergeClockV1(replica.clock,folded.hlc,settings.now());}catch{}}
   const rules=manifest.get(".gitignore");
   if(rules?.operation==="put"){
    const text=await readFile(blobPathV1(settings.storeRoot,rules.sha256!),"utf8");
    replica.ignore=guarded(compileIgnoreV1(text));replica.ignoreHash=digest(text);
   }
   const seenMs=settings.now(),found=new ReplicaFoundV1(book),conflicts:string[]=[],present=new Bits(),adoptions=new CompactTableV1(book,true);
   await scanSharedV1(settings.worktree,replica.ignore,signal,entry=>{found.push(entry);});
   const pace=pacer(paceEntries);
   await pool(indexes(found.length),settings.concurrency,async index=>{
    pace();
    const entry=found.at(index),id=book.find(entry.path);
    if(id!==undefined)present.mark(id);
    const operation=id===undefined?undefined:manifest.operationAt(id);
    let match=false;
    if(operation==="link")match=entry.kind==="link"&&entry.target===manifest.targets.get(id!);
    else if(operation==="put"&&entry.kind==="file"&&entry.bytes===manifest.bytes[id!])match=manifest.shaIs(id!,(await hashFileV1(absolute(settings.worktree,entry.path))).sha256);
    if(match)adoptions.write(id!,{...manifest.entryAt(id!),mtimeMs:entry.mtimeMs,seenMs});
    else conflicts.push(entry.path);
   },signal);
   if(conflicts.length>0)throw new JoinRefusedV1(conflicts.sort((left,right)=>left<right?-1:left>right?1:0));
   for(const id of adoptions.ids()){
    files.copyRow(adoptions,id);replica.folds.add(id);replica.changed.add(filesKey(id));adopted++;
   }
   for(const id of manifest.ids()){
    if(files.live(id))continue;
    if(manifest.operationAt(id)==="delete"&&!present.has(id)){
     files.write(id,{...manifest.entryAt(id),mtimeMs:0});replica.changed.add(filesKey(id));
    }else enqueueId(replica,id);
   }
   replica.dirty=true;
   await persist(replica,true);
  }
  const total=replica.files.size+replica.queue.size;
  await applyQueue(replica,signal,"join");
  await persist(replica,true);
  return {files:total,adopted,queued:replica.queue.size};
 }finally{await closeReplicaV1(replica);}
}

/** ⛔️ Raised by a join into a non-empty worktree whose shared files differ from the store. */
export class JoinRefusedV1 extends Error{
 readonly conflicts:readonly string[];
 constructor(conflicts:readonly string[]){super(`Worktree holds ${conflicts.length} shared files that differ from the store: ${conflicts.slice(0,5).join(", ")}`);this.name="JoinRefusedV1";this.conflicts=conflicts;}
}

/** 📸️ Writes a snapshot of the folded store as `snapshots/<originId>/<hlc>.json`; the hlc is the greatest folded hlc, so an unchanged store yields the same file. */
export async function snapshotV1(storeRoot:string,originId:string,signal?:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<SnapshotResultV1|undefined>{
 requireOriginIdV1(originId);
 const folded=await foldStoreV1(storeRoot,signal,sink);
 if(folded.hlc===""||folded.manifest.size===0)return undefined;
 const existing=join(storePathsV1(storeRoot).snapshots,originId,`${folded.hlc}.json`);
 if(await stat(existing).then(()=>true,()=>false))return {file:existing,hlc:folded.hlc,entries:folded.manifest.size,created:false};
 const file=await writeSnapshotV1(storeRoot,originId,folded.hlc,folded.manifest,folded.cursors);
 sink({version:1,event:"snapshot",path:file,hlc:folded.hlc,count:folded.manifest.size});
 return {file,hlc:folded.hlc,entries:folded.manifest.size,created:true};
}

async function snapshotCursors(file:string):Promise<Record<string,FleetCursorV1>>{
 let cursors:Record<string,FleetCursorV1>={};
 await streamMembersV1(file,new Set(["manifest"]),(outer,_inner,text)=>{if(outer==="cursors")cursors=JSON.parse(text) as Record<string,FleetCursorV1>;});
 return cursors;
}

async function referencesSinceV1(storeRoot:string,cursors:Record<string,FleetCursorV1>,into:Set<string>):Promise<boolean>{
 for(const origin of await listOriginsV1(storeRoot)){
  for(;;){
   const before=cursors[origin]??{segment:0,bytes:0};
   let read:Spans;
   try{read=await readEventSpansV1(storeRoot,origin,before,1);}catch(error){if(gone(error))return false;throw error;}
   for(const event of read.events)if(event.operation==="put")into.add(event.sha256!);
   cursors[origin]=read.cursor;
   if(read.cursor.segment===before.segment&&read.cursor.bytes===before.bytes)break;
  }
 }
 return true;
}

/** 🗜️ Compacts the own part of a store: writes a snapshot, then deletes this origin's sealed segments below the snapshot cursor and older own snapshots, and with `collectBlobs` (group servers only, default off, because a worker's view of the other origins may lag) also the blobs unreferenced by the folded manifest, all only when older than `retentionDays`; another origin's directory is never touched. Blobs are collected as a stream: each blob directory is listed entry by entry and at most 2,048 doomed blobs per worker wait for deletion, so memory follows the live tree and not the garbage; before every deletion batch the events published since the fold are added to the references, and a segment that vanished meanwhile stops the collection. `progress` (stage `compact`, directories done of 256, `count` blobs and `bytes` removed so far) is reported per directory and the signal cancels between files. Every compaction also removes staging files in `<store>/v1/tmp` older than 6 hours (leftovers of a `kill -9`, also those of the relay and of other origins). */
export async function compactV1(storeRoot:string,originId:string,retentionDays:number,signal?:AbortSignal,sink:ReplicaSinkV1=()=>{},now:()=>number=Date.now,options:Readonly<{collectBlobs?:boolean}>={}):Promise<CompactResultV1>{
 requireOriginIdV1(originId);
 let incomplete=false;
 const track:ReplicaSinkV1=event=>{if(event.event==="rejoin-required")incomplete=true;sink(event);};
 const snapshot=await snapshotV1(storeRoot,originId,signal,track),limit=now()-retentionDays*86_400_000,store=storePathsV1(storeRoot);
 let segments=0,blobs=0,snapshots=0,bytes=0;
 const remove=async (file:string,info:Stats):Promise<boolean>=>{
  if(info.mtimeMs>limit)return false;
  try{await unlink(file);bytes+=info.size;return true;}catch(error){if(gone(error))return false;throw error;}
 };
 const staging=await reclaimStagingV1(storeRoot,{olderThanMilliseconds:staleStagingMilliseconds,now:now()});
 bytes+=staging.bytes;
 if(snapshot===undefined)return {snapshot,segments,blobs,snapshots,staging:staging.files,bytes};
 const folded=await foldStoreV1(storeRoot,signal,track),own=(await snapshotCursors(snapshot.file))[originId],directory=join(store.origins,originId,"segments");
 if(own!==undefined&&!incomplete){
  for(const name of await readdir(directory).catch(()=>[] as string[])){
   if(!segmentPattern.test(name)||Number(name.slice(0,12))>=own.segment)continue;
   const file=join(directory,name);
   if(await remove(file,await stat(file)))segments++;
  }
 }
 const owned=join(store.snapshots,originId),names=(await readdir(owned).catch(()=>[] as string[])).filter(name=>name.endsWith(".json")).sort();
 for(const name of names.slice(0,-2)){const file=join(owned,name);if(await remove(file,await stat(file)))snapshots++;}
 if(options.collectBlobs===true){
  const referenced=new Set<string>(),seen={...folded.cursors};
  for(const entry of folded.manifest.values())if(entry.operation==="put")referenced.add(entry.sha256!);
  const prefixes=(await readdir(store.blobs).catch(()=>[] as string[])).filter(name=>/^[0-9a-f]{2}$/u.test(name));
  let done=0,syncing:Promise<void>=Promise.resolve();
  const refresh=():Promise<void>=>syncing=syncing.then(async()=>{if(!await referencesSinceV1(storeRoot,seen,referenced))incomplete=true;});
  const collect=async (doomed:[string,Stats][]):Promise<void>=>{
   await refresh();
   for(const [file] of doomed.splice(0)){
    signal?.throwIfAborted();
    if(incomplete)return;
    const fresh=await stat(file).catch(()=>undefined);
    if(fresh!==undefined&&!referenced.has(file.slice(file.length-64))&&await remove(file,fresh))blobs++;
   }
  };
  await pool(prefixes,8,async prefix=>{
   const folder=join(store.blobs,prefix),doomed:[string,Stats][]=[];
   for await(const item of await opendir(folder)){
    signal?.throwIfAborted();
    if(!hexPattern.test(item.name)||referenced.has(item.name))continue;
    const file=join(folder,item.name),info=await stat(file).catch(()=>undefined);
    if(info!==undefined&&info.mtimeMs<=limit)doomed.push([file,info]);
    if(doomed.length>=collectBatch)await collect(doomed);
   }
   await collect(doomed);
   sink({version:1,event:"progress",stage:"compact",done:++done,total:prefixes.length,count:blobs,bytes});
  },signal);
 }
 sink({version:1,event:"compacted",count:segments+blobs+snapshots+staging.files,bytes});
 return {snapshot,segments,blobs,snapshots,staging:staging.files,bytes};
}

/** 🔍️ Rehashes the local shared tree and compares it with the persisted state and with the folded store; the drift list is empty exactly when all three agree on every shared path. Every entry is compared as soon as it is hashed, so the tree is never held as hashes. */
export async function verifyV1(options:ReplicaOptionsV1,signal?:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<VerifyReportV1>{
 const settings=settle(options),paths=localPathsV1(settings.worktree),ignore=guarded(compileIgnoreV1(await readIgnoreText(paths))),tables=tablesV1(),{book,files,manifest,queue}=tables;
 await loadLocal(paths,tables);
 await foldInto(settings.storeRoot,manifest,signal,sink,settings.now);
 const found=new ReplicaFoundV1(book),seen=new Bits(),drifted=new Bits(),drift:Drift[]=[];
 const {refused}=await scanSharedV1(settings.worktree,ignore,signal,entry=>{found.push(entry);});
 let total=0;
 for(let index=0;index<found.length;index++)if((found.flags[index]!&1)===0)total++;
 let hashed=0;
 const pace=pacer(paceEntries);
 await pool(indexes(found.length),settings.concurrency,async index=>{
  pace();
  const entry=found.at(index),path=entry.path;
  let sha:string|undefined;
  if(entry.kind==="file"){
   try{sha=(await hashFileV1(absolute(settings.worktree,path))).sha256;}catch(error){if(!gone(error))throw error;}
   if(++hashed%500===0)sink({version:1,event:"progress",stage:"verify",done:hashed,total});
  }
  const id=book.find(path),operation=id===undefined?undefined:manifest.operationAt(id);
  if(id!==undefined)seen.mark(id);
  let bad=true;
  if(operation===undefined||operation==="delete")drift.push({path,kind:"extra-local",expected:"absent"});
  else if(operation==="link"){
   if(entry.kind!=="link"||entry.target!==manifest.targets.get(id!))drift.push({path,kind:"target",expected:manifest.targets.get(id!),actual:entry.target??"file"});else bad=false;
  }else if(entry.kind!=="file"||sha===undefined||!manifest.shaIs(id!,sha))drift.push({path,kind:"content",expected:manifest.shaAt(id!),actual:sha??"link"});
  else bad=false;
  if(bad){if(id!==undefined)drifted.mark(id);return;}
  if(!files.live(id!)||queue.hasId(id!)||files.operationAt(id!)==="delete")return;
  if(!files.sameHlcAs(manifest,id!))drift.push({path,kind:"state",expected:manifest.hlcAt(id!),actual:files.hlcAt(id!)});
  else if(files.operationAt(id!)==="put"&&(sha===undefined||!files.shaIs(id!,sha)))drift.push({path,kind:"state",expected:files.shaAt(id!),actual:sha});
 },signal);
 sink({version:1,event:"progress",stage:"verify",done:total,total});
 for(const id of manifest.ids()){
  const path=book.name(id),operation=manifest.operationAt(id);
  if(seen.has(id)||operation==="delete"||!sharedV1(path,false,ignore))continue;
  drift.push({path,kind:"missing-local",expected:operation==="put"?manifest.shaAt(id):manifest.targets.get(id)});
  drifted.mark(id);
 }
 for(const id of files.ids()){
  if(seen.has(id)||drifted.has(id)||queue.hasId(id)||files.operationAt(id)==="delete")continue;
  drift.push({path:book.name(id),kind:"state",expected:manifest.live(id)&&files.sameHlcAs(manifest,id)?undefined:manifest.live(id)?manifest.hlcAt(id):undefined,actual:files.hlcAt(id)});
 }
 drift.sort((left,right)=>left.path<right.path?-1:left.path>right.path?1:0);
 return {version:1,ok:drift.length===0,checked:found.length,total:drift.length,truncated:drift.length>driftLimit,drift:drift.slice(0,driftLimit),refusedTotal:refused.length,refused:refused.slice(0,driftLimit)};
}

async function countLines(file:string,start:number,end:number):Promise<number>{
 let count=0;
 for await(const chunk of createReadStream(file,{start,end:end-1}))for(const byte of chunk as Buffer)if(byte===10)count++;
 return count;
}

async function lagOf(storeRoot:string,origin:string,cursor:FleetCursorV1|undefined,head:FleetHeadV1):Promise<Readonly<{events:number;bytes:number;rejoin:boolean}>>{
 const from=cursor??{segment:0,bytes:0},directory=join(storePathsV1(storeRoot).origins,origin,"segments");
 let events=0,bytes=0;
 if(from.segment>head.segment||from.segment===head.segment&&from.bytes>=head.bytes)return {events,bytes,rejoin:false};
 try{
  for(let segment=from.segment;segment<=head.segment;segment++){
   const file=join(directory,segmentName(segment)),start=segment===from.segment?from.bytes:0,end=segment===head.segment?head.bytes:(await stat(file)).size;
   if(end<=start)continue;
   bytes+=end-start;events+=await countLines(file,start,end);
  }
 }catch(error){if(gone(error))return {events,bytes,rejoin:true};throw error;}
 return {events,bytes,rejoin:false};
}

async function countFiles(directory:string):Promise<number>{
 let count=0;
 for(const item of await readdir(directory,{withFileTypes:true}).catch(()=>[])){
  if(item.isDirectory())count+=await countFiles(join(directory,item.name));else count++;
 }
 return count;
}

/** 🩺️ Reports per origin how far behind this machine is (events and bytes after its cursor, age of the last hlc), the queue sizes (`deep` also counts local changes not yet published) and the number of preserved overwritten files. */
export async function statusV1(options:ReplicaOptionsV1,deep:boolean=false,signal?:AbortSignal):Promise<StatusReportV1>{
 const settings=settle(options),paths=localPathsV1(settings.worktree),tables=tablesV1(),local=await loadLocal(paths,tables),reachable=await storeReachableV1(settings.storeRoot),held=await readLock(paths.lock);
 const origins:StatusReportV1["origins"][number][]=[];
 if(reachable){
  for(const origin of await listOriginsV1(settings.storeRoot)){
   signal?.throwIfAborted();
   const head=await readHeadV1(settings.storeRoot,origin);
   if(head===undefined)continue;
   const lag=origin===settings.originId?{events:0,bytes:0,rejoin:false}:await lagOf(settings.storeRoot,origin,local.cursors[origin],head);
   origins.push({origin,behindEvents:lag.events,behindBytes:lag.bytes,lastHlc:head.hlc,ageSeconds:Math.max(0,Math.floor((settings.now()-parseHlcV1(head.hlc).ms)/1000)),rejoinRequired:lag.rejoin});
  }
 }
 let outgoing:number|null=null,refusedCount:readonly unknown[]|undefined;
 if(deep){
  const {book,files}=tables,seen=new Bits();
  outgoing=0;
  ({refused:refusedCount}=await scanSharedV1(settings.worktree,guarded(compileIgnoreV1(await readIgnoreText(paths))),signal,entry=>{
   const id=book.find(entry.path);
   if(id!==undefined)seen.mark(id);
   const operation=id===undefined?undefined:files.operationAt(id);
   if(entry.kind==="link"?operation!=="link"||files.targets.get(id!)!==entry.target:operation!=="put"||files.bytes[id!]!==entry.bytes||files.mtime[id!]!==entry.mtimeMs)outgoing!++;
  }));
  for(const id of files.ids())if(files.operationAt(id)!=="delete"&&!seen.has(id))outgoing++;
 }
 const report:StatusReportV1={version:1,originId:settings.originId,groupId:settings.groupId,storeRoot:settings.storeRoot,reachable,daemon:{running:held!==undefined&&!held.stale,pid:held!==undefined&&!held.stale&&held.pid>0?held.pid:null},files:tables.files.size-tables.files.count("delete"),queue:{incoming:tables.queue.size,outgoing},lost:await countFiles(paths.lost),refused:refusedCount?.length??null,origins};
 requireReplicaShapeV1("Status",report,"Closed replica status required");
 return report;
}

const homes=new Map<string,string>();

async function homeOf(store:string,origin:string):Promise<string|undefined>{
 const key=`${store}\0${origin}`,known=homes.get(key);
 if(known!==undefined)return known;
 try{
  const value=JSON.parse(await readFile(join(storePathsV1(store).origins,origin,"home.json"),"utf8")) as unknown;
  requireReplicaShapeV1("Home",value,"Closed replica home required");
  if(homes.size>=4096)homes.clear();
  homes.set(key,(value as {group:string}).group);
  return (value as {group:string}).group;
 }catch{return undefined;}
}

/** 🏠️ Lists the origins of a store whose recorded home group is the given group: the origins a relay may pull from that store. */
export async function homedOriginsV1(store:string,group:string):Promise<readonly string[]>{
 const homed:string[]=[];
 for(const origin of await listOriginsV1(store))if(await homeOf(store,origin)===group)homed.push(origin);
 return homed;
}

async function copyRange(source:string,from:number,to:number):Promise<Buffer>{
 const handle=await open(source,"r");
 try{
  const length=Math.max(0,to-from),data=Buffer.alloc(length);
  let done=0;
  while(done<length){const {bytesRead}=await handle.read(data,done,length-done,from+done);if(bytesRead===0)break;done+=bytesRead;}
  return data.subarray(0,done);
 }finally{await handle.close();}
}

async function replaceText(store:string,tmp:string,target:string,text:string):Promise<void>{
 await ensureStoreDirectoryV1(store,tmp);await ensureStoreDirectoryV1(store,dirname(target));
 const temporary=join(tmp,`relay-${token()}`);
 try{
  const handle=await open(temporary,"w");
  try{await handle.writeFile(text);await handle.sync();}finally{await handle.close();}
  await rename(temporary,target);
 }catch(error){await rm(temporary,{force:true});throw error;}
}

const auxiliaryChecked=new Map<string,number>(),auxiliaryMilliseconds=30_000;
const snapshotNamePattern=/^[0-9]{13}-[0-9]{6}-[a-z0-9][a-z0-9-]{0,39}\.json$/u;

async function relayBlob(ownStore:string,peerStore:string,sha256:string,origin:string,sink:ReplicaSinkV1):Promise<boolean>{
 const destination=blobPathV1(ownStore,sha256),own=storePathsV1(ownStore);
 if(await stat(destination).then(()=>true,()=>false))return false;
 const temporary=join(own.tmp,`relay-${token()}`);
 try{
  await ensureStoreDirectoryV1(ownStore,own.tmp);await ensureStoreDirectoryV1(ownStore,dirname(destination));
  await copyFile(blobPathV1(peerStore,sha256),temporary);
  if((await hashFileV1(temporary)).sha256!==sha256)throw Error(`Relay blob differs from its hash: ${sha256}`);
  await rename(temporary,destination);
  return true;
 }catch(error){
  await rm(temporary,{force:true});
  if(!gone(error)||!await storeReachableV1(peerStore)||!await storeReachableV1(ownStore))throw error;
  sink({version:1,event:"blob-missing",origin,reason:sha256});
  return false;
 }
}

async function relaySnapshot(ownStore:string,peerStore:string,origin:string,signal:AbortSignal|undefined,sink:ReplicaSinkV1):Promise<number>{
 const source=join(storePathsV1(peerStore).snapshots,origin),own=storePathsV1(ownStore);
 const newest=(await readdir(source).catch(()=>[] as string[])).filter(name=>snapshotNamePattern.test(name)).sort().slice(-1)[0];
 if(newest===undefined)return 0;
 const target=join(own.snapshots,origin,newest);
 if(await stat(target).then(()=>true,()=>false))return 0;
 const file=join(source,newest),found=new Set<string>();
 try{
  await streamMembersV1(file,new Set(["manifest"]),(outer,_inner,text)=>{
   if(outer!=="manifest")return;
   const entry=JSON.parse(text) as {operation?:string;sha256?:unknown};
   if(entry.operation==="put"&&typeof entry.sha256==="string"&&hexPattern.test(entry.sha256))found.add(entry.sha256);
  });
 }catch{return 0;}
 const shas=[...found];
 let copied=0;
 await pool(shas,8,async sha256=>{if(await relayBlob(ownStore,peerStore,sha256,origin,sink))copied++;},signal);
 await ensureStoreDirectoryV1(ownStore,own.tmp);await ensureStoreDirectoryV1(ownStore,dirname(target));
 const temporary=join(own.tmp,`relay-${token()}`);
 try{
  await copyFile(file,temporary);
  const handle=await open(temporary,"r+");
  try{await handle.sync();}finally{await handle.close();}
  await rename(temporary,target);
 }catch(error){await rm(temporary,{force:true});throw error;}
 return copied;
}

/** 🔀️ One relay round from a read-only peer store into the own store, for the origins homed at that peer: missing blobs first (verified by hash), then only the missing bytes of the segments (appended, earlier bytes are never rewritten; a blob that cannot be read because a store vanished aborts the round instead of letting the events overtake their blobs), then `head.json` through tmp + rename; repeating it copies nothing. The own store root is created only when `create` is true (a fresh store); the relay loop sets it false once the store has been seen. */
export async function relayOnceV1(ownStore:string,peerStore:string,originsHomedAtPeer:readonly string[],signal?:AbortSignal,sink:ReplicaSinkV1=()=>{},create:boolean=true,concurrency:number=relayConcurrency):Promise<RelayResultV1>{
 const own=storePathsV1(ownStore),peer=storePathsV1(peerStore);
 if(create)await mkdir(own.root,{recursive:true});
 let copiedOrigins=0,blobs=0,segments=0,bytes=0;
 await pool(originsHomedAtPeer,concurrency,async origin=>{
  signal?.throwIfAborted();
  requireOriginIdV1(origin);
  const head=await readHeadV1(peerStore,origin);
  if(head===undefined)return;
  const have=await readHeadV1(ownStore,origin),auxiliary=`${ownStore}\0${peerStore}\0${origin}`;
  if(have!==undefined&&have.segment===head.segment&&have.bytes===head.bytes&&Date.now()-(auxiliaryChecked.get(auxiliary)??0)<auxiliaryMilliseconds)return;
  if(auxiliaryChecked.size>=4096)auxiliaryChecked.clear();
  auxiliaryChecked.set(auxiliary,Date.now());
  const ownHome=join(own.origins,origin,"home.json"),home=await readFile(join(peer.origins,origin,"home.json"),"utf8").catch(()=>undefined);
  if(home!==undefined&&home!==await readFile(ownHome,"utf8").catch(()=>undefined))await replaceText(ownStore,own.tmp,ownHome,home);
  blobs+=await relaySnapshot(ownStore,peerStore,origin,signal,sink);
  if(have!==undefined&&have.segment===head.segment&&have.bytes===head.bytes)return;
  const sourceDirectory=join(peer.origins,origin,"segments"),targetDirectory=join(own.origins,origin,"segments");
  const numbers=(await readdir(sourceDirectory)).filter(name=>segmentPattern.test(name)).map(name=>Number(name.slice(0,12))).filter(number=>number<=head.segment&&number>=(have?.segment??0)).sort((left,right)=>left-right);
  let moved=0;
  for(const number of numbers){
   const source=join(sourceDirectory,segmentName(number)),target=join(targetDirectory,segmentName(number)),limit=number===head.segment?head.bytes:(await stat(source)).size,offset=(await stat(target).catch(()=>undefined))?.size??0;
   if(offset>=limit)continue;
   const delta=await copyRange(source,offset,limit),shas=new Set<string>();
   for(const line of delta.toString("utf8").split("\n")){
    if(line==="")continue;
    try{const event=JSON.parse(line) as {operation?:string;sha256?:string};if(event.operation==="put"&&typeof event.sha256==="string"&&hexPattern.test(event.sha256))shas.add(event.sha256);}catch{}
   }
   await pool([...shas],concurrency,async sha256=>{if(await relayBlob(ownStore,peerStore,sha256,origin,sink))blobs++;},signal);
   await ensureStoreDirectoryV1(ownStore,targetDirectory);
   const handle=await open(target,"a");
   try{await handle.write(delta);await handle.sync();}finally{await handle.close();}
   segments++;bytes+=delta.length;moved+=delta.length;
  }
  await replaceText(ownStore,own.tmp,join(own.origins,origin,"head.json"),JSON.stringify(head));
  copiedOrigins++;
  sink({version:1,event:"relayed",origin,bytes:moved});
 },signal);
 return {origins:copiedOrigins,blobs,segments,bytes};
}

/** 🛰️ The relay loop of a group server: for every peer store it lists the origins homed there and relays them into the own store every poll interval; an unreachable peer is reported once per outage and retried. Once the own store has been seen, a vanished own store (unmounted share) pauses the relay instead of being recreated as a local directory. */
export async function relayV1(options:RelayOptionsV1,signal:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<void>{
 const down=new Set<string>(),interval=options.pollMilliseconds??1000;
 let ownSeen=await storeReachableV1(options.ownStore),ownDown=false;
 sink({version:1,event:"started",stage:"relay",count:Object.keys(options.peers).length});
 while(!signal.aborted){
  if(ownSeen&&!await storeReachableV1(options.ownStore)){
   if(!ownDown){ownDown=true;sink({version:1,event:"store-unreachable",path:options.ownStore});}
  }else{
   if(ownDown){ownDown=false;sink({version:1,event:"store-reachable",path:options.ownStore});}
  await Promise.all(Object.entries(options.peers).map(async ([group,store])=>{
   if(signal.aborted)return;
   try{
    if(!await storeReachableV1(store)){
     if(!down.has(group)){down.add(group);sink({version:1,event:"store-unreachable",groupId:group,path:store});}
     return;
    }
    if(down.delete(group))sink({version:1,event:"store-reachable",groupId:group});
    await relayOnceV1(options.ownStore,store,await homedOriginsV1(store,group),signal,sink,!ownSeen,options.concurrency);
   }catch(error){
    if(signal.aborted)return;
    if(!await storeReachableV1(store)){if(!down.has(group)){down.add(group);sink({version:1,event:"store-unreachable",groupId:group,path:store});}}
    else if(!ownSeen||await storeReachableV1(options.ownStore))sink({version:1,event:"error",groupId:group,message:messageOf(error)});
   }
  }));
  ownSeen=ownSeen||await storeReachableV1(options.ownStore);
  }
  await new Promise<void>(done=>{
   const finish=()=>{clearTimeout(timer);signal.removeEventListener("abort",finish);done();};
   const timer=setTimeout(finish,interval);signal.addEventListener("abort",finish,{once:true});
  });
 }
 sink({version:1,event:"stopped",stage:"relay"});
}
