/** 🧬️ SemioVideoArtifact schema — full artifact state, mirrors `SemioVideoSnapshot` field for
 * field (see `📸️snapshot/🟦️component.ts`). */
export type { SemioVideoStream, SemioVideoSample, SemioVideoStreamKind, SemioRational } from "./📸️snapshot/🟦️component.ts";
import type { SemioVideoStream } from "./📸️snapshot/🟦️component.ts";

export interface SemioVideoArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ streams: SemioVideoStream[];
}
