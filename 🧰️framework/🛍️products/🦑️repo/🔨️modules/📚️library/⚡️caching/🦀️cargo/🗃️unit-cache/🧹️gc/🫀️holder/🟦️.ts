/** 🫀️ The evidence that a build directory (a unit, a profile, a lane) has a live holder, gathered by the collector that wants to remove it: `lane` a live marker of a process that entered the lane, `lease` a compiler holding the build lease of the profile, `lock` the compiler's own lock file of the unit probed without waiting (`held` by a running compiler, `free` taken by us, `missing` no lock file, `unsupported` no lock backend on this platform) and `idleMilliseconds` the time since anything in it changed. */
export type BuildHolderEvidenceV1=Readonly<{lane:boolean;lease:boolean;lock:"held"|"free"|"missing"|"unsupported";idleMilliseconds:number}>;

/** 🫀️ Whether a build directory has a live holder and so is never removed, however old, superseded or over budget: any one proof suffices, because Cargo reuses a fresh unit without taking its lock or touching its time (`fingerprint/invoked.timestamp` vanishes under a running build), so only the lease of the profile or the marker of the lane covers a unit the build has planned but not yet started. Without a lock backend only the age speaks: a directory that changed within the grace counts as held. Pure; the one predicate of every collector that removes build directories (the stale unit collector and, through its lane marker, the lane removal).
 * @see ../🧱️units/🟦️.ts */
export function buildHolderLiveV1(evidence:BuildHolderEvidenceV1,graceMilliseconds:number):boolean{
 return evidence.lane||evidence.lease||evidence.lock==="held"||evidence.lock==="unsupported"&&evidence.idleMilliseconds<graceMilliseconds;
}
