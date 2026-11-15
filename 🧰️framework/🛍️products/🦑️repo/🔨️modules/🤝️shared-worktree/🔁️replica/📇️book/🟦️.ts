import {join} from "node:path";
import type {FleetManifestEntryV1} from "../../📒️journal/🟦️.ts";
import {foldNameV1} from "../../🚧️boundary/🟦️.ts";
import {bucketOfV1,bucketsHexV1,digestHexV1,emptyBucketsV1,foldBucketV1,foldTermV1,manifestTermV1} from "../../🩺️attest/🟦️.ts";
import type {ReplicaQueueV1} from "../🟦️.ts";

/** 🧾️ One entry of the replica state: the manifest entry and the times the tree was seen with it. */
export type ReplicaStateEntryV1=FleetManifestEntryV1&Readonly<{mtimeMs:number;seenMs?:number}>;

/** 🗄️ The local state files below `<worktree>/.cache/f/`. */
export type LocalPathsV1=Readonly<{directory:string;machine:string;checkpoint:string;wal:string;takeoverState:string;takeoverLog:string;cursor:string;clock:string;lost:string;tmp:string;lock:string;sync:string;gitignore:string;hints:string}>;

/** 🗄️ Resolves the machine-local state files of a worktree. */
export function localPathsV1(worktree:string):LocalPathsV1{
 const directory=join(worktree,".cache","f");
 return {directory,machine:join(directory,"machine.json"),checkpoint:join(directory,"state.ckp"),wal:join(directory,"state.wal"),takeoverState:join(directory,"state.json"),takeoverLog:join(directory,"state.log"),cursor:join(directory,"cursor.json"),clock:join(directory,"clock.json"),lost:join(directory,"lost"),tmp:join(directory,"tmp"),lock:join(directory,"daemon.lock"),sync:join(directory,"sync.json"),gitignore:join(worktree,".gitignore"),hints:join(directory,"hints")};
}

export const racyMilliseconds=2000;
let lastSettle=-Infinity,lastSettleCost=0;
/** 🧹️ Forces a full synchronous collection, at most once per second and never within twenty times the cost of the previous one: every call site (growth of a large table, pacing loops, publish rounds) asks, so an adoption of hundreds of thousands of entries used to chain hundreds of full collections in one turn of the event loop. */
export const settleHeap=():void=>{
 const now=performance.now();
 if(now-lastSettle<Math.max(1000,lastSettleCost*20))return;
 (globalThis as {Bun?:{gc?:(force:boolean)=>void}}).Bun?.gc?.(true);
 lastSettleCost=performance.now()-now;lastSettle=performance.now();
};
export function widen<T extends Uint8Array|Uint16Array|Uint32Array|Float64Array|Int32Array>(array:T,length:number):T{
 const next=new (array.constructor as new(length:number)=>T)(length);
 next.set(array);
 if(next.byteLength>=settleBytes)queueMicrotask(settleHeap);
 return next;
}

const settleBytes=2<<20;
const hexValue=(code:number):number=>code>=48&&code<=57?code-48:code>=97&&code<=102?code-87:-1;

export const hashText=(text:string):number=>{
 let hash=2166136261;
 for(let index=0;index<text.length;index++){hash^=text.charCodeAt(index);hash=Math.imul(hash,16777619);}
 return hash>>>0;
};

export const encoder=new TextEncoder(),pageBytes=1<<20;

/** 📇️ The path table of one replica: every path ever seen gets one small integer id, and all tables of the replica store their columns by that id. The paths themselves live as UTF-8 in 1 MiB pages behind an open-addressing hash table of ids, so a path costs its bytes and a few table words, not a JavaScript string, and growing never copies the text. */
export class ReplicaBookV1{
 readonly origins:string[]=[];
 readonly originIds=new Map<string,number>();
 private lastOrigin=-1;
 count=0;
 private pages:Buffer[]=[];
 private used=pageBytes;
 private starts:Float64Array=new Float64Array(1025);
 private lengths:Uint16Array=new Uint16Array(1024);
 private hashes:Uint32Array=new Uint32Array(1024);
 private folded:Uint32Array=new Uint32Array(1024);
 private slots:Int32Array=new Int32Array(2048);
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
  if(this.pages.length===0||this.used+length>pageBytes){this.pages.push(Buffer.alloc(pageBytes));this.used=0;}
  const page=this.pages[this.pages.length-1]!;
  page.set(this.scratch.subarray(0,length),this.used);
  if(id+2>this.starts.length){
   const size=Math.ceil(this.starts.length*1.5);
   this.starts=widen(this.starts,size);this.lengths=widen(this.lengths,size-1);this.hashes=widen(this.hashes,size-1);this.folded=widen(this.folded,size-1);
  }
  this.starts[id]=(this.pages.length-1)*pageBytes+this.used;this.lengths[id]=length;this.hashes[id]=hash;this.folded[id]=hashText(foldNameV1(path));
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
 columns():Readonly<{starts:Float64Array;lengths:Uint16Array;hashes:Uint32Array;folded:Uint32Array;pages:readonly Buffer[];used:number}>{
  const count=this.count;
  return {starts:this.starts.subarray(0,count),lengths:this.lengths.subarray(0,count),hashes:this.hashes.subarray(0,count),folded:this.folded.subarray(0,count),pages:this.pages,used:this.used};
 }
 adopt(columns:Readonly<{starts:Float64Array;lengths:Uint16Array;hashes:Uint32Array;folded:Uint32Array;pages:Buffer[];used:number}>,count:number,origins:readonly string[]):void{
  this.starts=columns.starts;this.lengths=columns.lengths;this.hashes=columns.hashes;this.folded=columns.folded;this.pages=columns.pages;this.used=columns.used;this.count=count;
  this.origins.splice(0,this.origins.length,...origins);this.originIds.clear();origins.forEach((name,index)=>{this.originIds.set(name,index);});this.lastOrigin=-1;
  let size=2048;
  while(size<count*2+2)size*=2;
  this.slots=new Int32Array(size);
  for(let id=0;id<count;id++){let slot=this.hashes[id]!&(size-1);while(this.slots[slot]!==0)slot=(slot+1)&(size-1);this.slots[slot]=id+1;}
 }
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
 flags:Uint8Array=new Uint8Array(0);
 ms:Float64Array=new Float64Array(0);
 counter:Uint32Array=new Uint32Array(0);
 origin:Uint16Array=new Uint16Array(0);
 bytes:Float64Array=new Float64Array(0);
 sha:Uint8Array=new Uint8Array(0);
 mtime:Float64Array=new Float64Array(0);
 seen:Float64Array=new Float64Array(0);
 readonly targets=new Map<number,string>();
 size=0;
 readonly fold=new Uint8Array(32);
 readonly buckets=emptyBucketsV1();
 shared=0;
 writes=0;
 constructor(readonly book:ReplicaBookV1,readonly local:boolean){}
 private term(id:number):Uint8Array|undefined{
  if(!this.local||!this.live(id))return undefined;
  const operation=this.operationAt(id)!;
  return manifestTermV1({path:this.book.name(id),operation,value:operation==="put"?this.shaAt(id):this.targets.get(id)??""});
 }
 private swap(before:Uint8Array|undefined,after:Uint8Array|undefined,id:number):void{
  if(before===undefined&&after===undefined)return;
  const bucket=bucketOfV1(this.book.name(id));
  if(before!==undefined){foldTermV1(this.fold,before);foldBucketV1(this.buckets,bucket,before);this.shared--;}
  if(after!==undefined){foldTermV1(this.fold,after);foldBucketV1(this.buckets,bucket,after);this.shared++;}
 }
 /** 🧮️ The order-independent digest of the shared files, kept incrementally; a delete and a realm path contribute nothing. */
 manifestHex():string{return digestHexV1(this.fold);}
 /** 🪣️ The same digest split into 256 buckets by path, kept incrementally like `fold` and rebuilt in one pass when a saved state is adopted. */
 bucketsHex():string{return bucketsHexV1(this.buckets);}
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
 entryAt(id:number):ReplicaStateEntryV1{
  const flag=this.flags[id]!,code=(flag>>1)&3,hlc=this.hlcAt(id);
  const entry:Record<string,unknown>=code===1?{hlc,operation:"put",sha256:this.shaAt(id),bytes:this.bytes[id],executable:(flag&8)!==0}:code===3?{hlc,operation:"link",target:this.targets.get(id)}:{hlc,operation:"delete"};
  if(this.local){entry.mtimeMs=this.mtime[id];if((flag&16)!==0)entry.seenMs=this.seen[id];}
  return entry as ReplicaStateEntryV1;
 }
 write(id:number,entry:FleetManifestEntryV1&Partial<Readonly<{mtimeMs:number;seenMs:number}>>):void{
  this.writes++;this.grow(id+1);
  const before=this.term(id),hlc=entry.hlc;
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
  if(this.local)this.swap(before,this.term(id),id);
 }
 copyRow(from:CompactTableV1,id:number):void{
  this.grow(id+1);
  const before=this.term(id);
  if(!this.live(id))this.size++;
  this.flags[id]=(from.flags[id]!&~16)|(this.local&&from.local?from.flags[id]!&16:0);
  this.ms[id]=from.ms[id]!;this.counter[id]=from.counter[id]!;this.origin[id]=from.origin[id]!;this.bytes[id]=from.bytes[id]!;
  for(let index=id*32,end=index+32;index<end;index++)this.sha[index]=from.sha[index]!;
  const target=from.targets.get(id);
  if(target===undefined)this.targets.delete(id);else this.targets.set(id,target);
  if(this.local&&from.local){this.mtime[id]=from.mtime[id]!;this.seen[id]=from.seen[id]!;}
  if(this.local)this.swap(before,this.term(id),id);
 }
 clearId(id:number):boolean{
  if(!this.live(id))return false;
  this.writes++;
  const before=this.term(id);
  this.flags[id]=0;this.targets.delete(id);this.size--;
  this.swap(before,undefined,id);
  return true;
 }
 columns(rows:number):Readonly<{flags:Uint8Array;ms:Float64Array;counter:Uint32Array;origin:Uint16Array;bytes:Float64Array;sha:Uint8Array;mtime:Float64Array;seen:Float64Array}>{
  return {flags:this.flags.subarray(0,rows),ms:this.ms.subarray(0,rows),counter:this.counter.subarray(0,rows),origin:this.origin.subarray(0,rows),bytes:this.bytes.subarray(0,rows),sha:this.sha.subarray(0,rows*32),mtime:this.mtime.subarray(0,this.local?rows:0),seen:this.seen.subarray(0,this.local?rows:0)};
 }
 adopt(columns:Readonly<{flags:Uint8Array;ms:Float64Array;counter:Uint32Array;origin:Uint16Array;bytes:Float64Array;sha:Uint8Array;mtime:Float64Array;seen:Float64Array}>,fold:Uint8Array,shared:number):void{
  this.writes++;
  this.flags=columns.flags;this.ms=columns.ms;this.counter=columns.counter;this.origin=columns.origin;this.bytes=columns.bytes;this.sha=columns.sha;this.mtime=columns.mtime;this.seen=columns.seen;
  let size=0;
  for(let id=0;id<this.flags.length;id++)size+=this.flags[id]!&1;
  this.size=size;this.fold.set(fold);this.shared=shared;
  this.buckets.fill(0);
  if(this.local)for(let id=0;id<this.flags.length;id++){const term=this.term(id);if(term!==undefined)foldBucketV1(this.buckets,bucketOfV1(this.book.name(id)),term);}
 }
 adoptRows(ids:Int32Array,columns:Readonly<{flags:Uint8Array;ms:Float64Array;counter:Uint32Array;origin:Uint16Array;bytes:Float64Array;sha:Uint8Array}>):void{
  let top=0;
  for(const id of ids)top=Math.max(top,id+1);
  this.grow(top);
  ids.forEach((id,row)=>{
   if(!this.live(id))this.size++;
   this.flags[id]=columns.flags[row]!;this.ms[id]=columns.ms[row]!;this.counter[id]=columns.counter[row]!;this.origin[id]=columns.origin[row]!;this.bytes[id]=columns.bytes[row]!;
   this.sha.set(columns.sha.subarray(row*32,row*32+32),id*32);
  });
 }
 mirror(from:CompactTableV1,skip:ReplicaQueueV1):void{
  const rows=from.flags.length;
  this.grow(rows);
  for(let id=0;id<this.flags.length;id++){
   if(skip.hasId(id))continue;
   if(id>=rows||(from.flags[id]!&1)===0){
    if((this.flags[id]!&1)===1){this.flags[id]=0;this.targets.delete(id);this.size--;}
    continue;
   }
   if((this.flags[id]!&1)===0)this.size++;
   this.flags[id]=from.flags[id]!&~16;this.ms[id]=from.ms[id]!;this.counter[id]=from.counter[id]!;this.origin[id]=from.origin[id]!;this.bytes[id]=from.bytes[id]!;
   this.sha.set(from.sha.subarray(id*32,id*32+32),id*32);
   const target=from.targets.get(id);
   if(target===undefined)this.targets.delete(id);else this.targets.set(id,target);
  }
 }
 clear():void{this.writes++;this.flags.fill(0);this.targets.clear();this.size=0;this.fold.fill(0);this.buckets.fill(0);this.shared=0;}
 idOf(path:string):number|undefined{return this.book.find(path);}
 has(path:string):boolean{const id=this.book.find(path);return id!==undefined&&this.live(id);}
 get(path:string):ReplicaStateEntryV1|undefined{const id=this.book.find(path);return id===undefined||!this.live(id)?undefined:this.entryAt(id);}
 set(path:string,entry:FleetManifestEntryV1&Partial<Readonly<{mtimeMs:number;seenMs:number}>>):this{this.write(this.book.intern(path),entry);return this;}
 delete(path:string):boolean{const id=this.book.find(path);return id===undefined?false:this.clearId(id);}
 ids():Int32Array<ArrayBuffer>{
  const ids=new Int32Array(this.size);
  let count=0;
  for(let id=0;id<this.flags.length&&count<ids.length;id++)if((this.flags[id]!&1)===1)ids[count++]=id;
  return ids;
 }
 *keys():Generator<string>{for(const id of this.ids())yield this.book.name(id);}
 *values():Generator<ReplicaStateEntryV1>{for(const id of this.ids())yield this.entryAt(id);}
 *entries():Generator<[string,ReplicaStateEntryV1]>{for(const id of this.ids())yield [this.book.name(id),this.entryAt(id)];}
 [Symbol.iterator]():Generator<[string,ReplicaStateEntryV1]>{return this.entries();}
 count(operation:"put"|"delete"|"link"):number{let count=0;for(const id of this.ids())if(this.operationAt(id)===operation)count++;return count;}
}
