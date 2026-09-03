/** 🧬️ SemioArtifact schema — full artifact state, mirrors `SemioSnapshot` field for field (see
 * `📸️snapshot/🟦️.ts` for the real `SemioSubsetSnapshot` union shape). */
import type { SemioSubsetSnapshot } from "./📸️snapshot/🟦️";

export interface SemioArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ subset: SemioSubsetSnapshot;
}
