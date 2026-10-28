/** 🗃️ Content-addressed Cargo compile-unit cache: a bounded pool of lanes (one process each, crate-set affinity, trimmed to the machine disk budget), per-unit single-flight claims, host-wide compile slots, bounded cleanup and per-unit admission memos.
 * Cargo runs `repo 📜️script.ts <rustc arguments>` in place of rustc; a hit restores the recorded outputs, a miss compiles once across all lanes through the memory-admitting scheduler.
 * @see ./🧬️schema/🔣️.json */
export {UNIT_CACHE_POLICY_V1,parseRustcUnitV1,rustcBypassV1,unitKeyV1} from "./🔏️key/🟦️.ts";
export {acquireSlotV1,acquireUnitClaimV1,tryClaimV1} from "./🔒️lock/🟦️.ts";
export {lookupUnitV1,publishUnitV1,restoreUnitV1} from "./💾️store/🟦️.ts";
export {crateKeyV1,type LaneOwnerV1} from "./🧵️lane/🟦️.ts";
export {compilerOriginOwnedV1,laneBuildDirectoryOwnedV1,planRetentionV1,retainLaneOutputsV1,withLaneRetentionV1,type RetentionSummaryV1} from "./🧵️lane/📤️retention/🟦️.ts";
export {acquirePooledLaneV1,planPoolAcquireV1,planPoolTrimV1,poolQuotaBytesV1,trimPoolV1,type PooledLaneV1} from "./🧵️lane/🏊️pool/🟦️.ts";
export {collectGarbageV1,planGcV1} from "./🧹️gc/🟦️.ts";
export {focusFromCargoArgumentsV1,unitCacheEnvironmentV1,unitCacheRootV1} from "./🚪️launch/🟦️.ts";
export {unitAdmissionMemoV1} from "./📖️admission/🟦️.ts";
