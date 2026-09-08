/** 🧬️ PdfArtifact (1.4) schema — full artifact state, same fields as `PdfSnapshot`. */
import type { PageDoc } from './📸️snapshot/🟦️.ts';

export interface PdfArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ pages: PageDoc[];
}
