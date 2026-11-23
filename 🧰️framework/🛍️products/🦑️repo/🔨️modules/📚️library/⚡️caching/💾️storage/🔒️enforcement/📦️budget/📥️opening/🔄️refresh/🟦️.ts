import {createHash} from "node:crypto";
import {existsSync,lstatSync,mkdirSync,readFileSync,statfsSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {nativeImageRootV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts";
import {nativeRecomputableRetirementPendingV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🧹️whole/🟦️.ts";
import {nativeImageContainerFreeBytesSyncV1,nativeImageRefreshDueV1,nativeImageRefreshMechanismV1,nativeImageRefreshProbeDueV1,nativeImageRefreshRetryMillisecondsV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/⚖️availability/🟦️.ts";
import {createNativeImageReclaimPortV1,nativeImageReclaimMechanismV1,refreshNativeImageV1,type NativeImageReclaimPortV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim/🟦️.ts";
import {ensureNativeImageReserveV1,releaseNativeImageReserveV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🛟️reserve/🟦️.ts";
import {acquireResourceLease,checkResourceLeaseControl,ResourceLeaseTimeoutErrorV1,type ResourceLeaseControl} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🧾️ The measured reason a stale volume figure was not cured by this attempt. */
export type ImageRefreshSkipReasonV1=(typeof schema.$defs.SkipReason.enum)[number];
/** 🧾️ The receipt of one nonwaiting refresh attempt: skipped with its reason, or refreshed with the figures the volume reported before and after and whether the stale gap is gone. */
export type ImageRefreshReceiptV1=Readonly<{version:1;kind:"repository-image-refresh"}&(
 {outcome:"skipped";reason:ImageRefreshSkipReasonV1}|
 {outcome:"refreshed";beforeAvailableBytes?:number;afterAvailableBytes?:number;cured:boolean})>;
/** 🔌️ Supplies every effect of one refresh attempt, so a fake clock, fake lease, fake volume and fake port run the same decisions as the real machine; each member defaults to the host. */
export type ImageRefreshPortsV1=Readonly<{
 platform?:string;now?:()=>number;exists?:(path:string)=>boolean;retirementPending?:(directory:string)=>boolean;mounted?:(directory:string,root:string)=>boolean;
 volumeAvailableBytes?:(root:string)=>number|undefined;hostFreeBytes?:(directory:string)=>number|undefined;containerFreeBytes?:(root:string)=>number|undefined;
 lastCycleAt?:(directory:string)=>number|undefined;stampCycle?:(directory:string,at:number)=>void;
 lease?:(resource:string)=>Promise<Readonly<{release():void}>|undefined>;
 port?:(directory:string,image:string,root:string)=>NativeImageReclaimPortV1|null;
 releaseReserve?:(root:string)=>unknown;ensureReserve?:(root:string)=>unknown;
}>;
const stampPath=(directory:string)=>join(tmpdir(),"semio-image-refresh-"+createHash("sha256").update(directory).digest("hex").slice(0,16)+".json");
const free=(path:string)=>{try{const row=statfsSync(path);return Number(row.bavail)*row.bsize;}catch{return undefined;}};

/** 🔄️ Makes the volume of one idle image report its true available space again: when the volume figure is low and the container holds more than the tolerance beyond it, the image is quiescent (the exclusive `native-fixed-image:<directory>` lease is obtained without waiting, which proves that no repository process uses it) and no cycle ran within the retry window, it releases the reserve, detaches and attaches the image at its mount point (a busy refusal escalates to a forced unmount only under that proof) and creates the reserve again. Skipped with its reason on every other path; never throws except for cancellation. The cycle is the same effect as the reclaim of host blocks, so a macOS volume that sampled the host free space when it attached and latched at zero is cured without a manual remount.
 * @see ../♻️reset/🟦️.ts */
export async function refreshStaleImageIfQuiescentV1(options:Readonly<{directory:string;leaseDirectory:string;control:ResourceLeaseControl;ports?:ImageRefreshPortsV1}>):Promise<ImageRefreshReceiptV1>{
 const{directory,leaseDirectory,control}=options,ports=options.ports??{},platform=ports.platform??process.platform,now=ports.now??Date.now;
 const receipt={version:1,kind:"repository-image-refresh"} as const,skip=(reason:ImageRefreshSkipReasonV1):ImageRefreshReceiptV1=>({...receipt,outcome:"skipped",reason});
 const mechanism=nativeImageReclaimMechanismV1(platform);
 if(mechanism===null||nativeImageRefreshMechanismV1(platform)===null)return skip("platform");
 let failure:ImageRefreshSkipReasonV1="refresh-failed";
 try{
  checkResourceLeaseControl(control);
  const root=nativeImageRootV1(directory),image=join(directory,mechanism.imageName),resource="native-fixed-image:"+directory;
  if((ports.retirementPending??nativeRecomputableRetirementPendingV1)(directory))return skip("retirement-pending");
  if(!(ports.exists??existsSync)(image))return skip("backing-unavailable");
  if(!(ports.mounted??((parent,mount)=>{try{return lstatSync(mount).dev!==lstatSync(parent).dev;}catch{return false;}}))(directory,root))return skip("not-mounted");
  const volumeOf=ports.volumeAvailableBytes??free,hostOf=ports.hostFreeBytes??free,containerOf=ports.containerFreeBytes??nativeImageContainerFreeBytesSyncV1;
  const due=()=>{
   const volume=volumeOf(root),host=hostOf(directory);
   if(volume===undefined||host===undefined)return"measurement-unavailable" as const;
   if(!nativeImageRefreshProbeDueV1(volume,host))return"fresh" as const;
   const container=containerOf(root);
   if(container===undefined)return"measurement-unavailable" as const;
   return nativeImageRefreshDueV1(volume,container,host)?"due" as const:"fresh" as const;
  };
  const first=due();if(first!=="due")return skip(first);
  const last=(ports.lastCycleAt??(target=>{try{return(JSON.parse(readFileSync(stampPath(target),"utf8")) as {at:number}).at;}catch{return undefined;}}))(directory);
  if(last!==undefined&&now()-last<nativeImageRefreshRetryMillisecondsV1)return skip("retry-later");
  failure="lease-failed";
  const lease=await(ports.lease??(async name=>{
   try{return await acquireResourceLease({directory:leaseDirectory,resource:name,mode:"exclusive",signal:control.signal,control,timeoutMs:0});}
   catch(error){if(error instanceof ResourceLeaseTimeoutErrorV1)return undefined;throw error;}
  }))(resource);
  if(lease===undefined)return skip("lease-timeout");
  try{
   failure="refresh-failed";
   checkResourceLeaseControl(control);
   const second=due();if(second!=="due")return skip(second);
   (ports.stampCycle??((target,at)=>{try{mkdirSync(tmpdir(),{recursive:true});writeFileSync(stampPath(target),JSON.stringify({at}));}catch{}}))(directory,now());
   const run=(program:string,args:readonly string[])=>captureOwnedCommandV1(program,args,directory,{control,output:{maximumBytes:1048576,maximumLines:32768}});
   const port=(ports.port??((dir,file,mount)=>createNativeImageReclaimPortV1(platform,{image:file,cwd:dir,mountPoint:mount,quiescent:true,run,cleanupRun:(program,args)=>captureOwnedCommandV1(program,args,dir,{output:{maximumBytes:1048576,maximumLines:32768}})})))(directory,image,root);
   if(port===null)return skip("backing-unavailable");
   const result=await refreshNativeImageV1(port,{check:()=>checkResourceLeaseControl(control),before:()=>{(ports.releaseReserve??releaseNativeImageReserveV1)(root);},after:()=>{(ports.ensureReserve??ensureNativeImageReserveV1)(root);}});
   checkResourceLeaseControl(control);
   const volume=volumeOf(root),host=hostOf(directory),container=containerOf(root);
   return{...receipt,outcome:"refreshed",...result.beforeAvailableBytes===undefined?{}:{beforeAvailableBytes:result.beforeAvailableBytes},...result.afterAvailableBytes===undefined?{}:{afterAvailableBytes:result.afterAvailableBytes},cured:volume!==undefined&&host!==undefined&&container!==undefined&&!nativeImageRefreshDueV1(volume,container,host)};
  }finally{
   try{lease.release();}catch(error){failure="lease-release-failed";throw error;}
  }
 }catch(error){
  checkResourceLeaseControl(control);
  return skip(failure);
 }
}
