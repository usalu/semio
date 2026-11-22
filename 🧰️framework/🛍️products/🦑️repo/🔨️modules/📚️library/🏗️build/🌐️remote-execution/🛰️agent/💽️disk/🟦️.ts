import {freeSpaceSyncV1,type FreeSpaceOptionsV1} from "../../../../../../../../🔨️modules/🏃️process/🆓️free-space/🟦️.ts";

const GIBIBYTE=2**30;

/** 💽️ What a build worker needs, in gibibytes, on two volumes: `reserveGigabytes` stays free on the volume of the work directory (its partition image, which no other consumer protects), `hostReserveGigabytes` stays free on the volume of the checkout for the machine itself (swap, the replica and the owner's own builds, because a partition image grows into the host volume), `perActionGigabytes` is the worst case one action adds to each (the largest compiler outputs measured are 140 MB; an output is written beside a temporary copy and the dependency file, so 0.5 GiB leaves a margin). */
export type RemoteExecutionWorkerDiskV1=Readonly<{reserveGigabytes:number;hostReserveGigabytes:number;perActionGigabytes:number}>;

/** 🧮️ How many actions the free space allows at once and what that was measured on: the numbers are those of the volume that limits (`volume`), the work volume when both allow the same. */
export type RemoteExecutionDiskSlotsV1=Readonly<{slots:number;freeBytes:number|undefined;needBytes:number;oneActionBytes:number;volume:"work"|"host"}>;

/** 💾️ The bytes free to a normal user on the volume that holds `path` (or its nearest existing parent), from the one free-space question of the build system (the volume call on Windows, where `statfs` answers zeros); `undefined` when the volume cannot be asked. @see ../../../../../../../../🔨️modules/🏃️process/🆓️free-space/🟦️.ts */
export function remoteExecutionFreeBytesV1(path:string,options?:Omit<FreeSpaceOptionsV1,"nearest">):number|undefined{
 return freeSpaceSyncV1(path,{nearest:true,...options})?.availableBytes;
}

/** 📏️ The free space a worker with `concurrency` slots needs to serve all of them on the volume with the reserve `reserveGigabytes`: the reserve plus the worst case of every slot. */
export const remoteExecutionDiskNeedBytesV1=(disk:Readonly<{reserveGigabytes:number;perActionGigabytes:number}>,concurrency:number):number=>Math.ceil((disk.reserveGigabytes+Math.max(1,concurrency)*disk.perActionGigabytes)*GIBIBYTE);

const volumeSlotsV1=(freeBytes:number|undefined,concurrency:number,reserveGigabytes:number,perActionGigabytes:number):RemoteExecutionDiskSlotsV1=>{
 const reserve=reserveGigabytes*GIBIBYTE,per=perActionGigabytes*GIBIBYTE,need=remoteExecutionDiskNeedBytesV1({reserveGigabytes,perActionGigabytes},concurrency),one=Math.ceil(reserve+per);
 if(freeBytes===undefined)return Object.freeze({slots:concurrency,freeBytes,needBytes:need,oneActionBytes:one,volume:"work"});
 const slots=per<=0?concurrency:Math.max(0,Math.min(concurrency,Math.floor((freeBytes-reserve)/per)));
 return Object.freeze({slots,freeBytes,needBytes:need,oneActionBytes:one,volume:"work"});
};

/** 🎚️ The number of actions the free space of the two volumes allows to run at the same time: on each volume none below its reserve plus one action, one more for every `perActionGigabytes` above it, never more than `concurrency`; the smaller of the two counts, and the numbers reported are those of the volume that limits. A volume that cannot be asked does not restrict. A machine with little free space therefore keeps serving a few units instead of none, and a work volume that holds nothing but the worker is not charged the machine's reserve. Pure. */
export function remoteExecutionDiskSlotsV1(input:Readonly<{freeBytes:number|undefined;hostFreeBytes?:number|undefined;concurrency:number;disk:RemoteExecutionWorkerDiskV1}>):RemoteExecutionDiskSlotsV1{
 const {concurrency,disk}=input,work=volumeSlotsV1(input.freeBytes,concurrency,disk.reserveGigabytes,disk.perActionGigabytes),host=volumeSlotsV1(input.hostFreeBytes,concurrency,disk.hostReserveGigabytes,disk.perActionGigabytes);
 return host.slots<work.slots?Object.freeze({...host,volume:"host" as const}):work;
}
