/** 🧬️ SemioArtifact schema — full artifact state, mirrors `SemioSnapshot` field for field (see
 * `📸️snapshot/🟦️component.ts` for the real `SemioSubsetSnapshot` union shape). */
import type { SemioSubsetSnapshot } from "./📸️snapshot/🟦️component";

export interface SemioArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ subset: SemioSubsetSnapshot;
}
