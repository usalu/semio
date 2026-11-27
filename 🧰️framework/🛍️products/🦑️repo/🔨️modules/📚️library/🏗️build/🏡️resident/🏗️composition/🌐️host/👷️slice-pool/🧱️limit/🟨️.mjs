/** 🧱 A hard memory limit for every slice load: where cgroup v2 is delegated to the user, each load process gets its own leaf cgroup (a sibling of the service's, because a cgroup that holds processes cannot enable controllers for children) with `memory.max` = the service's limit minus its measured usage minus the safety margin, no swap and no `memory.high` (a throttle at 90% made an overrun crawl for the better part of an hour instead of ending as the typed `memory` refusal), and the kernel kills the whole group on an overrun (`memory.oom.group`, `cgroup.kill` on release), so an overrun costs only that load and the service never stalls at its own `memory.high`. Elsewhere (macOS, Windows, no delegation) `sliceLimiterV1` answers undefined and the pool's guard plus the process kill apply: one code path, one capability check.
 * @see ../🟨️.mjs */
import {cgroupPortsV1,createCgroupLimiterV1,detectCgroupV2V1} from "../../../../../🧱️job-limits/🐧️cgroup/🟦️.ts";

const MiB=1048576;
export const SLICE_LIMIT_FLOOR_MEGABYTES_V1=64;
export const SLICE_LIMIT_SOFT_BASIS_POINTS_V1=9000;

/** 🧭 The limiter of this host, or undefined where no cgroup v2 is delegated. */
export function sliceLimiterV1(ports=cgroupPortsV1()){
 const base=detectCgroupV2V1(ports);
 return base===undefined?undefined:createCgroupLimiterV1(base,ports);
}

/** 🧱 The most a learned peak may ever count for on this host: the elasticity maximum and `hostCapBasisPoints` of the physical memory (what the gate can grant a slice at all), so no stored history, however large, books a reservation the machine can never give.
 * @see ../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts */
export function sliceCeilingMegabytesV1({hostMegabytes=0,hostCapBasisPoints=10000,maximumMegabytes=0}){
 return Math.min(maximumMegabytes||Infinity,hostMegabytes>0?Math.floor(hostMegabytes*hostCapBasisPoints/10000):Infinity);
}

/** 📐 The memory one load books and the least it starts with, sized from this machine: what it is declared or learned to need (`declaredMegabytes` when the host's own limit sets it, else the learned peak capped by the elasticity maximum, never below the unlearned guess), capped at `hostCapBasisPoints` of the physical memory (a booking sized for a larger machine than this one is never granted: 8192 MB on a 16 GB host that also runs the replica daemon, three sidecars and the user's applications), but never below `minimumMegabytes`, the least a load starts with, and never above what the service limit still leaves (`serviceLeftMegabytes`, finite where a memory limit applies but no cgroup lends the load its own: a booking above it is cancelled at that limit anyway); a load that cannot get its booking in full starts with what is free once that covers the minimum.
 * @see ../../../../../../🌡️host-capacity/🧪️tests/📉️shrink/🟦️.ts */
export function sliceBookingV1({declaredMegabytes,learnedMegabytes=0,unlearnedMegabytes=0,maximumMegabytes=0,hostMegabytes=0,hostCapBasisPoints=10000,minimumMegabytes=0,serviceLeftMegabytes=Infinity}){
 const asked=declaredMegabytes??Math.max(Math.min(sliceCeilingMegabytesV1({hostMegabytes,hostCapBasisPoints,maximumMegabytes}),learnedMegabytes),unlearnedMegabytes),capped=hostMegabytes>0?Math.floor(hostMegabytes*hostCapBasisPoints/10000):Infinity,wanted=Math.max(0,Math.min(asked,Math.max(capped,minimumMegabytes),Math.max(serviceLeftMegabytes,SLICE_LIMIT_FLOOR_MEGABYTES_V1)));
 return{wantedMegabytes:wanted,minimumMegabytes:Math.min(minimumMegabytes,wanted)};
}

/** 🪶 The least a load of a measured booking starts with, derived from that measurement and not from a constant: `leanBasisPoints` of what it is booked at, never below `floorMegabytes` (the least a load starts with at all) and never above the booking. A host that cannot give the full booking admits the load at this size and the worker cuts its waves to the grant (`SEMIO_SLICE_WAVE_BASIS_POINTS`), so a 16 GB host reads the slice in smaller waves instead of waiting for the whole peak of a bigger machine.
 * @see ../🟨️.mjs */
export function sliceLeanMegabytesV1({bookedMegabytes,floorMegabytes=0,leanBasisPoints=10000}){
 return Math.min(bookedMegabytes,Math.max(floorMegabytes,Math.ceil(bookedMegabytes*leanBasisPoints/10000)));
}

/** 🏔️ The most one load may take before the plan guard refuses it (`planGuardVerdictV1`): the larger of the ceiling share of the physical memory and the plan limit, 0 where the guard is off or the memory unknown, so the kernel limit of a load never lies below the point where the guard settles and judges it.
 * @see ../🟨️.mjs */
export function slicePlanCeilingMegabytesV1({hostMegabytes=0,ceilingBasisPoints=0,planLimitMegabytes=Infinity}){
 const ceiling=hostMegabytes>0?Math.max(Math.floor(hostMegabytes*ceilingBasisPoints/10000),planLimitMegabytes):Infinity;
 return Number.isFinite(ceiling)?ceiling:0;
}

/** 📏 The policy of one load, which never reserves less than the peak earlier loads of this host were seen to need (`learnedMegabytes`, capped by the elasticity maximum), so builds that start while a plan runs queue behind the real use of the plan instead of shrinking the limit the next plan is given: given the service's own limit and usage, or undefined where the service has no limit. A host with more memory available than the service limit leaves is let lend the load `hostShareBasisPoints` of it (above `hostFloorMegabytes`), never beyond `maximumMegabytes` and never below what the service limit leaves. The kernel limit never lies below the plan ceiling plus the safety margin (`planCeilingMegabytes`): the kernel kills at `memory.max` without a settle, so a limit below the guard ceiling would make the kernel refuse a plan the guard (settle first, refuse only when memory, pressure or the ceiling say so) would let through. */
export function sliceLimitPolicyV1({limitBytes,currentBytes,availableBytes},marginMegabytes=0,{maximumMegabytes=0,hostShareBasisPoints=0,hostFloorMegabytes=0,hostMegabytes=0,hostCapBasisPoints=10000,planCeilingMegabytes=0}={},learnedMegabytes=0){
 if(!Number.isFinite(limitBytes))return undefined;
 const declared=Math.floor((limitBytes-currentBytes)/MiB)-marginMegabytes,lent=Number.isFinite(availableBytes)?Math.min(maximumMegabytes,Math.floor(availableBytes*hostShareBasisPoints/10000/MiB)-hostFloorMegabytes):0;
 const guarded=planCeilingMegabytes>0?planCeilingMegabytes+marginMegabytes:0,megabytes=Math.max(SLICE_LIMIT_FLOOR_MEGABYTES_V1,declared,lent,guarded,Math.min(sliceCeilingMegabytesV1({hostMegabytes,hostCapBasisPoints,maximumMegabytes}),learnedMegabytes));
 return{kind:"job",memoryMegabytes:megabytes,swapMegabytes:0,cpuShares:100,pids:0,softRatioBasisPoints:SLICE_LIMIT_SOFT_BASIS_POINTS_V1,enforcement:"hard"};
}

/** 🛟 What one slice load may reserve in the machine-wide gate: never more than the host can hand out, so a request can never wait for memory the host will never have (a host whose desktop holds half of its memory never has the 8 GB a learned peak asked for: the reservation waited for ever). `availableMegabytes` above `floorMegabytes` is what the gate can give while nothing of ours runs; `shareBasisPoints` of it is reserved (the rest is the margin against the sample moving), never below `minimumMegabytes` (the sane minimum a load starts with) and never more than requested. The load then runs with a narrower owner wave, `waveBasisPoints` of the usual one (the granted share of the request, between `minimumWaveBasisPoints` and 10000): slower, never never. Unknown availability changes nothing.
 * @see ../🟨️.mjs */
export function sliceAdmissionV1({requestedMegabytes,availableMegabytes,floorMegabytes=0,shareBasisPoints=10000,minimumMegabytes=1,minimumWaveBasisPoints=500}){
 const obtainable=Number.isFinite(availableMegabytes)?Math.max(minimumMegabytes,Math.floor(Math.max(0,availableMegabytes-floorMegabytes)*shareBasisPoints/10000)):Infinity,megabytes=Math.min(requestedMegabytes,obtainable);
 return{megabytes,reduced:megabytes<requestedMegabytes,waveBasisPoints:requestedMegabytes>0?Math.max(minimumWaveBasisPoints,Math.min(10000,Math.floor(megabytes*10000/requestedMegabytes))):10000};
}

/**
 * 🛡️ Decides what to do with a slice worker whose process tree passes the memory a plan may take: `continue` while the tree is within `limitMegabytes`, `refuse` at once above `ceilingMegabytes` (the absolute ceiling that nothing excuses), `settle` once when the tree is above the limit and the worker has not yet given back what it can (collect and shrink), and after the settle `refuse` only when the host is really endangered: the memory it can still give out is below `reserveMegabytes` (`memory`), unknown (`unknown`) or the memory pressure is `critical` (`pressure`; `warn` alone is the normal state of a busy 16 GB machine and refuses nothing while the reserve is there); otherwise `continue`.
 * @see ../../../../../🧠️memory/🧽️settle/🟨️.mjs
 */
export function planGuardVerdictV1({treeMegabytes,limitMegabytes,ceilingMegabytes,availableMegabytes,reserveMegabytes,pressure,settled}){
 if(treeMegabytes>ceilingMegabytes)return{action:"refuse",reason:"ceiling"};
 if(!(treeMegabytes>limitMegabytes))return{action:"continue"};
 if(!settled)return{action:"settle"};
 if(pressure==="critical")return{action:"refuse",reason:"pressure"};
 if(availableMegabytes===undefined)return{action:"refuse",reason:"unknown"};
 return availableMegabytes<reserveMegabytes?{action:"refuse",reason:"memory"}:{action:"continue"};
}
