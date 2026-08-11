/** 🧬️ PdfArtifact (1.4) schema — full artifact state, same fields as `PdfSnapshot`. */
import type { PageDoc } from './📸️snapshot/🟦️component.ts';

export interface PdfArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ page: PageDoc;
}
