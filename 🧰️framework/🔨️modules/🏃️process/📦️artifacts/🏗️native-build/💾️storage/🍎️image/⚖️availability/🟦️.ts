import{spawnSync}from"node:child_process";
import{realpathSync}from"node:fs";
import imageSchema from"../../🧬️schema/🔣️.json" with { type: "json" };
import{nativeImageInformationV1}from"../📥️information/🟦️.ts";
const policy=imageSchema.definitions.ImageProtocol.const.reclaim.availability,refresh=policy.refresh;
/** 📏️ The slack within which a volume figure and the container figure still count as the same number. */
export function nativeImageAvailabilityToleranceV1(containerFreeBytes:number):number{return Math.max(policy.toleranceMinimumBytes,Math.floor(containerFreeBytes/policy.toleranceDivisor));}
/** ⚖️ Decides the bytes a mounted image volume can really take: the volume figure, unless the container reports more by more than the tolerance, which means the volume sampled the host free space when it mounted and never refreshed it; then the container figure, capped by the host free space when known. Pure and language-agnostic. */
export function nativeImageEffectiveAvailableBytesV1(volumeAvailableBytes:number,containerFreeBytes:number|undefined,hostFreeBytes?:number):number{
 if(containerFreeBytes===undefined||containerFreeBytes-volumeAvailableBytes<=nativeImageAvailabilityToleranceV1(containerFreeBytes))return volumeAvailableBytes;
 return Math.max(volumeAvailableBytes,hostFreeBytes===undefined?containerFreeBytes:Math.min(containerFreeBytes,hostFreeBytes));
}
/** 🧭️ Decides the bytes a build can really write: the host-capped container estimate, so a stale volume figure can neither hide real space nor outlive the host's own. Pure and language-agnostic. */
export function nativeImageRealAvailableBytesV1(volumeAvailableBytes:number,containerFreeBytes:number|undefined,hostFreeBytes:number):number{
 return Math.min(hostFreeBytes,nativeImageEffectiveAvailableBytesV1(volumeAvailableBytes,containerFreeBytes,hostFreeBytes));
}
/** 🔒️ Decides whether a mounted volume is latched: it reports nothing available although its container still holds more than the tolerance and the host has at least the latch headroom free, so the kernel refuses every write until the volume is remounted. Pure and language-agnostic. */
export function nativeImageVolumeLatchedV1(volumeAvailableBytes:number,containerFreeBytes:number|undefined,hostFreeBytes:number):boolean{
 return volumeAvailableBytes===0&&containerFreeBytes!==undefined&&containerFreeBytes>nativeImageAvailabilityToleranceV1(containerFreeBytes)&&hostFreeBytes>=policy.latchedHostFreeBytes;
}
/** 🔄️ Decides whether a detach and attach cycle would cure the volume figure: the effective availability exceeds what the volume reports by more than the tolerance, so the volume sampled the host free space when it attached. Pure and language-agnostic. */
export function nativeImageRefreshDueV1(volumeAvailableBytes:number,containerFreeBytes:number|undefined,hostFreeBytes?:number):boolean{
 return containerFreeBytes!==undefined&&nativeImageEffectiveAvailableBytesV1(volumeAvailableBytes,containerFreeBytes,hostFreeBytes)-volumeAvailableBytes>nativeImageAvailabilityToleranceV1(containerFreeBytes);
}
/** 🔎️ Decides whether a volume figure is low enough to be worth asking the container: below the probe bound and below the host free space by more than the tolerance. Pure and language-agnostic. */
export function nativeImageRefreshProbeDueV1(volumeAvailableBytes:number,hostFreeBytes:number):boolean{
 return volumeAvailableBytes<refresh.probeBelowBytes&&hostFreeBytes-volumeAvailableBytes>nativeImageAvailabilityToleranceV1(hostFreeBytes);
}
/** 🧭️ Names how an operating system escalates a refused detach of a quiescent image, or null where no volume of the repository can go stale. */
export function nativeImageRefreshMechanismV1(platform:string):Readonly<{forceUnmountProgram:string;forceUnmountArguments:readonly string[]}>|null{
 const row=(refresh.mechanisms as Record<string,{forceUnmountProgram:string;forceUnmountArguments:readonly string[]}>)[platform==="win32"?"windows":platform];
 return row===undefined?null:Object.freeze({forceUnmountProgram:row.forceUnmountProgram,forceUnmountArguments:Object.freeze([...row.forceUnmountArguments])});
}
/** ⏳️ Names how long a cycle that did not cure the figure waits before another may start. */
export const nativeImageRefreshRetryMillisecondsV1:number=refresh.retryMilliseconds;
/** 🧯️ Names the closed arguments of the forced detach that only a latched volume may use, because its transactions can never flush and a polite detach reports busy forever. See https://ss64.com/mac/hdiutil.html */
export const nativeImageForceDetachArgumentsV1:readonly string[]=Object.freeze([...policy.forceDetachArguments]);
/** 🔎️ Reads the free bytes of the APFS container from the plist of `diskutil info -plist <mount>`; undefined when the document has none. */
export function nativeImageContainerFreeBytesV1(plist:string):number|undefined{
 try{const value=nativeImageInformationV1(plist)[policy.containerFreeKey];return typeof value==="number"&&Number.isSafeInteger(value)&&value>=0?value:undefined;}catch{return undefined;}
}
type ImageInformationRunV1=(program:string,args:readonly string[])=>string|undefined;
const runImageInformationV1:ImageInformationRunV1=(program,args)=>{
 const row=spawnSync(program,[...args],{windowsHide:true,encoding:"utf8",timeout:10000,env:{PATH:"/usr/bin:/bin:/usr/sbin:/sbin",LANG:"C",LC_ALL:"C"}});
 return row.status===0?row.stdout:undefined;
};

/** 🩺️ Asks the system for the container free bytes of a mounted volume, `undefined` off macOS or when the tool fails. See https://ss64.com/mac/diskutil.html */
export function nativeImageContainerFreeBytesSyncV1(mountPoint:string,platform:string=process.platform,run:ImageInformationRunV1=runImageInformationV1):number|undefined{
 if(platform!=="darwin")return undefined;
 const text=run(policy.infoProgram,[...policy.infoArguments,mountPoint]);return text===undefined?undefined:nativeImageContainerFreeBytesV1(text);
}
/** 🧮️ Measures occupied bytes independently of APFS availability clamped by the hosting filesystem. */
export function nativeImageVolumeUsedBytesSyncV1(mountPoint:string,volume:Readonly<{blocks:number;bfree:number;bsize:number}>,platform:string=process.platform,run:ImageInformationRunV1=runImageInformationV1):number|undefined{
 const capacity=volume.blocks*volume.bsize;
 if(!Number.isSafeInteger(capacity)||capacity<=0)return undefined;
 if(platform!=="darwin"){
  const used=(volume.blocks-volume.bfree)*volume.bsize;
  return Number.isSafeInteger(used)&&used>=0&&used<=capacity?used:undefined;
 }
 try{
  const text=run(policy.infoProgram,[...policy.infoArguments,mountPoint]);
  if(text===undefined)return undefined;
  const facts=nativeImageInformationV1(text),used=facts[policy.volumeUsedKey];
  return (facts[policy.mountPointKey]===mountPoint||facts[policy.mountPointKey]===realpathSync(mountPoint))&&facts[policy.filesystemTypeKey]===policy.darwinFilesystemType&&typeof used==="number"&&Number.isSafeInteger(used)&&used>0&&used<=capacity?used:undefined;
 }catch{return undefined;}
}
/** ⏳️ Names how long a stale-figure correction stays valid before the container is asked again. */
export const nativeImageAvailabilityCorrectionMillisecondsV1:number=policy.correctionMilliseconds;
