import type { RetainedCloneGrant, RetainedCloneProgress } from "@semio-tech/value-ts/retained-clone/contract";

/** 🧾️ Readiness and actual retained effects from an original mounted preparation callback. */
export interface MountedSnapshotReadPreparation { readonly ready: boolean; readonly progress: RetainedCloneProgress }
/** 🎟️ Borrows the original caller's physical and CPU recipient. */
export interface MountedSnapshotPreparationContext {
  retainedGrant(): RetainedCloneGrant;
  fuelRemaining(): number;
  consumeRetained(progress: RetainedCloneProgress): void;
  consumeFuel(items: number): void;
}

/** 🔐️ Records original effects before refusing excess physical or CPU authority. */
export function recordMountedSnapshotPreparation(step: MountedSnapshotReadPreparation, grant: RetainedCloneGrant, cx: MountedSnapshotPreparationContext): boolean {
  const original = cx.retainedGrant(), progress = step.progress;
  const covered = grant.maximumItems <= original.maximumItems && grant.maximumCopyBytes <= original.maximumCopyBytes && grant.maximumCapacityBytes <= original.maximumCapacityBytes && grant.maximumReleaseBytes <= original.maximumReleaseBytes && grant.maximumDepth <= original.maximumDepth && progress.copiedItems <= grant.maximumItems && progress.copiedBytes <= grant.maximumCopyBytes && progress.retainedCapacityBytes <= grant.maximumCapacityBytes && progress.releasedBytes <= grant.maximumReleaseBytes && (grant.maximumDepth > 0 || Object.values(progress).every(value => value === 0)) && progress.copiedItems <= cx.fuelRemaining();
  try { cx.consumeRetained(progress); } finally { cx.consumeFuel(progress.copiedItems); }
  if (!covered) throw Error("mounted snapshot preparation exceeds its original granted turn");
  return step.ready;
}

/** 🪪️ Complete original document authority for a mounted pending poll. */
export interface ArtifactPendingSourceVersion{readonly baseRevision:number;readonly generation:number;readonly canonicalRevision:string;readonly overlay:number}
export function sameArtifactPendingSourceVersion(a:ArtifactPendingSourceVersion,b:ArtifactPendingSourceVersion):boolean{return a.baseRevision===b.baseRevision&&a.generation===b.generation&&a.canonicalRevision===b.canonicalRevision&&a.overlay===b.overlay;}
