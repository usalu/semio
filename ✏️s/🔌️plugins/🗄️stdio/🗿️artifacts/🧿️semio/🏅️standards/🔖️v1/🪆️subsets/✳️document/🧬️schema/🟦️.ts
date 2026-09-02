/** 🧬️ SemioDocumentArtifact — full artifact state, mirrors `SemioDocumentSnapshot` field for
 * field (see `📸️snapshot/🟦️.ts` for the real per-field shapes). */
import type { DocBlock, DocImage, DocStyle } from "./📸️snapshot/🟦️component";

export interface SemioDocumentArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ styles: DocStyle[];
  /** @state artifact */ images: DocImage[];
  /** @state artifact */ blocks: DocBlock[];
}
