import {hash} from "node:crypto";
import {closeSync,constants,lstatSync,openSync,renameSync,rmSync,fsyncSync,writeFileSync,type Stats} from "node:fs";
import {lstat,mkdir,open,type FileHandle} from "node:fs/promises";
import {dirname,join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {decodeAuthorityObservationsV1,encodeAuthorityObservationsV1} from "../../../👁️observation/🔢️row-table/🟨️.mjs";
import admission from "../../../🧬️schema/🚪️admission.json" with {type:"json"};
import authority from "../../../🧬️schema/📥️authority.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

type OwnerEntryV1=Readonly<{facts:unknown;rows:readonly unknown[]}>;
type RecordIndexV1=Readonly<{key:string;bytes:number;hash:string}>;
type HeaderV1=Readonly<{version:1;scope:string;format:string;sequence:number;records:readonly RecordIndexV1[]}>;
type BatchV1={path:string;stamp:Stats;records:Map<string,LocationV1>};
type LocationV1={batch:BatchV1;offset:number;record:RecordIndexV1};
type CheckpointPortV1=Readonly<{
 check():void;
 refused?(rows:readonly unknown[],storedAt:number):boolean;
 yieldContinuation():Promise<void>;
 publish(paths:readonly string[],plan:{peakBytes:number;retainedDeltaBytes:number},write:()=>void):boolean;
}>;

const limits=schema.$defs.Limits.const,kinds=authority.definitions.ObservationKind.enum;
const validity=(kind:"Header"|"Record",value:unknown)=>validateJsonSchemaSubset(schema.$defs[kind],value,schema).length===0;
const ownerFactsSchema={$defs:admission.$defs,$ref:"#/$defs/ownerFacts"};
const digest=(value:string|Uint8Array)=>hash("sha256",value,"hex");
const slotV1=(sequence:number)=>(sequence%limits.slots).toString(16).padStart(2,"0");
const missing=(error:unknown)=>(error as NodeJS.ErrnoException).code==="ENOENT";
const decode=(bytes:Uint8Array)=>new TextDecoder("utf-8",{fatal:true}).decode(bytes);
const same=(a:Stats,b:Stats)=>a.dev===b.dev&&a.ino===b.ino&&a.nlink===b.nlink&&a.size===b.size&&a.mtimeMs===b.mtimeMs&&a.ctimeMs===b.ctimeMs;

function boundedFacts(entry:unknown,key:string,maximum:number,check:()=>void):string{
 let size=0,work=0;
 return JSON.stringify({key,facts:entry},function(this:unknown,field:string,value:unknown){
  if(++work%256===0)check();
  size+=(Array.isArray(this)?0:field.length)+(typeof value==="string"?value.length:0)+1;
  if(size>maximum)throw Error("Owner checkpoint facts exceed their finite size");
  return value;
 });
}

async function bytesAt(file:FileHandle,length:number,offset:number,check:()=>void):Promise<Buffer>{
 const bytes=Buffer.alloc(length);
 for(let at=0;at<length;){
  check();
  const {bytesRead}=await file.read(bytes,at,length-at,offset+at);
  if(!bytesRead)throw Error("Incomplete owner checkpoint");
  at+=bytesRead;
 }
 check();
 return bytes;
}

/** 🧷️ Looks up completed owner observations in quota-owned bounded packs. The caller holds the selected graph writer and revalidates every observation before adoption. */
export async function openOwnerCheckpointV1(directory:string,scope:string,format:string,port:CheckpointPortV1){
 const {check}=port,lookups=new Map<string,LocationV1>(),batches=new Map<string,BatchV1>(),pending=new Map<string,Buffer>();
 let pendingBytes=0,sequence=0,lastFlush=0;
 const keyOf=(suffix:string)=>digest(JSON.stringify([scope,format,suffix]));
 check();
 const parent=await lstat(dirname(directory));
 if(!parent.isDirectory())throw Error("Original owner checkpoint parent required");
 await mkdir(directory).catch(error=>{if(error.code!=="EEXIST")throw error;});
 const original=await lstat(directory);
 if(!original.isDirectory()||original.dev!==parent.dev)throw Error("Original owner checkpoint directory required");
 check();
 const directoryHolds=(status:Stats)=>status.isDirectory()&&status.dev===original.dev&&status.ino===original.ino;
 const owned=(stamp:Stats)=>stamp.isFile()&&stamp.nlink===1&&stamp.dev===original.dev&&Number.isSafeInteger(stamp.size)&&stamp.size>=0;
 const regular=(stamp:Stats)=>owned(stamp)&&stamp.size<=limits.maximumPackBytes;
 const read=async<T>(path:string,known:Stats|undefined,body:(file:FileHandle,stamp:Stats)=>Promise<T>):Promise<T|undefined>=>{
  try{
   check();
   const stamp=await lstat(path);
   if(!regular(stamp)||known&&!same(stamp,known)||!directoryHolds(await lstat(directory)))return undefined;
   const file=await open(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)|(constants.O_NONBLOCK??0));
   try{
    if(!same(stamp,await file.stat()))return undefined;
    const value=await body(file,stamp);
    if(!same(stamp,await file.stat())||!same(stamp,await lstat(path))||!directoryHolds(await lstat(directory)))return undefined;
    check();
    return value;
   }finally{await file.close();}
  }catch{check();return undefined;}
 };
 const adopt=(path:string,stamp:Stats,header:HeaderV1,start:number)=>{
  for(const [key,location] of batches.get(path)?.records??[])if(lookups.get(key)===location)lookups.delete(key);
  const batch:BatchV1={path,stamp,records:new Map()};
  let offset=start;
  for(const record of header.records){
   const location={batch,offset,record};
   batch.records.set(record.key,location);
   lookups.set(record.key,location);
   offset+=record.bytes;
  }
  batches.set(path,batch);
 };
 const found:{path:string;stamp:Stats;header:HeaderV1;start:number}[]=[];
 for(let slot=0;slot<limits.slots;slot++){
  const name=slotV1(slot),path=join(directory,name+".pack");
  const row=await read(path,undefined,async(file,stamp)=>{
   const bytes=await bytesAt(file,Math.min(stamp.size,limits.maximumHeaderBytes),0,check);
   const newline=bytes.indexOf(10);
   if(newline<0)return undefined;
   const header=JSON.parse(decode(bytes.subarray(0,newline))) as HeaderV1;
   if(!validity("Header",header)||header.scope!==scope||header.format!==format||slotV1(header.sequence)!==name)return undefined;
   const start=newline+1;
   if(start+header.records.reduce((total,record)=>total+record.bytes,0)!==stamp.size)return undefined;
   return {path,stamp,header,start};
  });
  if(row)found.push(row);
  if((slot+1)%16===0)await port.yieldContinuation();
 }
 for(const row of found.sort((left,right)=>left.header.sequence-right.header.sequence)){
  adopt(row.path,row.stamp,row.header,row.start);
  sequence=row.header.sequence+1;
 }

 const existing=(path:string)=>{
  let stamp:Stats;
  try{stamp=lstatSync(path);}catch(error){if(missing(error))return undefined;throw error;}
  if(!owned(stamp))throw Error("Original owner checkpoint leaf required");
  return stamp;
 };
 const flush=()=>{
  check();
  if(pending.size===0||sequence>=Number.MAX_SAFE_INTEGER)return;
  const records=[...pending].map(([key,bytes])=>({key,bytes:bytes.length,hash:digest(bytes)}));
  const header:HeaderV1={version:1,scope,format,sequence,records};
  const index=Buffer.from(JSON.stringify(header)+"\n");
  const bytes=Buffer.concat([index,...pending.values()],index.length+pendingBytes);
  pending.clear();pendingBytes=0;lastFlush=Date.now();
  const path=join(directory,slotV1(sequence)+".pack"),temporary=path+".tmp";
  try{
   if(!directoryHolds(lstatSync(directory)))throw Error("Owner checkpoint directory changed");
   const previous=existing(path),staged=existing(temporary);
   if(port.publish([path,temporary],{peakBytes:bytes.length,retainedDeltaBytes:bytes.length-(previous?.size??0)-(staged?.size??0)},()=>{
    check();
    if(!directoryHolds(lstatSync(directory)))throw Error("Owner checkpoint directory changed");
    if(staged)rmSync(temporary);
    const file=openSync(temporary,"wx");
    try{writeFileSync(file,bytes);fsyncSync(file);}finally{closeSync(file);}
    check();
    renameSync(temporary,path);
   })){
    adopt(path,lstatSync(path),header,index.length);
    sequence++;
   }
  }catch{check();}
 };

 return {
  async read(suffix:string):Promise<OwnerEntryV1|undefined>{
   const key=keyOf(suffix),location=lookups.get(key);
   if(!location)return undefined;
   return read(location.batch.path,location.batch.stamp,async(file,stamp)=>{
    const bytes=await bytesAt(file,location.record.bytes,location.offset,check);
    if(digest(bytes)!==location.record.hash)return undefined;
    const record=JSON.parse(decode(bytes));
    if(!validity("Record",record)||record.key!==key||validateJsonSchemaSubset(ownerFactsSchema,record.facts).length||record.facts.project.name!==suffix.split("\0",1)[0])return undefined;
    const rows=decodeAuthorityObservationsV1(record.observations,kinds,check);
    return rows.length&&!port.refused?.(rows,stamp.mtimeMs)?{facts:record.facts,rows}:undefined;
   });
  },
  save(suffix:string,entry:OwnerEntryV1):void{
   check();
   if(!entry.rows.length||sequence>=Number.MAX_SAFE_INTEGER)return;
   const key=keyOf(suffix),maximum=limits.maximumPackBytes-limits.maximumHeaderBytes;
   let bytes:Buffer;
   try{
    const prefix=boundedFacts(entry.facts,key,maximum,check).slice(0,-1)+',"observations":';
    bytes=Buffer.from(prefix+encodeAuthorityObservationsV1(entry.rows as never,maximum-Buffer.byteLength(prefix)-1,check)+"}");
    if(bytes.length>maximum)return;
   }catch{check();return;}
   if(pendingBytes+bytes.length>maximum)flush();
   pendingBytes+=bytes.length-(pending.get(key)?.length??0);
   pending.set(key,bytes);
   if(pending.size>=limits.maximumRecords||Date.now()-lastFlush>=limits.flushMilliseconds)flush();
  },
  flush,
 };
}
