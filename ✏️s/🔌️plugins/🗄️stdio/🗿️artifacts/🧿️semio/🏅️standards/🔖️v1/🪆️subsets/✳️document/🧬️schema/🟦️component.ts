/** 🧬️ SemioDocumentArtifact — full artifact state, mirrors `SemioDocumentSnapshot` field for
 * field (see `📸️snapshot/🟦️component.ts` for the real per-field shapes). */
import type { DocBlock, DocImage, DocStyle } from "./📸️snapshot/🟦️component";

export interface SemioDocumentArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ styles: DocStyle[];
  /** @state persistent */ images: DocImage[];
  /** @state persistent */ blocks: DocBlock[];
}
