import {readFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {nativeFreeSpaceSyncV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";

/** 🔄️ The partitions whose image is sized elastically when it is created (`elastic` of the policy). */
export type ElasticPartitionIdV1="graph"|"products"|"rust";
/** 📐️ The elastic sizing rule of an image (`elastic.<partition>` of the policy). */
export type ElasticRuleV1=Readonly<{needBytes:number;headroomPercent:number;hostFreeReservePercent:number;alignBytes:number;maximumBytes:number;automatic:boolean;pressureFreeBytes:number}>;
/** 🖼️ Sizes a new image from its working set and the host: need times headroom, capped by the rule's maximum and by what the host volume may give (its free bytes minus the reserve), aligned down, never below the declared capacity. */
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
/** 🪪️ The capacity of an elastic image: the one its receipt pins (clamped down to the rule's maximum, never snapped up to the declared minimum) when an image exists, else the elastic size a new image would get; an unreadable receipt keeps the declared capacity. */
export function effectiveCapacityV1(directory:string,minimumBytes:number,rule:ElasticRuleV1):number{
 const receipt=readImageReceiptBytesV1(directory);
 if(receipt===null)return minimumBytes;
 if(receipt!=="missing")return Math.min(receipt,rule.maximumBytes);
 const host=nativeFreeSpaceSyncV1(dirname(directory));return host?elasticCapacityV1(rule,minimumBytes,host.availableBytes):minimumBytes;
}
/** 📌️ The capacity of a partition without an elastic rule: the one its receipt pins when an image exists and fits the declared capacity (an image keeps the size it was born with, a larger declaration takes effect only through the explicit reset that births a new image), else the declared capacity; a larger or unreadable receipt keeps the declared capacity so the receiver refuses it as before. */
export function pinnedCapacityV1(directory:string,declaredBytes:number):number{
 const receipt=readImageReceiptBytesV1(directory);
 return typeof receipt==="number"&&receipt<=declaredBytes?receipt:declaredBytes;
}
