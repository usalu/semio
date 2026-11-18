import {freeSpaceSyncV1,type FreeSpaceOptionsV1} from "../../../../../../../../🔨️modules/🏃️process/🆓️free-space/🟦️.ts";

const GIBIBYTE=2**30;

/** 💽️ What a build worker needs on its volume, in gibibytes: `reserveGigabytes` stays free for the machine itself (its blob cache of 256 MiB, swap, the replica and the owner's own builds), `perActionGigabytes` is the worst case one action adds (the largest compiler outputs measured are 140 MB; an output is written beside a temporary copy and the dependency file, so 0.5 GiB leaves a margin). */
export type RemoteExecutionWorkerDiskV1=Readonly<{reserveGigabytes:number;perActionGigabytes:number}>;

/** 🧮️ How many actions the free space allows at once and what that was measured on. */
export type RemoteExecutionDiskSlotsV1=Readonly<{slots:number;freeBytes:number|undefined;needBytes:number;oneActionBytes:number}>;

/** 💾️ The bytes free to a normal user on the volume that holds `path` (or its nearest existing parent), from the one free-space question of the build system (the volume call on Windows, where `statfs` answers zeros); `undefined` when the volume cannot be asked. @see ../../../../../../../../🔨️modules/🏃️process/🆓️free-space/🟦️.ts */
export function remoteExecutionFreeBytesV1(path:string,options?:Omit<FreeSpaceOptionsV1,"nearest">):number|undefined{
 return freeSpaceSyncV1(path,{nearest:true,...options})?.availableBytes;
}

/** 📏️ The free space a worker with `concurrency` slots needs to serve all of them: the reserve plus the worst case of every slot. */
export const remoteExecutionDiskNeedBytesV1=(disk:RemoteExecutionWorkerDiskV1,concurrency:number):number=>Math.ceil((disk.reserveGigabytes+Math.max(1,concurrency)*disk.perActionGigabytes)*GIBIBYTE);

/** 🎚️ The number of actions the free space of the worker's volume allows to run at the same time: none below the reserve plus one action, one more for every `perActionGigabytes` above it, never more than `concurrency`. A volume that cannot be asked does not restrict. A machine with little free space therefore keeps serving a few units instead of none. Pure. */
export function remoteExecutionDiskSlotsV1(input:Readonly<{freeBytes:number|undefined;concurrency:number;disk:RemoteExecutionWorkerDiskV1}>):RemoteExecutionDiskSlotsV1{
 const {freeBytes,concurrency,disk}=input,reserve=disk.reserveGigabytes*GIBIBYTE,per=disk.perActionGigabytes*GIBIBYTE,need=remoteExecutionDiskNeedBytesV1(disk,concurrency);
 const one=Math.ceil(reserve+per);
 if(freeBytes===undefined)return Object.freeze({slots:concurrency,freeBytes,needBytes:need,oneActionBytes:one});
 const slots=per<=0?concurrency:Math.max(0,Math.min(concurrency,Math.floor((freeBytes-reserve)/per)));
 return Object.freeze({slots,freeBytes,needBytes:need,oneActionBytes:one});
}
