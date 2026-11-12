import {freeSpaceSyncV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🆓️free-space/🟦️.ts";
import {volumeMountOfV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🆓️free-space/💽️disk-full/🟦️.ts";
import type {CasPolicyV1} from "../🧬️schema/🟦️.ts";

/** 🌡️ The partition that holds a path: its mount point, size and the bytes a writer may still use. */
export type CasPartitionV1=Readonly<{partition:string;totalBytes:number;freeBytes:number}>;

/** 🔭️ Measures the partition of a path (of its nearest existing ancestor); `undefined` when the host cannot say how large the partition is, so that no collection starts on a guess. @see ../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🆓️free-space/🟦️.ts */
export function measureCasPartitionV1(path:string):CasPartitionV1|undefined{
 const space=freeSpaceSyncV1(path,{nearest:true});
 if(space?.totalBytes===undefined||!(space.totalBytes>0))return undefined;
 return Object.freeze({partition:volumeMountOfV1(path),totalBytes:space.totalBytes,freeBytes:Math.min(space.totalBytes,space.availableBytes)});
}

/** 📏️ The share of a partition in use, counting `incomingBytes` still to be written. */
export const casPartitionUsedFractionV1=(partition:Pick<CasPartitionV1,"totalBytes"|"freeBytes">,incomingBytes:number=0):number=>(partition.totalBytes-partition.freeBytes+incomingBytes)/partition.totalBytes;

/** 🧮️ The bytes the store has to free so that its partition falls to the low used share: 0 unless the partition is above its high used share. */
export function casPartitionNeedBytesV1(partition:Pick<CasPartitionV1,"totalBytes"|"freeBytes">|undefined,policy:Pick<CasPolicyV1,"partition">):number{
 if(partition===undefined)return 0;
 const used=partition.totalBytes-partition.freeBytes,{highUsedFraction,lowUsedFraction}=policy.partition;
 return used>partition.totalBytes*highUsedFraction?Math.ceil(used-partition.totalBytes*lowUsedFraction):0;
}

/** 🚦️ What a writer or a build does before it adds to the partition: go on, start a collection in the background and go on, wait for a collection, measure the store in the background, or be refused. */
export type CasEnforcementActionV1="allow"|"collect-background"|"collect-now"|"measure"|"refuse";
/** 🚦️ Why: it fits, the store is above its high watermark or its cap, the partition is above its high or critical share, or the size of the store is not known yet. */
export type CasEnforcementReasonV1="fits"|"cap-high"|"cap-exceeded"|"partition-high"|"partition-critical"|"unknown-usage";
/** 🚦️ Everything the pure admission rule decides on: the bytes the store holds (`undefined` when not known), the bytes about to be written, the partition (`undefined` when not measurable) and whether a collection already ran within the minimum age. */
export type CasEnforcementInputV1=Readonly<{policy:Pick<CasPolicyV1,"capBytes"|"highWatermark"|"partition">;storeBytes:number|undefined;incomingBytes:number;partition:Pick<CasPartitionV1,"totalBytes"|"freeBytes">|undefined;collected:boolean}>;
/** 🚦️ The verdict on one write or build. */
export type CasEnforcementDecisionV1=Readonly<{action:CasEnforcementActionV1;reason:CasEnforcementReasonV1}>;

/** 🚦️ Decides what a write or a build does. The partition is judged first: at its critical used share a write waits for a collection and is refused when one already ran and the share is still critical; at its high share a collection starts in the background. Then the store: above its cap a write waits for a collection (after one ran it goes on, because what is left is pinned, leased or too young and refusing would stop builds that need nothing from the store), above its high watermark a collection starts in the background, and an unknown size is measured in the background. Pure and deterministic; the independent Python oracle in the tests judges the same cases. */
export function decideCasEnforcementV1(input:CasEnforcementInputV1):CasEnforcementDecisionV1{
 const verdict=(action:CasEnforcementActionV1,reason:CasEnforcementReasonV1):CasEnforcementDecisionV1=>Object.freeze({action,reason});
 if(input.partition!==undefined){
  const used=casPartitionUsedFractionV1(input.partition,input.incomingBytes),{criticalUsedFraction,highUsedFraction}=input.policy.partition;
  if(used>=criticalUsedFraction)return verdict(input.collected?"refuse":"collect-now","partition-critical");
  if(used>=highUsedFraction&&!input.collected)return verdict("collect-background","partition-high");
 }
 if(input.storeBytes===undefined)return verdict("measure","unknown-usage");
 const held=input.storeBytes+input.incomingBytes;
 if(held>input.policy.capBytes&&!input.collected)return verdict("collect-now","cap-exceeded");
 if(held>input.policy.capBytes*input.policy.highWatermark&&!input.collected)return verdict("collect-background","cap-high");
 return verdict("allow","fits");
}
