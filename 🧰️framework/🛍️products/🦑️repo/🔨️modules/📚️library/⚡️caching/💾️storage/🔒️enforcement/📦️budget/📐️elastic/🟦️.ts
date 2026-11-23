import {mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {basename,dirname,join} from "node:path";
import {nativeImageReclaimDueV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim/🟦️.ts";
import {nativeFreeSpaceSyncV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";

/** 🔄️ The partitions whose image is sized elastically when it is created (`elastic` of the policy). */
export type ElasticPartitionIdV1="graph"|"products"|"rust";
/** 📐️ The elastic sizing rule of an image (`elastic.<partition>` of the policy). */
export type ElasticRuleV1=Readonly<{needBytes:number;floorBytes:number;headroomPercent:number;hostFreeReservePercent:number;alignBytes:number;maximumBytes:number;minimumBytes:number;reclaimFloorBytes:number;reclaimRatioPercent:number;reclaimTightHostFreeBytes:number;automatic:boolean;pressureFreeBytes:number}>;
/** 🖼️ Sizes a new image from its working set and the host: need times headroom, capped by the rule's maximum and by what the host volume may give (its free bytes minus the reserve), aligned down, never below the given minimum (the rule's `minimumBytes`, not the declared capacity: a host with less free space than the declared capacity still gets an image that fits). */
export function elasticCapacityV1(rule:ElasticRuleV1,minimumBytes:number,hostFreeBytes:number):number{
 const wanted=Math.ceil(rule.needBytes*(100+rule.headroomPercent)/100),ceiling=Math.min(rule.maximumBytes,Math.floor(hostFreeBytes*(100-rule.hostFreeReservePercent)/100)),target=Math.min(wanted,Math.max(ceiling,minimumBytes));
 return Math.max(minimumBytes,Math.floor(target/rule.alignBytes)*rule.alignBytes);
}
/** 🧮️ The smallest image a host of `hostFreeBytes` is born with for a rule: the rule's `minimumBytes` cut down to the share the host may give (`100 - hostFreeReservePercent` of its free bytes, aligned down to the rule's alignment), but never below the rule's `floorBytes`, the smallest image that still works (a compile needs its workspaces): the shares of all partitions add up to less than the host (see `hostSharePercentV1`) unless a floor needs more, and a host that cannot give the floor is refused up front (`requireImageFitsHostV1`) instead of getting an image that exhausts its storage mid-command. */
export function elasticMinimumV1(rule:Pick<ElasticRuleV1,"alignBytes"|"hostFreeReservePercent"|"floorBytes">,minimumBytes:number,hostFreeBytes:number):number{
 const share=Math.floor(hostFreeBytes*(100-rule.hostFreeReservePercent)/100);
 return Math.max(rule.floorBytes,Math.min(minimumBytes,Math.floor(share/rule.alignBytes)*rule.alignBytes));
}
/** 🥧️ The share of the host's free bytes one partition may be born with: an elastic one gets `100 - hostFreeReservePercent`, a fixed one its declared `hostSharePercent`; the shares of all partitions of the policy add up to at most 90 so that the images born one after the other can never promise more than the host has. */
export function hostSharePercentV1(policy:Readonly<{partitions:Readonly<Record<string,Readonly<{hostSharePercent?:number}>>>;elastic?:Readonly<Record<string,Pick<ElasticRuleV1,"hostFreeReservePercent">>>}>,partition:string):number{
 const rule=policy.elastic?.[partition];
 return rule?100-rule.hostFreeReservePercent:policy.partitions[partition]?.hostSharePercent??0;
}
/** 📏️ The capacity a new fixed image gets: its declared capacity, cut down to the share of the host's free bytes (aligned down to whole MiB) but never below the declared need (aligned up to whole MiB: a share that cannot hold what the partition is declared to need is no sizing), never below 64 MiB or the declaration when that is smaller. */
export function hostShareCapacityV1(declaredBytes:number,hostSharePercent:number,hostFreeBytes:number,needBytes:number=0):number{
 const share=Math.floor(hostFreeBytes*hostSharePercent/100/1048576)*1048576,need=Math.ceil(needBytes/1048576)*1048576;
 return Math.max(Math.min(declaredBytes,67108864),Math.min(declaredBytes,Math.max(share,need)));
}
/** 🧾️ The bytes the image of a partition must at least have to be of use: an elastic rule's `floorBytes`, else the declared `needBytes` of the host platform (the capacity that holds what its writers need, ledger share included), else 0. */
export function partitionNeedBytesV1(policy:Readonly<{partitions:Readonly<Record<string,Readonly<{needBytes?:Readonly<Record<string,number>>}>>>;elastic?:Readonly<Record<string,Pick<ElasticRuleV1,"floorBytes">>>}>,partition:string,platform:string=process.platform):number{
 return policy.elastic?.[partition]?.floorBytes??policy.partitions[partition]?.needBytes?.[platform]??0;
}
/** 🚫️ The typed bilingual refusal of an image the host cannot hold: it names the partition, what it needs and what the host has free, so that a short disk is told as such before any image is born instead of surfacing later as an exhausted storage. */
export class ImageDoesNotFitHostErrorV1 extends Error{
 readonly code="image-does-not-fit-host" as const;readonly partition:string;readonly requiredBytes:number;readonly hostFreeBytes:number;readonly en:string;readonly de:string;
 constructor(partition:string,requiredBytes:number,hostFreeBytes:number){
  const need=Math.ceil(requiredBytes/1048576),free=Math.floor(hostFreeBytes/1048576),en=`The "${partition}" cache image needs ${need} MiB but the host volume has only ${free} MiB free. Free disk space (for example with "repo cache gc") and run the command again; nothing was created.`,de=`Das Cache-Abbild "${partition}" braucht ${need} MiB, das Wirtsvolumen hat aber nur ${free} MiB frei. Speicherplatz freigeben (zum Beispiel mit "repo cache gc") und den Befehl erneut ausführen; es wurde nichts angelegt.`;
  super(en+" | "+de);this.name="ImageDoesNotFitHostErrorV1";this.partition=partition;this.requiredBytes=requiredBytes;this.hostFreeBytes=hostFreeBytes;this.en=en;this.de=de;
 }
}
/** 🌱️ Whether a born image smaller than its floor is to be recreated now: its receipt pins fewer bytes than the floor and the host can give the floor once the image's own host allocation (returned by its removal) is counted; an image the host cannot regrow is left alone and refused only where a compile needs it (`requireImageAtFloorV1`). */
export function elasticBelowFloorDueV1(receipt:ReturnType<typeof readImageReceiptBytesV1>,floorBytes:number,hostFreeBytes:number,allocatedBytes:number):boolean{
 return typeof receipt==="number"&&receipt<floorBytes&&hostFreeBytes+allocatedBytes>=floorBytes;
}
/** 🚫️ The typed bilingual refusal of a born image that is smaller than the floor its partition needs (an image born before the floor was declared, or under a pinned host): a compile in it would charge more than the image holds and end as an exhausted storage, so it is told as what it is, together with the remedy. */
export class ImageBelowFloorErrorV1 extends Error{
 readonly code="image-below-floor" as const;readonly partition:string;readonly capacityBytes:number;readonly floorBytes:number;readonly en:string;readonly de:string;
 constructor(partition:string,capacityBytes:number,floorBytes:number){
  const have=Math.floor(capacityBytes/1048576),need=Math.ceil(floorBytes/1048576),en=`The "${partition}" cache image holds ${have} MiB but at least ${need} MiB are needed. Free disk space and run "repo cache gc"; the image is recreated at its declared size by the next regrow, or remove it while no command runs.`,de=`Das Cache-Abbild "${partition}" fasst ${have} MiB, mindestens ${need} MiB sind nötig. Speicherplatz freigeben und "repo cache gc" ausführen; das Abbild wird beim nächsten Neuaufbau in der deklarierten Grösse neu angelegt, oder es wird entfernt, solange kein Befehl läuft.`;
  super(en+" | "+de);this.name="ImageBelowFloorErrorV1";this.partition=partition;this.capacityBytes=capacityBytes;this.floorBytes=floorBytes;this.en=en;this.de=de;
 }
}
/** 🧱️ Requires the capacity of a born image to reach the floor of its partition (a partition without a floor passes). */
export function requireImageAtFloorV1(partition:string,capacityBytes:number,floorBytes:number):void{
 if(floorBytes>0&&capacityBytes<floorBytes)throw new ImageBelowFloorErrorV1(partition,capacityBytes,floorBytes);
}
/** 🛑️ Refuses an image that is not born yet when the honest free bytes of its host volume cannot hold what the partition needs; an image that exists, or a partition without a declared need, is never refused here. */
export function requireImageFitsHostV1(partition:string,receipt:ReturnType<typeof readImageReceiptBytesV1>,requiredBytes:number,hostFreeBytes:number|undefined):void{
 if(receipt==="missing"&&hostFreeBytes!==undefined&&hostFreeBytes<requiredBytes)throw new ImageDoesNotFitHostErrorV1(partition,requiredBytes,hostFreeBytes);
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
let pinnedHostFreeBytesV1:number|undefined;
/** 📌️ Pins the host free bytes every image still to be born is sized from (`undefined` restores the live host volume), so a deterministic caller such as a private test repository sizes images independently of the disk and load of the host it runs on; images already born keep their receipt. */
export function pinHostFreeBytesForSizingV1(bytes:number|undefined):void{
 if(bytes!==undefined&&!(Number.isSafeInteger(bytes)&&bytes>0))throw Error("Positive pinned host free bytes required");
 pinnedHostFreeBytesV1=bytes;
}
/** 🌡️ The host free bytes a new image is sized from: the pinned bytes, else the live volume that holds the partitions, or undefined when it cannot be measured. */
export function hostFreeForSizingV1(directory:string):{availableBytes:number}|undefined{
 return pinnedHostFreeBytesV1===undefined?nativeFreeSpaceSyncV1(dirname(directory)):{availableBytes:pinnedHostFreeBytesV1};
}
/** 🪪️ The capacity of an elastic image: the one its receipt pins (clamped down to the rule's maximum, never snapped up to the declared minimum) when an image exists, else the elastic size a new image would get, raised to the grown size a pressed predecessor recorded as far as the host allows; an unreadable receipt keeps the declared capacity. */
export function effectiveCapacityV1(directory:string,minimumBytes:number,rule:ElasticRuleV1):number{
 const receipt=readImageReceiptBytesV1(directory);
 if(receipt===null)return minimumBytes;
 if(receipt!=="missing")return Math.min(receipt,rule.maximumBytes);
 const host=hostFreeForSizingV1(directory);if(!host)return minimumBytes;
 const base=elasticCapacityV1(rule,elasticMinimumV1(rule,minimumBytes,host.availableBytes),host.availableBytes),grown=readElasticGrowthV1(directory);
 return Math.max(base,Math.min(grown,rule.maximumBytes,Math.floor(host.availableBytes*(100-rule.hostFreeReservePercent)/100/rule.alignBytes)*rule.alignBytes));
}
/** 📌️ The capacity of a partition without an elastic rule: the one its receipt pins when an image exists and fits the declared capacity (an image keeps the size it was born with, a larger declaration takes effect only through the explicit reset that births a new image), else the declared capacity cut down to the host share of a fixed partition (`hostSharePercent`) when the image is not born yet; a larger or unreadable receipt keeps the declared capacity so the receiver refuses it as before. */
export function pinnedCapacityV1(directory:string,declaredBytes:number,hostSharePercent?:number,needBytes:number=0):number{
 const receipt=readImageReceiptBytesV1(directory);
 if(typeof receipt==="number"&&receipt<=declaredBytes)return receipt;
 if(receipt!=="missing"||hostSharePercent===undefined)return declaredBytes;
 const host=hostFreeForSizingV1(directory);
 return host?hostShareCapacityV1(declaredBytes,hostSharePercent,host.availableBytes,needBytes):declaredBytes;
}

/** 🧲️ Whether an image holds host blocks it no longer uses: the host allocation is compared with the bytes used inside (at least the ratio and at least the floor, an eighth of the floor while the host volume is nearly full). The decision is the framework's `nativeImageReclaimDueV1`; the rule only carries the three numbers. */
export function elasticBloatedV1(rule:Pick<ElasticRuleV1,"reclaimFloorBytes"|"reclaimRatioPercent"|"reclaimTightHostFreeBytes">,allocatedBytes:number,usedBytes:number,hostFreeBytes:number):boolean{
 return nativeImageReclaimDueV1(rule,allocatedBytes,usedBytes,hostFreeBytes);
}
