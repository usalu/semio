/** 🧬️ SemioObjectArtifact schema — mirrors `SemioObjectSnapshot` field for field; the
 * `🦀️component.rs` sibling is the real source of truth. */
import type { ObjectId, SemioObjectNode, SemioValue } from "./📸️snapshot/🟦️component";

export interface SemioObjectArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ root: SemioValue;
  /** @state persistent */ objects: SemioObjectNode[];
}
export type { ObjectId };
