/** 🧬️ SemioValueArtifact schema — mirrors `SemioValueSnapshot` field for field; the
 * `🦀️component.rs` sibling is the real source of truth. */
import type { ValueId, SemioValueNode, SemioValue } from "./📸️snapshot/🟦️component";

export interface SemioValueArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ root: SemioValue;
  /** @state persistent */ nodes: SemioValueNode[];
}
export type { ValueId };
