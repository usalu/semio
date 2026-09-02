/** 🧬️ SemioAnimationArtifact schema — full artifact state, mirrors `SemioAnimationSnapshot` field
 * for field (see `📸️snapshot/🟦️.ts` for the nested `AnimTimeline` shape). */
import type { AnimTimeline } from "./📸️snapshot/🟦️";

export interface SemioAnimationArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ timelines: AnimTimeline[];
}
