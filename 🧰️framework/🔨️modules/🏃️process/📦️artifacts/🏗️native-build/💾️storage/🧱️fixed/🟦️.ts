import {randomBytes} from "node:crypto";
import {existsSync,lstatSync,statSync,statfsSync,readFileSync,writeFileSync,openSync,closeSync,readSync} from "node:fs";
import {join,resolve} from "node:path";
import {acquireQueuedResourceLease,acquireResourceLease,checkResourceLeaseControl,requireMaterializedPathV1,type ResourceLeaseControl} from "../../../../🔒️leases/🟦️.ts";
import type {NativeStoragePortV1} from "../🟦️.ts";
import {nativeImageRootV1,type NativeImageStorageV1} from "../🌐️platform/📋️contract/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 🐧️ Identifies one complete writable loop filesystem rather than an ordinary directory or subtree bind. */
export function nativeLinuxLoopMountV1(text:string,root:string):Readonly<{id:number;device:string;source:string}>{
 const decode=(value:string)=>value.replace(/\\(040|011|012|134)/gu,(_,octal)=>String.fromCharCode(parseInt(octal,8))),rows=text.trim().split("\n").map(line=>line.split(" ")),matches=rows.filter(row=>decode(row[4]??"")===root);
 if(matches.length!==1)throw Error("One exact loop filesystem mount required");const row=matches[0]!,split=row.indexOf("-");
 if(row[3]!=="/"||!/^7:\d+$/u.test(row[2]??"")||!(row[5]??"").split(",").includes("rw")||split<6||row[split+1]!=="ext4"||!/^\/dev\/loop\d+$/u.test(row[split+2]??"")||rows.some(other=>decode(other[4]??"").startsWith(root+"/")))throw Error("Exact bounded loop filesystem required");
 const id=Number(row[0]);if(!Number.isSafeInteger(id)||id<1)throw Error("Actual kernel mount identity required");return Object.freeze({id,device:row[2]!,source:row[split+2]!});
}

/** 🪟️ Admits only a checksummed fixed VHD footer and its full preallocated file envelope. */
export function nativeFixedVhdFooterV1(footer:Uint8Array,fileBytes:number,maximumBytes:number,filesystemBytes:number):number{
 if(footer.byteLength!==512)throw Error("Complete fixed VHD footer required");const bytes=Buffer.from(footer),checksum=bytes.readUInt32BE(64);bytes.fill(0,64,68);const size=Number(bytes.readBigUInt64BE(48));
 if(bytes.subarray(0,8).toString("ascii")!=="conectix"||bytes.readUInt32BE(60)!==2||bytes.readBigUInt64BE(16)!==0xffffffffffffffffn||bytes.readBigUInt64BE(40)!==BigInt(size)||((~bytes.reduce((sum,byte)=>sum+byte,0))>>>0)!==checksum||!Number.isSafeInteger(size)||size<33554432||size>filesystemBytes||fileBytes!==size+512||fileBytes>maximumBytes)throw Error("Actual fixed VHD capacity required");return size;
}

/** 🧭️ Retains the original enforcing platform preparation and exact system observation. */
export interface NativeFixedPlatformPortV1{
 readonly filename:string;
 prepare(options:NativeFixedPlatformOptionsV1,image:string,root:string,fresh:boolean):Promise<void>;
 observe(options:NativeFixedPlatformOptionsV1,image:string,root:string):Promise<string>;
}
/** 📏️ Declares the same finite envelope consumed by each system image backend. */
export type NativeFixedPlatformOptionsV1=Readonly<{directory:string;leaseDirectory:string;filesystemBytes:number;maximumPhysicalBytes:number;control:ResourceLeaseControl;maximumLeaseDatabaseBytes?:number}>;

/** 🏪️ Holds shared backing custody and exact inode, kernel mapping and finite filesystem identity. */
export async function openNativeFixedPlatformV1(options:NativeFixedPlatformOptionsV1,platform:NativeFixedPlatformPortV1):Promise<NativeImageStorageV1>{
 const {control}=options;checkResourceLeaseControl(control);const directory=requireMaterializedPathV1(options.directory),root=nativeImageRootV1(directory),image=requireMaterializedPathV1(join(directory,platform.filename)),receipt=requireMaterializedPathV1(join(directory,"image.json"));
 if(validateJsonSchemaSubset(schema,{version:1,root,image,filesystemBytes:options.filesystemBytes,maximumPhysicalBytes:options.maximumPhysicalBytes}).length||options.filesystemBytes%1048576||options.maximumPhysicalBytes<options.filesystemBytes+1048576)throw Error("Fixed platform capacity declaration required");
 const resource="native-fixed-image:"+directory,lease=(mode:"shared"|"exclusive")=>options.maximumLeaseDatabaseBytes!==undefined?acquireResourceLease({directory:options.leaseDirectory,resource,mode,signal:control.signal,control,maximumDatabaseBytes:options.maximumLeaseDatabaseBytes}):acquireQueuedResourceLease({directory:options.leaseDirectory,resource,mode,owner:randomBytes(8).toString("hex"),signal:control.signal,control,...(mode==="shared"?{sharedCohort:true}:{})});let shared=await lease("shared");
 try{
  const receiptIdentity=()=>{const record=lstatSync(receipt);if(!record.isFile()||record.isSymbolicLink()||record.nlink!==1||record.size>4096)throw Error("Finite original fixed platform receipt required");const authored=JSON.parse(readFileSync(receipt,"utf8")),file=lstatSync(image);if(authored.version!==1||authored.platform!==process.platform||authored.dev!==file.dev||authored.ino!==file.ino||authored.size!==file.size||!file.isFile()||file.isSymbolicLink()||file.nlink!==1)throw Error("Original fixed platform receipt changed");};
  let prepare=!existsSync(receipt);if(!prepare){receiptIdentity();try{await platform.observe(options,image,root);}catch{prepare=true;}}
  if(prepare){shared.release();const exclusive=await lease("exclusive");try{const fresh=!existsSync(receipt);if(!fresh)receiptIdentity();let mounted=false;if(!fresh)try{await platform.observe(options,image,root);mounted=true;}catch{}if(!mounted)await platform.prepare(options,image,root,fresh);if(fresh){const mapping=await platform.observe(options,image,root),file=lstatSync(image);if(!file.isFile()||file.isSymbolicLink()||file.nlink!==1||file.size>options.maximumPhysicalBytes)throw Error("Original fixed platform image required");writeFileSync(receipt,JSON.stringify({version:1,platform:process.platform,dev:file.dev,ino:file.ino,size:file.size,mapping})+"\n",{flag:"wx"});}}finally{exclusive.release();}shared=await lease("shared");}
  const file=lstatSync(image),authored=JSON.parse(readFileSync(receipt,"utf8"));if(lstatSync(receipt).isSymbolicLink()||authored.version!==1||authored.platform!==process.platform||authored.dev!==file.dev||authored.ino!==file.ino||authored.size!==file.size)throw Error("Original fixed platform receipt changed");const mapping=await platform.observe(options,image,root),filesystem=statfsSync(root),filesystemBytes=filesystem.blocks*filesystem.bsize,device=statSync(root).dev;
  if(filesystemBytes>options.filesystemBytes||filesystemBytes<1)throw Error("Actual finite platform filesystem required");const identity=JSON.stringify([process.platform,file.dev,file.ino,file.size,mapping]);let closed=false;
  const recheck=async()=>{checkResourceLeaseControl(control);if(closed)throw Error("Original fixed platform user released");const actual=lstatSync(image),fs=statfsSync(root);if(!actual.isFile()||actual.isSymbolicLink()||actual.nlink!==1||actual.dev!==file.dev||actual.ino!==file.ino||actual.size!==file.size||actual.size>options.maximumPhysicalBytes||statSync(root).dev!==device||fs.blocks*fs.bsize!==filesystemBytes||await platform.observe(options,image,root)!==mapping)throw Error("Actual fixed platform identity changed");};
  const port:NativeStoragePortV1={observe:async(policy,original)=>{if(original!==control)throw Error("Original platform control required");await recheck();return policy.root===root&&policy.capacityBytes>=options.maximumPhysicalBytes?{identity,root,maximumPhysicalBytes:options.maximumPhysicalBytes}:null;},recheck:async(backing,original)=>{if(original!==control||backing.root!==root||backing.identity!==identity)throw Error("Original platform backing required");await recheck();},retire:async()=>{throw Error("Original writer/reader physical retirement custody required");}};
  await recheck();return{root,image,filesystemBytes,maximumPhysicalBytes:options.maximumPhysicalBytes,port,recheck,close:async()=>{if(closed)return;closed=true;shared.release();},withExclusiveMaintenance:async body=>{await recheck();closed=true;shared.release();const exclusive=await lease("exclusive");try{closed=false;await recheck();return await body(port);}finally{closed=true;exclusive.release();}}};
 }catch(error){shared.release();throw error;}
}

/** 🔎️ Reads only the bounded fixed VHD footer instead of materializing the entire backing. */
export function readNativeFixedVhdFooterV1(image:string):Buffer{const fd=openSync(image,"r");try{const file=lstatSync(image),bytes=Buffer.alloc(512);if(!file.isFile()||file.isSymbolicLink()||file.nlink!==1||file.size<512||readSync(fd,bytes,0,512,file.size-512)!==512)throw Error("Original complete VHD file required");return bytes;}finally{closeSync(fd);}}
