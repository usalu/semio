/** 🔺️ S Space index diff schema — TS twin of `🔺️diff/🦀️.rs`. */
import type { SpaceArtifactDialect, SpaceArtifactRow } from "../📸️snapshot/🟦️.ts";

export interface SSpaceArtifactPatch {
  name?: string | null;
  kindId?: string | null;
  schema?: string | null;
  dialect?: SpaceArtifactDialect | null;
  createdAtMs?: bigint | null;
  createdBy?: string | null;
  updatedAtMs?: bigint | null;
  updatedBy?: string | null;
}

export interface SSpaceArtifactRemoval {
  id: string;
  index: number;
}

export interface SSpaceArtifactInsertion {
  index: number;
  row: SpaceArtifactRow;
}

export interface SSpaceArtifactRelocation {
  id: string;
  from: number;
  to: number;
}

export interface SSpaceArtifactModification {
  id: string;
  patch: SSpaceArtifactPatch;
}

export interface SSpaceArtifactsDelta {
  removed: SSpaceArtifactRemoval[];
  inserted: SSpaceArtifactInsertion[];
  moved: SSpaceArtifactRelocation[];
  modified: SSpaceArtifactModification[];
}

export interface SSpaceDiff {
  schema?: string;
  artifacts?: SSpaceArtifactsDelta;
}
