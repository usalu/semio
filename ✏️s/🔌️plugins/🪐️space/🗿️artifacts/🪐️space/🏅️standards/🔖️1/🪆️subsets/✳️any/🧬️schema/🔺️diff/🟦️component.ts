/** 🔺️ S Space index diff schema — TS twin of `🔺️diff/🦀️component.rs`. */
import type { SpaceArtifactRow } from "../📸️snapshot/🟦️component.ts";

export interface SSpaceDiff {
  schema?: string;
  artifacts?: SpaceArtifactRow[];
}
