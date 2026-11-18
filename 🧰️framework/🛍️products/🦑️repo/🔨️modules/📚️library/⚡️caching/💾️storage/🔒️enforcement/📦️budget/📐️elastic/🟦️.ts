import {mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {basename,dirname,join} from "node:path";
import {nativeImageReclaimDueV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim/🟦️.ts";
import {nativeFreeSpaceSyncV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";

/** 🔄️ The partitions whose image is sized elastically when it is created (`elastic` of the policy). */
export type ElasticPartitionIdV1="graph"|"products"|"rust";
/** 📐️ The elastic sizing rule of an image (`elastic.<partition>` of the policy). */
export type ElasticRuleV1=Readonly<{needBytes:number;headroomPercent:number;hostFreeReservePercent:number;alignBytes:number;maximumBytes:number;minimumBytes:number;reclaimFloorBytes:number;reclaimRatioPercent:number;reclaimTightHostFreeBytes:number;automatic:boolean;pressureFreeBytes:number}>;
/** 🖼️ Sizes a new image from its working set and the host: need times headroom, capped by the rule's maximum and by what the host volume may give (its free bytes minus the reserve), aligned down, never below the given minimum (the rule's `minimumBytes`, not the declared capacity: a host with less free space than the declared capacity still gets an image that fits). */
export function elasticCapacityV1(rule:ElasticRuleV1,minimumBytes:number,hostFreeBytes:number):number{
 const wanted=Math.ceil(rule.needBytes*(100+rule.headroomPercent)/100),ceiling=Math.min(rule.maximumBytes,Math.floor(hostFreeBytes*(100-rule.hostFreeReservePercent)/100)),target=Math.min(wanted,Math.max(ceiling,minimumBytes));
 return Math.max(minimumBytes,Math.floor(target/rule.alignBytes)*rule.alignBytes);
}
/** 🧾️ The size an image receipt pins: a whole number of MiB of at least 32 MiB, else null for an unreadable or malformed receipt; `missing` marks an image that is not born yet. */
export function readImageReceiptBytesV1(directory:string):number|null|"missing"{
 try{
  const bytes=(JSON.parse(readFileSync(join(directory,"image.json"),"utf8")) as {maximumPhysicalBytes?:unknown}).maximumPhysicalBytes;
  return Number.isSafeInteger(bytes)&&(bytes as number)%1048576===0&&(bytes as number)>=33554432?bytes as number:null;
 }catch(error){return (error as NodeJS.ErrnoException)?.code==="ENOENT"?"missing":null;}
}
/** 📈️ The size a pressed image is recreated at: half again its capacity (at least the next alignment step), never above the rule's maximum or what the host volume may give (its free bytes minus the reserve, plus the image's own host allocation that its removal returns); a capacity that cannot grow answers itself. */
export function elasticGrowthV1(rule:ElasticRuleV1,capacityBytes:number,hostFreeBytes:number,allocatedBytes:number=0):number{
 const ceiling=Math.min(rule.maximumBytes,Math.floor((hostFreeBytes+allocatedBytes)*(100-rule.hostFreeReservePercent)/100)),wanted=Math.ceil(capacityBytes*3/2/rule.alignBytes)*rule.alignBytes,target=Math.floor(Math.min(wanted,ceiling)/rule.alignBytes)*rule.alignBytes;
 return Math.max(capacityBytes,target);
}
/** 🧷️ Where the grown size of an elastic image waits for the next image (the control storage beside the images, one file for every partition): the reset removes the image directory together with its receipt, so the size must live outside it. */
export function elasticGrowthFileV1(directory:string):string{return join(dirname(directory),"c","elastic-growth.json");}
/** 📖️ The grown size recorded for the partition of `directory`, or 0. */
export function readElasticGrowthV1(directory:string):number{
 try{const bytes=(JSON.parse(readFileSync(elasticGrowthFileV1(directory),"utf8")) as Record<string,unknown>)[basename(directory)];return Number.isSafeInteger(bytes)&&(bytes as number)>0?bytes as number:0;}catch{return 0;}
}
/** 📝️ Records the grown size of the partition of `directory` for its next image. */
export function writeElasticGrowthV1(directory:string,bytes:number):void{
 const file=elasticGrowthFileV1(directory);let rows:Record<string,unknown>={};
 try{rows=JSON.parse(readFileSync(file,"utf8"));}catch{}
 mkdirSync(dirname(file),{recursive:true});writeFileSync(file,JSON.stringify({...rows,[basename(directory)]:bytes}));
}
/** 🪪️ The capacity of an elastic image: the one its receipt pins (clamped down to the rule's maximum, never snapped up to the declared minimum) when an image exists, else the elastic size a new image would get, raised to the grown size a pressed predecessor recorded as far as the host allows; an unreadable receipt keeps the declared capacity. */
export function effectiveCapacityV1(directory:string,minimumBytes:number,rule:ElasticRuleV1):number{
 const receipt=readImageReceiptBytesV1(directory);
 if(receipt===null)return minimumBytes;
 if(receipt!=="missing")return Math.min(receipt,rule.maximumBytes);
 const host=nativeFreeSpaceSyncV1(dirname(directory));if(!host)return minimumBytes;
 const base=elasticCapacityV1(rule,minimumBytes,host.availableBytes),grown=readElasticGrowthV1(directory);
 return Math.max(base,Math.min(grown,rule.maximumBytes,Math.floor(host.availableBytes*(100-rule.hostFreeReservePercent)/100/rule.alignBytes)*rule.alignBytes));
}
/** 📌️ The capacity of a partition without an elastic rule: the one its receipt pins when an image exists and fits the declared capacity (an image keeps the size it was born with, a larger declaration takes effect only through the explicit reset that births a new image), else the declared capacity; a larger or unreadable receipt keeps the declared capacity so the receiver refuses it as before. */
export function pinnedCapacityV1(directory:string,declaredBytes:number):number{
 const receipt=readImageReceiptBytesV1(directory);
 return typeof receipt==="number"&&receipt<=declaredBytes?receipt:declaredBytes;
}

/** 🧲️ Whether an image holds host blocks it no longer uses: the host allocation is compared with the bytes used inside (at least the ratio and at least the floor, an eighth of the floor while the host volume is nearly full). The decision is the framework's `nativeImageReclaimDueV1`; the rule only carries the three numbers. */
export function elasticBloatedV1(rule:Pick<ElasticRuleV1,"reclaimFloorBytes"|"reclaimRatioPercent"|"reclaimTightHostFreeBytes">,allocatedBytes:number,usedBytes:number,hostFreeBytes:number):boolean{
 return nativeImageReclaimDueV1(rule,allocatedBytes,usedBytes,hostFreeBytes);
}
