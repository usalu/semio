/** 🧬️ SemioValueArtifact schema — mirrors `SemioValueSnapshot` field for field; the
 * `🦀️.rs` sibling is the real source of truth. */
import type { ValueId, SemioValueNode, SemioValue } from "./📸️snapshot/🟦️";

export interface SemioValueArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ root: SemioValue;
  /** @state artifact */ nodes: SemioValueNode[];
}
export type { ValueId };
