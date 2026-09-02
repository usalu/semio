/** 🧬️ S Space index snapshot schema — TS twin of `📸️snapshot/🦀️.rs`. */

export interface SpaceArtifactDialect {
  artifactKind: string;
  standard: string;
  subset: string;
}

export interface SpaceArtifactRow {
  id: string;
  name: string;
  kindId: string;
  schema: string;
  dialect: SpaceArtifactDialect;
  createdAtMs: number;
  createdBy: string;
  updatedAtMs: number;
  updatedBy: string;
}

/** 📸️ Persisted S Space index document snapshot — one per hub space, document id `index`. */
export interface SSpaceSnapshot {
  schema: string;
  spaceId: string;
  artifacts: SpaceArtifactRow[];
}
