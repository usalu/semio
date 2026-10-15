import {createHash} from "node:crypto";
import {constants,lstatSync,opendirSync,statfsSync,unlinkSync,type BigIntStats} from "node:fs";
import {open} from "node:fs/promises";
import {join} from "node:path";
import {captureOwnedCommandV1} from "../../../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {openNativePlatformStorageV1,requireNativeStoragePathV1,type NativeImageStorageV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts";
import type {NativeFixedPlatformOptionsV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import schema from "./🧬️schema/🔣️.json";
import authoritySchema from "../../../../../../🏗️build/🧬️schema/📥️authority.json";
const protocol=schema.definitions.Protocol.const,authorityKindsV1=authoritySchema.properties.observations.items.properties.kind.enum;
/** 📋️ Names the required physical floor without granting cleanup custody. */
export interface BuildGraphHeadroomRequestV1{readonly minimumAvailableBytes:number}
/** 🧾️ Reports observed available bytes after actual exclusive image maintenance. */
export interface BuildGraphHeadroomResultV1{readonly acquired:boolean;readonly availableBytes:number;readonly retiredBytes:number;readonly retiredFiles:number}
/** 🚦️ Refuses unbounded recovery requests before opening an enforcing image. */
export function readBuildGraphHeadroomRequestV1(value:unknown):BuildGraphHeadroomRequestV1{if(validateJsonSchemaSubset(schema,value).length)throw Error("Closed graph headroom request required");return Object.freeze({minimumAvailableBytes:(value as BuildGraphHeadroomRequestV1).minimumAvailableBytes});}
type LeafV1=Readonly<{path:string;stamp:string;bytes:number;allocated:number;modified:bigint}>;
const stamp=(row:BigIntStats)=>[row.dev,row.ino,row.size,row.nlink,row.mtimeNs,row.ctimeNs].join(":");
function leaf(image:NativeImageStorageV1,path:string,maximum:number):LeafV1{
 requireNativeStoragePathV1(image.root,path);const row=lstatSync(path,{bigint:true});if(!row.isFile()||row.isSymbolicLink()||row.nlink!==1n||row.dev!==lstatSync(image.root,{bigint:true}).dev||row.size>BigInt(maximum))throw Error("Owned bounded graph cache leaf required");return{path,stamp:stamp(row),bytes:Number(row.size),allocated:Number(row.blocks)*512,modified:row.mtimeNs};
}
function unchanged(image:NativeImageStorageV1,row:LeafV1):void{if(leaf(image,row.path,Math.max(protocol.maximumSnapshotBytes,protocol.maximumReceiptBytes)).stamp!==row.stamp)throw Error("Original graph retirement leaf changed");}
async function digest(image:NativeImageStorageV1,row:LeafV1,control:ResourceLeaseControl):Promise<string>{
 unchanged(image,row);const file=await open(row.path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)),hash=createHash("sha256"),buffer=Buffer.allocUnsafe(protocol.chunkBytes);let position=0;
 try{if(stamp(await file.stat({bigint:true}))!==row.stamp)throw Error("Original graph retirement descriptor changed");while(position<row.bytes){checkResourceLeaseControl(control);await image.recheck();const next=await file.read(buffer,0,Math.min(buffer.length,row.bytes-position),position);if(next.bytesRead===0)throw Error("Original graph retirement bytes shortened");hash.update(buffer.subarray(0,next.bytesRead));position+=next.bytesRead;if(position%(protocol.chunkBytes*16)===0){await control.publish({resource:"repository:graph-headroom-authentication",mode:"exclusive",elapsedMs:0});await control.yieldContinuation();}}if(stamp(await file.stat({bigint:true}))!==row.stamp)throw Error("Original graph retirement descriptor changed");unchanged(image,row);return hash.digest("hex");}finally{await file.close();}
}
const authorityProgramV1=`const fs=require('node:fs'),workerData=JSON.parse(process.argv[1]),file=fs.openSync(workerData.path,fs.constants.O_RDONLY|(fs.constants.O_NOFOLLOW||0)),stamp=row=>[row.dev,row.ino,row.size,row.nlink,row.mtimeNs,row.ctimeNs].join(':');try{if(stamp(fs.fstatSync(file,{bigint:true}))!==workerData.stamp)throw Error('Original graph receipt descriptor changed');const bytes=fs.readFileSync(file);if(stamp(fs.fstatSync(file,{bigint:true}))!==workerData.stamp)throw Error('Original graph receipt descriptor changed');const value=JSON.parse(bytes.toString()),shape=(v,keys)=>v!==null&&typeof v==='object'&&!Array.isArray(v)&&Object.keys(v).sort().join(',')===keys,hash=v=>typeof v==='string'&&/^[a-f0-9]{64}$/.test(v);if(!shape(value,'graph,membership,module,observations,version')||value.version!==3||!hash(value.graph)||!hash(value.membership)||!shape(value.module,'implementation,publication')||!hash(value.module.implementation)||!hash(value.module.publication)||!Array.isArray(value.observations)||value.observations.some(row=>!shape(row,'fact,kind,path')||!workerData.kinds.includes(row.kind)||typeof row.path!=='string'||row.path.length===0))throw Error('Current graph publication identity changed');console.log(JSON.stringify({graph:value.graph}));}finally{fs.closeSync(file);}`;
async function publicationGraphV1(image:NativeImageStorageV1,row:LeafV1,control:ResourceLeaseControl):Promise<string>{
 checkResourceLeaseControl(control);unchanged(image,row);await control.publish({resource:"repository:graph-receipt-authentication",mode:"exclusive",elapsedMs:0});const output=await captureOwnedCommandV1(process.versions.bun?"node":process.execPath,["--max-old-space-size="+protocol.receiptHeapMiB,"-e",authorityProgramV1,JSON.stringify({path:row.path,stamp:row.stamp,kinds:authorityKindsV1})],image.root,{control,output:{maximumBytes:8192,maximumLines:64}}),value=JSON.parse(output.trim());if(!value||Object.keys(value).join()!=="graph"||typeof value.graph!=="string"||!/^[a-f0-9]{64}$/u.test(value.graph))throw Error("Closed graph receipt child result required");checkResourceLeaseControl(control);unchanged(image,row);return value.graph;
}
function available(image:NativeImageStorageV1):number{const row=statfsSync(image.root),bytes=row.bavail*row.bsize;if(!Number.isSafeInteger(bytes)||bytes<0)throw Error("Actual graph image capacity required");return bytes;}
/** 🧹️ Reclaims only authenticated snapshots and current publications beneath the actual image's exclusive lifetime. */
export async function recoverBuildGraphImageHeadroomV1(options:NativeFixedPlatformOptionsV1,value:unknown):Promise<BuildGraphHeadroomResultV1>{
 const request=readBuildGraphHeadroomRequestV1(value),control=options.control;checkResourceLeaseControl(control);const image=await openNativePlatformStorageV1(options);let retiredBytes=0,retiredFiles=0;
 try{
  await image.recheck();if(available(image)>=request.minimumAvailableBytes)return Object.freeze({acquired:true,availableBytes:available(image),retiredBytes,retiredFiles});
  const result=await image.withExclusiveMaintenance({version:1,wait:false},async()=>{
   await image.recheck();const snapshots:LeafV1[]=[],directory=requireNativeStoragePathV1(image.root,join(image.root,"snapshots"));const names:string[]=[];let entries:ReturnType<typeof opendirSync>|undefined;try{entries=opendirSync(directory);for(;;){checkResourceLeaseControl(control);const entry=entries.readSync();if(!entry)break;if(names.length>=protocol.maximumEntries)throw Error("Bounded graph snapshot inventory required");names.push(entry.name);}}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}finally{entries?.closeSync();}
   for(const name of names){checkResourceLeaseControl(control);if(!/^[a-f0-9]{64}\.json$/u.test(name))continue;const row=leaf(image,join(directory,name),protocol.maximumSnapshotBytes);if(await digest(image,row,control)!==name.slice(0,64))throw Error("Graph snapshot content identity changed");snapshots.push(row);}
   const canonical=join(image.root,"canonical.json"),authority=join(image.root,"authority.json");let publication:readonly LeafV1[]=[];
   try{const graph=leaf(image,canonical,protocol.maximumSnapshotBytes),receipt=leaf(image,authority,protocol.maximumReceiptBytes),expected=await publicationGraphV1(image,receipt,control);if(await digest(image,graph,control)!==expected)throw Error("Current graph publication identity changed");publication=[receipt,graph];}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
   const remove=async(rows:readonly LeafV1[])=>{checkResourceLeaseControl(control);await image.recheck();for(const row of rows)unchanged(image,row);for(const row of rows){unlinkSync(row.path);retiredBytes+=row.allocated;retiredFiles++;}await control.publish({resource:"repository:graph-headroom-retirement",mode:"exclusive",elapsedMs:0});await control.yieldContinuation();};
   for(const row of snapshots.sort((a,b)=>a.modified<b.modified?-1:a.modified>b.modified?1:Buffer.compare(Buffer.from(a.path),Buffer.from(b.path)))){if(available(image)>=request.minimumAvailableBytes)break;await remove([row]);}
   if(available(image)<request.minimumAvailableBytes&&publication.length)await remove(publication);
   const availableBytes=available(image);if(availableBytes<request.minimumAvailableBytes)throw Error("Authenticated graph cache cannot fund required physical headroom: "+JSON.stringify({availableBytes,minimumAvailableBytes:request.minimumAvailableBytes,retiredBytes,retiredFiles}));console.log("[DEBUG] Actual exclusive graph image recovered authenticated snapshot/publication headroom "+JSON.stringify({availableBytes,retiredBytes,retiredFiles}));return availableBytes;
  });
  return Object.freeze({acquired:result.acquired,availableBytes:result.acquired?result.value:available(image),retiredBytes,retiredFiles});
 }finally{await image.close();}
}
