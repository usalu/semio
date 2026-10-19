/** 🎨️ Complete document authority for one mounted render projection. */
export interface RenderRevisionIdentity {
  readonly appInstanceId: number;
  readonly baseRevision: string;
  readonly generation: string;
  readonly canonicalBaseRevision: string;
}

/** 🔐️ Compares admitted render authority without allocating or reading work grants. */
export function renderRevisionMatches(left: RenderRevisionIdentity, right: RenderRevisionIdentity): boolean {
  return left.appInstanceId === right.appInstanceId && left.baseRevision === right.baseRevision && left.generation === right.generation && left.canonicalBaseRevision === right.canonicalBaseRevision;
}
