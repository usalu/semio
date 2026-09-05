/** 🧬️ SemioVideoArtifact schema — full artifact state, mirrors `SemioVideoSnapshot` field for
 * field (see `📸️snapshot/🟦️.ts`). */
export type { SemioVideoStream, SemioVideoSample, SemioVideoStreamKind, SemioRational } from "./📸️snapshot/🟦️.ts";
import type { SemioVideoStream } from "./📸️snapshot/🟦️.ts";

export interface SemioVideoArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ streams: SemioVideoStream[];
}
