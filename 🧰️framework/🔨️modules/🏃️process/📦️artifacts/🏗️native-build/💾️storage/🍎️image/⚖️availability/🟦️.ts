import{spawnSync}from"node:child_process";
import imageSchema from"../../🧬️schema/🔣️.json" with { type: "json" };
import{nativeImageInformationV1}from"../📥️information/🟦️.ts";
const policy=imageSchema.definitions.ImageProtocol.const.reclaim.availability;
/** 📏️ The slack within which a volume figure and the container figure still count as the same number. */
export function nativeImageAvailabilityToleranceV1(containerFreeBytes:number):number{return Math.max(policy.toleranceMinimumBytes,Math.floor(containerFreeBytes/policy.toleranceDivisor));}
/** ⚖️ Decides the bytes a mounted image volume can really take: the volume figure, unless the container reports more by more than the tolerance, which means the volume sampled the host free space when it mounted and never refreshed it; then the container figure, capped by the host free space when known. Pure and language-agnostic. */
export function nativeImageEffectiveAvailableBytesV1(volumeAvailableBytes:number,containerFreeBytes:number|undefined,hostFreeBytes?:number):number{
 if(containerFreeBytes===undefined||containerFreeBytes-volumeAvailableBytes<=nativeImageAvailabilityToleranceV1(containerFreeBytes))return volumeAvailableBytes;
 return Math.max(volumeAvailableBytes,hostFreeBytes===undefined?containerFreeBytes:Math.min(containerFreeBytes,hostFreeBytes));
}
/** 🔎️ Reads the free bytes of the APFS container from the plist of `diskutil info -plist <mount>`; undefined when the document has none. */
export function nativeImageContainerFreeBytesV1(plist:string):number|undefined{
 try{const value=nativeImageInformationV1(plist)[policy.containerFreeKey];return typeof value==="number"&&Number.isSafeInteger(value)&&value>=0?value:undefined;}catch{return undefined;}
}
type ImageInformationRunV1=(program:string,args:readonly string[])=>string|undefined;
const runImageInformationV1:ImageInformationRunV1=(program,args)=>{
 const row=spawnSync(program,[...args],{encoding:"utf8",timeout:10000,env:{PATH:"/usr/bin:/bin:/usr/sbin:/sbin",LANG:"C",LC_ALL:"C"}});
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
  return facts[policy.mountPointKey]===mountPoint&&facts[policy.filesystemTypeKey]===policy.darwinFilesystemType&&typeof used==="number"&&Number.isSafeInteger(used)&&used>0&&used<=capacity?used:undefined;
 }catch{return undefined;}
}
/** ⏳️ Names how long a stale-figure correction stays valid before the container is asked again. */
export const nativeImageAvailabilityCorrectionMillisecondsV1:number=policy.correctionMilliseconds;
