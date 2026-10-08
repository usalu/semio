/** 🔺️ S Space index diff schema — TS twin of `🔺️diff/🦀️.rs`. */
import type { SpaceArtifactDialect, SpaceArtifactRow } from "../📸️snapshot/🟦️.ts";

export interface SSpaceArtifactPatch {
  id: string;
  name?: string | null;
  kindId?: string | null;
  schema?: string | null;
  dialect?: SpaceArtifactDialect | null;
  createdAtMs?: bigint | null;
  createdBy?: string | null;
  updatedAtMs?: bigint | null;
  updatedBy?: string | null;
}

export interface SSpaceArtifactsDelta {
  added: SpaceArtifactRow[];
  removed: string[];
  patched: SSpaceArtifactPatch[];
  reordered?: string[] | null;
}

export interface SSpaceDiff {
  schema?: string;
  artifacts?: SSpaceArtifactsDelta;
}
