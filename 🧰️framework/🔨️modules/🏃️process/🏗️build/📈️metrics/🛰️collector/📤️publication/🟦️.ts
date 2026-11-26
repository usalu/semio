import {closeSync,mkdirSync,openSync,readSync,readdirSync,renameSync,rmSync,statSync,writeSync} from "node:fs";
import {join} from "node:path";
import {METRICS_FILES_V1,METRICS_RING_V1,METRICS_TIERS_V1} from "../../🟦️.ts";
import {processPidExistsV1} from "../../../../🔖️identity/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const P=schema.definitions.Policy.const,TEMPORARY=new RegExp(P.temporaryPattern,"u"),TEXTS=schema.definitions.Texts.const,R=METRICS_RING_V1;
const TWO32=4294967296,HEAD=40,STAMP=R.stampBytes,IDENTITY=[[0,40],[48,56]] as const,CHUNK_ROWS=64,COPY_BYTES=262144,APPLE_DOUBLE=P.appleDoublePrefix;

export interface MetricsPublicationOutcomeV1{readonly copied:number;readonly rows:number;readonly bytes:number;readonly failed:number}
export interface MetricsPublisherV1{publish(atMs:number,force?:boolean):MetricsPublicationOutcomeV1;sweep():readonly string[]}

const message=(file:string,reason:string):string=>`${TEXTS.en.publication.replace("{file}",file).replace("{reason}",reason)} / ${TEXTS.de.publication.replace("{file}",file).replace("{reason}",reason)}`;
const refuse=():never=>{throw new Error("not a metrics ring / kein Metrikring");};
const get64=(buffer:Buffer,at:number):number=>buffer.readUInt32LE(at)+buffer.readUInt32LE(at+4)*TWO32;
const put64=(buffer:Buffer,at:number,value:number):void=>{buffer.writeUInt32LE(value%TWO32,at);buffer.writeUInt32LE(Math.floor(value/TWO32),at+4);};

interface HeaderV1{readonly bytes:Buffer;readonly bucketMs:number;readonly buckets:number;readonly rowBytes:number;readonly dataOffset:number;readonly head:number}
const headerOf=(fd:number):HeaderV1=>{
 const bytes=Buffer.alloc(R.headerBytes);
 if(readSync(fd,bytes,0,R.headerBytes,0)!==R.headerBytes||bytes.toString("ascii",0,4)!==R.magic||bytes.readUInt16LE(4)!==R.version)return refuse();
 return {bytes,bucketMs:bytes.readUInt32LE(8),buckets:bytes.readUInt32LE(12),rowBytes:bytes.readUInt32LE(24),dataOffset:bytes.readUInt32LE(28),head:get64(bytes,HEAD)};
};
const copyPlain=(source:string,target:string):void=>{
 const from=openSync(source,"r");
 try{
  const to=openSync(target,"w");
  try{const buffer=Buffer.allocUnsafe(COPY_BYTES);for(let at=0;;){const read=readSync(from,buffer,0,COPY_BYTES,at);if(read===0)break;writeSync(to,buffer,0,read);at+=read;}}
  finally{closeSync(to);}
 }finally{closeSync(from);}
};
const sameLayout=(left:HeaderV1,right:HeaderV1):boolean=>IDENTITY.every(([from,to])=>left.bytes.compare(right.bytes,from,to,from,to)===0);

/** 📤️ Publishes the ring files of a machine into the fleet store directory `to` (`origins/<origin>/metrics/` or `fleet/metrics/<source>/`). A tier that has no row yet is not published. A published file is created whole and atomically (kernel copy to `<name>.<pid>.tmp`, header checked, rename) when it is absent or of another layout; from then on each look appends exactly the rows the writer closed since the last look, in place: the rows first, the header cursor last, so a reader (which trusts only a row whose start stamp at both ends agree and a cursor that points at a valid row) never sees half a row. A few KB per look instead of the whole file: the published state is read back from the published header, so a restart of the publisher continues where it stopped. A file that is not a ring of this version or exceeds `fileBytesMaximum` is refused; only the known tier files are ever written, so the directory is bounded by the tier table. Never throws.
 * @see ../🧬️schema/🔣️.json */
export function createMetricsPublisherV1(input:Readonly<{from:string;to:string;now?:()=>number;report?:(line:string)=>void}>):MetricsPublisherV1{
 const report=input.report??(()=>{}),noted=new Set<string>(),extension=METRICS_FILES_V1.extension,seen=new Map<string,string>();
 const sweep=():readonly string[]=>{
  const removed:string[]=[];
  try{
   for(const name of readdirSync(input.to)){
    if(name.startsWith(APPLE_DOUBLE)){try{rmSync(join(input.to,name),{force:true});removed.push(name);}catch{}continue;}
    const found=TEMPORARY.exec(name);if(found===null||!found[1]!.endsWith(extension))continue;
    const pid=Number(found[2]);if(pid===process.pid)continue;
    try{if(!processPidExistsV1(pid)){rmSync(join(input.to,name),{force:true});removed.push(name);}}catch{}
   }
  }catch{}
  return removed;
 };
 let sweptAt=-Infinity;
 const copyWhole=(source:string,target:string,name:string):number=>{
  const temporary=`${target}.${process.pid}.tmp`;
  try{
   copyPlain(source,temporary);
   const size=statSync(temporary).size;
   if(size>P.fileBytesMaximum||size<R.headerBytes)refuse();
   const fd=openSync(temporary,"r");let header:HeaderV1;try{header=headerOf(fd);}finally{closeSync(fd);}
   renameSync(temporary,target);
   try{rmSync(join(input.to,APPLE_DOUBLE+name),{force:true});}catch{}
   return size;
  }catch(error){try{rmSync(temporary,{force:true});}catch{}throw error;}
 };
 const append=(sourceFd:number,source:HeaderV1,target:string,name:string):Readonly<{rows:number;bytes:number}>=>{
  const fd=openSync(target,"r+");
  try{
   const published=headerOf(fd);
   if(!sameLayout(source,published)||published.head>source.head)return {rows:-1,bytes:0};
   const width=source.bucketMs,wholeRing=published.head===0||source.head-published.head>=source.buckets*width;
   let at=wholeRing?Math.max(width,source.head-(source.buckets-1)*width):published.head+width,rows=0,bytes=0;
   while(at<=source.head){
    const index=(at/width)%source.buckets,run=Math.min(CHUNK_ROWS,source.buckets-index,Math.round((source.head-at)/width)+1),buffer=Buffer.alloc(run*source.rowBytes),offset=source.dataOffset+index*source.rowBytes;
    if(readSync(sourceFd,buffer,0,buffer.length,offset)!==buffer.length)return {rows:-1,bytes:0};
    writeSync(fd,buffer,0,buffer.length,offset);
    rows+=run;bytes+=buffer.length;at+=run*width;
   }
   if(rows>0){const stamp=Buffer.alloc(STAMP);put64(stamp,0,source.head);writeSync(fd,stamp,0,STAMP,HEAD);bytes+=STAMP;}
   return {rows,bytes};
  }finally{closeSync(fd);}
 };
 return Object.freeze({
  sweep,
  publish(atMs:number,force:boolean=false):MetricsPublicationOutcomeV1{
   let copied=0,rows=0,total=0,failed=0;
   for(const tier of METRICS_TIERS_V1){
    const name=tier.id+extension,source=join(input.from,name),target=join(input.to,name);
    let sourceFd=-1;
    try{
     let stat;try{stat=statSync(source);}catch{continue;}
     if(stat.size>P.fileBytesMaximum||stat.size<R.headerBytes)continue;
     const stamp=`${stat.mtimeMs}:${stat.size}`;
     let targetSize=-1;try{targetSize=statSync(target).size;}catch{}
     if(!force&&targetSize===stat.size&&seen.get(name)===stamp)continue;
     sourceFd=openSync(source,"r");
     const header=headerOf(sourceFd);
     if(header.head===0)continue;
     if(atMs-sweptAt>=P.sweepMilliseconds){mkdirSync(input.to,{recursive:true});sweep();sweptAt=atMs;}
     let done:Readonly<{rows:number;bytes:number}>|undefined;
     if(targetSize===stat.size&&header.head>0)done=append(sourceFd,header,target,name);
     if(done===undefined||done.rows<0){const size=copyWhole(source,target,name);done={rows:header.buckets,bytes:size};}
     if(done.rows>0||done.bytes>0){copied++;rows+=done.rows;total+=done.bytes;}
     seen.set(name,stamp);noted.delete(name);
    }catch(error){
     failed++;
     if(!noted.has(name)){noted.add(name);report(message(name,(error as Error).message));}
    }finally{if(sourceFd>=0)try{closeSync(sourceFd);}catch{}}
   }
   return {copied,rows,bytes:total,failed};
  },
 });
}
