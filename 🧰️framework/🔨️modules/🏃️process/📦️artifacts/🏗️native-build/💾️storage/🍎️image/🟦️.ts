import {randomBytes} from "node:crypto";
import {mkdirSync,lstatSync,statSync,statfsSync,rmSync,existsSync,readdirSync,writeFileSync,readFileSync} from "node:fs";
import {join,resolve,relative,isAbsolute} from "node:path";
import {captureOwnedCommandV1} from "../../../../🎛️owned-execution/🟦️.ts";
import {acquireResourceLease,acquireQueuedResourceLease,checkResourceLeaseControl,requireMaterializedPathV1,type ResourceLeaseControl} from "../../../../🔒️leases/🟦️.ts";
import type {NativeStoragePortV1,NativeStoragePolicyV1,NativeStorageBackingV1} from "../🟦️.ts";
import {nativeStorageStateV1,nativeStorageEventV1,nativeStorageUsedBytesV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

import {readNativeImageRetirementFileV1,requireNativeImageRetirementStateV1} from "../🧱️fixed/🟦️.ts";
const protocol=schema.definitions.ImageProtocol.const;

import {readNativeImageMaintenanceRequestV1,requireNativeImageEnvelopeV1,nativeImageRootV1,requireNativeStoragePathV1,type NativeImageStorageV1,type NativeImageMaintenanceRequestV1} from "../🌐️platform/📋️contract/🟦️.ts";

/** 🏷️ Decodes the system tool's plist primitives without a runtime XML library. */
export function nativeImageInformationV1(xml:string):Record<string,unknown>{
 const tokens=xml.replace(/<\?[\s\S]*?\?>|<!DOCTYPE[^>]*>/gu,"").match(/<[^>]+>|[^<]+/gu)??[];let index=0;
 const peek=():string=>{while(index<tokens.length&&!tokens[index]!.trim())index++;return tokens[index]??"";},take=():string=>{const token=peek();index++;return token;};
 const decode=(value:string)=>value.replace(/&(amp|lt|gt|quot|apos);/gu,(_,key)=>({amp:"&",lt:"<",gt:">",quot:'"',apos:"'"} as Record<string,string>)[key]!);
 const value=():unknown=>{const token=take();if(token==="<true/>")return true;if(token==="<false/>")return false;if(token==="<array/>")return[];if(token==="<dict/>")return{};
  if(token==="<dict>"){const result:Record<string,unknown>=Object.create(null);while(peek()!=="</dict>"){if(take()!=="<key>")throw Error("Invalid image plist key");const key=decode(take());if(take()!=="</key>"||Object.hasOwn(result,key))throw Error("Invalid image plist dictionary");result[key]=value();}take();return result;}
  if(token==="<array>"){const result:unknown[]=[];while(peek()!=="</array>"){if(!peek())throw Error("Incomplete image plist array");result.push(value());}take();return result;}
  const scalar=/^<(string|integer|real|data|date)>$/u.exec(token);if(scalar){const text=peek()===`</${scalar[1]}>`?"":take();if(take()!==`</${scalar[1]}>`)throw Error("Invalid image plist scalar");return scalar[1]==="integer"||scalar[1]==="real"?Number(text):decode(text);}
  throw Error("Unsupported system image plist token");};
 if(!take().startsWith("<plist "))throw Error("System image plist required");const result=value();if(take()!=="</plist>"||peek()||!result||typeof result!=="object"||Array.isArray(result))throw Error("Invalid system image plist");return result as Record<string,unknown>;
}

/** 📄️ Captures complete bounded system image metadata before strict structural admission. */
async function readNativeImageInformationV1(directory:string,control:ResourceLeaseControl):Promise<Record<string,unknown>>{
 return nativeImageInformationV1(await captureOwnedCommandV1("hdiutil",["info","-plist"],directory,{control,output:{maximumBytes:protocol.informationMaximumBytes,maximumLines:protocol.informationMaximumLines}}));
}

/** 🏪️ Reopens one persistent fixed image under shared custody; its lease directory fixes the sole queue catalog. */
export async function openNativeImageStorageV1(options:Readonly<{directory:string;leaseDirectory:string;filesystemBytes:number;maximumPhysicalBytes:number;control:ResourceLeaseControl;maximumLeaseDatabaseBytes?:number}>):Promise<NativeImageStorageV1>{
 const {control}=options;checkResourceLeaseControl(control);if(process.platform!=="darwin")throw Error("A qualified fixed-volume backend is required on this platform");
 requireNativeImageEnvelopeV1(options);
 const directory=requireMaterializedPathV1(options.directory),image=requireMaterializedPathV1(join(directory,"fixed.dmg")),root=nativeImageRootV1(directory),receipt=requireMaterializedPathV1(join(directory,"image.json")),resource="native-fixed-image:"+directory;
 const acquire=async(mode:"shared"|"exclusive")=>options.maximumLeaseDatabaseBytes===undefined?acquireQueuedResourceLease({directory:options.leaseDirectory,resource,mode,owner:randomBytes(16).toString("base64url"),signal:control.signal,control,sharedCohort:mode==="shared"}):acquireResourceLease({directory:options.leaseDirectory,resource,mode,signal:control.signal,control,maximumDatabaseBytes:options.maximumLeaseDatabaseBytes});
 let shared=await acquire("shared");
 let created:NativeImageStorageV1|undefined;
 const capture=(command:string,args:readonly string[])=>captureOwnedCommandV1(command,args,directory,{control});
 if(!existsSync(receipt)||!existsSync(root)||!existsSync(image)||statSync(root).dev===lstatSync(image).dev){
 shared.release();const exclusive=await acquire("exclusive");
 try{
  if(!existsSync(directory)){created=await provisionNativeImageStorageV1(options);const physical=lstatSync(image);writeFileSync(receipt,JSON.stringify({version:1,filesystemBytes:options.filesystemBytes,maximumPhysicalBytes:created.maximumPhysicalBytes,dev:physical.dev,ino:physical.ino,size:physical.size})+"\n");}
  if(!existsSync(receipt)||lstatSync(receipt).isSymbolicLink())throw Error("Original fixed image receipt required");
  const authored=JSON.parse(readFileSync(receipt,"utf8")),physical=lstatSync(image);
  if(validateJsonSchemaSubset(schema.definitions.ImageReceipt,authored,schema).length||authored.version!==1||authored.filesystemBytes!==options.filesystemBytes||authored.maximumPhysicalBytes>options.maximumPhysicalBytes||!physical.isFile()||physical.isSymbolicLink()||physical.nlink!==1||physical.dev!==authored.dev||physical.ino!==authored.ino||physical.size!==authored.size)throw Error("Original fixed image identity changed");
  if(statSync(root).dev===physical.dev)await capture("hdiutil",["attach","-nobrowse","-noautoopen","-mountpoint",root,image]);
 }finally{exclusive.release();}
 shared=await acquire("shared");
 }
 try{
  const authored=JSON.parse(readFileSync(receipt,"utf8")),physical=lstatSync(image),mountedDevice=statSync(root).dev,filesystem=statfsSync(root),filesystemBytes=filesystem.blocks*filesystem.bsize;
  if(validateJsonSchemaSubset(schema.definitions.ImageReceipt,authored,schema).length||authored.version!==1||authored.filesystemBytes!==options.filesystemBytes||authored.maximumPhysicalBytes>options.maximumPhysicalBytes||!physical.isFile()||physical.isSymbolicLink()||physical.nlink!==1||physical.dev!==authored.dev||physical.ino!==authored.ino||physical.size!==authored.size)throw Error("Original fixed image identity changed");
  const information=await readNativeImageInformationV1(directory,control),images=information.images as Record<string,unknown>[]|undefined;
  if(!Array.isArray(images)||!images.some(row=>row["image-path"]===image&&Array.isArray(row["system-entities"])&&(row["system-entities"] as Record<string,unknown>[]).some(entity=>entity["mount-point"]===root))||mountedDevice===physical.dev||filesystemBytes>options.filesystemBytes)throw Error("Actual mounted image ownership required: "+JSON.stringify({image,root,mountedDevice,physicalDevice:physical.dev,filesystemBytes,maximumFilesystemBytes:options.filesystemBytes,observedImages:Array.isArray(images)?images.length:null,matchingImages:Array.isArray(images)?images.filter(row=>row["image-path"]===image).slice(0,4).map(row=>({path:row["image-path"],entities:Array.isArray(row["system-entities"])?(row["system-entities"] as Record<string,unknown>[]).slice(0,8).map(entity=>({device:String(entity["dev-entry"]??"").slice(0,512),mount:String(entity["mount-point"]??"").slice(0,512)})):null})):null}));
  const identity=JSON.stringify([physical.dev,physical.ino,physical.size,mountedDevice]);let closed=false;
  const recheck=async()=>{checkResourceLeaseControl(control);if(closed)throw Error("Original image invocation released");const actual=lstatSync(image),fs=statfsSync(root);if(actual.dev!==authored.dev||actual.ino!==authored.ino||actual.size!==authored.size||actual.nlink!==1||actual.blocks*512>authored.maximumPhysicalBytes||statSync(root).dev!==mountedDevice||fs.blocks*fs.bsize!==filesystemBytes)throw Error("Shared fixed image identity changed");};
  const port:NativeStoragePortV1={observe:async(policy,original)=>{if(original!==control)throw Error("Original fixed image control identity required");await recheck();return policy.root===root&&policy.capacityBytes>=authored.maximumPhysicalBytes?{identity,root,maximumPhysicalBytes:authored.maximumPhysicalBytes}:null;},recheck:async(backing,original)=>{if(original!==control||backing.identity!==identity||backing.root!==root)throw Error("Original fixed image backing required");await recheck();},retire:async()=>{throw Error("Original allocation writer/reader physical retirement custody required");}};
  await recheck();return{root,image,maximumPhysicalBytes:authored.maximumPhysicalBytes,filesystemBytes,port,recheck,close:async()=>{if(closed)return;closed=true;shared.release();},withExclusiveMaintenance:async(request,body)=>{const admitted=readNativeImageMaintenanceRequestV1(request);await recheck();checkResourceLeaseControl(control);closed=true;shared.release();let exclusive;try{exclusive=admitted.wait?await acquire("exclusive"):await acquireResourceLease({directory:options.leaseDirectory,resource,mode:"exclusive",signal:control.signal,control,timeoutMs:0,...(options.maximumLeaseDatabaseBytes===undefined?{}:{maximumDatabaseBytes:options.maximumLeaseDatabaseBytes})});}catch(error){checkResourceLeaseControl(control);if(!admitted.wait&&error instanceof Error&&error.message==="Resource lease timed out: "+resource)return{acquired:false};throw error;}try{closed=false;await recheck();checkResourceLeaseControl(control);const value=await body(port);checkResourceLeaseControl(control);await recheck();checkResourceLeaseControl(control);return{acquired:true,value};}finally{closed=true;exclusive.release();}}};
 }catch(error){shared.release();throw error;}
}

/** ♻️ Retires only the exact owned image after all original shared users have settled. */
export async function retireNativeImageStorageV1(options:Readonly<{directory:string;leaseDirectory:string;control:ResourceLeaseControl;maximumLeaseDatabaseBytes?:number;filesystemBytes:number;maximumPhysicalBytes:number}>,request:NativeImageMaintenanceRequestV1={version:1,wait:true}):Promise<boolean>{
 requireNativeImageEnvelopeV1(options);const admitted=readNativeImageMaintenanceRequestV1(request),{control}=options;checkResourceLeaseControl(control);if(process.platform!=="darwin")throw Error("macOS fixed image enforcement is unavailable on this platform");const directory=requireMaterializedPathV1(options.directory),image=join(directory,"fixed.dmg"),root=nativeImageRootV1(directory),receipt=join(directory,"image.json"),resource="native-fixed-image:"+directory;let exclusive;
 try{exclusive=admitted.wait&&options.maximumLeaseDatabaseBytes===undefined?await acquireQueuedResourceLease({directory:options.leaseDirectory,resource,mode:"exclusive",owner:randomBytes(16).toString("base64url"),signal:control.signal,control}):await acquireResourceLease({directory:options.leaseDirectory,resource,mode:"exclusive",signal:control.signal,control,...(!admitted.wait?{timeoutMs:0}:{}),...(options.maximumLeaseDatabaseBytes===undefined?{}:{maximumDatabaseBytes:options.maximumLeaseDatabaseBytes})});}catch(error){checkResourceLeaseControl(control);if(!admitted.wait&&error instanceof Error&&error.message==="Resource lease timed out: "+resource)return false;throw error;}
 try{
  const record=readNativeImageRetirementFileV1(receipt,4096),authored=JSON.parse(record.bytes.toString()),physical=lstatSync(image);if(validateJsonSchemaSubset(schema.definitions.ImageReceipt,authored,schema).length||!physical.isFile()||physical.isSymbolicLink()||physical.nlink!==1||physical.dev!==authored.dev||physical.ino!==authored.ino||physical.size!==authored.size||physical.size>authored.maximumPhysicalBytes||options.filesystemBytes!==authored.filesystemBytes||authored.maximumPhysicalBytes>options.maximumPhysicalBytes)throw Error("Original fixed image retirement identity changed");
  requireNativeImageRetirementStateV1(root,authored.maximumPhysicalBytes);
  if(statSync(root).dev!==physical.dev){const information=await readNativeImageInformationV1(directory,control),images=information.images as Record<string,unknown>[];if(!Array.isArray(images)||!images.some(row=>row["image-path"]===image&&Array.isArray(row["system-entities"])&&(row["system-entities"] as Record<string,unknown>[]).some(entity=>entity["mount-point"]===root)))throw Error("Actual owned image retirement mount required");if(readNativeImageRetirementFileV1(receipt,4096).stamp!==record.stamp)throw Error("Original image retirement receipt changed");await captureOwnedCommandV1("hdiutil",["detach",root],directory,{control});}
  checkResourceLeaseControl(control);const after=lstatSync(image);if(statSync(root).dev!==physical.dev||after.dev!==physical.dev||after.ino!==physical.ino||after.size!==physical.size||!after.isFile()||after.isSymbolicLink()||after.nlink!==1||readNativeImageRetirementFileV1(receipt,4096).stamp!==record.stamp)throw Error("Original image remains mounted or changed");rmSync(directory,{recursive:true});return true;
 }finally{exclusive.release();}
}

/** 🗜️ Provisions only a new caller-owned fixed image; ordinary volumes and sparse images are never adopted. */
export async function provisionNativeImageStorageV1(options:Readonly<{directory:string;filesystemBytes:number;maximumPhysicalBytes:number;control:ResourceLeaseControl}>):Promise<NativeImageStorageV1>{
 const {control}=options;checkResourceLeaseControl(control);if(process.platform!=="darwin")throw Error("macOS fixed image enforcement is unavailable on this platform");
 requireNativeImageEnvelopeV1(options);
 const directory=requireMaterializedPathV1(options.directory),image=requireMaterializedPathV1(join(directory,"fixed.dmg")),root=nativeImageRootV1(directory);
 if(existsSync(directory))throw Error("Fixed image owner must have a fresh directory");mkdirSync(directory,{recursive:true});mkdirSync(root);let mounted=false,closed=false;
 const capture=(command:string,args:readonly string[])=>captureOwnedCommandV1(command,args,directory,{control});
 try{
  const available=statfsSync(directory);if(available.bavail*available.bsize<options.maximumPhysicalBytes+67108864)throw Error("Host storage cannot admit fixed image envelope");
  await capture("hdiutil",["create","-size",String(options.filesystemBytes/protocol.unitBytes)+"m","-type",protocol.format,"-nospotlight","-fs",protocol.filesystem,"-volname","semio-"+randomBytes(4).toString("hex"),image]);
  const initial=lstatSync(image),hostBlock=statfsSync(directory).bsize,maximumPhysicalBytes=Math.ceil(initial.size/hostBlock)*hostBlock;
  if(!initial.isFile()||initial.isSymbolicLink()||initial.nlink!==1||maximumPhysicalBytes>options.maximumPhysicalBytes)throw Error("Fixed image host allocation exceeds admitted envelope");
  await capture("hdiutil",["attach","-nobrowse","-noautoopen","-mountpoint",root,image]);mounted=true;
  const mountedDevice=statSync(root).dev,filesystem=statfsSync(root),filesystemBytes=filesystem.blocks*filesystem.bsize;
  if(mountedDevice===initial.dev||filesystemBytes>options.filesystemBytes)throw Error("Actual mounted filesystem is not the bounded image");
  const identity=JSON.stringify([initial.dev,initial.ino,initial.size,mountedDevice]);
  const recheck=async():Promise<void>=>{
   checkResourceLeaseControl(control);if(closed||!mounted)throw Error("Fixed image owner is closed");const actual=lstatSync(image),fs=statfsSync(root);
   if(!actual.isFile()||actual.isSymbolicLink()||actual.dev!==initial.dev||actual.ino!==initial.ino||actual.size!==initial.size||actual.nlink!==1||actual.blocks*512>maximumPhysicalBytes||statSync(root).dev!==mountedDevice||fs.blocks*fs.bsize!==filesystemBytes)throw Error("Fixed image physical identity changed");
  };
  const port:NativeStoragePortV1={observe:async(policy:NativeStoragePolicyV1,original:ResourceLeaseControl)=>{if(original!==control)throw Error("Original fixed image control identity required");await recheck();return policy.root===root&&policy.capacityBytes>=maximumPhysicalBytes?{identity,root,maximumPhysicalBytes}:null;},recheck:async(backing:NativeStorageBackingV1,original:ResourceLeaseControl)=>{if(original!==control||backing.identity!==identity||backing.root!==root)throw Error("Original fixed image backing required");await recheck();},retire:async()=>{throw Error("Allocation physical retirement needs its original owned writer and reader custody");}};
  await recheck();console.log("[DEBUG] Fixed UDRW image "+JSON.stringify({image,root,fileBytes:initial.size,allocatedBytes:initial.blocks*512,maximumPhysicalBytes,filesystemBytes,device:mountedDevice}));
  return{root,image,maximumPhysicalBytes,filesystemBytes,port,recheck,withExclusiveMaintenance:async(request,body)=>{readNativeImageMaintenanceRequestV1(request);await recheck();checkResourceLeaseControl(control);const value=await body(port);checkResourceLeaseControl(control);await recheck();checkResourceLeaseControl(control);return{acquired:true,value};},close:async()=>{if(closed)return;await recheck();await capture("hdiutil",["detach",root]);mounted=false;if(statSync(root).dev===mountedDevice)throw Error("Fixed image remains mounted");closed=true;rmSync(directory,{recursive:true});console.log("[DEBUG] Original fixed image detached and owned backing retired");}};
 }catch(error){if(mounted)try{await capture("hdiutil",["detach",root]);mounted=false;}catch{}if(!mounted)rmSync(directory,{recursive:true});throw error;}
}

