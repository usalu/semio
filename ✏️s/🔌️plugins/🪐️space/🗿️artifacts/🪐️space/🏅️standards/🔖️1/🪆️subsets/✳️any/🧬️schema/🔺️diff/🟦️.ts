/** 🔺️ S Space index diff schema — TS twin of `🔺️diff/🦀️.rs`. */
import type { SpaceArtifactRow } from "../📸️snapshot/🟦️.ts";

export interface SSpaceDiff {
  schema?: string;
  artifacts?: SpaceArtifactRow[];
}
