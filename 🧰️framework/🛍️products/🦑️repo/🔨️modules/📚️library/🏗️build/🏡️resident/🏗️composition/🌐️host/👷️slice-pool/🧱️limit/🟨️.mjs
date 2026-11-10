/** 🧱 A hard memory limit for every slice load: where cgroup v2 is delegated to the user, each load process gets its own leaf cgroup (a sibling of the service's, because a cgroup that holds processes cannot enable controllers for children) with `memory.max` = the service's limit minus its measured usage minus the safety margin, `memory.high` at 90% of that, no swap, and the kernel kills the whole group on an overrun (`memory.oom.group`, `cgroup.kill` on release), so an overrun costs only that load and the service never stalls at its own `memory.high`. Elsewhere (macOS, Windows, no delegation) `sliceLimiterV1` answers undefined and the pool's guard plus the process kill apply: one code path, one capability check.
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

/** 📏 The policy of one load given the service's own limit and usage, or undefined where the service has no limit. */
export function sliceLimitPolicyV1({limitBytes,currentBytes},marginMegabytes=0){
 if(!Number.isFinite(limitBytes))return undefined;
 const megabytes=Math.max(SLICE_LIMIT_FLOOR_MEGABYTES_V1,Math.floor((limitBytes-currentBytes)/MiB)-marginMegabytes);
 return{kind:"service",memoryMegabytes:megabytes,swapMegabytes:0,cpuShares:100,pids:0,softRatioBasisPoints:SLICE_LIMIT_SOFT_BASIS_POINTS_V1,enforcement:"hard"};
}
