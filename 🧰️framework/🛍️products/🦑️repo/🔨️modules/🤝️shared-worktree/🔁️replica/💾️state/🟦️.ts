import {createHash} from "node:crypto";
import {open,readFile,rename,rm,type FileHandle} from "node:fs/promises";
import type {FleetManifestEntryV1} from "../../📒️journal/🟦️.ts";

/** 💾️ The binary container of the local replica state: a `state.ckp` checkpoint (96-byte header guarded by its own checksum, directory, raw column parts; every integer little-endian) and the `state.wal` delta log (16-byte header, frames of records each guarded by a length complement and a checksum). Both are described by `./🧬️schema/🔣️.json`; the Python oracle `./🧪️tests/🐍️.py` reads the same bytes. */
export const checkpointMagicV1="SEMIOSNP",walMagicV1="SEMIOWAL",formatVersionV1=1,checkpointHeaderBytesV1=96,walHeaderBytesV1=16,frameHeaderBytesV1=16,directoryEntryBytesV1=32,partNameBytesV1=24,frameLimitBytesV1=1<<20,chunkBytesV1=4<<20;

/** 🧩️ The element type of every part of a checkpoint; parts named `memo:<name>` are opaque bytes. */
export const checkpointPartsV1:Readonly<Record<string,"u8"|"u16"|"u32"|"i32"|"f64"|"text">>={
 "meta":"f64","origins":"text","fold":"u8",
 "book.starts":"f64","book.lengths":"u16","book.hashes":"u32","book.folded":"u32","book.pages":"u8",
 "files.flags":"u8","files.ms":"f64","files.counter":"u32","files.origin":"u16","files.bytes":"f64","files.sha":"u8","files.mtime":"f64","files.seen":"f64","files.targets":"u8",
 "queue.ids":"i32","queue.flags":"u8","queue.ms":"f64","queue.counter":"u32","queue.origin":"u16","queue.bytes":"f64","queue.sha":"u8","queue.targets":"u8",
 "folds.slots":"i32","folds.dirs":"i32","folds.parents":"i32",
 "dirs.mtime":"f64","dirs.present":"u8","dirs.members":"i32"
};

/** 📐️ The byte width of one element of each part type. */
export const partWidthV1:Readonly<Record<string,number>>={u8:1,u16:2,u32:4,i32:4,f64:8,text:1};

/** 🧾️ The decoded header of a checkpoint: the counts are the paths of the path book, the live local files and the queued store winners. */
export type CheckpointHeaderV1=Readonly<{version:1;generation:number;paths:number;files:number;queue:number;parts:number;payloadBytes:number;sha256:string}>;

/** 🧱️ What a checkpoint is made of when it is written: counts and named parts, each part a list of byte views written back to back. */
export type CheckpointV1=Readonly<{generation:number;paths:number;files:number;queue:number;parts:readonly Readonly<{name:string;bytes:readonly Uint8Array[]}>[]}>;

/** 💥️ Raised by the crash injection of a writer after exactly `killAt` bytes reached the file; the partial file stays, like after a kill. */
export class KilledV1 extends Error{constructor(){super("Killed by injection");this.name="KilledV1";}}

/** ☠️ Raised when a checkpoint or a log fails its closed shape, its length or its checksum. */
export class StateCorruptV1 extends Error{constructor(message:string){super(`Closed replica state required: ${message}`);this.name="StateCorruptV1";}}

/** 👁️ Views any typed array as the bytes it holds, without a copy. */
export const bytesOfV1=(array:ArrayBufferView):Uint8Array=>new Uint8Array(array.buffer,array.byteOffset,array.byteLength);

type Sink=Readonly<{write:(bytes:Uint8Array)=>Promise<void>;written:()=>number}>;

function sinkOf(handle:FileHandle,start:number,killAt:number|undefined,chunk:number):Sink{
 let position=start,total=0;
 return {
  written:()=>total,
  write:async bytes=>{
   for(let from=0;from<bytes.length;from+=chunk){
    let view=bytes.subarray(from,Math.min(bytes.length,from+chunk));
    const cut=killAt!==undefined&&total+view.length>killAt;
    if(cut)view=view.subarray(0,Math.max(0,killAt-total));
    if(view.length>0){let done=0;while(done<view.length){const {bytesWritten}=await handle.write(view,done,view.length-done,position+done);done+=bytesWritten;}}
    position+=view.length;total+=view.length;
    if(cut)throw new KilledV1();
   }
  }
 };
}

function headerOf(checkpoint:CheckpointV1,payloadBytes:number,sha256:Uint8Array):Uint8Array{
 const header=new Uint8Array(checkpointHeaderBytesV1),view=new DataView(header.buffer);
 header.set(Buffer.from(checkpointMagicV1,"ascii"),0);
 view.setUint32(8,formatVersionV1,true);view.setUint32(12,checkpoint.generation,true);view.setUint32(16,checkpoint.paths,true);view.setUint32(20,checkpoint.files,true);view.setUint32(24,checkpoint.queue,true);view.setUint32(28,checkpoint.parts.length,true);
 view.setBigUint64(32,BigInt(payloadBytes),true);header.set(sha256,40);
 header.set(createHash("sha256").update(header.subarray(0,72)).digest().subarray(0,8),72);
 return header;
}

/** 🖋️ Writes a checkpoint to `temporary` and renames it onto `file`: directory and parts are streamed in chunks (the event loop runs between chunks, nothing is copied), the checksum of the payload is taken on the way, the header with the checksum is written last, the file is fsynced before the rename. `killAt` cuts the write after that many bytes. Returns the file size. */
export async function writeCheckpointV1(file:string,temporary:string,checkpoint:CheckpointV1,options:Readonly<{killAt?:number;chunkBytes?:number}>={}):Promise<number>{
 const directory=new Uint8Array(checkpoint.parts.length*directoryEntryBytesV1),view=new DataView(directory.buffer),chunk=options.chunkBytes??chunkBytesV1;
 let payload=directory.length;
 checkpoint.parts.forEach((part,index)=>{
  const name=Buffer.from(part.name,"ascii");
  if(name.length>=partNameBytesV1)throw Error(`Closed replica part name required: ${part.name}`);
  directory.set(name,index*directoryEntryBytesV1);
  const length=part.bytes.reduce((sum,bytes)=>sum+bytes.length,0);
  view.setBigUint64(index*directoryEntryBytesV1+partNameBytesV1,BigInt(length),true);payload+=length;
 });
 const handle=await open(temporary,"w"),hash=createHash("sha256");
 try{
  try{
   const sink=sinkOf(handle,checkpointHeaderBytesV1,options.killAt,chunk);
   hash.update(directory);await sink.write(directory);
   for(const part of checkpoint.parts)for(const bytes of part.bytes){
    for(let from=0;from<bytes.length;from+=chunk)hash.update(bytes.subarray(from,Math.min(bytes.length,from+chunk)));
    await sink.write(bytes);
   }
   const head=headerOf(checkpoint,payload,hash.digest()),front=sinkOf(handle,0,options.killAt===undefined?undefined:Math.max(0,options.killAt-sink.written()),head.length);
   await front.write(head);
   await handle.sync();
  }finally{await handle.close();}
  await rename(temporary,file);
 }catch(error){if(!(error instanceof KilledV1))await rm(temporary,{force:true});throw error;}
 return checkpointHeaderBytesV1+payload;
}

/** 📖️ Reads a checkpoint part by part straight into the arrays `destination` hands out (one `read` per chunk into the final memory, no per-entry parse), verifying the header check, the length, the magic, the version and the sha256 of the payload; `destination` returns `undefined` for a part it does not want, which is hashed and skipped. Returns the header, or `undefined` when the file does not exist; throws `StateCorruptV1` otherwise. The destinations hold garbage after a throw. */
export async function readCheckpointV1(file:string,destination:(name:string,length:number,header:CheckpointHeaderV1)=>Uint8Array|undefined,options:Readonly<{chunkBytes?:number}>={}):Promise<CheckpointHeaderV1|undefined>{
 let handle:FileHandle;
 try{handle=await open(file,"r");}catch(error){if((error as {code?:string}).code==="ENOENT")return undefined;throw error;}
 const chunk=options.chunkBytes??chunkBytesV1;
 try{
  const size=(await handle.stat()).size;
  const take=async(into:Uint8Array,position:number):Promise<void>=>{
   let done=0;
   while(done<into.length){
    const {bytesRead}=await handle.read(into,done,Math.min(chunk,into.length-done),position+done);
    if(bytesRead===0)throw new StateCorruptV1("short read");
    done+=bytesRead;
   }
  };
  if(size<checkpointHeaderBytesV1)throw new StateCorruptV1("short header");
  const raw=new Uint8Array(checkpointHeaderBytesV1);await take(raw,0);
  const view=new DataView(raw.buffer);
  if(Buffer.from(raw.subarray(0,8)).toString("ascii")!==checkpointMagicV1)throw new StateCorruptV1("magic");
  if(!Buffer.from(raw.subarray(72,80)).equals(createHash("sha256").update(raw.subarray(0,72)).digest().subarray(0,8))||raw.subarray(80).some(byte=>byte!==0))throw new StateCorruptV1("header checksum");
  if(view.getUint32(8,true)!==formatVersionV1)throw new StateCorruptV1("format version");
  const payloadBig=view.getBigUint64(32,true);
  if(payloadBig!==BigInt(size-checkpointHeaderBytesV1))throw new StateCorruptV1("length");
  const header:CheckpointHeaderV1={version:1,generation:view.getUint32(12,true),paths:view.getUint32(16,true),files:view.getUint32(20,true),queue:view.getUint32(24,true),parts:view.getUint32(28,true),payloadBytes:Number(payloadBig),sha256:Buffer.from(raw.subarray(40,72)).toString("hex")};
  const directoryBytes=header.parts*directoryEntryBytesV1;
  if(directoryBytes>header.payloadBytes)throw new StateCorruptV1("directory");
  const directory=new Uint8Array(directoryBytes);await take(directory,checkpointHeaderBytesV1);
  const entries=new DataView(directory.buffer),hash=createHash("sha256");
  hash.update(directory);
  const parts:{name:string;length:number}[]=[];
  let end=directoryBytes;
  for(let index=0;index<header.parts;index++){
   const base=index*directoryEntryBytesV1,zero=directory.indexOf(0,base),name=Buffer.from(directory.subarray(base,zero<0||zero>base+partNameBytesV1?base+partNameBytesV1:zero)).toString("ascii"),length=Number(entries.getBigUint64(base+partNameBytesV1,true));
   parts.push({name,length});end+=length;
  }
  if(end!==header.payloadBytes)throw new StateCorruptV1("directory length");
  let position=checkpointHeaderBytesV1+directoryBytes;
  const scratch=new Uint8Array(Math.min(chunk,Math.max(1,...parts.map(part=>part.length))));
  for(const part of parts){
   const wanted=destination(part.name,part.length,header);
   if(wanted!==undefined&&wanted.length!==part.length)throw new StateCorruptV1(`part ${part.name}`);
   if(wanted!==undefined){await take(wanted,position);for(let from=0;from<wanted.length;from+=chunk)hash.update(wanted.subarray(from,Math.min(wanted.length,from+chunk)));}
   else for(let from=0;from<part.length;from+=scratch.length){const piece=scratch.subarray(0,Math.min(scratch.length,part.length-from));await take(piece,position+from);hash.update(piece);}
   position+=part.length;
  }
  if(hash.digest("hex")!==header.sha256)throw new StateCorruptV1("checksum");
  return header;
 }finally{await handle.close();}
}

/** 🪵️ The 16-byte header of a delta log: magic, format version and the generation of the checkpoint it extends. */
export function walHeaderV1(generation:number):Uint8Array{
 const header=new Uint8Array(walHeaderBytesV1),view=new DataView(header.buffer);
 header.set(Buffer.from(walMagicV1,"ascii"),0);view.setUint32(8,formatVersionV1,true);view.setUint32(12,generation,true);
 return header;
}

/** 📦️ Frames one payload: its length, the bitwise complement of the length (so that a damaged length is told from a short write), the first 8 bytes of its sha256 and the payload. */
export function frameV1(payload:Uint8Array):Uint8Array{
 const frame=new Uint8Array(frameHeaderBytesV1+payload.length),view=new DataView(frame.buffer);
 view.setUint32(0,payload.length,true);view.setUint32(4,~payload.length>>>0,true);
 frame.set(createHash("sha256").update(payload).digest().subarray(0,8),8);frame.set(payload,frameHeaderBytesV1);
 return frame;
}

/** 🔍️ What a scan of a delta log found: `good` is the offset after the last intact frame; `torn` means bytes follow it (an unfinished last frame), `corrupt` means a frame before the end failed its checksum or the header is foreign. */
export type WalScanV1=Readonly<{generation:number|undefined;payloads:readonly Uint8Array[];good:number;verdict:"empty"|"clean"|"torn"|"corrupt"}>;

/** 🔍️ Scans a delta log; a header or frame header cut short, a frame that runs past the end, a zero-filled tail and a failed checksum on the last frame are torn writes, any other damaged length or checksum is corruption. */
export function scanWalV1(data:Uint8Array):WalScanV1{
 if(data.length===0)return {generation:undefined,payloads:[],good:0,verdict:"empty"};
 if(data.length<walHeaderBytesV1)return {generation:undefined,payloads:[],good:0,verdict:"torn"};
 const view=new DataView(data.buffer,data.byteOffset,data.byteLength);
 if(Buffer.from(data.subarray(0,8)).toString("ascii")!==walMagicV1||view.getUint32(8,true)!==formatVersionV1)return {generation:undefined,payloads:[],good:0,verdict:"corrupt"};
 const generation=view.getUint32(12,true),payloads:Uint8Array[]=[];
 let position=walHeaderBytesV1;
 while(position<data.length){
  if(data.length-position<frameHeaderBytesV1)return {generation,payloads,good:position,verdict:"torn"};
  const length=view.getUint32(position,true),end=position+frameHeaderBytesV1+length;
  if((length^view.getUint32(position+4,true))>>>0!==0xFFFFFFFF)return {generation,payloads,good:position,verdict:data.subarray(position).every(byte=>byte===0)?"torn":"corrupt"};
  if(end>data.length)return {generation,payloads,good:position,verdict:"torn"};
  const payload=data.subarray(position+frameHeaderBytesV1,end);
  if(!Buffer.from(data.subarray(position+8,position+16)).equals(createHash("sha256").update(payload).digest().subarray(0,8)))return {generation,payloads,good:position,verdict:end===data.length?"torn":"corrupt"};
  payloads.push(payload);position=end;
 }
 return {generation,payloads,good:position,verdict:"clean"};
}

/** 📜️ A value of a local table as the log carries it: the entry of a path (`null` removes it) or an opaque named memo (`null` removes it). */
export type WalEntryV1=FleetManifestEntryV1&Readonly<{mtimeMs?:number;seenMs?:number}>;
export type WalRecordV1=Readonly<{kind:"files"|"queue";path:string;entry:WalEntryV1|null}|{kind:"memo";name:string;value:Uint8Array|null}>;

const kinds={files:1,queue:3,memo:5} as const,operations={put:1,delete:2,link:3} as const,names=["put","delete","link"] as const;
const encoder=new TextEncoder(),decoder=new TextDecoder("utf-8",{fatal:true});

class Out{
 bytes=new Uint8Array(1<<16);
 length=0;
 private view=new DataView(this.bytes.buffer);
 private need(count:number):void{
  if(this.length+count<=this.bytes.length)return;
  const next=new Uint8Array(Math.max(this.bytes.length*2,this.length+count));next.set(this.bytes.subarray(0,this.length));this.bytes=next;this.view=new DataView(next.buffer);
 }
 u8(value:number):void{this.need(1);this.bytes[this.length++]=value;}
 u16(value:number):void{this.need(2);this.view.setUint16(this.length,value,true);this.length+=2;}
 u32(value:number):void{this.need(4);this.view.setUint32(this.length,value,true);this.length+=4;}
 f64(value:number):void{this.need(8);this.view.setFloat64(this.length,value,true);this.length+=8;}
 raw(bytes:Uint8Array):void{this.need(bytes.length);this.bytes.set(bytes,this.length);this.length+=bytes.length;}
 text(value:string,wide:boolean):void{
  this.need(value.length*3+4);
  const room=this.bytes.subarray(this.length+(wide?4:2)),written=encoder.encodeInto(value,room).written;
  if(wide)this.view.setUint32(this.length,written,true);else this.view.setUint16(this.length,written,true);
  this.length+=(wide?4:2)+written;
 }
}

function encodeRecord(out:Out,record:WalRecordV1):void{
 if(record.kind==="memo"){
  out.u8(record.value===null?6:5);out.text(record.name,false);
  if(record.value!==null){out.u32(record.value.length);out.raw(record.value);}
  return;
 }
 const entry=record.entry;
 if(entry===null){out.u8(kinds[record.kind]+1);out.text(record.path,false);return;}
 out.u8(kinds[record.kind]);out.text(record.path,false);
 out.u8(operations[entry.operation]);
 out.u8((entry.executable===true?1:0)|(entry.seenMs!==undefined?2:0)|(entry.mtimeMs!==undefined?4:0));
 out.u8(entry.hlc.length);out.raw(Buffer.from(entry.hlc,"ascii"));
 out.f64(entry.mtimeMs??0);out.f64(entry.seenMs??0);out.f64(entry.bytes??0);
 if(entry.operation==="put")out.raw(Buffer.from(entry.sha256!,"hex"));
 else if(entry.operation==="link")out.text(entry.target!,true);
}

/** 🧵️ Encodes records into payloads of at most about 1 MiB each, lazily, so that a caller can write and yield between them. */
export function*payloadsV1(records:Iterable<WalRecordV1>,limit:number=frameLimitBytesV1):Generator<Uint8Array>{
 let out=new Out();
 for(const record of records){
  encodeRecord(out,record);
  if(out.length>=limit){yield out.bytes.slice(0,out.length);out=new Out();}
 }
 if(out.length>0)yield out.bytes.slice(0,out.length);
}

/** 🔓️ Decodes the records of one payload; throws `StateCorruptV1` on a malformed record (a checksum cannot catch a writer bug). */
export function decodeRecordsV1(payload:Uint8Array):WalRecordV1[]{
 const view=new DataView(payload.buffer,payload.byteOffset,payload.byteLength),records:WalRecordV1[]=[];
 let position=0;
 const fail=():never=>{throw new StateCorruptV1("log record");};
 const text=(wide:boolean):string=>{
  const length=wide?view.getUint32(position,true):view.getUint16(position,true),from=position+(wide?4:2);
  if(from+length>payload.length)fail();
  position=from+length;
  return decoder.decode(payload.subarray(from,from+length));
 };
 while(position<payload.length){
  const kind=view.getUint8(position++);
  if(kind<1||kind>6)fail();
  const name=text(false);
  if(kind===6){records.push({kind:"memo",name,value:null});continue;}
  if(kind===5){const length=view.getUint32(position,true);position+=4;if(position+length>payload.length)fail();records.push({kind:"memo",name,value:payload.slice(position,position+length)});position+=length;continue;}
  const family=kind<=2?"files":"queue";
  if(kind%2===0){records.push({kind:family,path:name,entry:null});continue;}
  const operation=names[view.getUint8(position++)-1],flags=view.getUint8(position++),hlcLength=view.getUint8(position++);
  if(operation===undefined||position+hlcLength+24>payload.length)fail();
  const hlc=Buffer.from(payload.subarray(position,position+hlcLength)).toString("ascii");position+=hlcLength;
  const mtime=view.getFloat64(position,true),seen=view.getFloat64(position+8,true),bytes=view.getFloat64(position+16,true);position+=24;
  const extras={...((flags&4)!==0?{mtimeMs:mtime}:{}),...((flags&2)!==0?{seenMs:seen}:{})};
  if(operation==="put"){
   if(position+32>payload.length)fail();
   records.push({kind:family,path:name,entry:{hlc,operation,sha256:Buffer.from(payload.subarray(position,position+32)).toString("hex"),bytes,executable:(flags&1)!==0,...extras}});position+=32;
  }else if(operation==="link")records.push({kind:family,path:name,entry:{hlc,operation,target:text(true),...extras}});
  else records.push({kind:family,path:name,entry:{hlc,operation,...extras}});
 }
 return records;
}

/** ➕️ Appends the payloads as frames to the delta log (a fresh log gets its header first), fsyncs and returns the bytes appended. `killAt` cuts the append after that many bytes. */
export async function appendWalV1(file:string,generation:number,fresh:boolean,payloads:Iterable<Uint8Array>,options:Readonly<{killAt?:number}>={}):Promise<number>{
 const handle=await open(file,"a");
 let total=0;
 try{
  const write=async(bytes:Uint8Array):Promise<void>=>{
   let view=bytes;
   const cut=options.killAt!==undefined&&total+view.length>options.killAt;
   if(cut)view=view.subarray(0,Math.max(0,options.killAt!-total));
   let done=0;
   while(done<view.length){const {bytesWritten}=await handle.write(view,done,view.length-done);done+=bytesWritten;}
   total+=view.length;
   if(cut)throw new KilledV1();
  };
  if(fresh)await write(walHeaderV1(generation));
  for(const payload of payloads)await write(frameV1(payload));
  await handle.sync();
 }finally{await handle.close();}
 return total;
}

/** 🧲️ Reads a whole delta log; `undefined` when it does not exist. */
export async function readWalV1(file:string):Promise<Readonly<{data:Uint8Array;scan:WalScanV1}>|undefined>{
 try{const data=await readFile(file);return {data,scan:scanWalV1(data)};}catch(error){if((error as {code?:string}).code==="ENOENT")return undefined;throw error;}
}
